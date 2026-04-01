//============================================================================
// SNN Izhikevich Neuron Module — n=6 Neuron Ring Topology
//
// Izhikevich model (fixed-point):
//   v' = 0.04*v^2 + 5*v + 140 - u + I
//   u' = a*(b*v - u)
//   if v >= 30 then v = c, u = u + d
//
// N6 Parameter Mapping:
//   Neurons:         n = 6 (ring topology, each connects to neighbors)
//   Parameters:      tau = 4 (a, b, c, d)
//   STDP windows:    tau = 4 time constants
//   Spike threshold: sopfr*n = 30 mV
//   Reset potential: -n*sigma = -65 (approximated as -72 = -n*sigma for EXACT)
//   Ring coupling:   1/n = 1/6 weight per synapse
//
// Fixed-point: Q8.10 format (sigma-phi=10 fractional bits, sigma-tau=8 integer bits)
//   Range: -128.0 to +127.999  (sufficient for Izhikevich dynamics)
//   Resolution: 1/1024 ~ 0.001
//
// Ring topology:
//   Neuron i connects to neuron (i+1)%n and (i-1)%n
//   Excitatory coupling weight: 1/n = 1/6 of spike amplitude
//============================================================================

module snn_izhikevich (
    input  wire        clk,
    input  wire        rst_n,

    // External excitation (applied to neuron 0)
    input  wire [17:0] excitation,     // Q8.10 fixed-point
    input  wire        enable,         // Trigger one simulation step

    // Spike outputs — n=6 neurons
    output wire [5:0]  spikes          // n=6 spike indicators
);

    // ========================================================================
    // N6 Constants
    // ========================================================================
    localparam N       = 6;            // n = 6 neurons
    localparam TAU     = 4;            // tau(6) = 4 parameters
    localparam SIGMA   = 12;           // sigma(6) = 12
    localparam FRAC_W  = 10;           // sigma-phi = 10 fractional bits
    localparam INT_W   = 8;            // sigma-tau = 8 integer bits
    localparam Q_W     = INT_W + FRAC_W; // 18-bit Q8.10

    // ========================================================================
    // Izhikevich parameters in Q8.10 fixed-point
    // tau=4 parameters: a, b, c, d
    // ========================================================================

    // a = 0.02 (regular spiking) → 0.02 * 1024 = 20 (rounded)
    localparam signed [Q_W-1:0] PARAM_A = 18'sd20;

    // b = 0.2 → 0.2 * 1024 = 205
    localparam signed [Q_W-1:0] PARAM_B = 18'sd205;

    // c = -65.0 (reset) → approximated as -n*sigma = -72 for n=6 EXACT
    // -72 * 1024 = -73728
    localparam signed [Q_W-1:0] PARAM_C = -18'sd73728;

    // d = 8.0 (= sigma-tau) → 8 * 1024 = 8192
    localparam signed [Q_W-1:0] PARAM_D = 18'sd8192;

    // Spike threshold = sopfr*n = 30 → 30 * 1024 = 30720
    localparam signed [Q_W-1:0] V_THRESH = 18'sd30720;

    // Constants for v' equation (in Q8.10):
    // 0.04 * 1024 = 41 (rounded from 40.96)
    localparam signed [Q_W-1:0] COEFF_004 = 18'sd41;

    // 5 * 1024 = 5120 (= sopfr * 1024)
    localparam signed [Q_W-1:0] COEFF_5 = 18'sd5120;

    // 140 * 1024 = 143360 — too large for 18-bit!
    // Scale: use 140 in integer part: Q8.10 can hold up to 127.999
    // Solution: compute in wider arithmetic then truncate
    // 140 * 1024 = 143360 → use 32-bit intermediate
    localparam signed [35:0] COEFF_140 = 36'sd143360;

    // Coupling weight: 1/n = 1/6 of spike amplitude
    // Spike amplitude ~= 30 (threshold), so coupling = 30/6 = 5 = sopfr
    // 5 * 1024 = 5120
    localparam signed [Q_W-1:0] COUPLING = 18'sd5120;

    // ========================================================================
    // Neuron state arrays — n=6 neurons
    // ========================================================================

    reg signed [Q_W-1:0] v [0:N-1];   // Membrane potential (n=6)
    reg signed [Q_W-1:0] u [0:N-1];   // Recovery variable (n=6)
    reg [N-1:0]          spike_reg;    // Spike output register (n=6)

    // ========================================================================
    // STDP learning — tau=4 time windows
    // ========================================================================

    // Synaptic weights: n=6 neurons, each has phi=2 connections (left+right ring)
    reg signed [Q_W-1:0] w_ring [0:N-1]; // Weight from neuron i to (i+1)%n

    // Spike timing: last spike time per neuron (sigma-tau=8 bit counter)
    reg [7:0] spike_time [0:N-1];      // sigma-tau=8 bit timestamp

    // STDP time constant (tau=4 in simulation steps)
    localparam STDP_TAU = TAU;         // tau(6) = 4

    // STDP learning rate (1/sigma = 1/12 ≈ 85 in Q8.10)
    localparam signed [Q_W-1:0] STDP_LR = 18'sd85;

    // Global timestep counter
    reg [7:0] timestep;                // sigma-tau=8 bit counter

    // ========================================================================
    // Simulation step — sequential neuron update
    // Uses n=6 cycles per enable pulse (time-multiplexed)
    // ========================================================================

    reg [2:0] neuron_idx;              // Current neuron being processed (0 to n-1)
    reg [1:0] phase;                   // Processing phase: 0=idle, 1=compute, 2=spike, 3=stdp
    reg       step_active;

    // Intermediate computation registers (36-bit for Q16.20 precision)
    reg signed [35:0] v_wide;
    reg signed [35:0] u_wide;
    reg signed [35:0] v_sq;           // v^2 intermediate

    // Ring neighbor indices
    wire [2:0] left_idx  = (neuron_idx == 3'd0) ? 3'd5 : (neuron_idx - 3'd1); // (i-1) mod n=6
    wire [2:0] right_idx = (neuron_idx == 3'd5) ? 3'd0 : (neuron_idx + 3'd1); // (i+1) mod n=6

    integer j;

    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            spike_reg   <= {N{1'b0}};
            neuron_idx  <= 3'd0;
            phase       <= 2'd0;
            step_active <= 1'b0;
            timestep    <= 8'd0;
            v_wide      <= 36'sd0;
            u_wide      <= 36'sd0;
            v_sq        <= 36'sd0;

            for (j = 0; j < N; j = j + 1) begin  // n=6 neurons
                // Initial resting potential: c = -72 (n*sigma)
                v[j]          <= PARAM_C;
                u[j]          <= 18'sd0;
                w_ring[j]     <= COUPLING;         // Initial weight = sopfr in Q8.10
                spike_time[j] <= 8'd0;
            end

        end else begin

            case (phase)
                2'd0: begin // IDLE — wait for enable
                    if (enable) begin
                        phase       <= 2'd1;
                        neuron_idx  <= 3'd0;
                        step_active <= 1'b1;
                        spike_reg   <= {N{1'b0}};  // Clear spikes for new step
                        timestep    <= timestep + 8'd1;
                    end
                end

                2'd1: begin // COMPUTE — v' and u' for current neuron
                    // v' = 0.04*v^2 + 5*v + 140 - u + I
                    // Compute in Q16.20 (36-bit) to avoid overflow

                    // v^2 (Q8.10 * Q8.10 = Q16.20)
                    v_sq <= $signed(v[neuron_idx]) * $signed(v[neuron_idx]);

                    // Next phase: apply equation
                    phase <= 2'd2;
                end

                2'd2: begin // SPIKE — apply equation and check threshold
                    // 0.04 * v^2 (Q16.20 → scale by COEFF_004/1024)
                    // Full equation in Q16.20:
                    v_wide <= (COEFF_004 * v_sq) >>> FRAC_W  // 0.04*v^2 (shift right by frac bits)
                            + (COEFF_5 * $signed({{18{v[neuron_idx][Q_W-1]}}, v[neuron_idx]})) // 5*v
                            + (COEFF_140 <<< FRAC_W)           // 140 in Q16.20
                            - $signed({{18{u[neuron_idx][Q_W-1]}}, u[neuron_idx]}) // -u
                            + ((neuron_idx == 3'd0) ?
                               $signed({{18{excitation[Q_W-1]}}, excitation}) :     // External I to neuron 0
                               36'sd0)
                            // Ring coupling: add spikes from neighbors
                            + ((spike_reg[left_idx]) ?
                               $signed({{18{w_ring[left_idx][Q_W-1]}}, w_ring[left_idx]}) : 36'sd0)
                            + ((spike_reg[right_idx]) ?
                               $signed({{18{w_ring[neuron_idx][Q_W-1]}}, w_ring[neuron_idx]}) : 36'sd0);

                    // u' = a*(b*v - u)
                    u_wide <= (PARAM_A * ($signed(PARAM_B) * $signed(v[neuron_idx]) / 18'sd1024
                              - $signed(u[neuron_idx]))) >>> FRAC_W;

                    phase <= 2'd3;
                end

                2'd3: begin // STDP — update state and advance to next neuron
                    // Update v with Euler step (already computed as v' delta)
                    // New v = old v + dt * v' (dt=1 for simplicity)
                    if ($signed(v[neuron_idx]) >= $signed(V_THRESH)) begin
                        // SPIKE: reset
                        v[neuron_idx] <= PARAM_C;              // v = c = -n*sigma = -72
                        u[neuron_idx] <= u[neuron_idx] + PARAM_D; // u = u + d = u + sigma-tau=8
                        spike_reg[neuron_idx] <= 1'b1;
                        spike_time[neuron_idx] <= timestep;

                        // STDP: potentiate synapses from recent pre-synaptic spikes
                        // Check left neighbor timing
                        if ((timestep - spike_time[left_idx]) < STDP_TAU &&  // Within tau=4 window
                            spike_time[left_idx] != 8'd0) begin
                            w_ring[left_idx] <= w_ring[left_idx] + STDP_LR; // Potentiate
                        end
                        // Check right neighbor timing
                        if ((timestep - spike_time[right_idx]) < STDP_TAU &&
                            spike_time[right_idx] != 8'd0) begin
                            w_ring[neuron_idx] <= w_ring[neuron_idx] + STDP_LR; // Potentiate
                        end
                    end else begin
                        // No spike: update v normally (truncate to Q8.10)
                        v[neuron_idx] <= v[neuron_idx] + v_wide[Q_W-1:0];
                        u[neuron_idx] <= u[neuron_idx] + u_wide[Q_W-1:0];
                    end

                    // Advance to next neuron or finish
                    if (neuron_idx == N[2:0] - 3'd1) begin  // Last neuron (n-1=5)
                        phase       <= 2'd0;                 // Back to idle
                        step_active <= 1'b0;
                    end else begin
                        neuron_idx <= neuron_idx + 3'd1;
                        phase      <= 2'd1;                  // Next neuron
                    end
                end

                default: phase <= 2'd0;
            endcase
        end
    end

    assign spikes = spike_reg;

endmodule
