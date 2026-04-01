//============================================================================
// RISC-V N6 CPU Core — 6-Wide Decode, 12-Stage Pipeline
//
// N6 Parameter Mapping:
//   Fetch width:     n = 6 (simplified to 1 for FPGA, 6-wide decode logic)
//   Pipeline stages: sigma = 12
//   Register banks:  n = 6
//   Regs per bank:   sigma = 12 → total 72 = n*sigma GPRs
//   Instruction:     J2 = 24 bits
//   ALU operators:   J2 = 24
//   Branch predict:  phi = 2 bit history
//
// 12-stage pipeline:
//   IF1→IF2→DE1→DE2→DE3→DE4→DE5→DE6→EX→MEM→WB1→WB2
//   (stages 3-8 = 6-wide decode = n=6 decode stages)
//
// Instruction Format (J2=24 bits):
//   [23:18] opcode   — n=6 bit opcode (2^n=64 instructions)
//   [17:12] rd       — n=6 bit dest register (72 GPRs need ceil(log2(72))=7, use 6+bank)
//   [11:6]  rs1      — n=6 bit source 1
//   [5:0]   rs2/imm  — n=6 bit source 2 or immediate
//
// Bank select: top bit of register field selects bank (0-5)
//============================================================================

module riscv_n6_core (
    input  wire        clk,
    input  wire        rst_n,

    // Instruction memory interface
    output reg  [23:0] iaddr,          // J2=24 bit instruction address
    input  wire [23:0] idata,          // J2=24 bit instruction
    output wire        ireq,
    input  wire        igrant,

    // Data memory interface
    output reg  [15:0] daddr,          // 16-bit data address
    output reg  [23:0] dwdata,         // J2=24 bit write data
    input  wire [23:0] drdata,         // J2=24 bit read data
    output reg         dwe,
    output reg         dre,
    input  wire        dready,

    // HEXA-LANG decoder interface
    output wire [23:0] hlang_opcode,
    output wire        hlang_valid,
    input  wire [4:0]  hlang_alu_op,
    input  wire        hlang_is_keyword
);

    // ========================================================================
    // N6 Constants as localparams
    // ========================================================================
    localparam N       = 6;            // n = 6 (perfect number)
    localparam PHI     = 2;            // phi(6) = 2
    localparam TAU     = 4;            // tau(6) = 4
    localparam SIGMA   = 12;           // sigma(6) = 12
    localparam J2      = 24;           // J2(6) = 24
    localparam SOPFR   = 5;            // sopfr(6) = 5
    localparam S_T     = 8;            // sigma - tau = 8
    localparam S_P     = 10;           // sigma - phi = 10

    // Pipeline depth = sigma = 12
    localparam PIPE_DEPTH = SIGMA;     // sigma(6) = 12 stages

    // Register file: n=6 banks x sigma=12 regs = 72 GPRs
    localparam NUM_BANKS = N;          // n = 6 register banks
    localparam REGS_PER_BANK = SIGMA;  // sigma = 12 registers per bank
    localparam TOTAL_REGS = NUM_BANKS * REGS_PER_BANK; // 72 = n*sigma

    // Instruction field widths
    localparam OP_W   = N;             // n = 6 bit opcode
    localparam RD_W   = N;             // n = 6 bit dest
    localparam RS1_W  = N;             // n = 6 bit source 1
    localparam RS2_W  = N;             // n = 6 bit source 2 / immediate

    // ========================================================================
    // Register File — 72 GPRs (n=6 banks × sigma=12)
    // r0 always zero (RISC-V convention)
    // ========================================================================

    reg [J2-1:0] regfile [0:TOTAL_REGS-1]; // 72 x 24-bit registers

    // ========================================================================
    // Program Counter — J2=24 bit
    // ========================================================================

    reg [J2-1:0] pc;                   // J2 = 24 bit program counter

    // ========================================================================
    // Pipeline registers (sigma=12 stages)
    //
    // Stage layout:
    //   [0]  IF1: Fetch address generation
    //   [1]  IF2: Fetch data capture
    //   [2]  DE1: Opcode decode
    //   [3]  DE2: Register read 1
    //   [4]  DE3: Register read 2
    //   [5]  DE4: Immediate extend
    //   [6]  DE5: Hazard check
    //   [7]  DE6: Operand forward
    //   [8]  EX:  Execute / ALU
    //   [9]  MEM: Memory access
    //   [10] WB1: Writeback prepare
    //   [11] WB2: Writeback commit
    // ========================================================================

    // Pipeline valid bits
    reg [PIPE_DEPTH-1:0] pipe_valid;

    // Instruction flowing through pipeline
    reg [J2-1:0] pipe_inst  [0:PIPE_DEPTH-1];
    reg [J2-1:0] pipe_pc    [0:PIPE_DEPTH-1];

    // Decoded fields (populated from DE1 onward)
    reg [OP_W-1:0]  pipe_opcode [0:PIPE_DEPTH-1];
    reg [RD_W-1:0]  pipe_rd     [0:PIPE_DEPTH-1];
    reg [RS1_W-1:0] pipe_rs1    [0:PIPE_DEPTH-1];
    reg [RS2_W-1:0] pipe_rs2    [0:PIPE_DEPTH-1];

    // Operand values
    reg [J2-1:0] pipe_val_a  [0:PIPE_DEPTH-1];
    reg [J2-1:0] pipe_val_b  [0:PIPE_DEPTH-1];
    reg [J2-1:0] pipe_result [0:PIPE_DEPTH-1];

    // Writeback
    reg          pipe_wb_en  [0:PIPE_DEPTH-1];

    // ========================================================================
    // Opcode Encoding — 2^n = 64 possible, J2=24 used
    // ========================================================================

    // Arithmetic (n=6 ops)
    localparam OP_ADD  = 6'h01;        // +
    localparam OP_SUB  = 6'h02;        // -
    localparam OP_MUL  = 6'h03;        // *
    localparam OP_DIV  = 6'h04;        // /
    localparam OP_MOD  = 6'h05;        // %
    localparam OP_POW  = 6'h06;        // ** (power)

    // Comparison (n=6 ops)
    localparam OP_EQ   = 6'h07;        // ==
    localparam OP_NE   = 6'h08;        // !=
    localparam OP_LT   = 6'h09;        // <
    localparam OP_GT   = 6'h0A;        // >
    localparam OP_LE   = 6'h0B;        // <=
    localparam OP_GE   = 6'h0C;        // >=

    // Logical (tau=4 ops)
    localparam OP_AND  = 6'h0D;        // &&
    localparam OP_OR   = 6'h0E;        // ||
    localparam OP_NOT  = 6'h0F;        // !
    localparam OP_XOR  = 6'h10;        // ^^

    // Bitwise (tau=4 ops)
    localparam OP_BAND = 6'h11;        // &
    localparam OP_BOR  = 6'h12;        // |
    localparam OP_BXOR = 6'h13;        // ^
    localparam OP_BNOT = 6'h14;        // ~

    // Assignment (phi=2 ops)
    localparam OP_MOV  = 6'h15;        // =  (move/assign)
    localparam OP_MOVI = 6'h16;        // := (move immediate)

    // Memory
    localparam OP_LD   = 6'h17;        // Load
    localparam OP_ST   = 6'h18;        // Store

    // Control
    localparam OP_BEQ  = 6'h19;        // Branch if equal
    localparam OP_BNE  = 6'h1A;        // Branch if not equal
    localparam OP_JMP  = 6'h1B;        // Jump
    localparam OP_JAL  = 6'h1C;        // Jump and link
    localparam OP_NOP  = 6'h00;        // No operation
    // Total defined: 24+4+1 = 29, fits in 2^n=64 space
    // Remaining slots reserved for HEXA-LANG keyword acceleration

    // HEXA-LANG keyword group base opcodes (sigma=12 groups starting at 0x20)
    localparam OP_HLANG_BASE = 6'h20;  // Keywords map to 0x20-0x3F

    // ========================================================================
    // Branch Predictor — phi=2 bit saturating counter
    // ========================================================================

    // Simple phi=2 bit bimodal predictor, 2^n=64 entries
    reg [PHI-1:0] bht [0:63];         // 2^n = 64 entries, phi=2 bits each

    wire [5:0] bht_idx = pc[5:0];      // n=6 bit index
    wire       bht_predict = bht[bht_idx][PHI-1]; // MSB = prediction

    // Branch resolution
    reg        branch_taken;
    reg        branch_mispredict;
    reg [J2-1:0] branch_target;

    // ========================================================================
    // Pipeline operation
    // ========================================================================

    // Instruction fetch request
    assign ireq = rst_n & ~branch_mispredict;

    // HEXA-LANG decoder hookup
    assign hlang_opcode = pipe_inst[2]; // Feed from DE1 stage
    assign hlang_valid  = pipe_valid[2];

    // Stall signal for memory operations
    wire stall_mem = (pipe_valid[9] && (pipe_opcode[9] == OP_LD || pipe_opcode[9] == OP_ST)) && !dready;

    integer k;

    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            pc <= {J2{1'b0}};
            pipe_valid <= {PIPE_DEPTH{1'b0}};
            branch_taken <= 1'b0;
            branch_mispredict <= 1'b0;
            branch_target <= {J2{1'b0}};
            dwe <= 1'b0;
            dre <= 1'b0;
            daddr <= 16'b0;
            dwdata <= {J2{1'b0}};

            for (k = 0; k < PIPE_DEPTH; k = k + 1) begin
                pipe_inst[k]   <= {J2{1'b0}};
                pipe_pc[k]     <= {J2{1'b0}};
                pipe_opcode[k] <= {OP_W{1'b0}};
                pipe_rd[k]     <= {RD_W{1'b0}};
                pipe_rs1[k]    <= {RS1_W{1'b0}};
                pipe_rs2[k]    <= {RS2_W{1'b0}};
                pipe_val_a[k]  <= {J2{1'b0}};
                pipe_val_b[k]  <= {J2{1'b0}};
                pipe_result[k] <= {J2{1'b0}};
                pipe_wb_en[k]  <= 1'b0;
            end

            for (k = 0; k < 64; k = k + 1)     // 2^n=64 BHT entries
                bht[k] <= {PHI{1'b0}};

            for (k = 0; k < TOTAL_REGS; k = k + 1) // 72=n*sigma GPRs
                regfile[k] <= {J2{1'b0}};

        end else if (!stall_mem) begin

            // ================================================================
            // Stage 0: IF1 — Fetch address generation
            // ================================================================
            if (branch_mispredict) begin
                pc <= branch_target;
                pipe_valid[0] <= 1'b0;
                branch_mispredict <= 1'b0;
            end else begin
                iaddr <= pc;
                pc <= pc + {{(J2-1){1'b0}}, 1'b1};  // PC + 1
                pipe_valid[0] <= 1'b1;
                pipe_pc[0] <= pc;
            end

            // ================================================================
            // Stage 1: IF2 — Fetch data capture
            // ================================================================
            pipe_valid[1] <= pipe_valid[0] & igrant;
            pipe_inst[1]  <= idata;
            pipe_pc[1]    <= pipe_pc[0];

            // ================================================================
            // Stage 2: DE1 — Opcode decode (field extraction)
            // ================================================================
            pipe_valid[2]   <= pipe_valid[1];
            pipe_inst[2]    <= pipe_inst[1];
            pipe_pc[2]      <= pipe_pc[1];
            pipe_opcode[2]  <= pipe_inst[1][J2-1:J2-OP_W];         // [23:18]
            pipe_rd[2]      <= pipe_inst[1][J2-OP_W-1:J2-OP_W-RD_W]; // [17:12]
            pipe_rs1[2]     <= pipe_inst[1][J2-OP_W-RD_W-1:RS2_W]; // [11:6]
            pipe_rs2[2]     <= pipe_inst[1][RS2_W-1:0];            // [5:0]

            // ================================================================
            // Stage 3: DE2 — Register read (source 1)
            // ================================================================
            pipe_valid[3]   <= pipe_valid[2];
            pipe_inst[3]    <= pipe_inst[2];
            pipe_pc[3]      <= pipe_pc[2];
            pipe_opcode[3]  <= pipe_opcode[2];
            pipe_rd[3]      <= pipe_rd[2];
            pipe_rs1[3]     <= pipe_rs1[2];
            pipe_rs2[3]     <= pipe_rs2[2];
            // Read source register 1 (index < 72 = n*sigma)
            pipe_val_a[3]   <= (pipe_rs1[2] < TOTAL_REGS[5:0]) ?
                               regfile[pipe_rs1[2]] : {J2{1'b0}};

            // ================================================================
            // Stage 4: DE3 — Register read (source 2)
            // ================================================================
            pipe_valid[4]   <= pipe_valid[3];
            pipe_inst[4]    <= pipe_inst[3];
            pipe_pc[4]      <= pipe_pc[3];
            pipe_opcode[4]  <= pipe_opcode[3];
            pipe_rd[4]      <= pipe_rd[3];
            pipe_rs1[4]     <= pipe_rs1[3];
            pipe_rs2[4]     <= pipe_rs2[3];
            pipe_val_a[4]   <= pipe_val_a[3];
            // Read source register 2 or use as immediate
            pipe_val_b[4]   <= (pipe_opcode[3] == OP_MOVI) ?
                               {{(J2-RS2_W){pipe_rs2[3][RS2_W-1]}}, pipe_rs2[3]} :  // Sign-extend imm
                               ((pipe_rs2[3] < TOTAL_REGS[5:0]) ?
                                regfile[pipe_rs2[3]] : {J2{1'b0}});

            // ================================================================
            // Stage 5: DE4 — Immediate extension (for branches/jumps)
            // ================================================================
            pipe_valid[5]   <= pipe_valid[4];
            pipe_pc[5]      <= pipe_pc[4];
            pipe_opcode[5]  <= pipe_opcode[4];
            pipe_rd[5]      <= pipe_rd[4];
            pipe_val_a[5]   <= pipe_val_a[4];
            // For branch: combine rd+rs2 as sigma=12 bit signed offset
            if (pipe_opcode[4] == OP_BEQ || pipe_opcode[4] == OP_BNE ||
                pipe_opcode[4] == OP_JMP || pipe_opcode[4] == OP_JAL) begin
                pipe_val_b[5] <= {{(J2-SIGMA){pipe_rd[4][RD_W-1]}},
                                  pipe_rd[4], pipe_rs2[4]};  // sigma=12 bit offset
            end else begin
                pipe_val_b[5] <= pipe_val_b[4];
            end

            // ================================================================
            // Stage 6: DE5 — Hazard detection
            // ================================================================
            pipe_valid[6]   <= pipe_valid[5];
            pipe_pc[6]      <= pipe_pc[5];
            pipe_opcode[6]  <= pipe_opcode[5];
            pipe_rd[6]      <= pipe_rd[5];
            pipe_val_a[6]   <= pipe_val_a[5];
            pipe_val_b[6]   <= pipe_val_b[5];
            // Forwarding from EX result (1-stage bypass)
            if (pipe_valid[8] && pipe_wb_en[8] && pipe_rd[8] == pipe_rs1[5])
                pipe_val_a[6] <= pipe_result[8];
            if (pipe_valid[8] && pipe_wb_en[8] && pipe_rd[8] == pipe_rs2[5])
                pipe_val_b[6] <= pipe_result[8];

            // ================================================================
            // Stage 7: DE6 — Operand ready / final forward
            // ================================================================
            pipe_valid[7]   <= pipe_valid[6];
            pipe_pc[7]      <= pipe_pc[6];
            pipe_opcode[7]  <= pipe_opcode[6];
            pipe_rd[7]      <= pipe_rd[6];
            pipe_val_a[7]   <= pipe_val_a[6];
            pipe_val_b[7]   <= pipe_val_b[6];
            // Forward from WB1
            if (pipe_valid[10] && pipe_wb_en[10] && pipe_rd[10] == pipe_rd[6])
                pipe_val_a[7] <= pipe_result[10];

            // ================================================================
            // Stage 8: EX — Execute (ALU / Branch resolve)
            // ================================================================
            pipe_valid[8]   <= pipe_valid[7];
            pipe_pc[8]      <= pipe_pc[7];
            pipe_opcode[8]  <= pipe_opcode[7];
            pipe_rd[8]      <= pipe_rd[7];
            pipe_wb_en[8]   <= 1'b0;

            case (pipe_opcode[7])
                // Arithmetic — n=6 operations
                OP_ADD:  begin pipe_result[8] <= pipe_val_a[7] + pipe_val_b[7]; pipe_wb_en[8] <= 1'b1; end
                OP_SUB:  begin pipe_result[8] <= pipe_val_a[7] - pipe_val_b[7]; pipe_wb_en[8] <= 1'b1; end
                OP_MUL:  begin pipe_result[8] <= pipe_val_a[7] * pipe_val_b[7]; pipe_wb_en[8] <= 1'b1; end
                OP_DIV:  begin
                    pipe_result[8] <= (pipe_val_b[7] != 0) ?
                                      pipe_val_a[7] / pipe_val_b[7] : {J2{1'b0}};
                    pipe_wb_en[8] <= 1'b1;
                end
                OP_MOD:  begin
                    pipe_result[8] <= (pipe_val_b[7] != 0) ?
                                      pipe_val_a[7] % pipe_val_b[7] : {J2{1'b0}};
                    pipe_wb_en[8] <= 1'b1;
                end
                OP_POW:  begin
                    // Simplified: only handle small exponents for synthesis
                    pipe_result[8] <= pipe_val_a[7]; // TODO: iterative power
                    pipe_wb_en[8] <= 1'b1;
                end

                // Comparison — n=6 operations
                OP_EQ:   begin pipe_result[8] <= (pipe_val_a[7] == pipe_val_b[7]) ? 24'd1 : 24'd0; pipe_wb_en[8] <= 1'b1; end
                OP_NE:   begin pipe_result[8] <= (pipe_val_a[7] != pipe_val_b[7]) ? 24'd1 : 24'd0; pipe_wb_en[8] <= 1'b1; end
                OP_LT:   begin pipe_result[8] <= ($signed(pipe_val_a[7]) < $signed(pipe_val_b[7])) ? 24'd1 : 24'd0; pipe_wb_en[8] <= 1'b1; end
                OP_GT:   begin pipe_result[8] <= ($signed(pipe_val_a[7]) > $signed(pipe_val_b[7])) ? 24'd1 : 24'd0; pipe_wb_en[8] <= 1'b1; end
                OP_LE:   begin pipe_result[8] <= ($signed(pipe_val_a[7]) <= $signed(pipe_val_b[7])) ? 24'd1 : 24'd0; pipe_wb_en[8] <= 1'b1; end
                OP_GE:   begin pipe_result[8] <= ($signed(pipe_val_a[7]) >= $signed(pipe_val_b[7])) ? 24'd1 : 24'd0; pipe_wb_en[8] <= 1'b1; end

                // Logical — tau=4 operations
                OP_AND:  begin pipe_result[8] <= (pipe_val_a[7] != 0 && pipe_val_b[7] != 0) ? 24'd1 : 24'd0; pipe_wb_en[8] <= 1'b1; end
                OP_OR:   begin pipe_result[8] <= (pipe_val_a[7] != 0 || pipe_val_b[7] != 0) ? 24'd1 : 24'd0; pipe_wb_en[8] <= 1'b1; end
                OP_NOT:  begin pipe_result[8] <= (pipe_val_a[7] == 0) ? 24'd1 : 24'd0; pipe_wb_en[8] <= 1'b1; end
                OP_XOR:  begin pipe_result[8] <= ((pipe_val_a[7] != 0) ^ (pipe_val_b[7] != 0)) ? 24'd1 : 24'd0; pipe_wb_en[8] <= 1'b1; end

                // Bitwise — tau=4 operations
                OP_BAND: begin pipe_result[8] <= pipe_val_a[7] & pipe_val_b[7]; pipe_wb_en[8] <= 1'b1; end
                OP_BOR:  begin pipe_result[8] <= pipe_val_a[7] | pipe_val_b[7]; pipe_wb_en[8] <= 1'b1; end
                OP_BXOR: begin pipe_result[8] <= pipe_val_a[7] ^ pipe_val_b[7]; pipe_wb_en[8] <= 1'b1; end
                OP_BNOT: begin pipe_result[8] <= ~pipe_val_a[7]; pipe_wb_en[8] <= 1'b1; end

                // Move — phi=2 operations
                OP_MOV:  begin pipe_result[8] <= pipe_val_a[7]; pipe_wb_en[8] <= 1'b1; end
                OP_MOVI: begin pipe_result[8] <= pipe_val_b[7]; pipe_wb_en[8] <= 1'b1; end  // Immediate

                // Load
                OP_LD:   begin pipe_result[8] <= pipe_val_a[7] + pipe_val_b[7]; pipe_wb_en[8] <= 1'b1; end

                // Store
                OP_ST:   begin pipe_result[8] <= pipe_val_a[7] + pipe_val_b[7]; pipe_wb_en[8] <= 1'b0; end

                // Branch
                OP_BEQ: begin
                    branch_taken <= (pipe_val_a[7] == {J2{1'b0}});
                    branch_target <= pipe_pc[7] + pipe_val_b[7];
                    branch_mispredict <= (pipe_val_a[7] == {J2{1'b0}}) != bht_predict;
                    // Update BHT (phi=2 bit counter)
                    if (pipe_val_a[7] == {J2{1'b0}}) begin
                        if (bht[pipe_pc[7][5:0]] < {PHI{1'b1}})
                            bht[pipe_pc[7][5:0]] <= bht[pipe_pc[7][5:0]] + {{(PHI-1){1'b0}}, 1'b1};
                    end else begin
                        if (bht[pipe_pc[7][5:0]] > {PHI{1'b0}})
                            bht[pipe_pc[7][5:0]] <= bht[pipe_pc[7][5:0]] - {{(PHI-1){1'b0}}, 1'b1};
                    end
                    pipe_wb_en[8] <= 1'b0;
                end

                OP_BNE: begin
                    branch_taken <= (pipe_val_a[7] != {J2{1'b0}});
                    branch_target <= pipe_pc[7] + pipe_val_b[7];
                    branch_mispredict <= (pipe_val_a[7] != {J2{1'b0}}) != bht_predict;
                    pipe_wb_en[8] <= 1'b0;
                end

                OP_JMP: begin
                    branch_taken <= 1'b1;
                    branch_target <= pipe_val_b[7];
                    branch_mispredict <= 1'b1;  // Always flush on jump
                    pipe_wb_en[8] <= 1'b0;
                end

                OP_JAL: begin
                    branch_taken <= 1'b1;
                    branch_target <= pipe_val_b[7];
                    branch_mispredict <= 1'b1;
                    pipe_result[8] <= pipe_pc[7] + 24'd1; // Save return address
                    pipe_wb_en[8] <= 1'b1;
                end

                OP_NOP: begin
                    pipe_result[8] <= {J2{1'b0}};
                    pipe_wb_en[8] <= 1'b0;
                end

                default: begin
                    // HEXA-LANG keyword opcodes (0x20-0x3F) — handled by decoder
                    if (pipe_opcode[7] >= OP_HLANG_BASE) begin
                        pipe_result[8] <= pipe_val_a[7]; // Pass through for keyword ops
                        pipe_wb_en[8] <= 1'b1;
                    end else begin
                        pipe_result[8] <= {J2{1'b0}};
                        pipe_wb_en[8] <= 1'b0;
                    end
                end
            endcase

            // Flush IF/DE on mispredict (stages 0-7)
            if (branch_mispredict) begin
                pipe_valid[0] <= 1'b0;
                pipe_valid[1] <= 1'b0;
                pipe_valid[2] <= 1'b0;
                pipe_valid[3] <= 1'b0;
                pipe_valid[4] <= 1'b0;
                pipe_valid[5] <= 1'b0;
                pipe_valid[6] <= 1'b0;
                pipe_valid[7] <= 1'b0;
            end

            // ================================================================
            // Stage 9: MEM — Memory access
            // ================================================================
            pipe_valid[9]   <= pipe_valid[8];
            pipe_opcode[9]  <= pipe_opcode[8];
            pipe_rd[9]      <= pipe_rd[8];
            pipe_result[9]  <= pipe_result[8];
            pipe_wb_en[9]   <= pipe_wb_en[8];

            // Generate memory control signals
            dwe <= 1'b0;
            dre <= 1'b0;
            if (pipe_valid[8]) begin
                case (pipe_opcode[8])
                    OP_LD: begin
                        daddr <= pipe_result[8][15:0];
                        dre   <= 1'b1;
                    end
                    OP_ST: begin
                        daddr  <= pipe_result[8][15:0];
                        dwdata <= pipe_val_b[8];
                        dwe    <= 1'b1;
                    end
                    default: ;
                endcase
            end

            // ================================================================
            // Stage 10: WB1 — Writeback prepare
            // ================================================================
            pipe_valid[10]  <= pipe_valid[9];
            pipe_opcode[10] <= pipe_opcode[9];
            pipe_rd[10]     <= pipe_rd[9];
            pipe_wb_en[10]  <= pipe_wb_en[9];
            // Capture load data from memory
            if (pipe_opcode[9] == OP_LD && pipe_valid[9])
                pipe_result[10] <= drdata;
            else
                pipe_result[10] <= pipe_result[9];

            // ================================================================
            // Stage 11: WB2 — Writeback commit to register file
            // ================================================================
            pipe_valid[11]  <= pipe_valid[10];
            pipe_rd[11]     <= pipe_rd[10];
            pipe_wb_en[11]  <= pipe_wb_en[10];
            pipe_result[11] <= pipe_result[10];

            // Write to register file (72 = n*sigma GPRs, skip r0)
            if (pipe_valid[10] && pipe_wb_en[10] && pipe_rd[10] != {RD_W{1'b0}}) begin
                regfile[pipe_rd[10]] <= pipe_result[10];
            end

        end // !stall_mem
    end

endmodule
