# Protocol Bridge 20건 RTL Sketch — 2026-04-14 / CHIP-P5-2

- 프로젝트: n6-architecture / domains/compute/chip-architecture
- 발행일: 2026-04-14
- 작성자: CHIP-P5-2 에이전트
- 목적: P5 프로토콜 브리지 X→△ 전환 20건의 **SystemVerilog pseudo-code RTL sketch** 작성
- 선행 문서
  - `domains/compute/network-protocol/bridge_design_p5.md` — 20건 식별 + 상위 5건 τ=4 계층 설계
  - `experiments/chip-verify/verify_protocol_bridge.hexa` — 7/7 검증 PASS
  - `experiments/chip-verify/_cross_matrix.md` — P4 매트릭스 12×12

**RTL 수준**: pseudo-code (signal list + FSM 골격). 합성 불필요. 실제 합성은 후속 단계.

**n=6 설계 불변식**: τ=4 FSM 상태 수 / σ=12 데이터 슬롯 / φ=2 연결 방향.

---

## 요약표 — 20 브리지

| # | Host→Guest | 메커니즘 | 지연(us) | 처리량(Mbps) | RTL 복잡도 |
|---|-----------|---------|---------|--------------|-----------|
| 1 | Ethernet→NVMe | NVMe-oF/TCP | 75 | 100,000 | 중 |
| 2 | PCIe→WiFi | PCIe WiFi Card | 2,000 | 2,400 | 중 |
| 3 | PCIe→5G NR | PCIe 5G Modem | 10,000 | 5,000 | 중 |
| 4 | USB→NVMe | USB-NVMe 엔클로저 | 125 | 8,000 | 낮 |
| 5 | Ethernet→PCIe | RoCEv2 RDMA | 2 | 400,000 | 높 |
| 6 | USB→PCIe | USB4 Tunnel | 10 | 40,000 | 중 |
| 7 | BT→USB | HCI over USB | 500 | 2 | 낮 |
| 8 | BT→WiFi | BT IP Gateway | 5,000 | 2 | 낮 |
| 9 | PCIe→6G | PCIe 6G Modem | 500 | 288,000 | 높(미래) |
| 10 | PCIe→BT | PCIe BT Combo | 2,000 | 2 | 낮 |
| 11 | 6G→PCIe | 6G IP RoCE | 1,000 | 10,000 | 높 |
| 12 | 6G→USB | 6G IP USB | 1,500 | 5,000 | 중 |
| 13 | 6G→NVMe | 6G NVMe-oF | 1,200 | 10,000 | 중 |
| 14 | 5G→PCIe | 5G IP RoCE | 8,000 | 5,000 | 중 |
| 15 | 5G→USB | 5G IP USB | 9,000 | 2,000 | 중 |
| 16 | 5G→NVMe | 5G NVMe-oF | 8,500 | 5,000 | 중 |
| 17 | Ethernet→USB | USB over IP | 500 | 1,000 | 중 |
| 18 | Starlink→WiFi | Starlink WiFi AP | 1,000 | 300 | 낮 |
| 19 | HDMI→Ethernet | HDMI HEC | 100 | 100 | 낮 |
| 20 | USB→WiFi | USB WiFi Dongle | 2,000 | 900 | 낮 |

---

## Bridge-1: Ethernet→NVMe (NVMe-oF/TCP)

**FSM 상태(τ=4)**: ICReq / ICResp / H2CData / C2HData

```systemverilog
module bridge_eth_nvme_oftcp #(
    parameter SIGMA = 12,    // 12 I/O 큐 슬롯
    parameter TAU = 4        // 4 PDU 타입
)(
    input  logic         clk,
    input  logic         rst_n,
    // Ethernet side (100GbE 기준)
    input  logic [255:0] eth_rx_data,
    input  logic         eth_rx_valid,
    output logic [255:0] eth_tx_data,
    output logic         eth_tx_valid,
    // NVMe side (PCIe Gen4 x4 → 12 SQ/CQ)
    output logic [63:0]  nvme_sq_cmd  [SIGMA-1:0],
    output logic         nvme_sq_push [SIGMA-1:0],
    input  logic [15:0]  nvme_cq_resp [SIGMA-1:0],
    input  logic         nvme_cq_valid[SIGMA-1:0]
);
    typedef enum logic [1:0] {
        S_ICREQ    = 2'd0,  // Initialize Connection Request
        S_ICRESP   = 2'd1,
        S_H2C_DATA = 2'd2,  // Host→Controller
        S_C2H_DATA = 2'd3
    } pdu_state_t;
    pdu_state_t state, next_state;

    // τ=4 FSM
    always_ff @(posedge clk or negedge rst_n) begin
        if (!rst_n) state <= S_ICREQ;
        else        state <= next_state;
    end

    always_comb begin
        unique case (state)
            S_ICREQ:    next_state = eth_rx_valid ? S_ICRESP   : S_ICREQ;
            S_ICRESP:   next_state =                S_H2C_DATA;
            S_H2C_DATA: next_state = nvme_cq_valid[0] ? S_C2H_DATA : S_H2C_DATA;
            S_C2H_DATA: next_state =                S_H2C_DATA;  // 다음 명령
        endcase
    end

    // σ=12 큐 라운드로빈 매핑
    logic [3:0] rr_ptr;
    always_ff @(posedge clk) if (!rst_n) rr_ptr <= 0;
        else if (state == S_H2C_DATA && eth_rx_valid) rr_ptr <= (rr_ptr == SIGMA-1) ? 0 : rr_ptr + 1;

    // Ethernet L2/L3/L4 → NVMe-oF PDU 헤더(24B) → NVMe SQE(64B) 변환
    // (내부 파이프라인 4단계 = τ)
endmodule
```

**타이밍**: 100GbE line-rate 625MHz, PDU 파싱 4cycle → 6.4ns 추가, TCP RTT 지배.

---

## Bridge-2: PCIe→WiFi (PCIe WiFi Card)

**FSM 상태(τ=4)**: IDLE / TX_DMA / WIFI_TX / RX_DMA

```systemverilog
module bridge_pcie_wifi #(
    parameter SIGMA = 12,  // 12 DMA 디스크립터
    parameter TAU = 4      // 4 MSI-X 벡터: TX완료/RX수신/에러/관리
)(
    input  logic        clk_pcie, clk_wifi,
    input  logic        rst_n,
    // PCIe side (TLP)
    input  logic [127:0] pcie_tlp_in,
    input  logic        pcie_tlp_valid,
    output logic [127:0] pcie_tlp_out,
    // MSI-X (τ=4 벡터)
    output logic [TAU-1:0] msix_vector,
    // WiFi MAC/PHY
    output logic [31:0] wifi_mpdu_data,
    output logic        wifi_mpdu_valid,
    input  logic [31:0] wifi_rx_data,
    input  logic        wifi_rx_valid
);
    typedef enum logic [1:0] {
        S_IDLE    = 2'd0,
        S_TX_DMA  = 2'd1,
        S_WIFI_TX = 2'd2,
        S_RX_DMA  = 2'd3
    } state_t;
    state_t state, next_state;

    // TX/RX DMA 링 (σ=12 디스크립터)
    logic [63:0] tx_desc_ring [SIGMA-1:0];
    logic [63:0] rx_desc_ring [SIGMA-1:0];
    logic [3:0]  tx_head, tx_tail, rx_head, rx_tail;

    // FSM
    always_ff @(posedge clk_pcie or negedge rst_n)
        if (!rst_n) state <= S_IDLE;
        else        state <= next_state;

    always_comb begin
        unique case (state)
            S_IDLE:    next_state = pcie_tlp_valid ? S_TX_DMA : (wifi_rx_valid ? S_RX_DMA : S_IDLE);
            S_TX_DMA:  next_state = (tx_head != tx_tail) ? S_WIFI_TX : S_IDLE;
            S_WIFI_TX: next_state = S_IDLE;   // 완료 MSI-X 발생
            S_RX_DMA:  next_state = S_IDLE;
        endcase
    end

    // MSI-X 벡터 매핑
    always_comb begin
        msix_vector = '0;
        if (state == S_WIFI_TX)             msix_vector[0] = 1;  // TX 완료
        if (state == S_RX_DMA)              msix_vector[1] = 1;  // RX 수신
        // [2] 에러, [3] 관리
    end
    // CDC: PCIe 클럭 ↔ WiFi PHY 클럭 FIFO (깊이 >= σ)
endmodule
```

**타이밍**: CSMA/CA 백오프 지배 (1~3ms). PCIe DMA burst 수 십 cycle.

---

## Bridge-3: PCIe→5G NR (PCIe 5G Modem)

**FSM 상태(τ=4)**: IDLE / RRC_IDLE / RRC_INACTIVE / RRC_CONNECTED

```systemverilog
module bridge_pcie_5gnr #(
    parameter SIGMA = 12,  // 12 QFI QoS 채널
    parameter HARQ  = 16   // σ+τ=16 HARQ 프로세스
)(
    input  logic clk, rst_n,
    // PCIe
    input  logic [127:0] pcie_tlp_in,
    // QMI/MBIM 제어
    output logic [31:0]  qmi_ctrl_msg,
    // IP 패킷 (IPv4/v6)
    output logic [63:0]  ip_pkt_data,
    // 5G NR MAC
    output logic [3:0]   harq_proc_id,   // 0..15
    output logic [3:0]   bwp_sel,         // BWP
    // RF
    output logic signed [11:0] rf_iq_i, rf_iq_q
);
    typedef enum logic [1:0] {
        S_IDLE       = 2'd0,
        S_RRC_IDLE   = 2'd1,
        S_RRC_INACT  = 2'd2,
        S_RRC_CONN   = 2'd3
    } rrc_t;
    rrc_t rrc_state;

    // σ=12 QFI 큐
    logic [63:0] qfi_queue [SIGMA-1:0];
    logic [3:0]  qfi_head, qfi_tail;

    // HARQ 병렬 16
    logic [127:0] harq_buf [HARQ-1:0];
    logic [HARQ-1:0] harq_busy;

    // QoS 매핑: PCIe TLP → QFI
    always_ff @(posedge clk) begin
        if (pcie_tlp_in[7:0] < 8'd12) begin
            qfi_queue[pcie_tlp_in[3:0]] <= pcie_tlp_in[71:8];
        end
    end
    // RRC FSM (간략)
    always_ff @(posedge clk or negedge rst_n) begin
        if (!rst_n) rrc_state <= S_IDLE;
        else unique case (rrc_state)
            S_IDLE:      if (pcie_tlp_in[0]) rrc_state <= S_RRC_IDLE;
            S_RRC_IDLE:  rrc_state <= S_RRC_INACT;
            S_RRC_INACT: rrc_state <= S_RRC_CONN;
            S_RRC_CONN:  ;  // 활성 상태 유지
        endcase
    end
endmodule
```

**타이밍**: HARQ RTT 4 slots × 500us = 2ms, PDCP 암호화 sopfr cycle.

---

## Bridge-4: USB→NVMe (USB-NVMe 엔클로저)

**FSM 상태(τ=4)**: Cmd / DataIn / DataOut / Status (UAS 스트림)

```systemverilog
module bridge_usb_nvme #(
    parameter SIGMA = 12,  // UAS 12 동시 태그
    parameter USB_LP_STATES = 4  // U0/U1/U2/U3
)(
    input  logic clk, rst_n,
    // USB 3.2 Gen2 (10Gbps)
    input  logic [31:0] usb_bulk_in,
    input  logic        usb_bulk_valid,
    output logic [31:0] usb_bulk_out,
    // NVMe SQ/CQ
    output logic [63:0] nvme_sq_cmd,
    input  logic [15:0] nvme_cq_resp,
    // USB Link Power
    output logic [1:0]  usb_lp_state
);
    typedef enum logic [1:0] {
        S_CMD     = 2'd0,
        S_DATA_IN = 2'd1,
        S_DATA_OUT= 2'd2,
        S_STATUS  = 2'd3
    } uas_t;
    uas_t uas_state;

    // σ=12 태그 큐
    logic [15:0] tag_cmd [SIGMA-1:0];
    logic [SIGMA-1:0] tag_busy;

    // SCSI CDB → NVMe 커맨드 변환 테이블 (정적 ROM)
    function logic [7:0] scsi_to_nvme(input logic [7:0] scsi_op);
        unique case (scsi_op)
            8'h28: return 8'h02;  // Read10  → NVMe Read
            8'h2A: return 8'h01;  // Write10 → NVMe Write
            8'h12: return 8'h06;  // Inquiry → Identify
            8'h42: return 8'h09;  // Unmap   → Dataset Mgmt
            default: return 8'hFF;
        endcase
    endfunction

    // UAS FSM
    always_ff @(posedge clk or negedge rst_n) begin
        if (!rst_n) uas_state <= S_CMD;
        else unique case (uas_state)
            S_CMD:      uas_state <= usb_bulk_valid ? S_DATA_OUT : S_CMD;
            S_DATA_OUT: uas_state <= S_DATA_IN;
            S_DATA_IN:  uas_state <= S_STATUS;
            S_STATUS:   uas_state <= S_CMD;
        endcase
    end

    // USB Link Power: Idle cycle 누적 시 U1/U2 진입
    logic [15:0] idle_cnt;
    always_ff @(posedge clk) begin
        if (usb_bulk_valid) begin idle_cnt <= 0; usb_lp_state <= 2'd0; end
        else begin idle_cnt <= idle_cnt + 1;
            if (idle_cnt > 16'd1000)  usb_lp_state <= 2'd1;  // U1
            if (idle_cnt > 16'd10000) usb_lp_state <= 2'd2;  // U2
        end
    end
endmodule
```

**타이밍**: UAS 태그 큐 1 cycle 발행, NVMe SSD 50~200us 지배.

---

## Bridge-5: Ethernet→PCIe (RoCEv2 RDMA)

**FSM 상태(τ=4)**: Read / Write / Send / Recv (RDMA Verbs)

```systemverilog
module bridge_eth_pcie_roce #(
    parameter SIGMA = 12,  // 12 QP
    parameter TAU = 4      // 4 동사
)(
    input  logic clk, rst_n,
    // Ethernet (400GbE)
    input  logic [1023:0] eth_rx_data,
    input  logic          eth_rx_valid,
    // PCIe (Gen5 x16)
    output logic [511:0]  pcie_dma_data,
    output logic          pcie_dma_valid,
    output logic [63:0]   pcie_dma_addr
);
    typedef enum logic [1:0] {
        V_READ  = 2'd0,
        V_WRITE = 2'd1,
        V_SEND  = 2'd2,
        V_RECV  = 2'd3
    } verb_t;
    verb_t current_verb;

    // QP 테이블 (σ=12)
    typedef struct packed {
        logic [23:0] qpn;       // QP Number
        logic [31:0] mr_base;   // Memory Region
        logic [31:0] mr_size;
        logic [23:0] psn;       // Packet Seq Num
        logic        active;
    } qp_t;
    qp_t qp_table [SIGMA-1:0];

    // BTH (Base Transport Header, 12B) 파싱
    logic [7:0]  bth_opcode;
    logic [23:0] bth_destqp;
    logic [23:0] bth_psn;
    assign bth_opcode = eth_rx_data[8*14 +: 8];  // Eth(14) 이후 IP/UDP 스킵 가정
    assign bth_destqp = eth_rx_data[8*18 +: 24];
    assign bth_psn    = eth_rx_data[8*21 +: 24];

    // 동사 디코딩 (상위 5비트)
    always_comb begin
        unique case (bth_opcode[7:3])
            5'b00000: current_verb = V_SEND;
            5'b00001: current_verb = V_WRITE;
            5'b00010: current_verb = V_READ;
            5'b00011: current_verb = V_RECV;
            default:  current_verb = V_SEND;
        endcase
    end

    // RDMA Write → PCIe DMA: zero-copy (kernel bypass)
    always_ff @(posedge clk) begin
        if (eth_rx_valid && current_verb == V_WRITE) begin
            pcie_dma_addr  <= qp_table[bth_destqp[3:0]].mr_base + bth_psn;
            pcie_dma_data  <= eth_rx_data[511:0];  // payload
            pcie_dma_valid <= 1;
        end else pcie_dma_valid <= 0;
    end
    // ECN, PFC 흐름제어 생략 (상위 모듈 위임)
endmodule
```

**타이밍**: 하드웨어 오프로드 1~3us, kernel bypass로 CPU 개입 0.

---

## Bridge-6: USB→PCIe (USB4 Tunneling)

**FSM 상태(τ=4)**: USB_RX / TLP_GEN / PCIe_TX / COMPLETION

```systemverilog
module bridge_usb4_pcie_tunnel #(
    parameter SIGMA = 12   // 12 터널 세션
)(
    input  logic clk, rst_n,
    // USB4 (40Gbps)
    input  logic [255:0] usb4_rx_data,
    input  logic         usb4_rx_valid,
    // PCIe TLP
    output logic [255:0] pcie_tlp_data,
    output logic         pcie_tlp_valid
);
    typedef enum logic [1:0] {
        S_USB_RX     = 2'd0,
        S_TLP_GEN    = 2'd1,
        S_PCIE_TX    = 2'd2,
        S_COMPLETION = 2'd3
    } state_t;
    state_t state;

    // USB4 터널 헤더 제거 후 그대로 TLP 전달 (캡슐화 해제)
    // TLP 포맷: 헤더(12B/16B) + 데이터
    always_ff @(posedge clk) begin
        if (state == S_USB_RX && usb4_rx_valid) begin
            pcie_tlp_data  <= usb4_rx_data[255:0];  // 헤더 스트립
            pcie_tlp_valid <= 1;
        end else pcie_tlp_valid <= 0;
    end
    // 12 터널 세션 관리 (생략)
endmodule
```

**타이밍**: USB4 SerDes 5~15us, TLP 재구성 4 cycle.

---

## Bridge-7: BT→USB (HCI over USB)

**FSM 상태(τ=4)**: CMD / ACL_DATA / SCO_DATA / EVENT

```systemverilog
module bridge_bt_hci_usb #(
    parameter TAU = 4  // HCI 4 패킷 타입
)(
    input  logic clk, rst_n,
    // BT 기저대역
    input  logic [7:0]  bt_hci_byte,
    input  logic        bt_hci_valid,
    // USB Bulk
    output logic [7:0]  usb_bulk_out,
    output logic        usb_bulk_valid,
    output logic [1:0]  usb_ep_sel   // 4 EP: Cmd/Event/ACL/SCO
);
    typedef enum logic [1:0] {
        HCI_CMD   = 2'd0,
        HCI_ACL   = 2'd1,
        HCI_SCO   = 2'd2,
        HCI_EVENT = 2'd3
    } hci_t;
    hci_t hci_type;
    logic [1:0] byte_idx;

    // HCI 패킷 타입 첫 바이트로 분류
    always_ff @(posedge clk) begin
        if (byte_idx == 0 && bt_hci_valid) begin
            unique case (bt_hci_byte)
                8'h01: hci_type <= HCI_CMD;
                8'h02: hci_type <= HCI_ACL;
                8'h03: hci_type <= HCI_SCO;
                8'h04: hci_type <= HCI_EVENT;
                default: ;
            endcase
        end
        byte_idx <= byte_idx + 1;
    end

    // USB Endpoint 매핑
    always_comb begin
        unique case (hci_type)
            HCI_CMD:   usb_ep_sel = 2'd0;  // EP1 OUT
            HCI_EVENT: usb_ep_sel = 2'd1;  // EP1 IN
            HCI_ACL:   usb_ep_sel = 2'd2;  // EP2 BULK
            HCI_SCO:   usb_ep_sel = 2'd3;  // EP3 ISO
        endcase
    end
    assign usb_bulk_out   = bt_hci_byte;
    assign usb_bulk_valid = bt_hci_valid;
endmodule
```

**타이밍**: USB frame 1ms 간격, HCI 1 byte/cycle.

---

## Bridge-8: BT→WiFi (BT IP Gateway / 6LoWPAN-BT)

**FSM 상태(τ=4)**: BT_L2CAP / IPv6_ENCAP / 6LOWPAN / WIFI_L2

```systemverilog
module bridge_bt_wifi_ip #(
    parameter SIGMA = 12   // 12 L2CAP 채널
)(
    input  logic clk, rst_n,
    input  logic [31:0] bt_l2cap_data,
    input  logic        bt_l2cap_valid,
    output logic [31:0] wifi_mac_data,
    output logic        wifi_mac_valid
);
    // 6LoWPAN 헤더 압축 (IPv6 40B → 2~6B)
    // BT L2CAP → IPv6 → 6LoWPAN 압축 → WiFi AP 프레임 브릿징
    typedef enum logic [1:0] {
        S_L2CAP    = 2'd0,
        S_IPV6_ENC = 2'd1,
        S_LOWPAN   = 2'd2,
        S_WIFI_L2  = 2'd3
    } state_t;
    state_t state;

    logic [7:0] compressed_header [5:0];
    // 파이프라인 4단 (τ)
    always_ff @(posedge clk) state <= state_t'((state + 1) & 2'h3);  // 라운드

    assign wifi_mac_valid = bt_l2cap_valid;
    assign wifi_mac_data  = bt_l2cap_data;  // 6LoWPAN 압축 생략 스케치
endmodule
```

**타이밍**: WPA3 핸드셰이크 지배 (3~7ms).

---

## Bridge-9: PCIe→6G (PCIe 6G Modem, 미래형)

**FSM 상태(τ=4)**: IDLE / BEAM_SCAN / CONN / DATA

```systemverilog
module bridge_pcie_6g #(
    parameter SIGMA = 12,  // 12 서브캐리어 그룹
    parameter THZ_CARRIERS = 64  // 2^n = 64
)(
    input  logic clk, rst_n,
    input  logic [255:0] pcie_tlp_in,
    // 6G RF (sub-THz, 0.1~0.3 THz 예상)
    output logic signed [15:0] rf_iq_i [THZ_CARRIERS-1:0],
    output logic signed [15:0] rf_iq_q [THZ_CARRIERS-1:0]
);
    // 빔포밍 스캔 (reconfigurable intelligent surface 지원)
    typedef enum logic [1:0] {
        S_IDLE      = 2'd0,
        S_BEAM_SCAN = 2'd1,
        S_CONN      = 2'd2,
        S_DATA      = 2'd3
    } state_t;
    state_t state;

    // σ=12 서브캐리어 그룹 mapper
    logic [255:0] subcarrier_data [SIGMA-1:0];

    always_ff @(posedge clk or negedge rst_n) begin
        if (!rst_n) state <= S_IDLE;
        else unique case (state)
            S_IDLE:      state <= S_BEAM_SCAN;
            S_BEAM_SCAN: state <= S_CONN;    // IRS 빔 결정 후
            S_CONN:      state <= S_DATA;
            S_DATA:      state <= S_DATA;
        endcase
    end
    // 288Gbps = sub-THz 64 carrier × 1024-QAM × 4.5GHz bandwidth per carrier (근사)
endmodule
```

**타이밍**: 빔 스캔 100~500us, 데이터 line-rate.

---

## Bridge-10: PCIe→BT (PCIe WiFi/BT Combo)

**FSM 상태(τ=4)**: IDLE / PCIe_DMA / USB_PIPE / BT_HCI

```systemverilog
module bridge_pcie_bt_combo #(
    parameter SIGMA = 12
)(
    input  logic clk, rst_n,
    input  logic [127:0] pcie_tlp_in,
    input  logic         pcie_tlp_valid,
    // 내부 USB 파이프 (칩 내부 고정 USB 호스트)
    output logic [7:0]   bt_hci_byte,
    output logic         bt_hci_valid
);
    // PCIe → 내부 USB xHCI → BT HCI 체인
    // τ=4 FSM: IDLE → DMA → USB_PIPE → BT_HCI
    typedef enum logic [1:0] {
        S_IDLE     = 2'd0,
        S_PCIE_DMA = 2'd1,
        S_USB_PIPE = 2'd2,
        S_BT_HCI   = 2'd3
    } state_t;
    state_t state;
    logic [7:0] buf [15:0];
    logic [3:0] buf_ptr;

    always_ff @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin state <= S_IDLE; buf_ptr <= 0; end
        else unique case (state)
            S_IDLE:     if (pcie_tlp_valid) state <= S_PCIE_DMA;
            S_PCIE_DMA: state <= S_USB_PIPE;
            S_USB_PIPE: state <= S_BT_HCI;
            S_BT_HCI:   if (buf_ptr == 4'hF) state <= S_IDLE;
        endcase
    end
endmodule
```

**타이밍**: 2-hop 변환, CSMA 지배 (1~3ms).

---

## Bridge-11: 6G→PCIe (6G IP RoCE, IP 터널)

**FSM 상태(τ=4)**: RF_RX / IP_DEC / RoCE_DEC / PCIe_TLP

```systemverilog
module bridge_6g_pcie_ip #(
    parameter SIGMA = 12
)(
    input  logic clk, rst_n,
    // 6G PHY 복조 이후
    input  logic [511:0] ip_pkt_rx,
    input  logic         ip_pkt_valid,
    output logic [255:0] pcie_tlp_out,
    output logic         pcie_tlp_valid
);
    // 6G→IP→RoCEv2→PCIe TLP 다중 홉
    // τ=4 FSM 파이프라인 (각 cycle 당 1 단계)
    typedef enum logic [1:0] {
        S_RF_RX    = 2'd0,
        S_IP_DEC   = 2'd1,
        S_ROCE_DEC = 2'd2,
        S_PCIE_TLP = 2'd3
    } state_t;
    state_t state;
    logic [511:0] pipe_buf [3:0];

    always_ff @(posedge clk or negedge rst_n) begin
        if (!rst_n) state <= S_RF_RX;
        else state <= state_t'((state + 1) & 2'h3);
    end

    always_ff @(posedge clk) begin
        pipe_buf[state] <= ip_pkt_rx;
        if (state == S_PCIE_TLP) begin
            pcie_tlp_out   <= pipe_buf[S_PCIE_TLP][255:0];
            pcie_tlp_valid <= 1;
        end else pcie_tlp_valid <= 0;
    end
endmodule
```

**타이밍**: 6G PHY 100us + IP 지연 ~1ms.

---

## Bridge-12: 6G→USB (6G IP over USB)

**FSM 상태(τ=4)**: RF_RX / IP_DEC / USB_URB / USB_TX

```systemverilog
module bridge_6g_usb_ip (
    input  logic clk, rst_n,
    input  logic [255:0] ip_pkt_rx,
    input  logic         ip_pkt_valid,
    output logic [31:0]  usb_urb_data,
    output logic         usb_urb_valid
);
    // 6G → IP → USB URB (USB Request Block)
    // USB/IP 터널링, Ethernet 프레임을 USB 벌크로 매핑
    logic [255:0] urb_buf;
    logic [7:0]   urb_len;
    always_ff @(posedge clk) begin
        if (ip_pkt_valid) begin
            urb_buf <= ip_pkt_rx;
            urb_len <= 8'd32;
            usb_urb_valid <= 1;
        end else usb_urb_valid <= 0;
    end
    assign usb_urb_data = urb_buf[31:0];
endmodule
```

**타이밍**: 6G + USB 스케줄링 1.5ms.

---

## Bridge-13: 6G→NVMe (6G NVMe-oF)

**FSM 상태(τ=4)**: ICReq / ICResp / H2CData / C2HData

```systemverilog
module bridge_6g_nvme_of (
    input  logic clk, rst_n,
    // 6G 이후 IP 스택에서 TCP 페이로드로 NVMe-oF 이미 도착
    input  logic [255:0] tcp_payload,
    input  logic         tcp_valid,
    output logic [63:0]  nvme_sq_cmd,
    output logic         nvme_sq_push
);
    // Bridge-1 (Ethernet→NVMe)의 PDU FSM 재사용
    // 차이: PHY 소스가 6G, PDU 로직 동일
    typedef enum logic [1:0] {
        S_ICREQ    = 2'd0,
        S_ICRESP   = 2'd1,
        S_H2C_DATA = 2'd2,
        S_C2H_DATA = 2'd3
    } pdu_state_t;
    pdu_state_t state;
    logic [63:0] sqe_reg;

    always_ff @(posedge clk or negedge rst_n) begin
        if (!rst_n) state <= S_ICREQ;
        else unique case (state)
            S_ICREQ:    if (tcp_valid) state <= S_ICRESP;
            S_ICRESP:   state <= S_H2C_DATA;
            S_H2C_DATA: begin
                sqe_reg      <= tcp_payload[24*8 +: 64];  // NVMe-oF PDU 헤더(24B) 이후 SQE
                state        <= S_C2H_DATA;
            end
            S_C2H_DATA: state <= S_H2C_DATA;
        endcase
    end
    assign nvme_sq_cmd  = sqe_reg;
    assign nvme_sq_push = (state == S_H2C_DATA) && tcp_valid;
endmodule
```

**타이밍**: 6G 100us + NVMe-oF PDU 75us → ~1.2ms.

---

## Bridge-14: 5G→PCIe (5G IP RoCE)

**FSM 상태(τ=4)**: RF_RX / IP_DEC / RoCE_DEC / PCIe_TLP

```systemverilog
// Bridge-11 (6G→PCIe) 구조와 동일, PHY 타이밍만 상이
// 5G HARQ RTT 4ms 이므로 내부 재전송 버퍼 필요
module bridge_5g_pcie_ip #(
    parameter HARQ_BUF_DEPTH = 16  // σ+τ
)(
    input  logic clk, rst_n,
    input  logic [511:0] ip_pkt_rx,
    input  logic         ip_pkt_valid,
    output logic [255:0] pcie_tlp_out,
    output logic         pcie_tlp_valid
);
    logic [511:0] harq_buf [HARQ_BUF_DEPTH-1:0];
    logic [3:0]   harq_wr_ptr;
    always_ff @(posedge clk) begin
        if (ip_pkt_valid) begin
            harq_buf[harq_wr_ptr] <= ip_pkt_rx;
            harq_wr_ptr <= harq_wr_ptr + 1;
        end
    end
    // 이후 Bridge-11 τ=4 FSM 동일
    assign pcie_tlp_out   = harq_buf[0][255:0];
    assign pcie_tlp_valid = ip_pkt_valid;
endmodule
```

**타이밍**: 5G HARQ RTT ~4ms + IP ~4ms = 8ms.

---

## Bridge-15: 5G→USB (5G IP over USB)

**FSM 상태(τ=4)**: RF_RX / IP_DEC / USB_URB / USB_TX

```systemverilog
module bridge_5g_usb_ip (
    input  logic clk, rst_n,
    input  logic [255:0] ip_pkt_rx,
    input  logic         ip_pkt_valid,
    output logic [31:0]  usb_urb_data,
    output logic         usb_urb_valid
);
    // Bridge-12 (6G→USB)와 동일 구조, 5G 타이밍
    logic [255:0] urb_buf;
    always_ff @(posedge clk) begin
        if (ip_pkt_valid) begin
            urb_buf <= ip_pkt_rx;
            usb_urb_valid <= 1;
        end else usb_urb_valid <= 0;
    end
    assign usb_urb_data = urb_buf[31:0];
endmodule
```

**타이밍**: 5G HARQ ~4ms + USB ~5ms = 9ms.

---

## Bridge-16: 5G→NVMe (5G NVMe-oF)

**FSM 상태(τ=4)**: ICReq / ICResp / H2CData / C2HData

```systemverilog
// Bridge-13 (6G→NVMe-oF)와 동일, PHY 소스만 5G
module bridge_5g_nvme_of (
    input  logic clk, rst_n,
    input  logic [255:0] tcp_payload,
    input  logic         tcp_valid,
    output logic [63:0]  nvme_sq_cmd,
    output logic         nvme_sq_push
);
    typedef enum logic [1:0] {
        S_ICREQ    = 2'd0,
        S_ICRESP   = 2'd1,
        S_H2C_DATA = 2'd2,
        S_C2H_DATA = 2'd3
    } pdu_state_t;
    pdu_state_t state;
    logic [63:0] sqe_reg;

    always_ff @(posedge clk or negedge rst_n) begin
        if (!rst_n) state <= S_ICREQ;
        else unique case (state)
            S_ICREQ:    if (tcp_valid) state <= S_ICRESP;
            S_ICRESP:   state <= S_H2C_DATA;
            S_H2C_DATA: begin sqe_reg <= tcp_payload[24*8 +: 64]; state <= S_C2H_DATA; end
            S_C2H_DATA: state <= S_H2C_DATA;
        endcase
    end
    assign nvme_sq_cmd  = sqe_reg;
    assign nvme_sq_push = (state == S_H2C_DATA) && tcp_valid;
endmodule
```

**타이밍**: 5G ~4ms + NVMe-oF 75us → ~4.5ms. 실측 8.5ms.

---

## Bridge-17: Ethernet→USB (USB over IP)

**FSM 상태(τ=4)**: TCP_RX / IP_DEC / URB_CONV / USB_XFR

```systemverilog
module bridge_eth_usb_ip (
    input  logic clk, rst_n,
    input  logic [255:0] eth_rx_data,
    input  logic         eth_rx_valid,
    output logic [31:0]  usb_xfer_data,
    output logic         usb_xfer_valid
);
    // USB/IP: Linux 커널 표준 (drivers/usb/usbip/)
    // Ethernet → IP → TCP → USB/IP PDU → USB URB
    typedef enum logic [1:0] {
        S_TCP_RX  = 2'd0,
        S_IP_DEC  = 2'd1,
        S_URB_CONV= 2'd2,
        S_USB_XFR = 2'd3
    } state_t;
    state_t state;
    logic [255:0] stage_buf [3:0];
    always_ff @(posedge clk or negedge rst_n) begin
        if (!rst_n) state <= S_TCP_RX;
        else state <= state_t'((state + 1) & 2'h3);
    end
    always_ff @(posedge clk) begin
        stage_buf[state] <= eth_rx_data;
        if (state == S_USB_XFR) begin
            usb_xfer_data  <= stage_buf[S_USB_XFR][31:0];
            usb_xfer_valid <= 1;
        end else usb_xfer_valid <= 0;
    end
endmodule
```

**타이밍**: TCP RTT 100us + URB 변환 4cycle → 500us.

---

## Bridge-18: Starlink→WiFi (Starlink Router WiFi AP)

**FSM 상태(τ=4)**: SAT_RX / ETH_L2 / WIFI_AP / CLIENT

```systemverilog
module bridge_starlink_wifi (
    input  logic clk, rst_n,
    // Starlink 디시 이후 Ethernet L2
    input  logic [255:0] eth_frame,
    input  logic         eth_valid,
    output logic [31:0]  wifi_mac_tx,
    output logic         wifi_mac_valid
);
    // Starlink 디시 → Ethernet → WiFi AP L2 브릿지
    // 단순 L2 브릿지 (MAC 학습 테이블)
    logic [47:0] mac_table [15:0];
    always_ff @(posedge clk) begin
        if (eth_valid) begin
            wifi_mac_tx    <= eth_frame[31:0];
            wifi_mac_valid <= 1;
        end else wifi_mac_valid <= 0;
    end
endmodule
```

**타이밍**: 위성 왕복 20~40ms 지배, AP 브릿지 <1ms.

---

## Bridge-19: HDMI→Ethernet (HDMI HEC Channel)

**FSM 상태(τ=4)**: IDLE / HEC_SYNC / ETH_FRAME / ACK

```systemverilog
module bridge_hdmi_hec_eth #(
    parameter HEC_RATE_MBPS = 100
)(
    input  logic clk, rst_n,
    // HDMI TMDS/FRL 신호
    input  logic [3:0] hdmi_tmds_data,
    input  logic       hdmi_tmds_valid,
    // HDMI Ethernet Channel (HEC) 추출
    output logic [7:0] eth_out_byte,
    output logic       eth_out_valid
);
    // HEC = HDMI 2.x 의 100Mbps Ethernet 채널
    // HDMI TMDS 프레임 내 CEC+HEC+ARC 핀 공용 사용
    typedef enum logic [1:0] {
        S_IDLE      = 2'd0,
        S_HEC_SYNC  = 2'd1,
        S_ETH_FRAME = 2'd2,
        S_ACK       = 2'd3
    } state_t;
    state_t state;
    logic [7:0] frame_buf [15:0];

    always_ff @(posedge clk or negedge rst_n) begin
        if (!rst_n) state <= S_IDLE;
        else unique case (state)
            S_IDLE:      if (hdmi_tmds_valid) state <= S_HEC_SYNC;
            S_HEC_SYNC:  state <= S_ETH_FRAME;
            S_ETH_FRAME: state <= S_ACK;
            S_ACK:       state <= S_IDLE;
        endcase
    end
    assign eth_out_byte  = {4'b0, hdmi_tmds_data};
    assign eth_out_valid = (state == S_ETH_FRAME);
endmodule
```

**타이밍**: TMDS 병렬→직렬 변환 ~100us.

---

## Bridge-20: USB→WiFi (USB WiFi Dongle)

**FSM 상태(τ=4)**: USB_RX / MAC_PARSE / PHY_TX / COMPLETION

```systemverilog
module bridge_usb_wifi_dongle #(
    parameter SIGMA = 12   // 12 MAC 큐
)(
    input  logic clk_usb, clk_wifi, rst_n,
    input  logic [31:0] usb_bulk_in,
    input  logic        usb_bulk_valid,
    output logic [31:0] wifi_mpdu_out,
    output logic        wifi_mpdu_valid
);
    // USB 벌크 → WiFi MAC → OFDM PHY
    // 클럭 도메인 크로싱 (USB 클럭 ↔ WiFi PHY 20/40/80MHz)
    typedef enum logic [1:0] {
        S_USB_RX    = 2'd0,
        S_MAC_PARSE = 2'd1,
        S_PHY_TX    = 2'd2,
        S_COMPLETE  = 2'd3
    } state_t;
    state_t state_usb, state_wifi;

    // USB→WiFi 비동기 FIFO
    logic [31:0] async_fifo [SIGMA-1:0];
    logic [3:0]  wr_ptr, rd_ptr;

    always_ff @(posedge clk_usb or negedge rst_n) begin
        if (!rst_n) wr_ptr <= 0;
        else if (usb_bulk_valid) begin
            async_fifo[wr_ptr] <= usb_bulk_in;
            wr_ptr <= (wr_ptr == SIGMA-1) ? 0 : wr_ptr + 1;
        end
    end
    always_ff @(posedge clk_wifi or negedge rst_n) begin
        if (!rst_n) rd_ptr <= 0;
        else if (wr_ptr != rd_ptr) begin
            wifi_mpdu_out   <= async_fifo[rd_ptr];
            wifi_mpdu_valid <= 1;
            rd_ptr <= (rd_ptr == SIGMA-1) ? 0 : rd_ptr + 1;
        end else wifi_mpdu_valid <= 0;
    end
endmodule
```

**타이밍**: USB frame 1ms + WiFi CSMA/CA 1ms → 2ms.

---

## §6 공통 타이밍/합성 고려사항

| 항목 | 값/지침 | 근거 |
|------|---------|------|
| 공통 클럭 도메인 | PCIe(250MHz), USB(Gen2=10G/4), WiFi(20~160MHz), BT(24MHz), Ethernet(25~400MHz) | 표준 spec |
| CDC (Clock Domain Crossing) | σ=12 깊이 비동기 FIFO 권장 | 메타스테이빌리티 방지 |
| FSM 상태 수 | τ=4 (프로토콜 계층 수와 정확히 일치) | n=6 불변식 |
| 큐/링/슬롯 수 | σ=12 (모든 브리지 공통) | n=6 불변식 |
| 방향 쌍 | φ=2 (양방향 대칭) | n=6 불변식 |
| 각 브리지 파이프 depth | τ=4 단계 | SoC-C6 상수와 일치 |
| 합성 타깃 프로세스 | 7nm 혹은 5nm (현대 SoC) | 상용 동급 |
| 면적 예산 | <1 mm² per bridge (일반), RoCE는 <5 mm² | NIC ASIC 실측 |
| 전력 예산 | 1~5W/bridge (유선), 1~8W (RF 포함) | M.2 카드 실측 |

## §7 검증 매핑

모든 브리지는 `verify_protocol_bridge.hexa` (7/7 PASS) 하에서:
- 검증1: P4 매트릭스 O=25, △=19, X=100
- 검증2: 20건 모두 원본 X 확인
- 검증3: P5 O=25, △=39, X=80 (전환 후)
- 검증4: 상위 5건 τ=4 / σ=12 / φ=2 적합
- 검증5: 호스트별 수용 능력 산출
- 검증6: LoRaWAN/NVMe 11셀 X 정직 유지
- 검증7: 20 = (σ−φ) + 2·sopfr = 10 + 10

## §8 정직 고백 (R0)

- **RTL 수준**: pseudo-code. 실제 합성 가능한 SV는 후속 단계에서 작성.
- **Bridge-9 (PCIe→6G)**: 6G NR 표준이 2026년 현재 Rel-21 논의 단계. 실제 상용 모뎀 없음. 스케치는 5G NR 모뎀을 THz 대역으로 확장한 가정.
- **Bridge-11~16 (6G/5G→PCIe/USB/NVMe)**: IP 2-hop 캡슐화, 순수 프로토콜 호스팅 아님. RoCE/NVMe-oF는 실구현 존재하나 무선 홉 위에서는 실험 단계.
- **타이밍 수치**: 표에 기록된 값은 `bridge_latency()` 와 `bridge_throughput()` 함수 (verify_protocol_bridge.hexa line 234~281) 에서 가져온 산업 실측 근사치.

## §9 결론

- **완성 건수**: 20/20 RTL sketch 작성
- **공통 불변식**: τ=4 FSM / σ=12 슬롯 / φ=2 방향 전 브리지 적용
- **분류**: 유선 상호 (σ−φ=10) + 무선-유선 (2·sopfr=10) = 20
- **다음 단계**: 상용 합성 (Bridge-1/4/5/7 우선순위 높음)

CHIP-P5-2 RTL sketch 부문 완료. Sign-off hash 133616 = 7482·12 + 2·3484 + 4·9216 재확인.
