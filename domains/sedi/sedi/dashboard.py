"""SEDI Web Dashboard — single-file, stdlib-only HTTP server.

Usage:
    sedi dashboard [--port 8080] [--source quantum-rng] [--log sedi_alerts.jsonl]
    python -m sedi.dashboard --port 8080

Serves on http://localhost:PORT
  GET /           → HTML dashboard (auto-refreshes every 5 s)
  GET /api/status → JSON snapshot of current state
"""
import argparse
import json
import os
import sys
import time
import threading
from http.server import BaseHTTPRequestHandler, HTTPServer
from urllib.parse import urlparse

# --------------------------------------------------------------------------- #
#  HTML template (dark radar theme)                                            #
# --------------------------------------------------------------------------- #

_HTML = r"""<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>SEDI — Signal Dashboard</title>
<style>
  :root {
    --bg: #060c0a;
    --bg2: #0a1410;
    --bg3: #0f1f1a;
    --border: #1a3a30;
    --green: #00ff88;
    --green-dim: #00aa55;
    --green-faint: #003322;
    --amber: #ffaa00;
    --amber-dim: #aa6600;
    --red: #ff3333;
    --red-dim: #881111;
    --orange: #ff6600;
    --yellow: #ffdd00;
    --noise: #336655;
    --text: #a0c8b0;
    --text-dim: #4a7a5a;
    --mono: 'Courier New', 'Lucida Console', monospace;
    --scan-speed: 4s;
  }

  * { box-sizing: border-box; margin: 0; padding: 0; }

  body {
    background: var(--bg);
    color: var(--text);
    font-family: var(--mono);
    font-size: 13px;
    min-height: 100vh;
    padding: 12px;
    overflow-x: hidden;
  }

  /* Scanline overlay */
  body::after {
    content: '';
    position: fixed;
    inset: 0;
    background: repeating-linear-gradient(
      0deg,
      transparent,
      transparent 2px,
      rgba(0,0,0,0.06) 2px,
      rgba(0,0,0,0.06) 4px
    );
    pointer-events: none;
    z-index: 9999;
  }

  h1 {
    color: var(--green);
    font-size: 22px;
    letter-spacing: 4px;
    text-transform: uppercase;
    text-shadow: 0 0 16px var(--green);
    margin-bottom: 4px;
  }

  .subtitle {
    color: var(--text-dim);
    font-size: 11px;
    letter-spacing: 2px;
    margin-bottom: 16px;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
    gap: 12px;
    margin-bottom: 12px;
  }

  .panel {
    background: var(--bg2);
    border: 1px solid var(--border);
    border-radius: 4px;
    padding: 12px 14px;
    position: relative;
    overflow: hidden;
  }

  .panel::before {
    content: '';
    position: absolute;
    top: 0; left: 0; right: 0;
    height: 1px;
    background: linear-gradient(90deg, transparent, var(--green-dim), transparent);
    opacity: 0.5;
  }

  .panel-title {
    color: var(--green-dim);
    font-size: 10px;
    letter-spacing: 3px;
    text-transform: uppercase;
    margin-bottom: 10px;
    border-bottom: 1px solid var(--green-faint);
    padding-bottom: 6px;
  }

  /* Status badge */
  .status-badge {
    display: inline-block;
    padding: 6px 18px;
    border-radius: 3px;
    font-size: 20px;
    font-weight: bold;
    letter-spacing: 3px;
    text-align: center;
    width: 100%;
    margin: 8px 0;
    text-shadow: 0 0 12px currentColor;
    border: 1px solid currentColor;
    transition: all 0.4s;
  }

  .status-SIGNAL  { color: var(--red);    background: rgba(255,51,51,0.1); }
  .status-WEAK    { color: var(--amber);  background: rgba(255,170,0,0.1); }
  .status-NOISE   { color: var(--green);  background: rgba(0,255,136,0.06); }
  .status-OFFLINE { color: var(--text-dim); background: rgba(0,0,0,0.3); }
  .status-STARTING     { color: var(--text-dim); background: rgba(0,0,0,0.3); }
  .status-CALIBRATING  { color: var(--yellow);   background: rgba(255,221,0,0.06); }
  .status-INITIALIZING { color: var(--text-dim); background: rgba(0,0,0,0.2); }

  .strength-bar-wrap {
    background: var(--bg3);
    border: 1px solid var(--border);
    border-radius: 2px;
    height: 8px;
    margin: 8px 0;
    overflow: hidden;
  }

  .strength-bar {
    height: 100%;
    border-radius: 2px;
    transition: width 0.5s, background 0.5s;
  }

  /* Alert count badges */
  .grade-counts {
    display: flex;
    gap: 10px;
    flex-wrap: wrap;
    margin-top: 8px;
  }

  .grade-pill {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 8px 16px;
    border-radius: 3px;
    border: 1px solid;
    min-width: 64px;
    flex: 1;
  }

  .grade-pill .count {
    font-size: 28px;
    font-weight: bold;
    line-height: 1;
    text-shadow: 0 0 10px currentColor;
  }

  .grade-pill .label {
    font-size: 9px;
    letter-spacing: 2px;
    margin-top: 4px;
  }

  .pill-RED    { color: var(--red);    border-color: var(--red-dim);    background: rgba(255,51,51,0.08); }
  .pill-ORANGE { color: var(--orange); border-color: var(--amber-dim);  background: rgba(255,102,0,0.08); }
  .pill-YELLOW { color: var(--yellow); border-color: var(--amber-dim);  background: rgba(255,221,0,0.06); }

  /* Timeline chart */
  .chart-wrap {
    background: var(--bg3);
    border: 1px solid var(--border);
    border-radius: 2px;
    padding: 4px 6px;
    overflow: hidden;
    height: 80px;
    position: relative;
  }

  .chart-canvas {
    width: 100%;
    height: 100%;
    display: block;
  }

  /* Constants table */
  .const-table {
    width: 100%;
    border-collapse: collapse;
  }

  .const-table td {
    padding: 3px 4px;
    border-bottom: 1px solid var(--green-faint);
    font-size: 12px;
  }

  .const-table td:first-child {
    color: var(--green-dim);
    width: 54%;
  }

  .const-table td:last-child {
    color: var(--amber);
    text-align: right;
  }

  /* Sources list */
  .source-item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 5px 0;
    border-bottom: 1px solid var(--green-faint);
    font-size: 12px;
  }

  .dot {
    width: 8px; height: 8px;
    border-radius: 50%;
    flex-shrink: 0;
    animation: pulse 2s infinite;
  }

  .dot-active { background: var(--green); box-shadow: 0 0 6px var(--green); }
  .dot-idle   { background: var(--text-dim); animation: none; }

  @keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.3; }
  }

  /* Alerts table */
  .alerts-wrap {
    max-height: 360px;
    overflow-y: auto;
    scrollbar-width: thin;
    scrollbar-color: var(--green-faint) transparent;
  }

  .alerts-wrap::-webkit-scrollbar { width: 4px; }
  .alerts-wrap::-webkit-scrollbar-thumb { background: var(--green-faint); }

  .alert-row {
    display: grid;
    grid-template-columns: 75px 64px 52px 1fr;
    gap: 8px;
    padding: 5px 4px;
    border-bottom: 1px solid var(--green-faint);
    font-size: 11px;
    align-items: start;
    transition: background 0.3s;
  }

  .alert-row:hover { background: var(--bg3); }
  .alert-row:first-child { animation: flash-in 0.5s; }

  @keyframes flash-in {
    from { background: var(--green-faint); }
    to   { background: transparent; }
  }

  .alert-time  { color: var(--text-dim); }
  .alert-grade-RED    { color: var(--red); }
  .alert-grade-ORANGE { color: var(--orange); }
  .alert-grade-YELLOW { color: var(--yellow); }
  .alert-z     { color: var(--amber); }
  .alert-detail { color: var(--text); word-break: break-word; }

  .no-alerts {
    color: var(--text-dim);
    font-size: 11px;
    padding: 16px 0;
    text-align: center;
  }

  /* Meta row */
  .meta-row {
    display: flex;
    flex-wrap: wrap;
    gap: 16px;
    font-size: 11px;
    color: var(--text-dim);
    margin-bottom: 12px;
    padding: 8px 10px;
    border: 1px solid var(--green-faint);
    border-radius: 3px;
    background: var(--bg2);
  }

  .meta-item span { color: var(--green-dim); }

  /* Radar sweep on status panel */
  .radar-ring {
    position: absolute;
    bottom: -30px; right: -30px;
    width: 120px; height: 120px;
    border-radius: 50%;
    border: 1px solid var(--green-faint);
    pointer-events: none;
  }

  .radar-ring::after {
    content: '';
    position: absolute;
    inset: 12px;
    border-radius: 50%;
    border: 1px solid var(--green-faint);
    opacity: 0.5;
  }

  /* Refresh indicator */
  #refresh-indicator {
    position: fixed;
    top: 10px; right: 14px;
    font-size: 10px;
    color: var(--text-dim);
    letter-spacing: 1px;
  }

  #refresh-dot {
    display: inline-block;
    width: 6px; height: 6px;
    border-radius: 50%;
    background: var(--green-dim);
    margin-right: 5px;
    vertical-align: middle;
    animation: blink 1s infinite;
  }

  @keyframes blink {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.1; }
  }

  .full-width { grid-column: 1 / -1; }
</style>
</head>
<body>

<div id="refresh-indicator">
  <span id="refresh-dot"></span>LIVE
</div>

<h1>&#9673; SEDI</h1>
<p class="subtitle">Search for Extra-Dimensional Intelligence &mdash; n=6 Signal Receiver</p>

<div class="meta-row" id="meta-row">
  <div class="meta-item"><span>UPTIME</span> <span id="m-uptime">--</span></div>
  <div class="meta-item"><span>BATCHES</span> <span id="m-batches">0</span></div>
  <div class="meta-item"><span>SOURCE</span> <span id="m-source">--</span></div>
  <div class="meta-item"><span>LAST UPDATE</span> <span id="m-update">--</span></div>
  <div class="meta-item"><span>TOTAL ALERTS</span> <span id="m-total">0</span></div>
</div>

<div class="grid">

  <!-- Status panel -->
  <div class="panel">
    <div class="panel-title">&#9632; Receiver Status</div>
    <div class="status-badge status-INITIALIZING" id="status-badge">INITIALIZING</div>
    <div class="strength-bar-wrap">
      <div class="strength-bar" id="strength-bar" style="width:0%;background:var(--green-dim)"></div>
    </div>
    <div style="font-size:11px; color:var(--text-dim); margin-top:4px;">
      Signal strength: <span id="strength-val" style="color:var(--amber)">0.0</span>&sigma;
      &nbsp;&nbsp;Anomalies: <span id="anomaly-count" style="color:var(--amber)">0</span>
    </div>
    <div class="radar-ring"></div>
  </div>

  <!-- Alert counts panel -->
  <div class="panel">
    <div class="panel-title">&#9632; Alert Grades</div>
    <div class="grade-counts">
      <div class="grade-pill pill-RED">
        <span class="count" id="cnt-RED">0</span>
        <span class="label">RED &gt;5&sigma;</span>
      </div>
      <div class="grade-pill pill-ORANGE">
        <span class="count" id="cnt-ORANGE">0</span>
        <span class="label">ORANGE &gt;3&sigma;</span>
      </div>
      <div class="grade-pill pill-YELLOW">
        <span class="count" id="cnt-YELLOW">0</span>
        <span class="label">YELLOW &gt;2&sigma;</span>
      </div>
    </div>
  </div>

  <!-- Active sources -->
  <div class="panel">
    <div class="panel-title">&#9632; Data Sources</div>
    <div id="sources-list"></div>
  </div>

  <!-- n=6 tuning constants -->
  <div class="panel">
    <div class="panel-title">&#9632; n=6 Tuning Constants</div>
    <table class="const-table" id="const-table"></table>
  </div>

  <!-- Signal timeline — full width -->
  <div class="panel full-width">
    <div class="panel-title">&#9632; Signal Strength Timeline</div>
    <div class="chart-wrap">
      <canvas class="chart-canvas" id="timeline-canvas"></canvas>
    </div>
  </div>

  <!-- Alert log — full width -->
  <div class="panel full-width">
    <div class="panel-title">&#9632; Alert Log (last 50)</div>
    <div class="alerts-wrap">
      <div id="alert-header" style="display:grid;grid-template-columns:75px 64px 52px 1fr;gap:8px;padding:4px;font-size:10px;color:var(--text-dim);letter-spacing:2px;border-bottom:1px solid var(--border);">
        <span>TIME</span><span>GRADE</span><span>Z</span><span>DETAIL</span>
      </div>
      <div id="alerts-body">
        <div class="no-alerts">Awaiting signals...</div>
      </div>
    </div>
  </div>

</div>

<script>
// ======================================================================
// Data fetching & state
// ======================================================================
let _data = null;
let _timeline = [];
const REFRESH_MS = 5000;

async function fetchStatus() {
  try {
    const resp = await fetch('/api/status');
    if (!resp.ok) return;
    _data = await resp.json();
    renderAll(_data);
  } catch (e) {
    // Silently ignore — we'll retry
  }
}

function startRefresh() {
  fetchStatus();
  setInterval(fetchStatus, REFRESH_MS);
}

// ======================================================================
// Render helpers
// ======================================================================

function renderAll(d) {
  renderMeta(d);
  renderStatus(d);
  renderAlertCounts(d.alert_counts);
  renderSources(d.active_sources, d.source);
  renderConstants(d.constants);
  renderTimeline(d.timeline);
  renderAlerts(d.recent_alerts);
}

function fmt_ts(unix) {
  if (!unix) return '--';
  const d = new Date(unix * 1000);
  return d.toLocaleTimeString('en-US', {hour12:false});
}

function fmt_uptime(s) {
  const h = Math.floor(s/3600);
  const m = Math.floor((s%3600)/60);
  const sec = s % 60;
  return `${String(h).padStart(2,'0')}:${String(m).padStart(2,'0')}:${String(sec).padStart(2,'0')}`;
}

function renderMeta(d) {
  document.getElementById('m-uptime').textContent   = fmt_uptime(d.uptime_seconds||0);
  document.getElementById('m-batches').textContent  = d.batches_processed||0;
  document.getElementById('m-source').textContent   = d.source||'--';
  document.getElementById('m-update').textContent   = fmt_ts(d.last_update);
  const total = (d.alert_counts.RED||0) + (d.alert_counts.ORANGE||0) + (d.alert_counts.YELLOW||0);
  document.getElementById('m-total').textContent = total;
}

function renderStatus(d) {
  const badge = document.getElementById('status-badge');
  const prev = badge.className.replace('status-badge ','').replace('status-','');
  const status = d.status || 'OFFLINE';

  badge.className = `status-badge status-${status}`;
  badge.textContent = status;

  // Signal strength bar (cap at 10 sigma)
  const strength = d.signal_strength || 0;
  const pct = Math.min(100, (strength / 10) * 100);
  const bar = document.getElementById('strength-bar');
  bar.style.width = pct + '%';
  if (status === 'SIGNAL') {
    bar.style.background = 'var(--red)';
    bar.style.boxShadow = '0 0 8px var(--red)';
  } else if (status === 'WEAK') {
    bar.style.background = 'var(--amber)';
    bar.style.boxShadow = '0 0 8px var(--amber)';
  } else {
    bar.style.background = 'var(--green-dim)';
    bar.style.boxShadow = 'none';
  }

  document.getElementById('strength-val').textContent = strength.toFixed(2);
  document.getElementById('anomaly-count').textContent = d.n_anomalies || 0;
}

function renderAlertCounts(counts) {
  document.getElementById('cnt-RED').textContent    = counts.RED    || 0;
  document.getElementById('cnt-ORANGE').textContent = counts.ORANGE || 0;
  document.getElementById('cnt-YELLOW').textContent = counts.YELLOW || 0;
}

const ALL_SOURCES = ['quantum-rng', 'ligo', 'test-signal', 'test-noise', 'synthetic'];

function renderSources(active, current) {
  const el = document.getElementById('sources-list');
  const show = new Set([...(active||[]), current].filter(Boolean));
  // Always show all known sources, mark active ones
  const items = ALL_SOURCES.map(src => {
    const isActive = show.has(src);
    return `<div class="source-item">
      <div class="dot ${isActive ? 'dot-active' : 'dot-idle'}"></div>
      <span style="color:${isActive ? 'var(--green)' : 'var(--text-dim)'}">${src}</span>
      ${isActive ? '<span style="color:var(--green-dim);font-size:10px;margin-left:auto">ACTIVE</span>' : ''}
    </div>`;
  });
  el.innerHTML = items.join('');
}

const CONST_LABELS = {
  N: 'n (perfect number)',
  sigma: 'σ(n) — sum of divisors',
  phi: 'φ(n) — Euler totient',
  tau: 'τ(n) — divisor count',
  sopfr: 'sopfr(n) — prime factor sum',
  omega: 'ω(n) — distinct primes',
  focal: 'focal = 1/(σφ)',
  delta_plus: 'δ+ = 1/n',
  delta_minus: 'δ− = 1/τ',
  einstein_theta: 'θ_E = √(3/2)',
  golden_width: 'GZ width = ln(4/3)',
  golden_center: 'GZ center = 1/e',
  alert_red: 'RED threshold (σ)',
  alert_orange: 'ORANGE threshold (σ)',
  alert_yellow: 'YELLOW threshold (σ)',
};

function renderConstants(c) {
  if (!c) return;
  const rows = Object.entries(CONST_LABELS).map(([k, label]) => {
    const val = c[k] !== undefined ? c[k] : '--';
    return `<tr><td>${label}</td><td>${val}</td></tr>`;
  });
  document.getElementById('const-table').innerHTML = rows.join('');
}

// ======================================================================
// Timeline canvas chart
// ======================================================================

function renderTimeline(points) {
  if (!points || points.length === 0) return;

  const canvas = document.getElementById('timeline-canvas');
  const ctx = canvas.getContext('2d');

  // Resize canvas to container
  const wrap = canvas.parentElement;
  canvas.width  = wrap.clientWidth  || 600;
  canvas.height = wrap.clientHeight || 76;

  const W = canvas.width;
  const H = canvas.height;
  const PAD = 4;

  ctx.clearRect(0, 0, W, H);

  // Background
  ctx.fillStyle = '#0a1410';
  ctx.fillRect(0, 0, W, H);

  // Grid lines
  ctx.strokeStyle = '#1a3a30';
  ctx.lineWidth = 1;
  for (let y = 0; y <= 4; y++) {
    const yPos = PAD + (H - 2*PAD) * (1 - y/4);
    ctx.beginPath();
    ctx.moveTo(PAD, yPos);
    ctx.lineTo(W - PAD, yPos);
    ctx.stroke();
  }

  if (points.length < 2) return;

  const maxVal = Math.max(10, ...points.map(p => p.strength || 0));
  const n = points.length;

  // Fill gradient
  ctx.beginPath();
  for (let i = 0; i < n; i++) {
    const x = PAD + (W - 2*PAD) * i / (n - 1);
    const s = points[i].strength || 0;
    const y = PAD + (H - 2*PAD) * (1 - s / maxVal);
    if (i === 0) ctx.moveTo(x, y);
    else ctx.lineTo(x, y);
  }
  ctx.lineTo(PAD + (W - 2*PAD), H - PAD);
  ctx.lineTo(PAD, H - PAD);
  ctx.closePath();

  const grad = ctx.createLinearGradient(0, PAD, 0, H - PAD);
  grad.addColorStop(0, 'rgba(0,255,136,0.30)');
  grad.addColorStop(1, 'rgba(0,255,136,0.02)');
  ctx.fillStyle = grad;
  ctx.fill();

  // Line
  ctx.beginPath();
  for (let i = 0; i < n; i++) {
    const x = PAD + (W - 2*PAD) * i / (n - 1);
    const s = points[i].strength || 0;
    const y = PAD + (H - 2*PAD) * (1 - s / maxVal);
    if (i === 0) ctx.moveTo(x, y);
    else ctx.lineTo(x, y);
  }
  ctx.strokeStyle = '#00ff88';
  ctx.lineWidth = 1.5;
  ctx.shadowColor = '#00ff88';
  ctx.shadowBlur = 4;
  ctx.stroke();
  ctx.shadowBlur = 0;

  // Threshold lines
  const thresholds = [
    { label: '5σ', val: 5, color: '#ff3333' },
    { label: '3σ', val: 3, color: '#ff6600' },
    { label: '2σ', val: 2, color: '#ffdd00' },
  ];
  ctx.setLineDash([3, 4]);
  ctx.lineWidth = 1;
  for (const t of thresholds) {
    if (t.val <= maxVal) {
      const y = PAD + (H - 2*PAD) * (1 - t.val / maxVal);
      ctx.strokeStyle = t.color;
      ctx.globalAlpha = 0.5;
      ctx.beginPath();
      ctx.moveTo(PAD, y);
      ctx.lineTo(W - PAD, y);
      ctx.stroke();
      ctx.globalAlpha = 1;
      ctx.fillStyle = t.color;
      ctx.font = '9px Courier New';
      ctx.fillText(t.label, W - PAD - 20, y - 2);
    }
  }
  ctx.setLineDash([]);

  // Dots for SIGNAL/WEAK verdicts
  for (let i = 0; i < n; i++) {
    const p = points[i];
    if (p.verdict === 'SIGNAL' || p.verdict === 'WEAK') {
      const x = PAD + (W - 2*PAD) * i / (n - 1);
      const s = p.strength || 0;
      const y = PAD + (H - 2*PAD) * (1 - s / maxVal);
      ctx.beginPath();
      ctx.arc(x, y, 3, 0, 2 * Math.PI);
      ctx.fillStyle = p.verdict === 'SIGNAL' ? '#ff3333' : '#ffaa00';
      ctx.shadowColor = ctx.fillStyle;
      ctx.shadowBlur = 6;
      ctx.fill();
      ctx.shadowBlur = 0;
    }
  }
}

// ======================================================================
// Alert log
// ======================================================================

function renderAlerts(alerts) {
  const body = document.getElementById('alerts-body');
  if (!alerts || alerts.length === 0) {
    body.innerHTML = '<div class="no-alerts">No alerts recorded yet.</div>';
    return;
  }
  const rows = alerts.map(a => {
    const gradeClass = `alert-grade-${a.grade || 'YELLOW'}`;
    const detail = escHtml(a.detail || `${a.type||''} / ${a.pattern||''}`);
    const z = (a.z_score||0).toFixed(1);
    const ts = fmt_ts(a.timestamp);
    return `<div class="alert-row">
      <span class="alert-time">${ts}</span>
      <span class="${gradeClass}">${a.grade||'?'}</span>
      <span class="alert-z">${z}σ</span>
      <span class="alert-detail">${detail}</span>
    </div>`;
  });
  body.innerHTML = rows.join('');
}

function escHtml(s) {
  return String(s)
    .replace(/&/g,'&amp;')
    .replace(/</g,'&lt;')
    .replace(/>/g,'&gt;');
}

// ======================================================================
// Boot
// ======================================================================
window.addEventListener('DOMContentLoaded', startRefresh);
window.addEventListener('resize', () => {
  if (_data) renderTimeline(_data.timeline);
});
</script>
</body>
</html>"""

# --------------------------------------------------------------------------- #
#  HTTP request handler                                                        #
# --------------------------------------------------------------------------- #

class SEDIHandler(BaseHTTPRequestHandler):
    """Minimal HTTP handler serving the dashboard HTML and /api/status JSON."""

    log_file: str = "sedi_alerts.jsonl"

    def do_GET(self):
        parsed = urlparse(self.path)
        path = parsed.path

        if path in ('/', '/index.html'):
            self._serve_html()
        elif path == '/api/status':
            self._serve_json()
        else:
            self._404()

    # ------------------------------------------------------------------ #

    def _serve_html(self):
        body = _HTML.encode('utf-8')
        self.send_response(200)
        self.send_header('Content-Type', 'text/html; charset=utf-8')
        self.send_header('Content-Length', str(len(body)))
        self.send_header('Cache-Control', 'no-store')
        self.end_headers()
        self.wfile.write(body)

    def _serve_json(self):
        from . import dashboard_data as dd
        snap = dd.get_snapshot()

        # Merge persisted alerts from disk if in-memory list is empty
        if not snap['recent_alerts']:
            snap['recent_alerts'] = dd.load_alerts_from_file(self.log_file)

        body = json.dumps(snap, default=str).encode('utf-8')
        self.send_response(200)
        self.send_header('Content-Type', 'application/json')
        self.send_header('Content-Length', str(len(body)))
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Cache-Control', 'no-store')
        self.end_headers()
        self.wfile.write(body)

    def _404(self):
        body = b'Not found'
        self.send_response(404)
        self.send_header('Content-Type', 'text/plain')
        self.send_header('Content-Length', str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    def log_message(self, fmt, *args):
        # Suppress noisy default logging; only print errors
        if args and str(args[1]) not in ('200', '304'):
            super().log_message(fmt, *args)


# --------------------------------------------------------------------------- #
#  Server launch                                                               #
# --------------------------------------------------------------------------- #

def serve(port: int = 8080,
          source: str = "quantum-rng",
          sensitivity: float = 3.0,
          interval: float = 5.0,
          batch: int = 1024,
          log_file: str = "sedi_alerts.jsonl") -> None:
    """Start the dashboard HTTP server (blocking)."""

    from . import dashboard_data as dd

    # Attach log_file to handler class
    SEDIHandler.log_file = log_file

    # Start background data collector
    dd.start_collector(
        source=source,
        sensitivity=sensitivity,
        interval=interval,
        batch=batch,
        log_file=log_file,
    )

    server = HTTPServer(('localhost', port), SEDIHandler)
    print(f"SEDI Dashboard  ->  http://localhost:{port}/")
    print(f"  API endpoint  ->  http://localhost:{port}/api/status")
    print(f"  Source        ->  {source}")
    print(f"  Log file      ->  {log_file}")
    print("  Press Ctrl+C to stop.")
    print()

    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("\nShutting down...")
        dd.stop_collector()
        server.server_close()


# --------------------------------------------------------------------------- #
#  CLI entry point (sedi dashboard)                                            #
# --------------------------------------------------------------------------- #

def cmd_dashboard(args):
    """CLI handler registered in cli.py."""
    serve(
        port=args.port,
        source=args.source,
        sensitivity=args.sensitivity,
        interval=args.interval,
        batch=args.batch,
        log_file=args.log,
    )


def _build_parser(subparsers=None):
    desc = "SEDI Web Dashboard — real-time signal monitoring"
    if subparsers is not None:
        p = subparsers.add_parser('dashboard', help=desc)
    else:
        p = argparse.ArgumentParser(prog='sedi dashboard', description=desc)

    p.add_argument('--port',        type=int,   default=8080,           help='Port to listen on (default 8080)')
    p.add_argument('--source',      default='quantum-rng',              help='Data source: quantum-rng, test-signal, test-noise')
    p.add_argument('--sensitivity', type=float, default=3.0,            help='Z-score alert threshold')
    p.add_argument('--interval',    type=float, default=5.0,            help='Poll interval seconds')
    p.add_argument('--batch',       type=int,   default=1024,           help='Batch size')
    p.add_argument('--log',         default='sedi_alerts.jsonl',        help='Alert log file path')
    return p


# Allow running as: python -m sedi.dashboard
if __name__ == '__main__':
    p = _build_parser()
    args = p.parse_args()
    serve(
        port=args.port,
        source=args.source,
        sensitivity=args.sensitivity,
        interval=args.interval,
        batch=args.batch,
        log_file=args.log,
    )
