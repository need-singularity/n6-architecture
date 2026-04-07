"""Anomaly detector: combines R-filter results into alerts."""
import time
import json
from .constants import ALERT_RED, ALERT_ORANGE, ALERT_YELLOW


def grade_anomaly(z_score):
    """Grade an anomaly by Z-score."""
    if z_score >= ALERT_RED:
        return 'RED', '🔴'
    elif z_score >= ALERT_ORANGE:
        return 'ORANGE', '🟠'
    elif z_score >= ALERT_YELLOW:
        return 'YELLOW', '🟡'
    return 'NORMAL', '⚪'


def analyze(filter_result, source_name='unknown'):
    """Analyze R-filter output and generate alerts."""
    alerts = []
    timestamp = time.time()

    # Check ratio hits
    for hit in filter_result.get('ratio_hits', []):
        z = hit['z_score']
        grade, emoji = grade_anomaly(z)
        if grade != 'NORMAL':
            alerts.append({
                'timestamp': timestamp,
                'source': source_name,
                'type': 'ratio',
                'pattern': hit['pattern'],
                'target': hit['target'],
                'z_score': z,
                'grade': grade,
                'emoji': emoji,
                'count': hit['count'],
                'expected': hit['expected'],
            })

    # Check spectral peaks
    for name, peak in filter_result.get('spectral_peaks', {}).items():
        snr = peak['snr']
        z = (snr - 1) * 2  # rough Z conversion
        grade, emoji = grade_anomaly(z)
        if grade != 'NORMAL':
            alerts.append({
                'timestamp': timestamp,
                'source': source_name,
                'type': 'spectral',
                'pattern': name,
                'snr': snr,
                'z_score': z,
                'grade': grade,
                'emoji': emoji,
            })

    return sorted(alerts, key=lambda a: -a['z_score'])


def format_alert(alert):
    """Format an alert for display."""
    ts = time.strftime('%H:%M:%S', time.localtime(alert['timestamp']))
    return (
        f"{alert['emoji']} [{ts}] {alert['grade']} "
        f"Z={alert['z_score']:.1f} | "
        f"{alert['source']}/{alert['type']}: {alert['pattern']} "
        f"(target={alert.get('target', alert.get('snr', '?'))})"
    )


def log_alert(alert, logfile='sedi_alerts.jsonl'):
    """Append alert to JSONL log file."""
    with open(logfile, 'a') as f:
        f.write(json.dumps(alert) + '\n')
