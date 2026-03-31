#!/bin/bash
# ═══════════════════════════════════════════════════════════════════
# BitNet b1.58 2B4T — Mac (Apple Silicon) Setup Script
# ═══════════════════════════════════════════════════════════════════
#
# Prerequisites (install BEFORE running this script):
#   - macOS with Apple Silicon (M1/M2/M3/M4)
#   - Python 3.9+
#   - cmake >= 3.22:
#       Option A: brew install cmake  (if you have Homebrew)
#       Option B: Download from https://cmake.org/download/
#       Option C: /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)" && brew install cmake
#   - clang (Xcode Command Line Tools — already installed on most Macs)
#   - conda or python3 venv (script handles both, venv is fine)
#
# Model size estimate:
#   2B params * 1.58 bits = ~395 MB (ternary weights)
#   GGUF i2_s format: ~500-800 MB on disk
#   Runtime memory: ~1-2 GB — fits easily in 24GB unified memory
#
# Usage:
#   chmod +x experiments/setup_bitnet.sh
#   ./experiments/setup_bitnet.sh
#
# ═══════════════════════════════════════════════════════════════════

set -euo pipefail

BITNET_DIR="$HOME/Dev/BitNet"
MODEL_NAME="microsoft/BitNet-b1.58-2B-4T-gguf"
MODEL_DIR="models/BitNet-b1.58-2B-4T"
QUANT="i2_s"

echo "═══════════════════════════════════════════════════"
echo "  BitNet b1.58 2B4T Setup — Apple Silicon Mac"
echo "═══════════════════════════════════════════════════"
echo ""

# ── Step 0: Check prerequisites ──
echo "[0/5] Checking prerequisites..."

if ! command -v cmake &>/dev/null; then
    echo "  ERROR: cmake not found. Install with: brew install cmake"
    echo "  (need cmake >= 3.22)"
    exit 1
fi

if ! command -v clang &>/dev/null; then
    echo "  ERROR: clang not found. Install Xcode Command Line Tools:"
    echo "  xcode-select --install"
    exit 1
fi

if ! command -v python3 &>/dev/null; then
    echo "  ERROR: python3 not found."
    exit 1
fi

CMAKE_VER=$(cmake --version | head -1 | grep -oE '[0-9]+\.[0-9]+')
echo "  cmake: $(cmake --version | head -1)"
echo "  clang: $(clang --version | head -1)"
echo "  python: $(python3 --version)"
echo "  OK"
echo ""

# ── Step 1: Clone BitNet repo ──
echo "[1/5] Cloning microsoft/BitNet..."
if [ -d "$BITNET_DIR" ]; then
    echo "  Already exists at $BITNET_DIR — pulling latest..."
    cd "$BITNET_DIR" && git pull --rebase 2>/dev/null || true
else
    git clone --recursive https://github.com/microsoft/BitNet.git "$BITNET_DIR"
fi
cd "$BITNET_DIR"
echo "  OK"
echo ""

# ── Step 2: Set up Python environment ──
echo "[2/5] Setting up Python environment..."

# Try conda first, fall back to venv
if command -v conda &>/dev/null; then
    echo "  Using conda..."
    if conda env list | grep -q "bitnet-cpp"; then
        echo "  Environment 'bitnet-cpp' already exists — activating..."
    else
        conda create -n bitnet-cpp python=3.9 -y
    fi
    # Source conda for this script
    eval "$(conda shell.bash hook)"
    conda activate bitnet-cpp
else
    echo "  conda not found — using venv..."
    if [ ! -d ".venv" ]; then
        python3 -m venv .venv
    fi
    source .venv/bin/activate
fi

pip install -r requirements.txt 2>&1 | tail -3
echo "  OK"
echo ""

# ── Step 3: Download model ──
echo "[3/5] Downloading $MODEL_NAME..."
echo "  (ternary 2B model ~ 500-800 MB)"

if [ -d "$MODEL_DIR" ] && ls "$MODEL_DIR"/*.gguf &>/dev/null 2>&1; then
    echo "  Model already downloaded — skipping."
else
    pip install huggingface-hub 2>/dev/null
    huggingface-cli download "$MODEL_NAME" --local-dir "$MODEL_DIR"
fi

echo "  Model files:"
ls -lh "$MODEL_DIR"/ 2>/dev/null | grep -E '\.(gguf|json|txt)' || echo "  (listing model dir...)"
echo ""

# ── Step 4: Build with quantization ──
echo "[4/5] Building BitNet.cpp with $QUANT quantization..."
python setup_env.py -md "$MODEL_DIR" -q "$QUANT" 2>&1 | tail -5
echo "  OK"
echo ""

# ── Step 5: Quick benchmark ──
echo "[5/5] Running quick benchmark..."
echo ""

GGUF_FILE="$MODEL_DIR/ggml-model-${QUANT}.gguf"
if [ ! -f "$GGUF_FILE" ]; then
    # Try to find any gguf file
    GGUF_FILE=$(find "$MODEL_DIR" -name "*.gguf" | head -1)
fi

if [ -n "$GGUF_FILE" ] && [ -f "$GGUF_FILE" ]; then
    echo "  Model file: $GGUF_FILE"
    echo "  Size: $(ls -lh "$GGUF_FILE" | awk '{print $5}')"
    echo ""

    # Quick inference test
    echo "── Quick Inference Test ──"
    python run_inference.py \
        -m "$GGUF_FILE" \
        -p "What is 6 times 4?" \
        -t 4 \
        -n 64 2>&1 || echo "  (inference test failed — see above)"
    echo ""

    # Benchmark: 200 tokens, 256 prompt tokens, 4 threads
    echo "── Benchmark (200 tokens, 4 threads) ──"
    python utils/e2e_benchmark.py \
        -m "$GGUF_FILE" \
        -n 200 \
        -p 256 \
        -t 4 2>&1 || echo "  (benchmark failed — see above)"
else
    echo "  ERROR: No .gguf file found in $MODEL_DIR"
    echo "  You may need to run setup_env.py manually."
fi

echo ""
echo "═══════════════════════════════════════════════════"
echo "  Setup Complete!"
echo "═══════════════════════════════════════════════════"
echo ""
echo "  BitNet dir:  $BITNET_DIR"
echo "  Model dir:   $BITNET_DIR/$MODEL_DIR"
echo "  GGUF file:   $GGUF_FILE"
echo ""
echo "  To run inference:"
echo "    cd $BITNET_DIR"
echo "    python run_inference.py -m $GGUF_FILE -p 'Your prompt' -cnv"
echo ""
echo "  To run n=6 verification experiment:"
echo "    python3 ~/Dev/n6-architecture/experiments/experiment_bitnet_n6.py"
echo ""
