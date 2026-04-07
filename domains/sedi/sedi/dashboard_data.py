"""SEDI Dashboard Data Provider.

Runs the signal receiver in a background thread and collects results
for the dashboard to display. Thread-safe via locks.
"""
import json
import math
import os
import threading
import time
from collections import deque
from typing import Dict, List, Optional

from .constants import (
    N, SIGMA, PHI, TAU, SOPFR, OMEGA,
    FOCAL, DELTA_PLUS, DELTA_MINUS, EINSTEIN_THETA,
    GOLDEN_WIDTH, GOLDEN_CENTER,
    ALERT_RED, ALERT_ORANGE, ALERT_YELLOW,
)

# --- Shared state (written by collector thread, read by HTTP thread) ---

_lock = threading.Lock()

_state: Dict = {
    "status": "INITIALIZING",          # SIGNAL / WEAK / NOISE / OFFLINE
    "signal_strength": 0.0,
    "n_anomalies": 0,
    "last_update": None,
    "source": "quantum-rng",
    "batches_processed": 0,
    "uptime_start": time.time(),
}

# Rolling history of (timestamp, signal_strength, verdict) — last 200 entries
_timeline: deque = deque(maxlen=200)

# Alert counts keyed by grade
_alert_counts: Dict[str, int] = {"RED": 0, "ORANGE": 0, "YELLOW": 0}

# Recent alerts (last 50)
_recent_alerts: deque = deque(maxlen=50)

# Active sources
_active_sources: List[str] = []

# Background thread handle
_collector_thread: Optional[threading.Thread] = None
_stop_event = threading.Event()


# --- Public API ---

def get_snapshot() -> Dict:
    """Return a JSON-serialisable snapshot of current state (thread-safe)."""
    with _lock:
        alerts = list(_recent_alerts)
        timeline = list(_timeline)
        counts = dict(_alert_counts)
        state = dict(_state)
        sources = list(_active_sources)

    uptime = int(time.time() - state["uptime_start"])

    return {
        "status": state["status"],
        "signal_strength": state["signal_strength"],
        "n_anomalies": state["n_anomalies"],
        "last_update": state["last_update"],
        "source": state["source"],
        "batches_processed": state["batches_processed"],
        "uptime_seconds": uptime,
        "alert_counts": counts,
        "active_sources": sources,
        "timeline": timeline[-60:],   # last 60 ticks for chart
        "recent_alerts": alerts,
        "constants": _constants_dict(),
    }


def load_alerts_from_file(path: str = "sedi_alerts.jsonl") -> List[Dict]:
    """Read persisted alerts from disk (newest-first, up to 50)."""
    if not os.path.exists(path):
        return []
    alerts = []
    try:
        with open(path) as f:
            for line in f:
                line = line.strip()
                if line:
                    try:
                        alerts.append(json.loads(line))
                    except json.JSONDecodeError:
                        pass
    except OSError:
        pass
    return list(reversed(alerts[-50:]))


def start_collector(source: str = "quantum-rng",
                    sensitivity: float = 3.0,
                    interval: float = 5.0,
                    batch: int = 1024,
                    log_file: str = "sedi_alerts.jsonl") -> None:
    """Start the background receiver thread (idempotent)."""
    global _collector_thread
    if _collector_thread and _collector_thread.is_alive():
        return  # already running

    _stop_event.clear()
    with _lock:
        _state["source"] = source
        _active_sources.clear()
        _active_sources.append(source)

    _collector_thread = threading.Thread(
        target=_collector_loop,
        args=(source, sensitivity, interval, batch, log_file),
        daemon=True,
        name="sedi-collector",
    )
    _collector_thread.start()


def stop_collector() -> None:
    """Signal the collector to stop."""
    _stop_event.set()


# --- Internal collector loop ---

def _collector_loop(source, sensitivity, interval, batch, log_file):
    """Main collection loop — runs in a daemon thread."""
    import numpy as np

    with _lock:
        _state["status"] = "STARTING"

    try:
        from .receiver import SignalReceiver
        rx = SignalReceiver(sensitivity=sensitivity, ph_enabled=False)

        if source == "quantum-rng":
            _run_quantum_loop(rx, sensitivity, interval, batch, log_file)
        elif source in ("test-signal", "test-noise"):
            _run_test_loop(rx, source, interval, log_file)
        else:
            # Fallback: synthetic noise loop so dashboard stays alive
            _run_synthetic_loop(rx, interval, log_file)

    except Exception as exc:
        with _lock:
            _state["status"] = "OFFLINE"
        # Keep the dashboard alive even if receiver crashes
        _synthetic_fallback(interval)


def _run_quantum_loop(rx, sensitivity, interval, batch, log_file):
    import numpy as np
    try:
        from .sources.quantum_rng import stream
    except ImportError:
        _synthetic_fallback(interval)
        return

    calibrated = False
    gen = stream(interval=interval, batch=batch)

    with _lock:
        _state["status"] = "CALIBRATING"

    for chunk in gen:
        if _stop_event.is_set():
            break
        data = np.array(chunk["data"], dtype=float)
        if not calibrated:
            rx.calibrate(data)
            calibrated = True
            with _lock:
                _state["status"] = "NOISE"
            continue
        _process_batch(rx, data, "quantum-rng", log_file)

    with _lock:
        _state["status"] = "OFFLINE"


def _run_test_loop(rx, source, interval, log_file):
    import numpy as np

    noise = np.random.randn(1024)
    rx.calibrate(noise)
    with _lock:
        _state["status"] = "NOISE"

    while not _stop_event.is_set():
        if source == "test-signal":
            t = np.linspace(0, 10, 1024)
            data = np.random.randn(1024) + 0.8 * np.sin(2 * math.pi * 6 * t)
        else:
            data = np.random.randn(1024)
        _process_batch(rx, data, source, log_file)
        _stop_event.wait(interval)

    with _lock:
        _state["status"] = "OFFLINE"


def _run_synthetic_loop(rx, interval, log_file):
    import numpy as np

    noise = np.random.randn(1024)
    rx.calibrate(noise)
    with _lock:
        _state["status"] = "NOISE"

    while not _stop_event.is_set():
        data = np.random.randn(1024)
        _process_batch(rx, data, "synthetic", log_file)
        _stop_event.wait(interval)

    with _lock:
        _state["status"] = "OFFLINE"


def _synthetic_fallback(interval):
    """Minimal loop that keeps the timeline ticking even with no real data."""
    import numpy as np
    while not _stop_event.is_set():
        with _lock:
            ts = time.time()
            _timeline.append({"ts": ts, "strength": 0.0, "verdict": "NOISE"})
            _state["last_update"] = ts
        _stop_event.wait(interval)


def _process_batch(rx, data, source_name, log_file):
    """Run receiver on one batch and update shared state."""
    result = rx.receive(data)
    verdict = result["verdict"]
    strength = result["signal_strength"]
    anomalies = result["anomalies"]
    ts = time.time()

    # Build alert objects for significant anomalies
    new_alerts = []
    for a in anomalies:
        z = a.get("z_score", 0.0)
        if z >= ALERT_YELLOW:
            grade = "RED" if z >= ALERT_RED else ("ORANGE" if z >= ALERT_ORANGE else "YELLOW")
            alert = {
                "timestamp": ts,
                "source": source_name,
                "type": a.get("type", "unknown"),
                "pattern": a.get("test", ""),
                "detail": a.get("detail", ""),
                "z_score": z,
                "grade": grade,
            }
            new_alerts.append(alert)
            # Persist to disk
            try:
                with open(log_file, "a") as f:
                    f.write(json.dumps(alert) + "\n")
            except OSError:
                pass

    with _lock:
        _state["status"] = verdict
        _state["signal_strength"] = strength
        _state["n_anomalies"] = len(anomalies)
        _state["last_update"] = ts
        _state["batches_processed"] += 1

        _timeline.append({"ts": ts, "strength": round(strength, 3), "verdict": verdict})

        for alert in new_alerts:
            _recent_alerts.appendleft(alert)
            grade = alert["grade"]
            if grade in _alert_counts:
                _alert_counts[grade] += 1


# --- Constants dict ---

def _constants_dict() -> Dict:
    return {
        "N": N,
        "sigma": SIGMA,
        "phi": PHI,
        "tau": TAU,
        "sopfr": SOPFR,
        "omega": OMEGA,
        "focal": round(FOCAL, 8),
        "delta_plus": round(DELTA_PLUS, 8),
        "delta_minus": round(DELTA_MINUS, 8),
        "einstein_theta": round(EINSTEIN_THETA, 8),
        "golden_width": round(GOLDEN_WIDTH, 8),
        "golden_center": round(GOLDEN_CENTER, 8),
        "alert_red": ALERT_RED,
        "alert_orange": ALERT_ORANGE,
        "alert_yellow": ALERT_YELLOW,
    }
