#=============================================================================
# HEXA-EDGE SoC — Build System
#
# 도구: HEXA RTL -> Verilog 변환 후 Icarus Verilog (시뮬레이션), Yosys (합성)
# 대상: Xilinx Zynq-7020 (PYNQ-Z2)
#
# N6 Constants:
#   n=6  phi=2  tau=4  sigma=12  J2=24  sigma-tau=8  sigma-phi=10
#
# Targets:
#   make sim     — Compile and run simulation (Icarus Verilog)
#   make wave    — Open waveform viewer (GTKWave)
#   make synth   — Yosys synthesis report (area, timing estimate)
#   make lint    — Verilator lint check
#   make clean   — Remove generated files
#   make all     — sim + synth
#=============================================================================

# Directories
RTL_DIR   := rtl
SIM_DIR   := sim
CONST_DIR := constraints
BUILD_DIR := build

# RTL 소스 파일 (n=6 RTL 모듈, HEXA-FIRST)
RTL_SRCS := \
    $(RTL_DIR)/hexa_edge_top.hexa \
    $(RTL_DIR)/riscv_n6_core.hexa \
    $(RTL_DIR)/egyptian_mem_ctrl.hexa \
    $(RTL_DIR)/hexalang_decoder.hexa \
    $(RTL_DIR)/snn_izhikevich.hexa \
    $(RTL_DIR)/egyptian_moe.hexa

# 테스트벤치
TB_SRC := $(SIM_DIR)/tb_hexa_edge.hexa

# Output files
SIM_OUT   := $(BUILD_DIR)/hexa_edge_sim
VCD_FILE  := hexa_edge.vcd
SYNTH_LOG := $(BUILD_DIR)/synth_report.txt

# Tools
IVERILOG  := iverilog
VVP       := vvp
GTKWAVE   := gtkwave
YOSYS     := yosys
VERILATOR := verilator

# Icarus Verilog flags
IV_FLAGS  := -g2005-sv -Wall

#-----------------------------------------------------------------------------
# Default target
#-----------------------------------------------------------------------------
.PHONY: all sim wave synth lint clean

all: sim synth

#-----------------------------------------------------------------------------
# Build directory
#-----------------------------------------------------------------------------
$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)

#-----------------------------------------------------------------------------
# Simulation — Icarus Verilog
#-----------------------------------------------------------------------------
sim: $(BUILD_DIR) $(RTL_SRCS) $(TB_SRC)
	@echo "============================================"
	@echo "  HEXA-EDGE SoC Simulation"
	@echo "  Pipeline: sigma = 12 stages"
	@echo "  GPRs: n*sigma = 72 registers"
	@echo "  Keywords: sigma*tau+sopfr = 53"
	@echo "============================================"
	$(IVERILOG) $(IV_FLAGS) -o $(SIM_OUT) $(TB_SRC) $(RTL_SRCS)
	cd $(BUILD_DIR) && $(VVP) hexa_edge_sim
	@if [ -f $(VCD_FILE) ]; then mv $(VCD_FILE) $(BUILD_DIR)/; fi
	@echo "============================================"
	@echo "  Simulation complete"
	@echo "  VCD: $(BUILD_DIR)/$(VCD_FILE)"
	@echo "============================================"

#-----------------------------------------------------------------------------
# Waveform viewer
#-----------------------------------------------------------------------------
wave: $(BUILD_DIR)/$(VCD_FILE)
	$(GTKWAVE) $(BUILD_DIR)/$(VCD_FILE) &

#-----------------------------------------------------------------------------
# Synthesis — Yosys (area and timing estimate)
#-----------------------------------------------------------------------------
synth: $(BUILD_DIR) $(RTL_SRCS)
	@echo "============================================"
	@echo "  HEXA-EDGE Yosys Synthesis"
	@echo "  Target: Xilinx Zynq-7020 (xc7z020)"
	@echo "  Clock: 100 MHz (sigma-phi = 10 ns)"
	@echo "============================================"
	$(YOSYS) -p "\
		read_verilog $(RTL_SRCS:.hexa=.v); \
		hierarchy -top hexa_edge_top; \
		proc; opt; fsm; opt; memory; opt; \
		techmap; opt; \
		stat; \
		write_json $(BUILD_DIR)/hexa_edge_netlist.json; \
	" 2>&1 | tee $(SYNTH_LOG)
	@echo "============================================"
	@echo "  Synthesis report: $(SYNTH_LOG)"
	@echo "  Netlist: $(BUILD_DIR)/hexa_edge_netlist.json"
	@echo "============================================"

#-----------------------------------------------------------------------------
# Lint — Verilator static analysis
#-----------------------------------------------------------------------------
lint: $(RTL_SRCS)
	@echo "============================================"
	@echo "  HEXA-EDGE Lint Check (Verilator)"
	@echo "============================================"
	$(VERILATOR) --lint-only -Wall $(RTL_SRCS) --top-module hexa_edge_top 2>&1 || true
	@echo "============================================"
	@echo "  Lint complete"
	@echo "============================================"

#-----------------------------------------------------------------------------
# Clean
#-----------------------------------------------------------------------------
clean:
	rm -rf $(BUILD_DIR)
	rm -f $(VCD_FILE)
	@echo "Build artifacts cleaned"

#-----------------------------------------------------------------------------
# Info
#-----------------------------------------------------------------------------
.PHONY: info
info:
	@echo "HEXA-EDGE SoC RTL"
	@echo "  RTL modules:  $(words $(RTL_SRCS)) files (n=6 modules)"
	@echo "  Testbench:    $(TB_SRC)"
	@echo "  Constraints:  $(CONST_DIR)/zynq7020.xdc"
	@echo "  Target FPGA:  Xilinx Zynq-7020 (xc7z020clg400-1)"
	@echo ""
	@echo "N6 Architecture Parameters:"
	@echo "  CPU pipeline:  sigma = 12 stages"
	@echo "  Fetch width:   n = 6 (logical)"
	@echo "  Register file: n*sigma = 72 GPRs"
	@echo "  Instruction:   J2 = 24 bits"
	@echo "  ALU ops:       J2 = 24 operators"
	@echo "  Keywords:      sigma*tau + sopfr = 53"
	@echo "  Memory split:  1/2 + 1/3 + 1/6 = 1 (Egyptian)"
	@echo "  SNN neurons:   n = 6 (Izhikevich ring)"
	@echo "  MoE experts:   sigma = 12 (phi = 2 active)"
	@echo "  GPIO pins:     J2 = 24"
	@echo "  LEDs:          tau = 4"
	@echo "  SPI channels:  n = 6 (ch0 active)"
	@echo "  Clock:         100 MHz (period = sigma-phi = 10 ns)"
