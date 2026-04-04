//============================================================================
// HEXA-EDGE SoC Top-Level — FPGA Subset for Xilinx Zynq-7020
//
// N6 Arithmetic Constants:
//   n=6  phi=2  tau=4  sigma=12  sopfr=5  mu=1  J2=24  R=1
//   sigma-tau=8  sigma-phi=10  sigma-mu=11  sigma*tau=48
//   2^n=64  phi^tau=16  sigma^2=144  sigma*J2=288
//
// Architecture:
//   - RISC-V N6 core (6-wide decode, 12-stage pipeline)
//   - Egyptian memory controller (1/2+1/3+1/6=1 address split)
//   - HEXA-LANG 53-keyword CAM decoder
//   - SNN Izhikevich 6-neuron ring
//   - Egyptian MoE router (sigma=12 slots, phi=2 active)
//   - SPI n=6 channels, GPIO J2=24 pins
//============================================================================

module hexa_edge_top (
    input  wire        clk,            // 100 MHz system clock
    input  wire        rst_n,          // Active-low reset

    // GPIO — J2=24 pins
    inout  wire [23:0] gpio,           // J2(6)=24 GPIO pins

    // SPI — n=6 channels (directly expose channel 0 for FPGA demo)
    output wire        spi0_sclk,
    output wire        spi0_mosi,
    input  wire        spi0_miso,
    output wire        spi0_cs_n,

    // UART — debug port
    input  wire        uart_rx,
    output wire        uart_tx,

    // LED indicators — tau=4 LEDs
    output wire [3:0]  led,            // tau(6)=4 status LEDs

    // SNN spike output — n=6 neurons
    output wire [5:0]  snn_spike_out   // n=6 neuron spike indicators
);

    // ========================================================================
    // Internal buses
    // ========================================================================

    // CPU instruction fetch interface
    wire [23:0] cpu_iaddr;             // J2=24 bit instruction address
    wire [23:0] cpu_idata;             // J2=24 bit instruction word
    wire        cpu_ireq;
    wire        cpu_igrant;

    // CPU data interface
    wire [15:0] cpu_daddr;             // 16-bit data address (64K space)
    wire [23:0] cpu_dwdata;            // J2=24 bit write data
    wire [23:0] cpu_drdata;            // J2=24 bit read data
    wire        cpu_dwe;               // Write enable
    wire        cpu_dre;               // Read enable
    wire        cpu_dready;            // Data ready

    // Memory controller outputs
    wire [1:0]  mem_region;            // 00=stack, 01=heap, 10=arena
    wire        mem_valid;

    // HEXA-LANG decoder interface
    wire [23:0] hlang_opcode;          // J2=24 bit opcode from CPU
    wire        hlang_valid;
    wire [5:0]  hlang_kw_group;        // Keyword group index (0-11, sigma=12)
    wire [5:0]  hlang_kw_index;        // Keyword index within group
    wire        hlang_is_keyword;      // Decoded as keyword
    wire [4:0]  hlang_alu_op;          // ALU operation (J2=24 ops need 5 bits)

    // Egyptian MoE interface
    wire [11:0] moe_expert_scores;     // sigma=12 expert scores
    wire [1:0]  moe_selected;          // phi=2 selected experts
    wire [11:0] moe_active_mask;       // Which experts are active

    // SNN interface
    wire [5:0]  snn_spikes;            // n=6 neuron spikes
    wire [23:0] snn_excitation;        // J2=24 bit excitation input

    // ========================================================================
    // Instruction memory — tau^tau = 256 words (FPGA Block RAM)
    // Each word is J2=24 bits
    // ========================================================================

    reg [23:0] imem [0:255];           // 2^(sigma-tau) = 2^8 = 256 words

    // Simple instruction fetch (single cycle for FPGA)
    reg [23:0] imem_rdata;
    always @(posedge clk) begin
        if (cpu_ireq)
            imem_rdata <= imem[cpu_iaddr[7:0]]; // Use lower 8 bits for 256 words
    end
    assign cpu_idata  = imem_rdata;
    assign cpu_igrant = cpu_ireq;      // Always grant in FPGA demo

    // ========================================================================
    // Data memory — 2^sigma = 4096 × J2-bit words (FPGA BRAM)
    // Full 16-bit address mapped via Egyptian memory controller
    // ========================================================================

    reg [23:0] dmem [0:4095];          // 2^sigma(6) = 4096 words

    reg [23:0] dmem_rdata;
    reg        dmem_ready;
    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            dmem_rdata <= 24'b0;
            dmem_ready <= 1'b0;
        end else begin
            dmem_ready <= cpu_dre | cpu_dwe;
            if (cpu_dwe && mem_valid)
                dmem[cpu_daddr[11:0]] <= cpu_dwdata;
            if (cpu_dre)
                dmem_rdata <= dmem[cpu_daddr[11:0]];
        end
    end
    assign cpu_drdata = dmem_rdata;
    assign cpu_dready = dmem_ready;

    // ========================================================================
    // RISC-V N6 Core
    // ========================================================================

    riscv_n6_core u_cpu (
        .clk        (clk),
        .rst_n      (rst_n),
        // Instruction interface
        .iaddr      (cpu_iaddr),
        .idata      (cpu_idata),
        .ireq       (cpu_ireq),
        .igrant     (cpu_igrant),
        // Data interface
        .daddr      (cpu_daddr),
        .dwdata     (cpu_dwdata),
        .drdata     (cpu_drdata),
        .dwe        (cpu_dwe),
        .dre        (cpu_dre),
        .dready     (cpu_dready),
        // HEXA-LANG decoder
        .hlang_opcode   (hlang_opcode),
        .hlang_valid    (hlang_valid),
        .hlang_alu_op   (hlang_alu_op),
        .hlang_is_keyword (hlang_is_keyword)
    );

    // ========================================================================
    // Egyptian Memory Controller (1/2 + 1/3 + 1/6 = 1)
    // ========================================================================

    egyptian_mem_ctrl u_memctrl (
        .clk        (clk),
        .rst_n      (rst_n),
        .addr       (cpu_daddr),
        .we         (cpu_dwe),
        .re         (cpu_dre),
        .region     (mem_region),
        .valid      (mem_valid)
    );

    // ========================================================================
    // HEXA-LANG Hardware Keyword Decoder (53 keywords)
    // ========================================================================

    hexalang_decoder u_hlang (
        .clk            (clk),
        .rst_n          (rst_n),
        .opcode_in      (hlang_opcode),
        .valid_in       (hlang_valid),
        .kw_group       (hlang_kw_group),
        .kw_index       (hlang_kw_index),
        .is_keyword     (hlang_is_keyword),
        .alu_op         (hlang_alu_op)
    );

    // ========================================================================
    // SNN Izhikevich 6-Neuron Ring
    // ========================================================================

    // Excitation input from data bus (memory-mapped I/O at top of arena)
    assign snn_excitation = cpu_dwdata;

    snn_izhikevich u_snn (
        .clk            (clk),
        .rst_n          (rst_n),
        .excitation     (snn_excitation[17:0]), // 18-bit fixed-point input
        .enable         (cpu_dwe && (cpu_daddr == 16'hFFF0)), // MMIO trigger
        .spikes         (snn_spikes)
    );

    assign snn_spike_out = snn_spikes;

    // ========================================================================
    // Egyptian MoE Router (sigma=12 experts, phi=2 active)
    // ========================================================================

    // Score input from data bus (memory-mapped)
    wire moe_trigger = cpu_dwe && (cpu_daddr == 16'hFFE0);

    egyptian_moe u_moe (
        .clk            (clk),
        .rst_n          (rst_n),
        .scores_in      ({cpu_dwdata, cpu_dwdata}), // 12 × 8-bit scores
        .valid_in       (moe_trigger),
        .active_mask    (moe_active_mask),
        .top_expert_0   (),
        .top_expert_1   (),
        .done           ()
    );

    // ========================================================================
    // SPI Controller — n=6 channels (channel 0 active for FPGA)
    // Minimal SPI master: sigma-tau=8 bit shift register
    // ========================================================================

    reg [7:0]  spi_shift;              // sigma-tau=8 bit shift register
    reg [2:0]  spi_cnt;               // n/phi=3 bit counter (counts to 8)
    reg        spi_active;
    reg        spi_cs;

    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            spi_shift  <= 8'b0;        // sigma-tau=8 bits
            spi_cnt    <= 3'b0;
            spi_active <= 1'b0;
            spi_cs     <= 1'b1;
        end else if (cpu_dwe && (cpu_daddr == 16'hFFD0)) begin
            // Write to SPI data register triggers transfer
            spi_shift  <= cpu_dwdata[7:0];
            spi_cnt    <= 3'd0;
            spi_active <= 1'b1;
            spi_cs     <= 1'b0;
        end else if (spi_active) begin
            spi_shift <= {spi_shift[6:0], spi_miso};
            spi_cnt   <= spi_cnt + 3'd1;
            if (spi_cnt == 3'd7) begin  // sigma-tau=8 bits transferred
                spi_active <= 1'b0;
                spi_cs     <= 1'b1;
            end
        end
    end

    // SPI clock = system clock / (phi*n) = clk/12 when active
    reg [3:0] spi_div;                 // sigma=12 divider
    reg       spi_clk_out;
    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            spi_div     <= 4'd0;
            spi_clk_out <= 1'b0;
        end else if (spi_active) begin
            if (spi_div == 4'd5) begin  // n-1=5, divide by n=6 (half period)
                spi_div     <= 4'd0;
                spi_clk_out <= ~spi_clk_out;
            end else begin
                spi_div <= spi_div + 4'd1;
            end
        end else begin
            spi_clk_out <= 1'b0;
        end
    end

    assign spi0_sclk = spi_clk_out;
    assign spi0_mosi = spi_shift[7];   // MSB first
    assign spi0_cs_n = spi_cs;

    // ========================================================================
    // GPIO Controller — J2=24 pins
    // ========================================================================

    reg [23:0] gpio_dir;               // J2=24 direction bits (1=output)
    reg [23:0] gpio_out;               // J2=24 output data
    wire [23:0] gpio_in;

    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            gpio_dir <= 24'b0;         // All inputs on reset
            gpio_out <= 24'b0;
        end else if (cpu_dwe) begin
            case (cpu_daddr)
                16'hFFC0: gpio_dir <= cpu_dwdata;  // Direction register
                16'hFFC4: gpio_out <= cpu_dwdata;  // Output register
                default: ;
            endcase
        end
    end

    // Tristate GPIO with J2=24 pins
    genvar gi;
    generate
        for (gi = 0; gi < 24; gi = gi + 1) begin : gen_gpio // J2(6)=24
            assign gpio[gi] = gpio_dir[gi] ? gpio_out[gi] : 1'bz;
        end
    endgenerate

    assign gpio_in = gpio;

    // ========================================================================
    // UART — minimal TX for debug (baud = 100MHz / 1085 ~ 92160, close to 9600*sigma-phi)
    // ========================================================================

    reg [9:0]  uart_div;               // Baud divider (sigma-phi=10 bits)
    reg [3:0]  uart_bit;               // Bit counter
    reg [9:0]  uart_sr;                // sigma-phi=10 bit shift (start+8+stop)
    reg        uart_busy;

    localparam UART_DIV = 10'd1085;    // 100MHz / 92160 ~ 1085

    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            uart_div  <= 10'd0;
            uart_bit  <= 4'd0;
            uart_sr   <= 10'h3FF;      // Idle high
            uart_busy <= 1'b0;
        end else if (cpu_dwe && (cpu_daddr == 16'hFFB0) && !uart_busy) begin
            // Load: start(0) + data[7:0] + stop(1) = sigma-phi=10 bits
            uart_sr   <= {1'b1, cpu_dwdata[7:0], 1'b0};
            uart_bit  <= 4'd0;
            uart_div  <= 10'd0;
            uart_busy <= 1'b1;
        end else if (uart_busy) begin
            if (uart_div == UART_DIV) begin
                uart_div <= 10'd0;
                uart_sr  <= {1'b1, uart_sr[9:1]};  // Shift right, idle high fill
                uart_bit <= uart_bit + 4'd1;
                if (uart_bit == 4'd9) begin         // sigma-phi=10 bits total
                    uart_busy <= 1'b0;
                end
            end else begin
                uart_div <= uart_div + 10'd1;
            end
        end
    end

    assign uart_tx = uart_sr[0];       // LSB out first

    // ========================================================================
    // LED status — tau=4 LEDs
    // ========================================================================

    assign led[0] = uart_busy;            // UART activity
    assign led[1] = spi_active;           // SPI activity
    assign led[2] = |snn_spikes;          // Any SNN spike
    assign led[3] = |moe_active_mask;     // MoE active

    // ========================================================================
    // Instruction memory initialization (for simulation)
    // ========================================================================
    // synopsys translate_off
    integer i;
    initial begin
        for (i = 0; i < 256; i = i + 1)
            imem[i] = 24'b0;
        // NOP sled (opcode 0x000000)
    end
    // synopsys translate_on

endmodule
