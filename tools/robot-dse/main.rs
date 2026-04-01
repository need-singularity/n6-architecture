/// N6 궁극의 로봇 DSE — 6단 전수 조합 탐색기
///
/// 구조체 → 구동기 → 센서 → 제어 → 지능 → 시스템
/// 목표: 가사·간병·배달 로봇 최적 아키텍처
///
/// 평가: n6_EXACT + 자유도 + 페이로드 + 비용 + 자율성
/// 출력: Pareto frontier + 최적 경로

// N6 constants
const N: u64 = 6;
const PHI: u64 = 2;
const TAU: u64 = 4;
const SIGMA: u64 = 12;
const SOPFR: u64 = 5;
const _MU: u64 = 1;
const J2: u64 = 24;

// ============================================================
// Level 1: 구조체/Body (K₁=6)
// ============================================================
#[derive(Clone, Copy)]
struct Body {
    name: &'static str,
    dof_total: u64,         // 총 자유도
    legs: u64,              // 다리/바퀴 수 (0=고정)
    dof_per_limb: u64,      // 관절/축 per limb
    payload_kg: u64,        // 페이로드 (kg)
    weight_kg: u64,         // 자중 (kg)
    cost_rank: u64,         // 1=cheap .. 5=expensive
}

const BODIES: &[Body] = &[
    // 6-DOF 매니퓰레이터 (산업용 로봇팔)
    Body { name: "Arm-6DOF",    dof_total: 6,  legs: 0, dof_per_limb: 6, payload_kg: 5,  weight_kg: 30,  cost_rank: 3 },
    // 쿼드러펫 (4다리, 3DOF/leg = 12 DOF)
    Body { name: "Quadruped",   dof_total: 12, legs: 4, dof_per_limb: 3, payload_kg: 15, weight_kg: 25,  cost_rank: 4 },
    // 휴머노이드 (2다리 + 2팔, 각 6DOF = 24 DOF)
    Body { name: "Humanoid",    dof_total: 24, legs: 2, dof_per_limb: 6, payload_kg: 10, weight_kg: 70,  cost_rank: 5 },
    // 바퀴형 (4바퀴 + 6DOF 팔)
    Body { name: "Wheeled-Arm", dof_total: 10, legs: 4, dof_per_limb: 1, payload_kg: 50, weight_kg: 40,  cost_rank: 2 },
    // 쿼드로터 드론 (4로터)
    Body { name: "Quadrotor",   dof_total: 6,  legs: 4, dof_per_limb: 1, payload_kg: 2,  weight_kg: 3,   cost_rank: 2 },
    // 모듈러 큐브 (6면 연결)
    Body { name: "ModularCube", dof_total: 6,  legs: 0, dof_per_limb: 1, payload_kg: 1,  weight_kg: 1,   cost_rank: 1 },
];

// ============================================================
// Level 2: 구동기/Actuator (K₂=6)
// ============================================================
#[derive(Clone, Copy)]
struct Actuator {
    name: &'static str,
    torque_rank: u64,       // 1=low .. 5=high
    speed_rank: u64,        // 1=slow .. 5=fast
    precision_bits: u64,    // 제어 분해능 (bits)
    efficiency_pct: u64,    // 효율 %
    cost_rank: u64,
}

const ACTUATORS: &[Actuator] = &[
    Actuator { name: "BLDC-Gear",    torque_rank: 4, speed_rank: 3, precision_bits: 12, efficiency_pct: 85,  cost_rank: 2 },
    Actuator { name: "DirectDrive",  torque_rank: 3, speed_rank: 5, precision_bits: 24, efficiency_pct: 95,  cost_rank: 4 },
    Actuator { name: "SeriesElastic",torque_rank: 4, speed_rank: 3, precision_bits: 12, efficiency_pct: 80,  cost_rank: 3 },
    Actuator { name: "Hydraulic",    torque_rank: 5, speed_rank: 2, precision_bits: 12, efficiency_pct: 60,  cost_rank: 4 },
    Actuator { name: "Pneumatic",    torque_rank: 2, speed_rank: 4, precision_bits: 8,  efficiency_pct: 50,  cost_rank: 1 },
    Actuator { name: "SMA-Soft",     torque_rank: 1, speed_rank: 1, precision_bits: 6,  efficiency_pct: 5,   cost_rank: 3 },
];

// ============================================================
// Level 3: 센서/Sensor (K₃=5)
// ============================================================
#[derive(Clone, Copy)]
struct Sensor {
    name: &'static str,
    modalities: u64,        // 센서 모달리티 수
    imu_axes: u64,          // IMU 축 수 (0=없음)
    camera_count: u64,      // 카메라 수
    lidar: bool,            // LiDAR 유무
    cost_rank: u64,
}

const SENSORS: &[Sensor] = &[
    // 기본: 6축 IMU + 2 카메라
    Sensor { name: "Basic-6IMU",    modalities: 3, imu_axes: 6,  camera_count: 2,  lidar: false, cost_rank: 1 },
    // 고급: 6축 IMU + 4 카메라 + LiDAR
    Sensor { name: "Full-Vision",   modalities: 5, imu_axes: 6,  camera_count: 4,  lidar: true,  cost_rank: 3 },
    // 촉각: 6축 IMU + 촉각 어레이 + 2 카메라
    Sensor { name: "Tactile-Array", modalities: 4, imu_axes: 6,  camera_count: 2,  lidar: false, cost_rank: 4 },
    // 최소: 6축 IMU만
    Sensor { name: "IMU-Only",      modalities: 1, imu_axes: 6,  camera_count: 0,  lidar: false, cost_rank: 1 },
    // 풀스택: 6축 IMU + 4 카메라 + LiDAR + 촉각 + F/T
    Sensor { name: "FullStack-12",  modalities: 6, imu_axes: 12, camera_count: 4,  lidar: true,  cost_rank: 5 },
];

// ============================================================
// Level 4: 제어/Control (K₄=5)
// ============================================================
#[derive(Clone, Copy)]
struct Control {
    name: &'static str,
    loop_hz: u64,           // 제어 주기 (Hz)
    layers: u64,            // 제어 계층 수
    latency_ms: u64,        // 응답 지연 (ms)
    compute_w: u64,         // 소비전력 (W)
    cost_rank: u64,
}

const CONTROLS: &[Control] = &[
    // MCU 기본 (STM32급)
    Control { name: "MCU-1kHz",     loop_hz: 1000,  layers: 2, latency_ms: 1,  compute_w: 2,   cost_rank: 1 },
    // 실시간 SoC (Jetson급)
    Control { name: "SoC-RT",       loop_hz: 1000,  layers: 4, latency_ms: 5,  compute_w: 15,  cost_rank: 3 },
    // 고성능 (GPU 탑재)
    Control { name: "GPU-Onboard",  loop_hz: 500,   layers: 5, latency_ms: 10, compute_w: 50,  cost_rank: 4 },
    // 클라우드 (지연 있음)
    Control { name: "Cloud-Edge",   loop_hz: 100,   layers: 6, latency_ms: 50, compute_w: 5,   cost_rank: 2 },
    // HEXA-1 칩 (n=6 최적)
    Control { name: "HEXA-1",       loop_hz: 1000,  layers: 6, latency_ms: 1,  compute_w: 12,  cost_rank: 5 },
];

// ============================================================
// Level 5: 지능/AI (K₅=5)
// ============================================================
#[derive(Clone, Copy)]
struct Intelligence {
    name: &'static str,
    params_m: u64,          // 파라미터 수 (M)
    modality: u64,          // 입력 모달리티 수
    rl_policy: bool,        // RL 정책 사용
    llm_planner: bool,      // LLM 플래너 사용
    autonomy_rank: u64,     // 자율성 1=tele .. 5=full
    cost_rank: u64,
}

const INTELLIGENCES: &[Intelligence] = &[
    // 텔레오퍼레이션 (원격제어)
    Intelligence { name: "TeleOp",       params_m: 0,    modality: 1, rl_policy: false, llm_planner: false, autonomy_rank: 1, cost_rank: 1 },
    // 클래식 로보틱스 (PID + 경로 계획)
    Intelligence { name: "Classic-PID",  params_m: 0,    modality: 2, rl_policy: false, llm_planner: false, autonomy_rank: 2, cost_rank: 1 },
    // RL 정책 (PPO/SAC)
    Intelligence { name: "RL-Policy",    params_m: 12,   modality: 3, rl_policy: true,  llm_planner: false, autonomy_rank: 3, cost_rank: 3 },
    // VLA (Vision-Language-Action)
    Intelligence { name: "VLA-1B",       params_m: 1000, modality: 4, rl_policy: true,  llm_planner: true,  autonomy_rank: 4, cost_rank: 4 },
    // 궁극: Egyptian RL + LLM + n=6 최적화
    Intelligence { name: "N6-Egyptian",  params_m: 4096, modality: 6, rl_policy: true,  llm_planner: true,  autonomy_rank: 5, cost_rank: 5 },
];

// ============================================================
// Level 6: 시스템/System (K₆=5)
// ============================================================
#[derive(Clone, Copy)]
struct RobotSystem {
    name: &'static str,
    units_per_fleet: u64,   // 플릿 당 대수
    battery_kwh_10x: u64,   // 배터리 ×10 (kWh)
    runtime_hr_10x: u64,    // 운전시간 ×10 (hr)
    target: &'static str,   // 목표 응용
    cost_rank: u64,
}

const SYSTEMS: &[RobotSystem] = &[
    RobotSystem { name: "Home-Care",    units_per_fleet: 1,  battery_kwh_10x: 12, runtime_hr_10x: 60,  target: "가사간병",  cost_rank: 3 },
    RobotSystem { name: "Delivery-6",   units_per_fleet: 6,  battery_kwh_10x: 24, runtime_hr_10x: 40,  target: "배달",      cost_rank: 2 },
    RobotSystem { name: "Factory-12",   units_per_fleet: 12, battery_kwh_10x: 48, runtime_hr_10x: 80,  target: "공장",      cost_rank: 4 },
    RobotSystem { name: "Swarm-24",     units_per_fleet: 24, battery_kwh_10x: 5,  runtime_hr_10x: 20,  target: "탐사",      cost_rank: 3 },
    RobotSystem { name: "Surgery-1",    units_per_fleet: 1,  battery_kwh_10x: 6,  runtime_hr_10x: 120, target: "수술",      cost_rank: 5 },
];

// ============================================================
// 호환성 검사
// ============================================================
fn is_compatible(body: &Body, act: &Actuator, sens: &Sensor, ctrl: &Control, ai: &Intelligence, sys: &RobotSystem) -> bool {
    // 드론은 BLDC만 가능
    if body.name == "Quadrotor" && act.name != "BLDC-Gear" && act.name != "DirectDrive" {
        return false;
    }
    // SMA-Soft는 ModularCube나 소프트 로봇만
    if act.name == "SMA-Soft" && body.name != "ModularCube" {
        return false;
    }
    // Hydraulic은 대형만 (Humanoid, Quadruped)
    if act.name == "Hydraulic" && body.name != "Humanoid" && body.name != "Quadruped" {
        return false;
    }
    // 수술용은 Arm-6DOF만
    if sys.name == "Surgery-1" && body.name != "Arm-6DOF" {
        return false;
    }
    // 텔레오퍼레이션은 수술에만 적합, 나머지는 최소 PID
    if ai.name == "TeleOp" && sys.name != "Surgery-1" && sys.name != "Home-Care" {
        return false;
    }
    // 드론 배달용
    if sys.name == "Delivery-6" && body.name == "Humanoid" {
        return false; // 휴머노이드 배달 비현실적
    }
    // Swarm은 드론 또는 모듈러
    if sys.name == "Swarm-24" && body.name != "Quadrotor" && body.name != "ModularCube" {
        return false;
    }
    // VLA/N6-Egyptian은 최소 SoC급 제어 필요
    if (ai.name == "VLA-1B" || ai.name == "N6-Egyptian") && ctrl.name == "MCU-1kHz" {
        return false;
    }
    // N6-Egyptian은 GPU 또는 HEXA-1 필요
    if ai.name == "N6-Egyptian" && ctrl.name != "GPU-Onboard" && ctrl.name != "HEXA-1" && ctrl.name != "Cloud-Edge" {
        return false;
    }
    // FullStack 센서는 대형 로봇만
    if sens.name == "FullStack-12" && (body.name == "Quadrotor" || body.name == "ModularCube") {
        return false;
    }
    // IMU-Only는 간단한 것만
    if sens.name == "IMU-Only" && (sys.name == "Surgery-1" || ai.name == "VLA-1B" || ai.name == "N6-Egyptian") {
        return false;
    }
    true
}

// ============================================================
// N6 EXACT 계산
// ============================================================
fn count_n6_exact(body: &Body, act: &Actuator, sens: &Sensor, ctrl: &Control, ai: &Intelligence, sys: &RobotSystem) -> (u64, u64) {
    let mut exact = 0u64;
    let mut total = 0u64;

    // ── 구조체 (6 checks) ──
    // SE(3) = 6 DOF 기본
    total += 1;
    if body.dof_total == N || body.dof_total == SIGMA || body.dof_total == J2 { exact += 1; }

    // 다리/바퀴 수: 4=τ, 2=φ, 6=n
    total += 1;
    if body.legs == TAU || body.legs == PHI || body.legs == N { exact += 1; }

    // DOF per limb: 6=n, 3=n/φ, 1=μ
    total += 1;
    if body.dof_per_limb == N || body.dof_per_limb == N / PHI || body.dof_per_limb == 1 { exact += 1; }

    // 큐브 6면 (ModularCube)
    total += 1;
    if body.name == "ModularCube" { exact += 1; } // 6면=n
    else if body.dof_total == N { exact += 1; }

    // 쿼드러펫 12 DOF = σ
    total += 1;
    if body.dof_total == SIGMA { exact += 1; }

    // 휴머노이드 24 DOF = J₂
    total += 1;
    if body.dof_total == J2 { exact += 1; }

    // ── 구동기 (4 checks) ──
    // 12-bit PWM = σ
    total += 1;
    if act.precision_bits == SIGMA { exact += 1; }

    // 24-bit = J₂ (DirectDrive)
    total += 1;
    if act.precision_bits == J2 { exact += 1; }

    // 효율 85% ≈ 5/6 = 1-1/n (Egyptian)
    total += 1;
    if act.efficiency_pct >= 83 && act.efficiency_pct <= 87 { exact += 1; }

    // 토크 + 속도 합 = n 또는 관련
    total += 1;
    if act.torque_rank + act.speed_rank == N || act.torque_rank + act.speed_rank == SIGMA - N { exact += 1; }

    // ── 센서 (4 checks) ──
    // 6축 IMU = n
    total += 1;
    if sens.imu_axes == N { exact += 1; }

    // 12축 IMU = σ
    total += 1;
    if sens.imu_axes == SIGMA { exact += 1; }

    // 모달리티 수: 6=n (FullStack)
    total += 1;
    if sens.modalities == N { exact += 1; }

    // 카메라 2=φ 또는 4=τ
    total += 1;
    if sens.camera_count == PHI || sens.camera_count == TAU { exact += 1; }

    // ── 제어 (4 checks) ──
    // 제어 계층 6 = n (Cloud, HEXA-1)
    total += 1;
    if ctrl.layers == N { exact += 1; }

    // 1000Hz 제어 주기 (1ms)
    total += 1;
    if ctrl.loop_hz == 1000 { exact += 1; } // 10^(n/φ) = 10^3

    // 소비전력 12W = σ (HEXA-1)
    total += 1;
    if ctrl.compute_w == SIGMA { exact += 1; }

    // 지연 1ms = μ
    total += 1;
    if ctrl.latency_ms == 1 { exact += 1; }

    // ── 지능 (4 checks) ──
    // 모달리티 6 = n
    total += 1;
    if ai.modality == N { exact += 1; }

    // 파라미터 12M = σ, 4096M = 2^σ
    total += 1;
    if ai.params_m == SIGMA || ai.params_m == (1u64 << SIGMA) { exact += 1; }

    // RL + LLM 듀얼 = φ
    total += 1;
    if ai.rl_policy && ai.llm_planner { exact += 1; }

    // 자율성 5 = sopfr
    total += 1;
    if ai.autonomy_rank == SOPFR { exact += 1; }

    // ── 시스템 (4 checks) ──
    // 플릿 6=n, 12=σ, 24=J₂
    total += 1;
    if sys.units_per_fleet == N || sys.units_per_fleet == SIGMA || sys.units_per_fleet == J2 { exact += 1; }

    // 배터리 2.4kWh=J₂/10, 4.8kWh=σ·τ/10, 1.2kWh=σ/10
    total += 1;
    if sys.battery_kwh_10x == J2 || sys.battery_kwh_10x == SIGMA * TAU || sys.battery_kwh_10x == SIGMA || sys.battery_kwh_10x == N { exact += 1; }

    // 운전시간: 12hr=σ
    total += 1;
    if sys.runtime_hr_10x == SIGMA * 10 || sys.runtime_hr_10x == N * 10 { exact += 1; }

    // 보편: SE(3)=6 (모든 로봇)
    total += 1;
    exact += 1;

    (exact, total)
}

// ============================================================
// 성능 스코어
// ============================================================
fn perf_score(body: &Body, act: &Actuator, ai: &Intelligence) -> u64 {
    let dof_score = body.dof_total.min(24) * 2;
    let payload = body.payload_kg;
    let torque = act.torque_rank * 10;
    let autonomy = ai.autonomy_rank * 15;
    (dof_score + payload + torque + autonomy).min(100)
}

// ============================================================
// 비용 스코어 (낮을수록 좋음)
// ============================================================
fn cost_total(body: &Body, act: &Actuator, sens: &Sensor, ctrl: &Control, ai: &Intelligence, sys: &RobotSystem) -> u64 {
    body.cost_rank * 15
    + act.cost_rank * 15
    + sens.cost_rank * 10
    + ctrl.cost_rank * 10
    + ai.cost_rank * 10
    + sys.cost_rank * 10
}

// ============================================================
// 결과 구조체
// ============================================================
#[derive(Clone)]
struct DseResult {
    body: &'static str,
    actuator: &'static str,
    sensor: &'static str,
    control: &'static str,
    intelligence: &'static str,
    system: &'static str,
    n6_exact: u64,
    n6_total: u64,
    n6_pct: f64,
    perf: u64,
    cost: u64,
    autonomy: u64,
    score: f64,
}

fn main() {
    let total_combos = BODIES.len() * ACTUATORS.len() * SENSORS.len()
                     * CONTROLS.len() * INTELLIGENCES.len() * SYSTEMS.len();

    println!("═══════════════════════════════════════════════════════════════════════════");
    println!("  N6 궁극의 로봇 DSE — 6단 전수 조합 탐색");
    println!("  구조체 × 구동기 × 센서 × 제어 × 지능 × 시스템");
    println!("  목표: 가사·간병·배달 로봇 최적 아키텍처");
    println!("═══════════════════════════════════════════════════════════════════════════");
    println!();
    println!("  후보군:");
    println!("    구조체: {} ({})", BODIES.iter().map(|b| b.name).collect::<Vec<_>>().join(", "), BODIES.len());
    println!("    구동기: {} ({})", ACTUATORS.iter().map(|a| a.name).collect::<Vec<_>>().join(", "), ACTUATORS.len());
    println!("    센서:   {} ({})", SENSORS.iter().map(|s| s.name).collect::<Vec<_>>().join(", "), SENSORS.len());
    println!("    제어:   {} ({})", CONTROLS.iter().map(|c| c.name).collect::<Vec<_>>().join(", "), CONTROLS.len());
    println!("    지능:   {} ({})", INTELLIGENCES.iter().map(|i| i.name).collect::<Vec<_>>().join(", "), INTELLIGENCES.len());
    println!("    시스템: {} ({})", SYSTEMS.iter().map(|s| s.name).collect::<Vec<_>>().join(", "), SYSTEMS.len());
    println!();
    println!("  총 조합: {} (호환 필터 전)", total_combos);
    println!();

    let mut results: Vec<DseResult> = Vec::with_capacity(total_combos);
    let mut filtered = 0u64;

    for body in BODIES {
        for act in ACTUATORS {
            for sens in SENSORS {
                for ctrl in CONTROLS {
                    for ai in INTELLIGENCES {
                        for sys in SYSTEMS {
                            if !is_compatible(body, act, sens, ctrl, ai, sys) {
                                filtered += 1;
                                continue;
                            }

                            let (n6_exact, n6_total) = count_n6_exact(body, act, sens, ctrl, ai, sys);
                            let n6_pct = if n6_total > 0 { (n6_exact as f64) / (n6_total as f64) * 100.0 } else { 0.0 };
                            let perf = perf_score(body, act, ai);
                            let cost = cost_total(body, act, sens, ctrl, ai, sys);
                            let autonomy = ai.autonomy_rank;

                            // Pareto: 20% n6 + 25% perf + 20% autonomy + 20% (1/cost) + 15% versatility
                            let perf_norm = perf as f64;
                            let auto_norm = autonomy as f64 * 20.0;
                            let cost_norm = (350.0 - cost as f64).max(0.0) / 3.5;
                            let versatility = (body.dof_total as f64 / 24.0 * 50.0
                                + ai.modality as f64 / 6.0 * 50.0).min(100.0);

                            let score = n6_pct * 0.20
                                + perf_norm * 0.25
                                + auto_norm * 0.20
                                + cost_norm * 0.20
                                + versatility * 0.15;

                            results.push(DseResult {
                                body: body.name,
                                actuator: act.name,
                                sensor: sens.name,
                                control: ctrl.name,
                                intelligence: ai.name,
                                system: sys.name,
                                n6_exact,
                                n6_total,
                                n6_pct,
                                perf,
                                cost,
                                autonomy,
                                score,
                            });
                        }
                    }
                }
            }
        }
    }

    let valid = results.len();
    println!("  호환 필터: {} 제거 → {} 유효 조합", filtered, valid);
    println!();

    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

    // ── TOP 30 ──
    println!("══════════════════════════════════════════════════════════════════════════════════════════════════════════════════════");
    println!("  TOP 30 CONFIGURATIONS (by Pareto score)");
    println!("══════════════════════════════════════════════════════════════════════════════════════════════════════════════════════");
    println!();
    println!("  {:>3} │ {:>11} │ {:>12} │ {:>12} │ {:>11} │ {:>12} │ {:>10} │ {:>5} │ {:>4} │ {:>3} │ {:>4} │ {:>6}",
        "#", "Body", "Actuator", "Sensor", "Control", "AI", "System", "n6%", "Perf", "Aut", "Cost", "Score");
    println!("  ────┼─────────────┼──────────────┼──────────────┼─────────────┼──────────────┼────────────┼───────┼──────┼─────┼──────┼───────");

    for (i, r) in results.iter().take(30).enumerate() {
        println!("  {:>3} │ {:>11} │ {:>12} │ {:>12} │ {:>11} │ {:>12} │ {:>10} │{:>4.0}% │ {:>4} │  {:>1}  │ {:>4} │ {:>6.2}",
            i + 1, r.body, r.actuator, r.sensor, r.control, r.intelligence, r.system,
            r.n6_pct, r.perf, r.autonomy, r.cost, r.score);
    }

    // ── 가사·간병 특화 ──
    println!();
    println!("══════════════════════════════════════════════════════════════════════════════════════════════════════════════════════");
    println!("  HOME-CARE ROBOTS — 가사·간병 특화 경로");
    println!("══════════════════════════════════════════════════════════════════════════════════════════════════════════════════════");
    println!();

    let home: Vec<&DseResult> = results.iter()
        .filter(|r| r.system == "Home-Care")
        .collect();

    println!("  {:>3} │ {:>11} │ {:>12} │ {:>12} │ {:>11} │ {:>12} │ {:>5} │ {:>4} │ {:>3} │ {:>6}",
        "#", "Body", "Actuator", "Sensor", "Control", "AI", "n6%", "Perf", "Aut", "Score");
    println!("  ────┼─────────────┼──────────────┼──────────────┼─────────────┼──────────────┼───────┼──────┼─────┼───────");

    for (i, r) in home.iter().take(15).enumerate() {
        println!("  {:>3} │ {:>11} │ {:>12} │ {:>12} │ {:>11} │ {:>12} │{:>4.0}% │ {:>4} │  {:>1}  │ {:>6.2}",
            i + 1, r.body, r.actuator, r.sensor, r.control, r.intelligence,
            r.n6_pct, r.perf, r.autonomy, r.score);
    }
    println!();
    println!("  가사·간병 조합: {} / {} 유효 ({:.1}%)",
        home.len(), valid, home.len() as f64 / valid as f64 * 100.0);

    // ── 배달 특화 ──
    println!();
    println!("══════════════════════════════════════════════════════════════════════════════════════════════════════════════════════");
    println!("  DELIVERY ROBOTS — 배달 특화 경로");
    println!("══════════════════════════════════════════════════════════════════════════════════════════════════════════════════════");
    println!();

    let delivery: Vec<&DseResult> = results.iter()
        .filter(|r| r.system == "Delivery-6")
        .collect();

    println!("  {:>3} │ {:>11} │ {:>12} │ {:>12} │ {:>11} │ {:>12} │ {:>5} │ {:>4} │ {:>3} │ {:>6}",
        "#", "Body", "Actuator", "Sensor", "Control", "AI", "n6%", "Perf", "Aut", "Score");
    println!("  ────┼─────────────┼──────────────┼──────────────┼─────────────┼──────────────┼───────┼──────┼─────┼───────");

    for (i, r) in delivery.iter().take(10).enumerate() {
        println!("  {:>3} │ {:>11} │ {:>12} │ {:>12} │ {:>11} │ {:>12} │{:>4.0}% │ {:>4} │  {:>1}  │ {:>6.2}",
            i + 1, r.body, r.actuator, r.sensor, r.control, r.intelligence,
            r.n6_pct, r.perf, r.autonomy, r.score);
    }
    println!();
    println!("  배달 조합: {} / {} 유효 ({:.1}%)",
        delivery.len(), valid, delivery.len() as f64 / valid as f64 * 100.0);

    // ── STATISTICS ──
    println!();
    println!("══════════════════════════════════════════════════════════════════════════════════════════════════════════════════════");
    println!("  STATISTICS");
    println!("══════════════════════════════════════════════════════════════════════════════════════════════════════════════════════");

    let max_n6 = results.iter().map(|r| r.n6_pct).fold(0.0f64, f64::max);
    let avg_n6 = if !results.is_empty() { results.iter().map(|r| r.n6_pct).sum::<f64>() / valid as f64 } else { 0.0 };
    let above80 = results.iter().filter(|r| r.n6_pct >= 80.0).count();
    let above60 = results.iter().filter(|r| r.n6_pct >= 60.0).count();

    println!();
    println!("  Total raw:     {}", total_combos);
    println!("  Filtered:      {}", filtered);
    println!("  Valid:         {}", valid);
    println!();
    println!("  n6 EXACT max:  {:.1}%", max_n6);
    println!("  n6 EXACT avg:  {:.1}%", avg_n6);
    println!("  ≥80% EXACT:    {} ({:.1}%)", above80, if valid > 0 { above80 as f64 / valid as f64 * 100.0 } else { 0.0 });
    println!("  ≥60% EXACT:    {} ({:.1}%)", above60, if valid > 0 { above60 as f64 / valid as f64 * 100.0 } else { 0.0 });

    // ── BEST BY CATEGORY ──
    println!();
    println!("  BEST BY CATEGORY:");
    if let Some(r) = results.iter().max_by(|a, b| a.n6_pct.partial_cmp(&b.n6_pct).unwrap()) {
        println!("    Best n6:     {} + {} + {} + {} + {} + {} ({:.1}%)",
            r.body, r.actuator, r.sensor, r.control, r.intelligence, r.system, r.n6_pct);
    }
    if let Some(r) = results.iter().max_by_key(|r| r.perf) {
        println!("    Best Perf:   {} + {} + {} + {} + {} + {} (perf={})",
            r.body, r.actuator, r.sensor, r.control, r.intelligence, r.system, r.perf);
    }
    if let Some(r) = results.iter().min_by_key(|r| r.cost) {
        println!("    Best Cost:   {} + {} + {} + {} + {} + {} (cost={})",
            r.body, r.actuator, r.sensor, r.control, r.intelligence, r.system, r.cost);
    }
    if let Some(r) = results.first() {
        println!("    Best Pareto: {} + {} + {} + {} + {} + {} (score={:.2})",
            r.body, r.actuator, r.sensor, r.control, r.intelligence, r.system, r.score);
    }

    // ── N6 PATTERN MAP ──
    println!();
    println!("══════════════════════════════════════════════════════════════════════════════════════════════════════════════════════");
    println!("  N6 PATTERN MAP — 로봇 n=6 일치 지도");
    println!("══════════════════════════════════════════════════════════════════════════════════════════════════════════════════════");
    println!();
    println!("  ┌────────────────────────────────────────────────────────────────┐");
    println!("  │ EXACT 일치 (물리/수학적 근거)                                  │");
    println!("  │   SE(3) = 6 DOF: 모든 로봇의 기본 자유도 = n                  │");
    println!("  │   6축 IMU: 3가속도+3자이로 = n (최소 자세 추정)               │");
    println!("  │   6-DOF 매니퓰레이터: 산업 표준 (UR, FANUC, ABB)             │");
    println!("  │   Quadruped 12 DOF = σ (4다리×3관절)                         │");
    println!("  │   Humanoid 24 DOF = J₂ (4지×6관절)                           │");
    println!("  │   12-bit PWM = σ (ADC/DAC 산업 표준)                          │");
    println!("  │   24-bit DirectDrive = J₂ (고정밀)                            │");
    println!("  │   Quadrotor 4 로터 = τ                                        │");
    println!("  │   ModularCube 6면 = n (공간 충전)                             │");
    println!("  ├────────────────────────────────────────────────────────────────┤");
    println!("  │ 시스템 n=6                                                    │");
    println!("  │   배달 플릿 6대 = n                                           │");
    println!("  │   공장 플릿 12대 = σ                                          │");
    println!("  │   탐사 스웜 24대 = J₂                                        │");
    println!("  │   배터리 2.4kWh = J₂/10, 4.8kWh = σ·τ/10                   │");
    println!("  │   Egyptian RL: 1/2 보행 + 1/3 효율 + 1/6 안정 = 1            │");
    println!("  │   HEXA-1 칩: 12W=σ, 6계층=n, 1ms=μ 지연                     │");
    println!("  └────────────────────────────────────────────────────────────────┘");

    println!();
    println!("══════════════════════════════════════════════════════════════════════════════════════════════════════════════════════");
    println!("  ✅ DSE 완료 — {} 전수 / {} 유효 / 가사·간병·배달 최적 경로 도출", total_combos, valid);
    println!("══════════════════════════════════════════════════════════════════════════════════════════════════════════════════════");
}
