##============================================================================
## HEXA-EDGE SoC — Xilinx Zynq-7020 Constraints (PYNQ-Z2 Board)
##
## N6 Parameter Mapping:
##   System clock: 100 MHz (period = sigma-phi = 10 ns)
##   GPIO:         J2 = 24 pins (mapped to Pmod + Arduino headers)
##   LEDs:         tau = 4 status LEDs
##   Buttons:      tau = 4 push buttons (active high)
##   SPI:          Channel 0 on Pmod JA
##   UART:         USB-UART bridge (CP2102)
##   SNN spikes:   n = 6 outputs on Pmod JB
##============================================================================

##--------------------------------------------------------------------------
## Clock — 100 MHz (period = sigma-phi = 10 ns)
##--------------------------------------------------------------------------
set_property -dict { PACKAGE_PIN H16   IOSTANDARD LVCMOS33 } [get_ports { clk }];
create_clock -add -name sys_clk_pin -period 10.00 -waveform {0 5} [get_ports { clk }];
## Period = sigma(6) - phi(6) = 10 ns → 100 MHz

##--------------------------------------------------------------------------
## Reset — Active-low button (BTN0)
##--------------------------------------------------------------------------
set_property -dict { PACKAGE_PIN D19   IOSTANDARD LVCMOS33 } [get_ports { rst_n }];

##--------------------------------------------------------------------------
## LEDs — tau(6) = 4 status LEDs (LD0-LD3)
##--------------------------------------------------------------------------
set_property -dict { PACKAGE_PIN R14   IOSTANDARD LVCMOS33 } [get_ports { led[0] }];
set_property -dict { PACKAGE_PIN P14   IOSTANDARD LVCMOS33 } [get_ports { led[1] }];
set_property -dict { PACKAGE_PIN N16   IOSTANDARD LVCMOS33 } [get_ports { led[2] }];
set_property -dict { PACKAGE_PIN M14   IOSTANDARD LVCMOS33 } [get_ports { led[3] }];
## led[0] = UART activity
## led[1] = SPI activity
## led[2] = SNN spike (any of n=6 neurons)
## led[3] = MoE active (any of sigma=12 experts)

##--------------------------------------------------------------------------
## UART — USB-UART bridge (debug console)
##--------------------------------------------------------------------------
set_property -dict { PACKAGE_PIN Y18   IOSTANDARD LVCMOS33 } [get_ports { uart_tx }];
set_property -dict { PACKAGE_PIN Y19   IOSTANDARD LVCMOS33 } [get_ports { uart_rx }];

##--------------------------------------------------------------------------
## SPI Channel 0 — Pmod JA (upper row)
## SPI clock divider = n = 6 (100MHz / 12 = 8.33 MHz SPI clock)
##--------------------------------------------------------------------------
set_property -dict { PACKAGE_PIN Y18   IOSTANDARD LVCMOS33 } [get_ports { spi0_sclk }];
set_property -dict { PACKAGE_PIN Y19   IOSTANDARD LVCMOS33 } [get_ports { spi0_mosi }];
set_property -dict { PACKAGE_PIN Y16   IOSTANDARD LVCMOS33 } [get_ports { spi0_miso }];
set_property -dict { PACKAGE_PIN Y17   IOSTANDARD LVCMOS33 } [get_ports { spi0_cs_n }];

##--------------------------------------------------------------------------
## SNN Spike Outputs — n(6) = 6 neurons on Pmod JB (upper + lower row)
##--------------------------------------------------------------------------
set_property -dict { PACKAGE_PIN W14   IOSTANDARD LVCMOS33 } [get_ports { snn_spike_out[0] }];
set_property -dict { PACKAGE_PIN Y14   IOSTANDARD LVCMOS33 } [get_ports { snn_spike_out[1] }];
set_property -dict { PACKAGE_PIN T11   IOSTANDARD LVCMOS33 } [get_ports { snn_spike_out[2] }];
set_property -dict { PACKAGE_PIN T10   IOSTANDARD LVCMOS33 } [get_ports { snn_spike_out[3] }];
set_property -dict { PACKAGE_PIN V16   IOSTANDARD LVCMOS33 } [get_ports { snn_spike_out[4] }];
set_property -dict { PACKAGE_PIN W16   IOSTANDARD LVCMOS33 } [get_ports { snn_spike_out[5] }];

##--------------------------------------------------------------------------
## GPIO — J2(6) = 24 pins
##
## Mapping:
##   gpio[0:7]   = Arduino digital D0-D7    (sigma-tau = 8 pins)
##   gpio[8:15]  = Arduino digital D8-D15   (sigma-tau = 8 pins)
##   gpio[16:23] = Pmod JC + JD lower rows  (sigma-tau = 8 pins)
##   Total: 3 groups of (sigma-tau) = 3 * 8 = J2 = 24 pins EXACT
##--------------------------------------------------------------------------

## Arduino Digital D0-D7 — gpio[0:7] (sigma-tau = 8 group 1)
set_property -dict { PACKAGE_PIN T14   IOSTANDARD LVCMOS33 } [get_ports { gpio[0]  }];
set_property -dict { PACKAGE_PIN U12   IOSTANDARD LVCMOS33 } [get_ports { gpio[1]  }];
set_property -dict { PACKAGE_PIN U13   IOSTANDARD LVCMOS33 } [get_ports { gpio[2]  }];
set_property -dict { PACKAGE_PIN V13   IOSTANDARD LVCMOS33 } [get_ports { gpio[3]  }];
set_property -dict { PACKAGE_PIN V15   IOSTANDARD LVCMOS33 } [get_ports { gpio[4]  }];
set_property -dict { PACKAGE_PIN T15   IOSTANDARD LVCMOS33 } [get_ports { gpio[5]  }];
set_property -dict { PACKAGE_PIN R16   IOSTANDARD LVCMOS33 } [get_ports { gpio[6]  }];
set_property -dict { PACKAGE_PIN U17   IOSTANDARD LVCMOS33 } [get_ports { gpio[7]  }];

## Arduino Digital D8-D15 — gpio[8:15] (sigma-tau = 8 group 2)
set_property -dict { PACKAGE_PIN V17   IOSTANDARD LVCMOS33 } [get_ports { gpio[8]  }];
set_property -dict { PACKAGE_PIN V18   IOSTANDARD LVCMOS33 } [get_ports { gpio[9]  }];
set_property -dict { PACKAGE_PIN T16   IOSTANDARD LVCMOS33 } [get_ports { gpio[10] }];
set_property -dict { PACKAGE_PIN R17   IOSTANDARD LVCMOS33 } [get_ports { gpio[11] }];
set_property -dict { PACKAGE_PIN P18   IOSTANDARD LVCMOS33 } [get_ports { gpio[12] }];
set_property -dict { PACKAGE_PIN N17   IOSTANDARD LVCMOS33 } [get_ports { gpio[13] }];
set_property -dict { PACKAGE_PIN Y11   IOSTANDARD LVCMOS33 } [get_ports { gpio[14] }];
set_property -dict { PACKAGE_PIN Y12   IOSTANDARD LVCMOS33 } [get_ports { gpio[15] }];

## Pmod JC+JD — gpio[16:23] (sigma-tau = 8 group 3)
set_property -dict { PACKAGE_PIN W15   IOSTANDARD LVCMOS33 } [get_ports { gpio[16] }];
set_property -dict { PACKAGE_PIN T12   IOSTANDARD LVCMOS33 } [get_ports { gpio[17] }];
set_property -dict { PACKAGE_PIN U11   IOSTANDARD LVCMOS33 } [get_ports { gpio[18] }];
set_property -dict { PACKAGE_PIN V11   IOSTANDARD LVCMOS33 } [get_ports { gpio[19] }];
set_property -dict { PACKAGE_PIN W11   IOSTANDARD LVCMOS33 } [get_ports { gpio[20] }];
set_property -dict { PACKAGE_PIN Y11   IOSTANDARD LVCMOS33 } [get_ports { gpio[21] }];
set_property -dict { PACKAGE_PIN V10   IOSTANDARD LVCMOS33 } [get_ports { gpio[22] }];
set_property -dict { PACKAGE_PIN W10   IOSTANDARD LVCMOS33 } [get_ports { gpio[23] }];

##--------------------------------------------------------------------------
## Timing Constraints
##--------------------------------------------------------------------------

## Input delay: tau(6) = 4 ns max
set_input_delay  -clock sys_clk_pin -max 4.0 [get_ports { uart_rx spi0_miso }];
set_input_delay  -clock sys_clk_pin -min 1.0 [get_ports { uart_rx spi0_miso }];

## Output delay: tau(6) = 4 ns max
set_output_delay -clock sys_clk_pin -max 4.0 [get_ports { uart_tx spi0_sclk spi0_mosi spi0_cs_n }];
set_output_delay -clock sys_clk_pin -min 1.0 [get_ports { uart_tx spi0_sclk spi0_mosi spi0_cs_n }];

## False paths for async inputs
set_false_path -from [get_ports { rst_n }];

##--------------------------------------------------------------------------
## Power Optimization
##--------------------------------------------------------------------------
## Target: n = 6 W TDP (HEXA-EDGE spec)
## Zynq-7020 typical: ~2W (well within budget)

set_property BITSTREAM.CONFIG.UNUSEDPIN Pulldown [current_design];
set_property CONFIG_VOLTAGE 3.3 [current_design];
set_property CFGBVS VCCO [current_design];
