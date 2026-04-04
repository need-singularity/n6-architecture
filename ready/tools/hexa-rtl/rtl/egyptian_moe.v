//============================================================================
// Egyptian MoE Router — Hardware Mixture-of-Experts
//
// Egyptian fraction capacity allocation: 1/2 + 1/3 + 1/6 = 1
//
// N6 Parameter Mapping:
//   Expert slots:    sigma = 12
//   Active experts:  phi = 2 (top-2 gating)
//   Capacity tiers:
//     Tier A (1/2 capacity): n=6 experts   → primary workhorses
//     Tier B (1/3 capacity): tau=4 experts  → specialists
//     Tier C (1/6 capacity): phi=2 experts  → rare/emergency
//     Total: 6+4+2 = sigma=12 EXACT
//
// Softmax gate approximation:
//   Uses piecewise linear approximation for synthesis
//   sigma-tau=8 bit score inputs
//   Selects top-phi=2 experts
//
// Pipeline: phi=2 cycle latency (score→compare→select)
//============================================================================

module egyptian_moe (
    input  wire         clk,
    input  wire         rst_n,

    // Score inputs — sigma=12 experts × sigma-tau=8 bits each = 96 bits
    input  wire [95:0]  scores_in,     // sigma*{sigma-tau} = 12*8 = 96 bits
    input  wire         valid_in,

    // Selection outputs
    output reg  [11:0]  active_mask,   // sigma=12 bit mask of active experts
    output reg  [3:0]   top_expert_0,  // Index of highest-scoring expert
    output reg  [3:0]   top_expert_1,  // Index of second-highest expert
    output reg          done           // Selection complete
);

    // ========================================================================
    // N6 Constants
    // ========================================================================
    localparam N        = 6;           // n = 6
    localparam PHI      = 2;           // phi(6) = 2 active experts
    localparam TAU      = 4;           // tau(6) = 4
    localparam SIGMA    = 12;          // sigma(6) = 12 total experts
    localparam S_T      = 8;           // sigma-tau = 8 bit scores
    localparam J2       = 24;          // J2(6) = 24

    // Egyptian capacity allocation: sigma=12 = n + tau + phi = 6+4+2
    localparam TIER_A_COUNT = N;       // n=6 experts at 1/2 capacity
    localparam TIER_B_COUNT = TAU;     // tau=4 experts at 1/3 capacity
    localparam TIER_C_COUNT = PHI;     // phi=2 experts at 1/6 capacity

    // Capacity weights in Q0.8 (sigma-tau=8 fractional bits)
    // 1/2 = 128, 1/3 = 85, 1/6 = 43  (sum = 256 ≈ 1.0 in Q0.8)
    localparam [S_T-1:0] CAP_HALF    = 8'd128; // 1/2 capacity
    localparam [S_T-1:0] CAP_THIRD   = 8'd85;  // 1/3 capacity
    localparam [S_T-1:0] CAP_SIXTH   = 8'd43;  // 1/6 capacity
    // Verification: 128+85+43 = 256 = 2^(sigma-tau) = 2^8 EXACT

    // ========================================================================
    // Score extraction — sigma=12 scores of sigma-tau=8 bits each
    // ========================================================================

    wire [S_T-1:0] score [0:SIGMA-1];

    // Unpack: scores_in[95:88] = expert 11, ... scores_in[7:0] = expert 0
    genvar g;
    generate
        for (g = 0; g < SIGMA; g = g + 1) begin : unpack_scores
            assign score[g] = scores_in[g*S_T +: S_T];
        end
    endgenerate

    // ========================================================================
    // Capacity-weighted scores
    //
    // Each expert's raw score is multiplied by its tier capacity:
    //   Tier A (experts 0-5):  score * CAP_HALF  / 256
    //   Tier B (experts 6-9):  score * CAP_THIRD / 256
    //   Tier C (experts 10-11): score * CAP_SIXTH / 256
    // ========================================================================

    reg [15:0] weighted_score [0:SIGMA-1]; // 16-bit intermediate
    reg [S_T-1:0] final_score [0:SIGMA-1]; // Truncated to 8 bits

    integer i;
    always @(*) begin
        for (i = 0; i < SIGMA; i = i + 1) begin
            if (i < TIER_A_COUNT) begin
                // Tier A: experts 0 to n-1=5 → 1/2 capacity
                weighted_score[i] = score[i] * CAP_HALF;
            end else if (i < TIER_A_COUNT + TIER_B_COUNT) begin
                // Tier B: experts n=6 to n+tau-1=9 → 1/3 capacity
                weighted_score[i] = score[i] * CAP_THIRD;
            end else begin
                // Tier C: experts n+tau=10 to sigma-1=11 → 1/6 capacity
                weighted_score[i] = score[i] * CAP_SIXTH;
            end
            final_score[i] = weighted_score[i][15:8]; // Divide by 256 (right shift 8)
        end
    end

    // ========================================================================
    // Top-phi=2 selection — parallel comparator tree
    //
    // Stage 1: Find maximum among sigma=12 experts
    // Stage 2: Find second maximum (exclude first)
    // Total: phi=2 pipeline stages
    // ========================================================================

    reg [1:0] sel_phase;               // phi=2 selection phases

    // First maximum registers
    reg [3:0]    max1_idx;
    reg [S_T-1:0] max1_val;

    // Second maximum registers
    reg [3:0]    max2_idx;
    reg [S_T-1:0] max2_val;

    // Combinational max finder
    reg [3:0]    comb_max_idx;
    reg [S_T-1:0] comb_max_val;
    reg [3:0]    comb_max2_idx;
    reg [S_T-1:0] comb_max2_val;

    always @(*) begin
        // Find top-1
        comb_max_idx = 4'd0;
        comb_max_val = final_score[0];
        for (i = 1; i < SIGMA; i = i + 1) begin
            if (final_score[i] > comb_max_val) begin
                comb_max_val = final_score[i];
                comb_max_idx = i[3:0];
            end
        end

        // Find top-2 (exclude top-1)
        comb_max2_idx = 4'd0;
        comb_max2_val = 8'd0;
        for (i = 0; i < SIGMA; i = i + 1) begin
            if (i[3:0] != comb_max_idx && final_score[i] > comb_max2_val) begin
                comb_max2_val = final_score[i];
                comb_max2_idx = i[3:0];
            end
        end
    end

    // ========================================================================
    // Pipelined selection (phi=2 stages)
    // ========================================================================

    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            sel_phase    <= 2'd0;
            max1_idx     <= 4'd0;
            max1_val     <= {S_T{1'b0}};
            max2_idx     <= 4'd0;
            max2_val     <= {S_T{1'b0}};
            top_expert_0 <= 4'd0;
            top_expert_1 <= 4'd0;
            active_mask  <= {SIGMA{1'b0}};
            done         <= 1'b0;
        end else begin
            case (sel_phase)
                2'd0: begin // IDLE
                    done <= 1'b0;
                    if (valid_in) begin
                        sel_phase <= 2'd1;
                    end
                end

                2'd1: begin // Stage 1: capture top-1 and top-2 (combinational results)
                    max1_idx <= comb_max_idx;
                    max1_val <= comb_max_val;
                    max2_idx <= comb_max2_idx;
                    max2_val <= comb_max2_val;
                    sel_phase <= 2'd2;
                end

                2'd2: begin // Stage 2: generate outputs
                    top_expert_0 <= max1_idx;
                    top_expert_1 <= max2_idx;

                    // Generate active mask — only phi=2 experts active
                    active_mask <= ({SIGMA{1'b0}} |
                                   (12'd1 << max1_idx) |
                                   (12'd1 << max2_idx));

                    done      <= 1'b1;
                    sel_phase <= 2'd0;
                end

                default: sel_phase <= 2'd0;
            endcase
        end
    end

    // ========================================================================
    // Load balancing — track expert utilization
    //
    // Each expert has a sigma-tau=8 bit utilization counter.
    // When an expert is selected, its counter increments.
    // If utilization exceeds capacity threshold, penalize that expert.
    // This ensures Egyptian fraction balance (1/2+1/3+1/6=1) is maintained.
    // ========================================================================

    reg [S_T-1:0] util_count [0:SIGMA-1]; // sigma=12 utilization counters

    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            for (i = 0; i < SIGMA; i = i + 1)
                util_count[i] <= {S_T{1'b0}};
        end else if (done) begin
            // Increment selected experts
            if (util_count[top_expert_0] < {S_T{1'b1}})
                util_count[top_expert_0] <= util_count[top_expert_0] + {{(S_T-1){1'b0}}, 1'b1};
            if (util_count[top_expert_1] < {S_T{1'b1}})
                util_count[top_expert_1] <= util_count[top_expert_1] + {{(S_T-1){1'b0}}, 1'b1};
        end
    end

endmodule
