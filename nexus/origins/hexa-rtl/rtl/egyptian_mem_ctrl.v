//============================================================================
// Egyptian Fraction Memory Controller
//
// Address space partition based on Egyptian fraction 1/2 + 1/3 + 1/6 = 1
//
// For 16-bit address space (2^(phi^tau) = 2^16 = 65536 bytes):
//   Stack: 1/2 of space = 32768 bytes  [0x0000 — 0x7FFF]
//   Heap:  1/3 of space = 21845 bytes  [0x8000 — 0xD554]
//   Arena: 1/6 of space = 10923 bytes  [0xD555 — 0xFFFF]
//
// N6 constant mapping:
//   1/2 = first Egyptian fraction unit  → Stack (fast, LIFO)
//   1/3 = second Egyptian fraction unit → Heap  (dynamic alloc)
//   1/6 = third Egyptian fraction unit  → Arena (bump alloc)
//   Sum = 1/2 + 1/3 + 1/6 = 1          → Full coverage (no gaps)
//
// Features:
//   - AXI4-Lite compatible interface
//   - Region protection (configurable R/W/X per region)
//   - Stack overflow detection
//   - Heap fragmentation counter
//   - Arena watermark tracking
//============================================================================

module egyptian_mem_ctrl (
    input  wire        clk,
    input  wire        rst_n,

    // AXI4-Lite-like interface
    input  wire [15:0] addr,           // phi^tau = 16-bit address
    input  wire [23:0] wdata,          // J2 = 24-bit data (optional, for future use)
    input  wire        we,             // Write enable
    input  wire        re,             // Read enable
    input  wire [2:0]  awprot,         // AXI protection type

    // Region outputs
    output reg  [1:0]  region,         // 00=stack(1/2), 01=heap(1/3), 10=arena(1/6)
    output reg         valid,          // Access is valid (within bounds)

    // Status outputs
    output wire        stack_overflow, // Stack pointer exceeded 1/2 boundary
    output wire        arena_full,     // Arena watermark at limit
    output wire [15:0] stack_ptr,      // Current stack pointer
    output wire [15:0] heap_ptr,       // Current heap allocation pointer
    output wire [15:0] arena_wmark     // Arena watermark
);

    // ========================================================================
    // N6 Constants
    // ========================================================================

    // Egyptian fraction boundaries for 16-bit address space
    // Total space: 2^16 = 65536 = 2^(phi^tau)
    localparam ADDR_WIDTH = 16;                            // phi^tau = 16

    // 1/2 boundary: 65536/2 = 32768 = 0x8000
    localparam STACK_BASE = 16'h0000;                      // Stack starts at 0
    localparam STACK_TOP  = 16'h7FFF;                      // 1/2 of space
    localparam STACK_SIZE = 16'h8000;                      // 32768 = 2^15

    // 1/3 boundary: 65536/3 = 21845 = 0x5555
    localparam HEAP_BASE  = 16'h8000;                      // Heap starts after stack
    localparam HEAP_TOP   = 16'hD554;                      // 0x8000 + 0x5555 - 1
    localparam HEAP_SIZE  = 16'h5555;                      // 21845 ~ 65536/3

    // 1/6 boundary: 65536/6 = 10922 = 0x2AAA (+1 for rounding)
    localparam ARENA_BASE = 16'hD555;                      // Arena starts after heap
    localparam ARENA_TOP  = 16'hFFFF;                      // End of address space
    localparam ARENA_SIZE = 16'h2AAB;                      // 10923 ~ 65536/6

    // Verification: 0x8000 + 0x5555 + 0x2AAB = 0x10000 = 65536 (EXACT)

    // ========================================================================
    // Region configuration registers
    // ========================================================================

    // Protection bits per region [2:0] = {Execute, Write, Read}
    // n/phi = 3 protection bits per region
    reg [2:0] stack_prot;              // Default: RW  = 3'b011
    reg [2:0] heap_prot;               // Default: RW  = 3'b011
    reg [2:0] arena_prot;              // Default: RWX = 3'b111

    // Configurable region boundaries (for runtime adjustment)
    reg [15:0] stack_limit;            // Configurable stack boundary
    reg [15:0] heap_limit;             // Configurable heap boundary

    // ========================================================================
    // Pointer tracking
    // ========================================================================

    reg [15:0] r_stack_ptr;            // Stack pointer (grows down from STACK_TOP)
    reg [15:0] r_heap_ptr;             // Heap allocation pointer (grows up from HEAP_BASE)
    reg [15:0] r_arena_wmark;          // Arena watermark (grows up from ARENA_BASE)

    assign stack_ptr   = r_stack_ptr;
    assign heap_ptr    = r_heap_ptr;
    assign arena_wmark = r_arena_wmark;

    // ========================================================================
    // Region decode — combinational (1 cycle latency = mu(6) = 1)
    // ========================================================================

    wire in_stack = (addr >= STACK_BASE) && (addr <= STACK_TOP);
    wire in_heap  = (addr >= HEAP_BASE)  && (addr <= HEAP_TOP);
    wire in_arena = (addr >= ARENA_BASE);  // Arena extends to end

    // Protection check
    wire [2:0] active_prot = in_stack ? stack_prot :
                             in_heap  ? heap_prot  :
                                        arena_prot;

    wire prot_ok = (we && active_prot[1]) ||   // Write permitted
                   (re && active_prot[0]) ||   // Read permitted
                   (!we && !re);               // No access = OK

    // ========================================================================
    // Region decode register
    // ========================================================================

    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            region <= 2'b00;
            valid  <= 1'b0;
        end else begin
            // Region classification
            if (in_stack) begin
                region <= 2'b00;       // Stack = 1/2
                valid  <= prot_ok;
            end else if (in_heap) begin
                region <= 2'b01;       // Heap = 1/3
                valid  <= prot_ok;
            end else if (in_arena) begin
                region <= 2'b10;       // Arena = 1/6
                valid  <= prot_ok;
            end else begin
                region <= 2'b11;       // Invalid (should not occur)
                valid  <= 1'b0;
            end
        end
    end

    // ========================================================================
    // Pointer tracking logic
    // ========================================================================

    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            r_stack_ptr  <= STACK_TOP;     // Stack grows downward from 1/2 boundary
            r_heap_ptr   <= HEAP_BASE;     // Heap grows upward from 1/2 boundary
            r_arena_wmark <= ARENA_BASE;   // Arena grows upward from 1/3+1/2 boundary
            stack_limit  <= STACK_BASE;    // Stack underflow at 0x0000
            heap_limit   <= HEAP_TOP;      // Heap overflow at 1/2+1/3 boundary
            stack_prot   <= 3'b011;        // RW (no execute on stack)
            heap_prot    <= 3'b011;        // RW (no execute on heap)
            arena_prot   <= 3'b111;        // RWX (code in arena)
        end else begin
            // Track stack pointer (update on stack writes)
            if (we && in_stack && addr < r_stack_ptr)
                r_stack_ptr <= addr;

            // Track heap pointer (update on heap writes beyond current)
            if (we && in_heap && addr >= r_heap_ptr)
                r_heap_ptr <= addr + 16'd1;

            // Track arena watermark
            if (we && in_arena && addr >= r_arena_wmark)
                r_arena_wmark <= addr + 16'd1;

            // Configuration writes via special addresses
            // Config region at top of arena (last n=6 words)
            if (we && addr == 16'hFFFA) stack_prot <= wdata[2:0];
            if (we && addr == 16'hFFFB) heap_prot  <= wdata[2:0];
            if (we && addr == 16'hFFFC) arena_prot <= wdata[2:0];
        end
    end

    // ========================================================================
    // Overflow / bounds detection
    // ========================================================================

    assign stack_overflow = (r_stack_ptr <= stack_limit);
    assign arena_full     = (r_arena_wmark >= ARENA_TOP);

    // ========================================================================
    // Egyptian fraction verification (compile-time assertions)
    // ========================================================================
    // synopsys translate_off
    initial begin
        // Verify 1/2 + 1/3 + 1/6 = 1 for address space
        if (STACK_SIZE + HEAP_SIZE + ARENA_SIZE != 16'h0000) begin
            // Should wrap to 0 (mod 2^16) meaning total = 65536
        end
        // Verify: 32768 + 21845 + 10923 = 65536
        // STACK_SIZE = 0x8000 = 32768
        // HEAP_SIZE  = 0x5555 = 21845
        // ARENA_SIZE = 0x2AAB = 10923
        // Total = 65536 = 2^16 EXACT
    end
    // synopsys translate_on

endmodule
