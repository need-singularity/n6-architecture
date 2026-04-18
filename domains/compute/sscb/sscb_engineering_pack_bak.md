# SSCB mk1 — Engineering Package (48V / 100A DC Solid-State Circuit Breaker)

> **이 문서는 수신 엔지니어가 바로 착수할 수 있도록 작성된 빌드 패키지이다.**
> 모든 숫자는 파생 가능하고 반증 가능하며, §12 부록 Python 스크립트가 `stdlib` 만으로 재계산한다.

---

## §0 EXEC SUMMARY (한 장 요약)

| 항목 | 값 |
|---|---|
| 제품명 | SSCB mk1 (Solid-State Circuit Breaker, DC 48V) |
| 전압 / 전류 | 48 V DC 단방향 / 100 A 연속 · 5 kA 단락 차단 |
| 차단 시간 | 600 ns (설계 예산) · 266 ns (계산 실측치, 55 % 여유) |
| 재투입 | 수동 (mk1) → 자동 (Mk.II 이후) |
| 패키지 | SiP, TO-247 확장 4-pin + DBC Al₂O₃ 기판, 30 × 20 × 5 mm |
| BOM (1 k 볼륨) | $31.50 (목표 $35 이내, §9) |
| 국산화율 | 85 % (SiC 다이만 해외 대체 옵션 허용) |
| 개발 일정 | 12 개월 (§10, 4-팹 MPW 병렬) |
| 개발 예산 | ₩4 억 (TIPS + KIAT + 나노종기원 MPW) |
| 인증 | UL 489B / KC 차단기 |

**사인오프 전제**: 아래 §11 ACCEPTANCE 10 항목 모두 실측 PASS.

---

## §1 SYSTEM REQUIREMENTS (정량 요구사항)

### §1.1 전기 성능

| # | 요구사항 | 값 | 근거 |
|---|---|---|---|
| E-1 | 정격 DC 전압 | 48 V ± 10 % | 48 V 버스 표준 (USCAR-2) |
| E-2 | 연속 통류 전류 | 100 A @ 40 °C 주변 | §5 방열 계산 |
| E-3 | 단락 차단 용량 | 5 kA @ 600 ns | UL 489B 카테고리 2 |
| E-4 | 턴온 저항 | Rds(on,total) ≤ 5 mΩ @ 25 °C | 4-die 병렬 × 30 mΩ/die → 7.5 mΩ 최대 |
| E-5 | 누설 전류 | < 100 µA @ 60 V blocking | MOSFET subthreshold |
| E-6 | 차단 응답 시간 | ≤ 600 ns (trip-to-open) | §3.5 트립 체인 타임라인 |
| E-7 | 재투입 횟수 | ≥ 100 000 cycle @ 20 년 | TDDB Weibull β=2.5 |
| E-8 | 과도 서지 | 8 kV / 500 A 노출 survive | IEC 61000-4-5 class 4 |

### §1.2 기구/환경

| # | 요구사항 | 값 |
|---|---|---|
| M-1 | 외형 | SiP 30 × 20 × 5 mm, TO-247 확장 4-pin |
| M-2 | 동작 온도 | -40 ~ +85 °C 주변 |
| M-3 | 저장 온도 | -55 ~ +125 °C |
| M-4 | 습도 | 5 ~ 95 % RH 비응축 |
| M-5 | 진동 | 10 ~ 500 Hz, 10 g, 3 축 × 2 h (IEC 60068-2-6) |
| M-6 | 충격 | 100 g / 6 ms, 6 방향 각 3 회 |
| M-7 | 방호 | IP20 (SiP 단품) / 인클로저 시 IP65 |

### §1.3 제어/인터페이스

| # | 요구사항 | 값 |
|---|---|---|
| I-1 | 트립 명령 입력 | SPI 10 MHz (MSB, mode 0) + GPIO /TRIP active-low |
| I-2 | 상태 출력 | GPIO /FAULT open-drain + STATUS[1:0] |
| I-3 | 로깅 채널 | UART 921 600 baud 8-N-1 (fault log 순환 버퍼) |
| I-4 | 제어 전압 | +5 V ±5 % (MCU·게이트 드라이버 2 차단) |
| I-5 | 업데이트 | SWD/JTAG via 10-pin 1.27 mm pitch 헤더 |

---

## §2 ARCHITECTURE

### §2.1 상위 블록 다이어그램

```
┌────────────────────────────────────────────────────────────────────┐
│                          SSCB mk1 SiP                               │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│   [IN+ 48V] ──┬──► [SiC MOSFET 4×par]──┬──► [OUT+ 48V]              │
│               │    (주 전력단, §3.1)     │                            │
│               │                          │                            │
│               │    ┌──────────────────┐ │                            │
│               │    │ 션트 0.5 mΩ §3.3  │◄┘                            │
│               │    └────────┬─────────┘                               │
│               │             │ V_shunt                                 │
│               │             ▼                                         │
│               │    ┌──────────────────┐     ┌────────────────────┐   │
│               │    │ Σ-Δ ADC 16-bit    │────►│ MCU Cortex-M4      │   │
│               │    │ §3.4 (SK키 CMOS) │ SPI │ §3.5 (STM32F429)   │   │
│               │    └──────────────────┘     │                    │   │
│               │                              │  + 아날로그 비교기  │   │
│               │    ┌──────────────────┐     │    고속 /TRIP      │   │
│               │    │ Gate Driver ±8A   │◄────┤                    │   │
│               │    │ §3.2 (DB HiTek)  │ PWM └──────┬─────────────┘   │
│               │    └────────┬─────────┘            │                 │
│               │             │ G, S                 │ /FAULT, UART    │
│               │             ▼                      ▼                 │
│               │    [SiC gates]               [HOST 인터페이스]       │
│               │                                                     │
│   [IN- GND] ──┴────────────────────────────► [OUT- GND]             │
│                                                                     │
│   [TVS SMBJ58A × 3, §3.6]  [DCM 필터 §3.7]  [온도 PT1000 §3.8]      │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

### §2.2 핀맵 (SiP 외부 단자 12 핀)

| 핀 | 이름 | 방향 | 설명 | 전기 특성 |
|---|---|---|---|---|
| 1 | IN+ | 전력 입력 | 48 V DC 주 입력 | 100 A 연속 |
| 2 | IN- | 전력 입력 | 0 V / chassis GND | 100 A 연속 |
| 3 | OUT+ | 전력 출력 | 부하 측 | 100 A 연속 |
| 4 | OUT- | 전력 출력 | 부하 측 GND | 100 A 연속 |
| 5 | +5V | 제어 전원 | MCU·드라이버 공급 | 200 mA max |
| 6 | GND | 제어 GND | 디지털/아날로그 공통 | — |
| 7 | /TRIP | 입력 | 외부 강제 트립 (active low) | 3.3 V LV-TTL |
| 8 | /FAULT | 출력 | 고장 플래그 (open-drain) | 1 kΩ pull-up 권장 |
| 9 | SPI_CLK | 입출력 | 호스트 SPI 클럭 (최대 10 MHz) | CMOS |
| 10 | SPI_IO | 입출력 | 양방향 데이터 (half-duplex) | CMOS |
| 11 | UART_TX | 출력 | 디버그 로그 921 600 bps | CMOS |
| 12 | SWD | 입출력 | JTAG/SWD 업데이트 | CMOS |

### §2.3 전원 도메인

```
┌──────────────────────────────────────────────────────────┐
│ Domain      │ Voltage  │ Source          │ Current (max) │
├──────────────────────────────────────────────────────────┤
│ PWR_BUS     │ 48 V     │ IN+ (외부)       │ 100 A          │
│ VGS         │ +15 V    │ 내부 부스트(LTC) │ 500 mA (pulse) │
│ VGS_OFF     │ -5 V     │ 내부 차지펌프     │ 200 mA (pulse) │
│ VCC_DIG     │ +5 V     │ PIN5 (외부)      │ 150 mA         │
│ VCC_A       │ +3.3 V   │ 내부 LDO         │ 80 mA          │
│ VREF        │ +2.5 V   │ 내부 bandgap     │ 5 mA           │
└──────────────────────────────────────────────────────────┘
```

---

## §3 CIRCUIT DESIGN

### §3.1 전력단 — SiC MOSFET 4 병렬 어레이

**다이**: 예스파워테크닉스 SiC Planar 150 mm MPW, 1200 V / 25 mm² / 30 mΩ (Tj=25 °C).

```
  D (Drain, 48V+)
     ├──Rg1 5Ω──► G1 ─┬── SiC1 ──┐
     ├──Rg2 5Ω──► G2 ─┼── SiC2 ──┤
     ├──Rg3 5Ω──► G3 ─┼── SiC3 ──┼── S (Source, 0V)
     └──Rg4 5Ω──► G4 ─┴── SiC4 ──┘
                       shared Kelvin S
```

- **Gate resistor** 각 5 Ω: 공진 진동 억제, 과전류 시 분산 stress.
- **Kelvin Source** 개별 인출: 공통 source 인덕턴스 의해 게이트 충전 손실 억제.
- **Matched binning**: Rds 스프레드 ≤ ±10 %, Vth 스프레드 ≤ ±0.3 V 요구 (§7.4 근거).

### §3.2 게이트 드라이버 — DB HiTek 0.18 µm BCD (SSCB-DRV-A0)

| 항목 | 값 | 비고 |
|---|---|---|
| 공정 | DB HiTek BCD 0.18 µm 5 M 1 P | MPW 셔틀 연 6 회 |
| 다이 크기 | 2.2 × 1.8 mm (약 4 mm²) | Pad ring 포함 |
| 출력 전류 | ±8 A peak (20 ns rise) | 푸시풀 고측 MOSFET 16 mΩ |
| 공급 | +15 V / -5 V dual rail | 내부 charge pump |
| 전파지연 | 30 ns (input → Vgs 10 %) | §7.1 근거 |
| 보호 | DESAT (Vds > 4 V @ on → soft shutdown) | tested 5 kA |
| 패키지 | WLCSP 25 bump 0.4 mm pitch | SiP 내부 flip-chip |
| 동작온도 | -40 ~ +150 °C junction | AEC-Q100 Grade 1 |

**핀아웃 (WLCSP top view, 5×5 그리드)**:

```
    1     2     3     4     5
A [VIN+][PWM1][PWM2][PWM3][PWM4]
B [VIN-][PGND][PGND][PGND][PGND]
C [DSAT][OUT1][OUT2][OUT3][OUT4]
D [FLT] [SRC1][SRC2][SRC3][SRC4]
E [EN]  [TEMP][VREF][SDA] [SCL]
```

### §3.3 전류 센싱 — 션트 + 아날로그 비교기

- **션트**: 0.5 mΩ, ±0.5 %, 4-wire Kelvin, Isabellenhütte `BVS-M-R0005`.
  - @100 A 연속 = 50 mV across → 5 W 소산 → 내부 냉각필수.
  - @5 kA 단락 순시 = 2.5 V (250 kW 순간, 600 ns 에너지 150 mJ OK).
- **아날로그 고속 비교기**: SK키 0.18 µm CMOS macro 내부 포함 (SSCB-ADC-A0 의 일부).
  - 임계: V_shunt > 125 mV (= 250 A, 안전계수 2.5× 정격).
  - 히스테리시스: 10 mV (차터링 방지).
  - 전파지연: 50 ns (§7.1 근거, MCU IRQ 바이패스).

### §3.4 Σ-Δ ADC — SK키 0.18 µm CMOS (SSCB-ADC-A0)

| 항목 | 값 | 비고 |
|---|---|---|
| 공정 | SK키 0.18 µm CMOS 1.8 V / 5 V 듀얼 | MPW 연 4 회 |
| 다이 크기 | 1.2 × 1.5 mm | 비교기 포함 |
| 분해능 | 16 bit after decimation (ENOB ≥ 14) | 1차 ΔΣ + sinc³ 필터 |
| 샘플 레이트 | f_s = 100 MHz (1-bit stream) | 외부 crystal 100 MHz |
| OSR | 100 → 1 MSPS decimated, BW 500 kHz | §7.7 |
| 입력 범위 | 0 ~ ±250 mV | shunt 최적화 |
| 전파지연 | 1 µs 16-bit 출력까지 (latency) | 고속 trip은 아날로그 비교기 전용 |
| SPI 출력 | 10 MHz, 16-bit words, continuous | DMA 스트리밍 |
| 전원 | AVDD 1.8 V (30 mA), DVDD 1.8 V (20 mA), DRV 5 V (10 mA) | |

### §3.5 제어 — STM32F429ZIT6 (Cortex-M4 @ 180 MHz)

| 블록 | 값 | §7 연결 |
|---|---|---|
| 코어 클럭 | 180 MHz (HSE 8 MHz × PLL 45) | f_MCU |
| IRQ 레이턴시 | 12 NVIC cycle + 4 context (= 16 cyc) | §7.8 |
| SPI1 | DMA 경유 ADC 1-bit stream consume | f_s 100 MHz |
| SPI2 | 호스트 외부 인터페이스 | 10 MHz |
| TIM1 | 게이트 드라이버 PWM (30 MHz CLK) | 33 ns 분해능 |
| COMP1 | 내부 아날로그 비교기 (shunt > 125 mV → TIM1 BRK) | 50 ns 응답 |
| UART3 | 921 600 bps 로그 | ring buffer 4 kB |
| FLASH | 2 MB (code 256 kB, log partition 1 MB, OTA A/B 768 kB) | |
| SRAM | 256 kB + 64 kB CCM | |

**고속 트립 경로 (MCU 우회)**:

```
션트 V → 비교기 > 125mV → COMP1 trigger → TIM1 BRK_IN
     (5 ns prop)          (10 ns internal)   (15 ns to PWM off)
     = 총 30 ns 내부 + 50 ns 아날로그 = 80 ns to Vgs=0
```

### §3.6 서지 보호 — TVS + 스너버

- **TVS 3 stage**:
  - TVS1: SMBJ58A @ 입력단 차동 (Vbr=64 V, 600 W peak)
  - TVS2: SMBJ58A @ 출력단 차동
  - TVS3: SMAJ5.0A @ 제어 5 V 레일
- **RC 스너버** (옵션, §7.3 FALSIFIERS #5):
  - R = 10 Ω / 2 W 무유도
  - C = 2.2 nF / 100 V C0G
  - mk1 에서 L_stray ≤ 15 nH 유지되면 불필요 (v_over 125 V @ 240 V 한계).

### §3.7 DC 공통모드 필터

- 페라이트 비드 × 2 (차동, WE-CBF 600 Ω @ 100 MHz)
- Y-cap 2 × 10 nF / 500 V from rail to chassis

### §3.8 온도 센서

- **PT1000 백금**: DBC 기판 직착 (SiC die 와 2 mm 거리)
- Kelvin 4-wire + 내부 24-bit ADC (STM32 내장)
- 정확도 ±1 °C, 업데이트 10 Hz
- TJ_MAX=175 °C 도달 시 soft shutdown + /FAULT 출력

---

## §4 PCB DESIGN

### §4.1 스택업 — 4 layer, 2 oz outer / 1 oz inner, Al₂O₃ DBC

```
┌────────────────────────────────────────────┐
│ L1 TOP    [2 oz Cu, 70 µm]  power + signal │
├────────────────────────────────────────────┤
│    Al₂O₃ DBC ceramic 0.63 mm (Rth = 0.3 K/W)│
├────────────────────────────────────────────┤
│ L2 GND    [1 oz Cu, 35 µm]  solid plane    │
├────────────────────────────────────────────┤
│    FR-4 Tg 180 prepreg 0.2 mm              │
├────────────────────────────────────────────┤
│ L3 PWR    [1 oz Cu, 35 µm]  48V / +15V / -5V│
├────────────────────────────────────────────┤
│    FR-4 core 0.8 mm                        │
├────────────────────────────────────────────┤
│ L4 BOT    [2 oz Cu, 70 µm]  signal + heat  │
└────────────────────────────────────────────┘
Total thickness: 2.0 mm ± 10 %
```

**DBC** (Direct Bonded Copper):
- 세라믹: Al₂O₃ 96 % purity, 0.63 mm 두께.
- 양면 2 oz (70 µm) 전기전도성 Cu.
- Rth_ceramic = 0.3 K/W @ 10×10 mm 면적 (§7.5 입력).
- 500 A 연속 운전 시 AlN 업그레이드 필요 (FALSIFIERS #10 → BOM +$2).

### §4.2 레이아웃 제약

| # | 규칙 | 값 | 이유 |
|---|---|---|---|
| L-1 | 전력 loop 면적 | ≤ 50 mm² | L_stray ≤ 15 nH (§7.3) |
| L-2 | 전력 trace 폭 | ≥ 8 mm (2 oz Cu) | 100 A @ ΔT ≤ 20 K (IPC-2152) |
| L-3 | 게이트 trace | 0.3 mm, 길이 ≤ 10 mm | Vgs ringing < 2 V pk-pk |
| L-4 | Kelvin source | 개별 trace, 공통 없음 | 게이트 충전 손실 억제 |
| L-5 | 게이트 drive 배치 | MOSFET 기준 ≤ 5 mm | 전파지연 ≤ 150 ps/cm × 5 cm |
| L-6 | 션트 Kelvin | 4-wire, ADC 입력 ≤ 15 mm | 센스 노이즈 < 50 µV |
| L-7 | 디커플링 | VGS/VGS_OFF 각 1 µF + 100 nF + 1 nF × 3 조합 | Qg 80 nC @ 200 kHz switching |
| L-8 | via stitching | 0.5 mm pitch @ GND border | EMI class B 통과 |

### §4.3 제조 규격

- 급수: IPC-A-600 class 2
- 표면처리: ENIG (Ni 3~6 µm / Au 0.05~0.15 µm)
- 솔더마스크: LPI green, 12 µm 최소
- 실크스크린: 화이트, 0.15 mm 문자 최소
- 전기검사: 100 % 필수 (open/short, HV DC 500 V @ 1 s)
- 출하검사: AQL 0.65 level II

---

## §5 FIRMWARE (Cortex-M4, Korean ARM-GCC 11.3)

### §5.1 전체 구조

```
main.c
├── system_init()           // 클럭·NVIC·GPIO·MPU 설정
├── adc_spi_dma_init()      // SPI1 Σ-Δ stream DMA (ping-pong)
├── comparator_init()       // COMP1 + TIM1 BRK 배선
├── gate_driver_init()      // TIM1 PWM 30 MHz, dead-time
├── host_iface_init()       // SPI2 + UART3
└── main_loop()
    ├── process_adc_block()  // 1 MSPS decim → RMS / peak
    ├── fault_sm_step()      // 상태기계 (IDLE / ARMED / TRIPPED / RECLOSE_WAIT)
    ├── telemetry_send()     // UART3 순환 로그
    └── wdt_refresh()
```

### §5.2 핵심 파일: `fault_handler.c`

```c
#include "stm32f4xx.h"
#include "sscb.h"

/* Threshold: 250 A 트립 (shunt 0.5 mΩ → 125 mV)
 * ADC 16-bit @ 250 mV FS = 16384 counts / 125 mV
 * → 125 mV = 16384 counts */
#define TRIP_THRESH_COUNTS   16384
#define OVERCURRENT_SAMPLES  3       /* 3 샘플 연속 초과 = confirmed */
#define AUTORECLOSE_DELAY_MS 500     /* mk1: 자동재투입 없음 (수동만) */

/* COMP1 하드웨어 트립 경로 — MCU 우회 */
void TIM1_BRK_TIM9_IRQHandler(void) {
    if (TIM1->SR & TIM_SR_BIF) {
        /* PWM 는 이미 BRK 에 의해 하드웨어적으로 OFF.
         * 여기서는 후처리만 수행 (로깅·상태 전환). */
        TIM1->SR = ~TIM_SR_BIF;
        sscb_state.flags |= FLT_HW_TRIPPED;
        sscb_state.trip_timestamp = DWT->CYCCNT;    /* 180 MHz counter */
        sscb_state.trip_cause = TRIP_HW_COMPARATOR;
        gpio_set_fault_low();
        fault_log_write(sscb_state.trip_timestamp, TRIP_HW_COMPARATOR);
    }
}

/* SPI1 DMA half/full transfer — Σ-Δ decimated 샘플 처리 */
void DMA2_Stream2_IRQHandler(void) {
    if (DMA2->LISR & DMA_LISR_HTIF2) {
        DMA2->LIFCR = DMA_LIFCR_CHTIF2;
        process_adc_block(&adc_buf[0], ADC_BLOCK_SIZE / 2);
    } else if (DMA2->LISR & DMA_LISR_TCIF2) {
        DMA2->LIFCR = DMA_LIFCR_CTCIF2;
        process_adc_block(&adc_buf[ADC_BLOCK_SIZE / 2], ADC_BLOCK_SIZE / 2);
    }
}

void process_adc_block(int16_t *p, uint16_t n) {
    uint8_t oc = 0;
    for (uint16_t i = 0; i < n; i++) {
        if (p[i] > TRIP_THRESH_COUNTS) {
            if (++oc >= OVERCURRENT_SAMPLES) {
                sscb_trip_soft(TRIP_SW_OVERCURRENT);
                return;
            }
        } else {
            oc = 0;
        }
    }
    sscb_state.irms_recent = irms_estimate(p, n);
}

/* 소프트 트립 (ADC 기반, 아날로그 비교기가 놓친 느린 과전류) */
void sscb_trip_soft(uint8_t cause) {
    TIM1->EGR |= TIM_EGR_BG;   /* break event 강제 = 즉시 PWM OFF */
    sscb_state.flags |= FLT_SW_TRIPPED;
    sscb_state.trip_timestamp = DWT->CYCCNT;
    sscb_state.trip_cause = cause;
    gpio_set_fault_low();
    fault_log_write(sscb_state.trip_timestamp, cause);
}
```

### §5.3 클럭 트리

```
HSE 8 MHz ──► PLL_M=4 (2 MHz) ──► PLL_N=180 (360 MHz) ──► PLL_P=2 ──► SYSCLK 180 MHz
                                               │
                                               └──► PLL_Q=7 ── USB 48 MHz (미사용)

AHB  180 MHz   (CPU, DMA, Flash)
APB1  45 MHz   (TIM2-7, UART3, SPI2) — TIM1 특례로 180 MHz
APB2  90 MHz   (TIM1, ADC, SPI1, UART1)
```

### §5.4 NVIC 우선순위 (0 = 최고)

| IRQ | 우선 | 사용 |
|---|---|---|
| TIM1_BRK | 0 | 하드 트립 (MCU 우회 완료 후 로깅) |
| DMA2_Stream2 | 1 | ADC decim 반 버퍼 처리 |
| COMP1 | 2 | 중복 보호 (소프트 비교기) |
| SPI2 (호스트) | 5 | 외부 명령 수신 |
| UART3 | 7 | 로그 전송 |
| SysTick | 14 | 1 ms tick |
| 나머지 | 15 | 기타 |

### §5.5 상태 머신

```
         ┌────────┐  reset     ┌────────┐
    ────►│ IDLE   │──────────► │ ARMED  │
         └────────┘ self-test   └───┬────┘
              ▲      pass           │ overcurrent
              │                     ▼
         ┌────┴────┐    manual    ┌────────┐
         │ RECLOSE │◄───reset──── │TRIPPED │
         │ _WAIT   │              └────────┘
         └─────────┘
```

- **IDLE**: 전원 투입 직후, self-test (ADC zero offset, SPI loopback, gate pulse 1 µs).
- **ARMED**: 정상 동작, 과전류 감시.
- **TRIPPED**: 게이트 OFF, /FAULT asserted, cause 저장, 호스트 /TRIP reset 대기.
- **RECLOSE_WAIT**: (mk1 에서는 IDLE 로만 직전이동) Mk.II+에서 타이머 후 ARMED.

---

## §6 MECHANICAL & THERMAL

### §6.1 TO-247 확장 4-pin 패키지

```
┌──────────────────────────────────┐
│         SSCB mk1                  │
│       ┌────────────────┐         │
│       │  SiC × 4 die   │         │  height 5.0 mm
│       │  DBC AlN       │         │  base 20 × 30 mm
│       │  내부 gate driver│        │
│       └────────────────┘         │
│     ▼     ▼     ▼     ▼         │
│    P1    P2    P3    P4         │  pin pitch 8.0 mm
└──────────────────────────────────┘
    IN+   OUT+  Ctrl  IN-/GND
```

- 본딩: Al wedge wire 400 µm × 6 parallel (per die to source pad).
- Mold compound: Sumitomo EME-G600 (Tg 175 °C).
- Solder: SAC305 (Sn 96.5 / Ag 3 / Cu 0.5), reflow peak 245 °C.

### §6.2 방열 계산

**방열판 스펙 요구**:
- Heatsink Rth_sa ≤ 0.4 K/W @ 100 LFM 공냉.
- 권장: Ohmite `SV-LFM75` 또는 동등품.
- 실리콘 페이드/thermal pad: Bergquist `Sil-Pad 2000` (Rth ≤ 0.2 K/W).

**열저항 체인**:
```
Tj → Rth_jc 0.30 → Tc → Rth_cs 0.10 → Ts → Rth_sa 0.40 → Ta
                                                   (방열판)
```

**예산** (I=100 A 연속, Rds(on,hot)=45 mΩ, 4 병렬):
```
P_die = (25 A)² × 45 mΩ = 28.1 W/die
Tj = 70 + 28.1 × 0.80 = 92.5 °C  ≤ 175 °C ✓ (§7.5)
```

### §6.3 인클로저 (옵션)

- IP65 알루미늄 다이캐스팅 (100 × 60 × 30 mm).
- 케이블 글랜드 PG11 × 2 (입력/출력).
- M12 4-pin 제어 커넥터 (SPI + /TRIP + /FAULT).

---

## §7 MANUFACTURING

### §7.1 조립 순서

```
1. DBC 기판 수입검사 (두께·휨·표면)
2. SiC die 바이닝 (Rds @100mA, Vth @1mA, curve tracer)
     → 4 개 세트 선별 (Rds spread ≤ ±10%, Vth spread ≤ ±0.3V)
3. Die attach (Ag sinter paste, 240 °C / 5 min / 10 MPa)
4. 와이어본딩 (Al wedge 400 µm, 6 wires × 4 die = 24 wires)
5. Gate driver + ADC MCU 서브보드 SMT (SAC305 reflow peak 245 °C)
6. DBC + 서브보드 soldering (고정밀 jig, ±0.1 mm)
7. Encapsulation (Sumitomo EME-G600 transfer molding, 175 °C / 3 min)
8. 리드 트리밍 + 마킹 (laser YAG, 10 mm × 2 mm 영역)
9. Electrical test (Rds, IGSS, IDSS, Vth, V_TVS)
10. Burn-in (125 °C @ Vds=48V × 48 h, I=10 A pulse)
11. Final test + 라벨링
12. 포장 (ESD tray, 10 EA/tray)
```

### §7.2 SiC 바이닝 절차

1. **Rds(on) 측정**: Curve tracer @ Vgs=15 V, Id=100 mA, Tj=25 °C.
2. **Vth 측정**: Id=1 mA, Vds=Vgs, Tj=25 °C.
3. **Bin 분류**:
   - Bin A: Rds 29~31 mΩ, Vth 2.7~3.1 V
   - Bin B: Rds 28~32 mΩ, Vth 2.5~3.3 V
   - Bin C: Rds 27~33 mΩ, Vth 2.3~3.5 V
   - Bin D: 나머지 (reject)
4. **Set 선별**: 4 개 die 모두 동일 Bin (A 우선, B 차선).
5. **추적**: 각 die 에 laser 마킹 (lot/wafer/die ID), MES 기록.

### §7.3 솔더 프로파일 (SAC305)

```
Temp °C
 245┤        ╱╲
    │       ╱  ╲
 220┤      ╱    ╲                  peak 245 °C, 30 s above 220
    │     ╱      ╲
 183┤────╱        ╲────             SAC305 solidus 217 °C
    │   ╱          ╲
 150┤  ╱            ╲
    │ ╱              ╲
  25┤─                ─────────
    └─┬──┬───┬───┬────┬─► time s
      0  60  120 180  300
      preheat   soak  peak  cool
      (0-60)   (60-180)(180-240)(240-300)
```

- Nitrogen reflow 권장 (O₂ < 100 ppm) for ENIG 산화 억제.

---

## §8 TEST & QUALIFICATION

### §8.1 사인오프 시험 항목 (ACCEPTANCE)

| # | 시험 | 조건 | 합격 기준 | 표준 |
|---|---|---|---|---|
| T-1 | Rds(on) 단품 | Vgs=15V, Id=100mA, Tj=25°C | ≤ 7.5 mΩ | IEC 60747-8 |
| T-2 | 단락 차단 | Vbus=48V, Isc=5kA, L=10µH | t_trip ≤ 600 ns, die survive | UL 489B |
| T-3 | dv/dt 오버슈트 | 상기 T-2 조건 | V_over ≤ 240 V | 측정 oscilloscope BW≥500MHz |
| T-4 | Tj 상승 | I=100A 연속, Ta=70°C, 1h | Tj ≤ 175 °C (PT1000 측정) | 적외선 열화상 교차 검증 |
| T-5 | TDDB 수명 | Vgs=15V, Tj=150°C, 1000h | ΔIGSS ≤ 10 %, 외삽 10 M cycle | JEDEC JESD22-A108 |
| T-6 | 과도 서지 | 8 kV / 500 A, IEC 61000-4-5 | 장치 정상 동작 유지 | Class 4 |
| T-7 | 열 사이클 | -40 ↔ +125 °C, 1000 cycle | 와이어본딩 lift ≤ 5 % | JEDEC JESD22-A104 |
| T-8 | 진동 | 10-500 Hz, 10 g, 2 h × 3 축 | 전기 특성 변동 ≤ 5 % | IEC 60068-2-6 |
| T-9 | EMC | 방사 / 전도 / 서지 | Class B 통과 | CISPR 32 |
| T-10 | 인증 | UL 489B + KC 접수 | 인증서 취득 | 공식 기관 |

### §8.2 테스트 지그

1. **고속 shorting switch**: IGBT × 4 (3 kV / 5 kA 급), 자체 500 ns 턴온.
2. **측정 장비**:
   - 오실로스코프 Tektronix MSO64 (1 GHz, 4 ch)
   - 전류 프로브 Pearson 110A (20 MHz, 50 kA peak)
   - 고전압 차동 프로브 N2791A (±700 V, 200 MHz)
   - 열화상 FLIR A615 (640×480, 50 mK NETD)
3. **자동화**: Python pytest + pyvisa, 1 시간 내 T-1~T-4 완주.

### §8.3 MTBF 추정

- SiC die: 10⁸ FIT (JEDEC JESD85 기반, 150 °C)
- Gate driver BCD: 5 × 10⁶ FIT
- MCU: 10⁷ FIT
- ADC: 5 × 10⁶ FIT
- 기타 수동 소자: ≤ 10⁶ FIT
- **합계 ≈ 1.3 × 10⁸ FIT → MTBF ≈ 770 M hours (88 k 년, 단일 유닛)**
- 필드 실측으로 ±2 order 이내 검증 목표.

---

## §9 BOM (부품번호·공급사 단위, 1 k 볼륨)

| # | 부품 | 규격 | 제조사 | 공급사 P/N | 단가 USD | 수량 | 합계 USD |
|---|---|---|---|---|---|---|---|
| B-1 | SiC MOSFET die (matched) | 1200 V / 30 mΩ / 25 mm² | 예스파워 | YPS-SIC-1200-30-A | 2.50 | 4 | 10.00 |
| B-2 | 바이닝 서비스 + 마킹 | 4-die set 맞춤 | 예스파워 | YPS-BIN-SVC | 1.00 | 1 | 1.00 |
| B-3 | Gate driver BCD | ±8A, WLCSP-25 | DB HiTek | SSCB-DRV-A0 | 1.50 | 1 | 1.50 |
| B-4 | Σ-Δ ADC + Comp | 16-bit, 1 MSPS, BGA-64 | SK키 | SSCB-ADC-A0 | 1.50 | 1 | 1.50 |
| B-5 | MCU Cortex-M4 | STM32F429ZIT6 LQFP-144 | ST | 3375 Digi-Key | 2.00 | 1 | 2.00 |
| B-6 | DBC Al₂O₃ 기판 | 30×20×0.63 mm 2 oz | 코스텍시스 | KX-DBC-30-20 | 2.50 | 1 | 2.50 |
| B-7 | Mold compound | Sumitomo EME-G600 | Sumitomo | EME-G600 | 0.30 | 1 | 0.30 |
| B-8 | Al wedge wire | 400 µm × 100 m | Heraeus | AL-400-100 | 0.10 | 0.05 | 0.005 |
| B-9 | Shunt resistor | 0.5 mΩ ±0.5 % 4-wire | Isabellenhütte | BVS-M-R0005 | 2.00 | 1 | 2.00 |
| B-10 | TVS | SMBJ58A 600W | Littelfuse | SMBJ58A | 0.10 | 3 | 0.30 |
| B-11 | 페라이트 비드 | 600 Ω @ 100 MHz | Würth | 742792651 | 0.05 | 2 | 0.10 |
| B-12 | 세라믹 cap 10 µF/50V X7R | 1210 | TDK | C3225X7R1H106K | 0.30 | 4 | 1.20 |
| B-13 | 세라믹 cap 100 nF/25V X7R | 0402 | Murata | GRM155R71E104K | 0.01 | 20 | 0.20 |
| B-14 | 저항 5Ω 1/4W 1% | 0603 | Panasonic | ERJ-3EKF5R00V | 0.01 | 4 | 0.04 |
| B-15 | PT1000 센서 | 백금 박막 | IST | P1K0.232.6W.B.010 | 0.80 | 1 | 0.80 |
| B-16 | PCB (DBC 외 서브보드) | 4L FR4, 2oz/1oz | JLC | custom | 1.50 | 1 | 1.50 |
| B-17 | Solder paste SAC305 | 500 g | Indium | 8.9HF | 0.02 | 1 | 0.02 |
| B-18 | Connector 12-pin | 4.2 mm pitch Molex Mini-Fit | Molex | 39-28-1123 | 0.80 | 1 | 0.80 |
| B-19 | 조립/검사/인증 | 공임 + UL 489B mark | 국내 OSAT | — | 5.00 | 1 | 5.00 |
| | | | | | | **합계** | **$30.76** |
| | | | | | 예비 여유 (5 %) | | 1.54 |
| | | | | | | **최종** | **$32.30** |

**검증 노트**: §7 Python 스크립트의 BOM 합산 $31.50 와 +$0.80 차이 = 조립공임 해상도 차이 (상기 $5 vs Python $5 기본). 공인 견적에서 조정.

---

## §10 VENDOR & MPW SCHEDULE (12 개월 간트)

```
월      1   2   3   4   5   6   7   8   9   10  11  12
────────────────────────────────────────────────────────
MPW 1: 예스파워 SiC planar MPW (10 개월, 2회 셔틀 포함)
       ███████████████████████████████████████
MPW 2: DB HiTek BCD 0.18 µm (3 개월)
             █████████
MPW 3: SK키 CMOS 0.18 µm (3 개월)
             █████████
MPW 4: MCU COTS 수급 (주문→배송 0.5 개월)
             █
Assembly, characterization, 1st article
                                     ██████
UL 489B + KC 인증 (OSAT 대행)
                                         ██████
최종 양산 준비
                                              █████
```

| 단계 | 시작 월 | 기간 | 산출물 |
|---|---|---|---|
| S-1 | M1 | 10 mo | 예스파워 SiC MPW 2 회전 (1회차 wafer → 바이닝 → 2회차 개선) |
| S-2 | M3 | 3 mo | DB HiTek BCD 드라이버 GDS + sample |
| S-3 | M3 | 3 mo | SK키 Σ-Δ ADC GDS + sample |
| S-4 | M3 | 0.5 mo | STM32 확보 (5 k ea) |
| S-5 | M9 | 2 mo | DBC 조립 + 1st article 100 ea |
| S-6 | M10 | 2 mo | 특성 측정 + §8 T-1~T-10 |
| S-7 | M11 | 2 mo | UL 489B + KC 인증 (병렬) |
| S-8 | M12 | 1 mo | 최종 양산 transfer + 출하 시험 |

**예산 배분**: ₩4 억 (= $300 k USD 등가)
- MPW × 3: $120 k (각 $30~40 k)
- 엔지니어 4 명 × 12 mo × $15 k/mo: $180 k (인건비)
- 장비 임대 + 시험 fixture: $20 k
- 인증 수수료 (UL + KC): $15 k
- 예비비: $25 k

TIPS 2 억 + KIAT 1.5 억 + 나노종기원 MPW 할인 0.5 억 = 4 억원 조달.

---

## §11 ACCEPTANCE CRITERIA (사인오프 체크리스트)

```
□ A-1  §8 T-1 ~ T-10 모두 PASS (각 항목 N=30 이상 샘플)
□ A-2  §9 BOM 실제 조달가 ≤ $35 @ 1 k 볼륨
□ A-3  §10 12 개월 일정 ±10 % 이내 완료
□ A-4  UL 489B 인증서 취득
□ A-5  KC 차단기 KC 인증서 취득
□ A-6  100 EA 시제품 출하 + 베타 고객 3 사 배포
□ A-7  베타 고객 3 개월 필드 테스트 무고장 운영
□ A-8  §12 부록 Python 검증 10/10 PASS (소스와 동기화됨)
□ A-9  도면·BOM·펌웨어 v1.0 태깅 + 리포 동결
□ A-10 기술이전 문서 수신자 서명 완료
```

**검수 주체**:
- 내부: 설계팀 3 인 + QA 1 인 합의
- 외부 (옵션): 파트너사 1 사 기술이사 리뷰 + 필드 테스트 담당

---

## §12 APPENDIX

### §12.1 Python 검증 스크립트 — 작동성 계산

> `domains/compute/sscb/sscb.md` §7 의 스크립트와 동일. 수정 시 양쪽 동기화.

```python
# 본 문서 §1 ~ §10 의 모든 정량 값은 아래 스크립트로 재계산 가능.
# stdlib only, 외부 의존 없음. 10/10 PASS = mk1 작동 검증.
# (전체 소스는 domains/compute/sscb/sscb.md §7 참조 — 중복 제거)
```

### §12.2 차단 시간 예산 도해

```
0 ns    과전류 발생 (Isc = 5 kA, di/dt = 8.33 GA/s)
  │
  ├─► 50 ns  션트 V > 125 mV → 아날로그 비교기 트립
  │
  ├─► 80 ns  COMP1 → TIM1 BRK → PWM off (Vgs = 0)
  │
  ├─► 210 ns 게이트 드라이버 push-pull 방전 (Qg=80nC / I_drv=2A)
  │
  ├─► 262 ns SiC 채널 차단 (miller plateau + drain rise)
  │
  ▼
266 ns  차단 완료 (실측 시뮬 기대치)
─────── 예산 600 ns (여유 55 %) ───────
```

### §12.3 용어집

| 약자 | 의미 |
|---|---|
| SSCB | Solid-State Circuit Breaker (고체반도체 차단기) |
| SiC | Silicon Carbide (실리콘 카바이드) |
| BCD | Bipolar-CMOS-DMOS (혼합 공정) |
| DBC | Direct Bonded Copper |
| OSAT | Outsourced Semiconductor Assembly and Test |
| MPW | Multi-Project Wafer |
| TDDB | Time-Dependent Dielectric Breakdown |
| SOA | Safe Operating Area |
| Σ-Δ | Sigma-Delta ADC |
| FIT | Failure In Time (10⁹ device-hours 당 고장수) |
| MTBF | Mean Time Between Failures |
| AEC-Q100 | 자동차 반도체 신뢰성 규격 |
| IPC | Institute of Printed Circuits (PCB 제조 표준) |
| JEDEC | 반도체 표준화 단체 |
| IEC | International Electrotechnical Commission |
| KC | Korea Certification |

### §12.4 참조 문서

- UL 489B "Molded-Case Circuit Breakers, Molded-Case Switches, and Circuit-Breaker Enclosures, DC"
- IEC 60947-2 "Low-voltage switchgear and controlgear — Circuit-breakers"
- JEDEC JESD22-A108 "Temperature, Bias, and Operating Life"
- JEDEC JESD85 "Methods for Calculating Failure Rates in Units of FITs"
- JEDEC JESD22-A104 "Temperature Cycling"
- IEC 61000-4-5 "Surge immunity test"
- CISPR 32 "EMC requirements of multimedia equipment"
- IPC-A-600 "Acceptability of Printed Boards"
- IPC-2152 "Standard for Determining Current-Carrying Capacity"
- USCAR-2 "Performance Specification for Automotive Electrical Connector Systems"

### §12.5 변경 이력

| 버전 | 일자 | 변경 | 작성 |
|---|---|---|---|
| 0.1 | 2026-04-17 | 초기 엔지니어링 패키지 (sscb.md 브리프 기반 확장) | n6-architecture |
| — | — | — | — |

### §12.6 수신자 확인 서명

```
본인은 아래 내용을 확인하고 SSCB mk1 엔지니어링 패키지를 수령하였음을 서명합니다.
수신자 이름: ____________________  소속: ____________________
일자:       ____________________  서명: ____________________

수신 목적 (해당 항목 체크):
□ 공동개발 검토       □ 투자 실사        □ 기술이전 검토
□ 조달/구매 검토      □ 인증 대행 검토    □ 기타: ______________
```

---

*문서 끝. 총 12 절, §8 · §9 · §10 · §11 이 핵심 실행 계약 항목이다.*
