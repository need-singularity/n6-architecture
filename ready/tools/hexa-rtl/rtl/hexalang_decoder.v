//============================================================================
// HEXA-LANG Hardware Keyword Decoder
//
// 53-entry Content-Addressable Memory (CAM) for keyword recognition
// 53 = sigma*tau + sopfr = 48 + 5
//
// Keyword groups (sigma = 12 groups):
//   Group  0: Control flow  — n=6 keywords     (if else match for while loop)
//   Group  1: Type decl     — sopfr=5 keywords  (type struct enum trait impl)
//   Group  2: Function      — sopfr=5 keywords  (fn return yield async await)
//   Group  3: Variable      — tau=4 keywords     (let mut const static)
//   Group  4: Module        — tau=4 keywords     (mod use pub crate)
//   Group  5: Memory        — tau=4 keywords     (own borrow move drop)
//   Group  6: Concurrency   — tau=4 keywords     (spawn channel select atomic)
//   Group  7: Effect        — tau=4 keywords     (effect handle resume pure)
//   Group  8: Proof         — tau=4 keywords     (proof assert invariant theorem)
//   Group  9: Meta          — tau=4 keywords     (macro derive where comptime)
//   Group 10: Error         — sopfr=5 keywords   (try catch throw panic recover)
//   Group 11: AI            — tau=4 keywords     (intent generate verify optimize)
//
// J2=24 bit opcode format:
//   [23:18] = primary opcode (n=6 bits)
//   [17:14] = keyword group  (tau=4 bits, 0-11 for sigma=12 groups)
//   [13:11] = keyword index  (n/phi=3 bits, 0-5 within group)
//   [10:8]  = modifier flags (n/phi=3 bits)
//   [7:0]   = operand/immediate (sigma-tau=8 bits)
//
// Pipeline: mu(6)=1 cycle decode latency
//============================================================================

module hexalang_decoder (
    input  wire        clk,
    input  wire        rst_n,

    // Instruction input
    input  wire [23:0] opcode_in,      // J2=24 bit instruction
    input  wire        valid_in,

    // Decoded output (registered — 1 cycle latency = mu(6))
    output reg  [3:0]  kw_group,       // Keyword group (0-11, sigma=12 groups)
    output reg  [2:0]  kw_index,       // Index within group (0-5, max n=6)
    output reg         is_keyword,     // Instruction is a HEXA-LANG keyword
    output reg  [4:0]  alu_op,         // ALU operation for J2=24 operators
    output reg  [2:0]  modifier,       // Modifier flags
    output reg  [7:0]  operand,        // sigma-tau=8 bit operand
    output reg         valid_out       // Output valid
);

    // ========================================================================
    // N6 Constants
    // ========================================================================
    localparam N       = 6;            // n = 6
    localparam SIGMA   = 12;           // sigma(6) = 12
    localparam TAU     = 4;            // tau(6) = 4
    localparam SOPFR   = 5;            // sopfr(6) = 5
    localparam J2      = 24;           // J2(6) = 24
    localparam S_T     = 8;            // sigma-tau = 8
    localparam N_KEYWORDS = 53;        // sigma*tau + sopfr = 48 + 5

    // ========================================================================
    // Opcode field extraction (combinational)
    // ========================================================================
    wire [5:0] primary_op = opcode_in[23:18]; // n=6 bit primary opcode
    wire [3:0] grp_field  = opcode_in[17:14]; // tau=4 bit group
    wire [2:0] idx_field  = opcode_in[13:11]; // n/phi=3 bit index
    wire [2:0] mod_field  = opcode_in[10:8];  // n/phi=3 bit modifier
    wire [7:0] opr_field  = opcode_in[7:0];   // sigma-tau=8 bit operand

    // HEXA-LANG keyword opcode range: 0x20-0x3F (32 primary codes)
    localparam OP_HLANG_BASE = 6'h20;
    wire is_hlang_range = (primary_op >= OP_HLANG_BASE);

    // ========================================================================
    // CAM: 53-entry keyword recognition
    //
    // Each CAM entry stores:
    //   - group (4 bits) + index (3 bits) = 7-bit tag
    //   - valid bit
    //
    // The CAM matches on {group, index} from the opcode
    // and outputs keyword classification
    // ========================================================================

    // Group size table (sigma=12 groups)
    // Stored as combinational LUT for synthesis efficiency
    reg [2:0] group_size [0:11]; // sigma=12 entries

    // Initialize group sizes from HEXA-LANG spec
    // synopsys translate_off
    initial begin
        group_size[0]  = 3'd6;  // Control flow: n=6
        group_size[1]  = 3'd5;  // Type decl: sopfr=5
        group_size[2]  = 3'd5;  // Function: sopfr=5
        group_size[3]  = 3'd4;  // Variable: tau=4
        group_size[4]  = 3'd4;  // Module: tau=4
        group_size[5]  = 3'd4;  // Memory: tau=4
        group_size[6]  = 3'd4;  // Concurrency: tau=4
        group_size[7]  = 3'd4;  // Effect: tau=4
        group_size[8]  = 3'd4;  // Proof: tau=4
        group_size[9]  = 3'd4;  // Meta: tau=4
        group_size[10] = 3'd5;  // Error: sopfr=5
        group_size[11] = 3'd4;  // AI: tau=4
    end
    // synopsys translate_on

    // Synthesizable group size lookup (combinational)
    reg [2:0] max_idx;
    always @(*) begin
        case (grp_field)
            4'd0:    max_idx = 3'd6;   // Control flow: n=6 keywords
            4'd1:    max_idx = 3'd5;   // Type decl: sopfr=5 keywords
            4'd2:    max_idx = 3'd5;   // Function: sopfr=5 keywords
            4'd3:    max_idx = 3'd4;   // Variable: tau=4 keywords
            4'd4:    max_idx = 3'd4;   // Module: tau=4 keywords
            4'd5:    max_idx = 3'd4;   // Memory: tau=4 keywords
            4'd6:    max_idx = 3'd4;   // Concurrency: tau=4 keywords
            4'd7:    max_idx = 3'd4;   // Effect: tau=4 keywords
            4'd8:    max_idx = 3'd4;   // Proof: tau=4 keywords
            4'd9:    max_idx = 3'd4;   // Meta: tau=4 keywords
            4'd10:   max_idx = 3'd5;   // Error: sopfr=5 keywords
            4'd11:   max_idx = 3'd4;   // AI: tau=4 keywords
            default: max_idx = 3'd0;   // Invalid group
        endcase
    end

    // CAM hit: group in range AND index within group size
    wire cam_hit = is_hlang_range &&
                   (grp_field < SIGMA[3:0]) &&  // group < sigma=12
                   (idx_field < max_idx);        // index < group size

    // ========================================================================
    // ALU operation decode for J2=24 operators
    //
    // J2=24 operators mapped to 5-bit ALU op (2^sopfr=32 slots):
    //   Arithmetic (n=6):  ADD SUB MUL DIV MOD POW   → 0x00-0x05
    //   Comparison (n=6):  EQ NE LT GT LE GE         → 0x06-0x0B
    //   Logical (tau=4):   AND OR NOT XOR             → 0x0C-0x0F
    //   Bitwise (tau=4):   BAND BOR BXOR BNOT         → 0x10-0x13
    //   Assign (phi=2):    MOV MOVI                   → 0x14-0x15
    //   Special (phi=2):   RANGE ARROW                → 0x16-0x17
    //   Total: 6+6+4+4+2+2 = 24 = J2(6)
    // ========================================================================

    reg [4:0] alu_op_comb;
    always @(*) begin
        if (!is_hlang_range && primary_op != 6'h00) begin
            // Standard ALU operations (primary opcodes 0x01-0x1F)
            alu_op_comb = primary_op[4:0];
        end else begin
            alu_op_comb = 5'h00;       // NOP / keyword (no ALU)
        end
    end

    // ========================================================================
    // Keyword-to-operation mapping
    //
    // Some keywords trigger specific hardware operations:
    //   own/borrow/move/drop → memory region operations
    //   spawn               → SNN tile trigger
    //   atomic              → bus lock
    //   assert/invariant    → hardware trap
    // ========================================================================

    reg [4:0] kw_hw_action;
    always @(*) begin
        kw_hw_action = 5'h00;  // Default: no hardware action
        if (cam_hit) begin
            case ({grp_field, idx_field})
                // Memory group (5): own=region_lock, borrow=region_share, move=dma, drop=free
                {4'd5, 3'd0}: kw_hw_action = 5'h18; // own   → region lock
                {4'd5, 3'd1}: kw_hw_action = 5'h19; // borrow→ region share
                {4'd5, 3'd2}: kw_hw_action = 5'h1A; // move  → DMA trigger
                {4'd5, 3'd3}: kw_hw_action = 5'h1B; // drop  → free region

                // Concurrency group (6): spawn=launch, channel=fifo, select=mux, atomic=lock
                {4'd6, 3'd0}: kw_hw_action = 5'h1C; // spawn  → SNN/coprocessor launch
                {4'd6, 3'd3}: kw_hw_action = 5'h1D; // atomic → bus lock

                // Proof group (8): assert=trap, invariant=check
                {4'd8, 3'd1}: kw_hw_action = 5'h1E; // assert    → hardware trap
                {4'd8, 3'd2}: kw_hw_action = 5'h1F; // invariant → bounds check

                default: kw_hw_action = 5'h00;
            endcase
        end
    end

    // ========================================================================
    // sigma-tau=8 primitive type accelerator
    //
    // When operand field encodes a type ID (0-7 = sigma-tau=8 types):
    //   0=int, 1=float, 2=bool, 3=char, 4=string, 5=byte, 6=void, 7=any
    // Hardware provides type-width and alignment info in 1 cycle
    // ========================================================================

    reg [4:0] type_width;   // Width in bytes (up to 2^sopfr=32)
    reg [2:0] type_align;   // Alignment requirement (log2)

    always @(*) begin
        case (opr_field[2:0])  // Lower n/phi=3 bits select type
            3'd0: begin type_width = 5'd8;  type_align = 3'd3; end // int:    sigma-tau=8 bytes
            3'd1: begin type_width = 5'd8;  type_align = 3'd3; end // float:  sigma-tau=8 bytes
            3'd2: begin type_width = 5'd1;  type_align = 3'd0; end // bool:   mu=1 byte
            3'd3: begin type_width = 5'd4;  type_align = 3'd2; end // char:   tau=4 bytes (UTF-32)
            3'd4: begin type_width = 5'd24; type_align = 3'd3; end // string: J2=24 bytes (ptr+len+cap)
            3'd5: begin type_width = 5'd1;  type_align = 3'd0; end // byte:   mu=1 byte
            3'd6: begin type_width = 5'd0;  type_align = 3'd0; end // void:   0 bytes
            3'd7: begin type_width = 5'd16; type_align = 3'd3; end // any:    phi^tau=16 bytes (vtable+data)
        endcase
    end

    // ========================================================================
    // Output register (mu=1 cycle latency)
    // ========================================================================

    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            kw_group   <= 4'b0;
            kw_index   <= 3'b0;
            is_keyword <= 1'b0;
            alu_op     <= 5'b0;
            modifier   <= 3'b0;
            operand    <= 8'b0;        // sigma-tau=8 bits
            valid_out  <= 1'b0;
        end else begin
            valid_out  <= valid_in;
            kw_group   <= grp_field;
            kw_index   <= idx_field;
            is_keyword <= cam_hit;
            modifier   <= mod_field;
            operand    <= opr_field;

            // ALU op: keyword hardware action takes priority over standard ALU
            if (cam_hit && kw_hw_action != 5'h00)
                alu_op <= kw_hw_action;
            else
                alu_op <= alu_op_comb;
        end
    end

endmodule
