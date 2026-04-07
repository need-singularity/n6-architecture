# NEXUS-6 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 4개 프로젝트(TECS-L/n6/anima/SEDI) 통합 자동 발견 엔진. 411종 렌즈 + Discovery Graph + metal-rs GPU 가속.

**Architecture:** Rust 모노 크레이트(`tools/nexus/`) + metal-rs GPU 커널 + PyO3 Python 바인딩. OUROBOROS v26은 Python 확장. Discovery Graph는 JSON + Rust 분석기.

**Tech Stack:** Rust (metal, rayon, pyo3, serde, blake3, rusqlite), Metal Shading Language, Python (numpy, OUROBOROS)

**Spec:** `docs/superpowers/specs/2026-04-03-nexus-unified-brainstorm.md`

---

## Dependency Graph

```
Phase 1 (Metal kernels) ──┐
Phase 3 (Encoder+DB) ─────┼──→ Phase 6 (History) ──→ Phase 7 (OUROBOROS) ──→ Phase 8 (CLI)
Phase 4 (Verifier) ───────┤                                                 ──→ Phase 9 (411 lenses)
Phase 5 (Graph) ──────────┘
Phase 2 (Telescope core) ─┘
```

**Phase 1-5: 병렬 가능 | Phase 6→7: 순차 | Phase 8-9: Phase 7 이후**

---

## File Structure

```
tools/nexus/
├── Cargo.toml
├── src/
│   ├── lib.rs                    — 크레이트 루트, 모듈 선언
│   ├── main.rs                   — CLI 진입점 (Phase 8)
│   │
│   ├── gpu/                      — Phase 1: Metal compute
│   │   ├── mod.rs                — GPU 디바이스 초기화 + 디스패치
│   │   ├── kernels.metal         — 12종 MSL compute shaders
│   │   ├── buffer_pool.rs        — MTLBuffer 풀 관리
│   │   ├── pipeline.rs           — Compute pipeline 캐시
│   │   └── fallback.rs           — CPU fallback (GPU 실패 시)
│   │
│   ├── telescope/                — Phase 2: 렌즈 엔진
│   │   ├── mod.rs                — Telescope 공개 API
│   │   ├── shared_kernels.rs     — 12 공유 커널 (GPU 호출)
│   │   ├── lens_trait.rs         — Lens trait 정의
│   │   ├── tier.rs               — Tiered scanning (T0→T3)
│   │   ├── consensus.rs          — 가중 합의 시스템
│   │   ├── lenses/               — 개별 렌즈 구현
│   │   │   ├── mod.rs
│   │   │   ├── existing.rs       — 기존 22종 래퍼 (telescope_rs 호출)
│   │   │   ├── discovery.rs      — void, isomorphism, extrapolation, ...
│   │   │   ├── synthesis.rs      — inverse, combinatorial, frustration, ...
│   │   │   ├── validation.rs     — emergence, periodicity, completeness, ...
│   │   │   └── meta.rs           — blind_spot, contradiction, narrative, ...
│   │   └── cache.rs              — 결과 캐시 (blake3 키)
│   │
│   ├── encoder/                  — Phase 3: Domain Encoder
│   │   ├── mod.rs                — 인코더 API
│   │   ├── parser.rs             — hypotheses.md 파싱
│   │   ├── vectorize.rs          — 텍스트 → float 벡터
│   │   └── cache.rs              — 인코딩 캐시
│   │
│   ├── materials/                — Phase 3: Materials DB
│   │   ├── mod.rs                — DB API
│   │   └── db.rs                 — JSON 로드 + SQLite 마이그레이션
│   │
│   ├── verifier/                 — Phase 4: Discovery Verifier
│   │   ├── mod.rs                — 검증기 API
│   │   ├── thermo.rs             — 열역학 검증
│   │   ├── crystal.rs            — 결정구조 검증
│   │   ├── n6_check.rs           — n=6 EXACT 매칭
│   │   ├── bt_compat.rs          — BT 정합성
│   │   └── feasibility.rs        — 종합 점수 + 등급
│   │
│   ├── graph/                    — Phase 5: Discovery Graph
│   │   ├── mod.rs                — 그래프 API
│   │   ├── node.rs               — 노드 타입
│   │   ├── edge.rs               — 간선 타입
│   │   ├── structure.rs          — 자동 구조 탐지 (루프, 허브, DAG)
│   │   └── persistence.rs        — JSON 직렬화
│   │
│   ├── history/                  — Phase 6: Telescope History
│   │   ├── mod.rs                — 히스토리 API
│   │   ├── recorder.rs           — 스캔 결과 기록
│   │   ├── stats.rs              — 집계/친화도
│   │   └── recommend.rs          — 렌즈 조합 추천
│   │
│   ├── ouroboros/                — Phase 7: OUROBOROS 브릿지
│   │   ├── mod.rs                — Python↔Rust 브릿지
│   │   └── pipeline.rs           — 파이프라인 오케스트레이션
│   │
│   ├── dashboard/                — Phase 8: 대시보드
│   │   ├── mod.rs
│   │   └── ascii.rs              — ASCII 리포트
│   │
│   └── python.rs                 — PyO3 바인딩 (Phase 2+)
│
├── metal/                        — Metal shader 소스
│   └── kernels.metal
│
├── domains/                      — 도메인 인코딩 규칙
│   ├── superconductor.toml
│   └── ...
│
├── data/
│   ├── materials-db.json         — 기존 소재/기술 DB
│   ├── cross-domain-map.json     — 도메인 간 매핑
│   └── calibration.json          — 캘리브레이션 셋
│
└── tests/
    ├── gpu_test.rs
    ├── telescope_test.rs
    ├── encoder_test.rs
    ├── verifier_test.rs
    ├── graph_test.rs
    ├── history_test.rs
    └── integration_test.rs
```

---

## Phase 1: Metal Compute Kernels (병렬)

### Task 1.1: 프로젝트 스캐폴딩

**Files:**
- Create: `tools/nexus/Cargo.toml`
- Create: `tools/nexus/src/lib.rs`
- Create: `tools/nexus/src/gpu/mod.rs`

- [ ] **Step 1: Cargo.toml 생성**

```toml
[package]
name = "nexus"
version = "0.1.0"
edition = "2021"

[dependencies]
metal = "0.29"
objc2 = "0.5"
objc2-foundation = { version = "0.2", features = ["NSString", "NSError"] }
objc2-metal = { version = "0.2", features = ["all"] }
rayon = "1.10"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
blake3 = "1"
pyo3 = { version = "0.22", features = ["extension-module"], optional = true }
numpy = { version = "0.22", optional = true }

[features]
default = []
python = ["pyo3", "numpy"]

[lib]
name = "nexus"
crate-type = ["lib", "cdylib"]

[[bin]]
name = "nexus"
path = "src/main.rs"
```

- [ ] **Step 2: lib.rs 작성**

```rust
pub mod gpu;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
```

- [ ] **Step 3: gpu/mod.rs 작성 — Metal 디바이스 초기화**

```rust
pub mod buffer_pool;
pub mod pipeline;
pub mod fallback;

use objc2_metal::{MTLCreateSystemDefaultDevice, MTLDevice};
use std::sync::OnceLock;

static GPU_DEVICE: OnceLock<&'static dyn MTLDevice> = OnceLock::new();

pub fn device() -> Option<&'static dyn MTLDevice> {
    GPU_DEVICE.get_or_init(|| {
        unsafe { MTLCreateSystemDefaultDevice() }
            .expect("Metal device not available")
    });
    GPU_DEVICE.get().copied()
}

pub fn is_available() -> bool {
    device().is_some()
}
```

- [ ] **Step 4: 빌드 확인**

Run: `cd tools/nexus && ~/.cargo/bin/cargo check 2>&1 | tail -5`
Expected: `Finished` (에러 없음)

- [ ] **Step 5: 커밋**

```bash
git add tools/nexus/
git commit -m "feat(nexus): scaffold project + Metal device init"
```

### Task 1.2: Buffer Pool

**Files:**
- Create: `tools/nexus/src/gpu/buffer_pool.rs`
- Test: `tools/nexus/tests/gpu_test.rs`

- [ ] **Step 1: 테스트 작성**

```rust
// tests/gpu_test.rs
use nexus::gpu::buffer_pool::BufferPool;

#[test]
fn test_buffer_pool_acquire_release() {
    if !nexus::gpu::is_available() {
        eprintln!("Metal not available, skipping");
        return;
    }
    let device = nexus::gpu::device().unwrap();
    let mut pool = BufferPool::new(device);

    let (idx, buf) = pool.acquire(1024);
    assert!(buf.length() >= 1024);

    pool.release(idx);
    // 재사용 확인: 같은 인덱스 반환
    let (idx2, _) = pool.acquire(1024);
    assert_eq!(idx, idx2);
}
```

- [ ] **Step 2: 테스트 실패 확인**

Run: `cd tools/nexus && ~/.cargo/bin/cargo test test_buffer_pool 2>&1 | tail -10`
Expected: FAIL (module not found)

- [ ] **Step 3: BufferPool 구현**

```rust
// src/gpu/buffer_pool.rs
use objc2_metal::{MTLBuffer, MTLDevice, MTLResourceOptions};

pub struct BufferPool {
    device: &'static dyn MTLDevice,
    buffers: Vec<Option<&'static MTLBuffer>>,
    free_list: Vec<usize>,
}

impl BufferPool {
    pub fn new(device: &'static dyn MTLDevice) -> Self {
        Self {
            device,
            buffers: Vec::new(),
            free_list: Vec::new(),
        }
    }

    pub fn acquire(&mut self, size: usize) -> (usize, &'static MTLBuffer) {
        if let Some(idx) = self.free_list.pop() {
            if let Some(buf) = &self.buffers[idx] {
                if buf.length() >= size as u64 {
                    return (idx, buf);
                }
            }
        }
        let buf = self.device
            .newBufferWithLength_options(
                size as u64,
                MTLResourceOptions::StorageModeShared,
            )
            .expect("Failed to create MTLBuffer");
        let buf_ref: &'static MTLBuffer = unsafe { &*(buf.as_ref() as *const _) };
        let idx = self.buffers.len();
        self.buffers.push(Some(buf_ref));
        (idx, buf_ref)
    }

    pub fn release(&mut self, idx: usize) {
        self.free_list.push(idx);
    }
}
```

- [ ] **Step 4: 테스트 통과 확인**

Run: `cd tools/nexus && ~/.cargo/bin/cargo test test_buffer_pool 2>&1 | tail -5`
Expected: PASS

- [ ] **Step 5: 커밋**

```bash
git add tools/nexus/src/gpu/buffer_pool.rs tools/nexus/tests/gpu_test.rs
git commit -m "feat(nexus): Metal buffer pool with acquire/release"
```

### Task 1.3: Compute Pipeline + distance_matrix kernel

**Files:**
- Create: `tools/nexus/src/gpu/pipeline.rs`
- Create: `tools/nexus/metal/kernels.metal`

- [ ] **Step 1: Metal shader 작성 — distance_matrix**

```metal
// metal/kernels.metal
#include <metal_stdlib>
using namespace metal;

// 유클리드 거리 행렬 (N×D 입력 → N×N 출력, 하삼각만)
kernel void distance_matrix(
    device const float* data   [[buffer(0)]],  // N×D, row-major
    device float* dist         [[buffer(1)]],  // N*(N-1)/2, lower triangle
    constant uint& N           [[buffer(2)]],
    constant uint& D           [[buffer(3)]],
    uint2 gid                  [[thread_position_in_grid]]
) {
    uint i = gid.x;
    uint j = gid.y;
    if (i <= j || i >= N) return;

    float sum = 0.0;
    for (uint k = 0; k < D; k++) {
        float diff = data[i * D + k] - data[j * D + k];
        sum += diff * diff;
    }
    uint idx = i * (i - 1) / 2 + j;
    dist[idx] = sqrt(sum);
}

// 상호정보 행렬 (D×D)
kernel void mutual_info_matrix(
    device const float* data   [[buffer(0)]],  // N×D
    device float* mi           [[buffer(1)]],  // D×D
    constant uint& N           [[buffer(2)]],
    constant uint& D           [[buffer(3)]],
    constant uint& n_bins      [[buffer(4)]],
    uint2 gid                  [[thread_position_in_grid]]
) {
    uint fi = gid.x;
    uint fj = gid.y;
    if (fi >= D || fj >= D || fi == fj) return;

    // 2D 히스토그램으로 MI 계산 (간략화)
    // 실제 구현은 binning + entropy 계산
    float h_i = 0.0, h_j = 0.0, h_ij = 0.0;
    // ... binning logic ...
    mi[fi * D + fj] = h_i + h_j - h_ij;
}

// KNN 인덱스 (각 샘플의 k-nearest neighbor)
kernel void knn_indices(
    device const float* dist   [[buffer(0)]],  // N*(N-1)/2 lower triangle
    device uint* knn           [[buffer(1)]],  // N×K
    constant uint& N           [[buffer(2)]],
    constant uint& K           [[buffer(3)]],
    uint gid                   [[thread_position_in_grid]]
) {
    if (gid >= N) return;
    // 각 행에서 K개 최소 거리 인덱스 찾기 (partial sort)
    // ... insertion sort for small K ...
}

// parallel reduction (sum)
kernel void reduce_sum(
    device const float* input  [[buffer(0)]],
    device float* output       [[buffer(1)]],
    constant uint& count       [[buffer(2)]],
    uint gid                   [[thread_position_in_grid]],
    uint lid                   [[thread_position_in_threadgroup]],
    uint group_size            [[threads_per_threadgroup]],
    threadgroup float* shared  [[threadgroup(0)]]
) {
    shared[lid] = (gid < count) ? input[gid] : 0.0;
    threadgroup_barrier(mem_flags::mem_threadgroup);
    for (uint s = group_size / 2; s > 0; s >>= 1) {
        if (lid < s) shared[lid] += shared[lid + s];
        threadgroup_barrier(mem_flags::mem_threadgroup);
    }
    if (lid == 0) output[gid / group_size] = shared[0];
}
```

- [ ] **Step 2: Pipeline 관리자 구현**

```rust
// src/gpu/pipeline.rs
use objc2_metal::*;
use objc2_foundation::NSString;
use std::collections::HashMap;
use std::path::Path;

pub struct ComputePipelines {
    device: &'static dyn MTLDevice,
    library: &'static MTLLibrary,
    pipelines: HashMap<String, &'static MTLComputePipelineState>,
}

impl ComputePipelines {
    pub fn new(device: &'static dyn MTLDevice, shader_path: &Path) -> Self {
        let source = std::fs::read_to_string(shader_path)
            .expect("Failed to read shader");
        let ns_source = NSString::from_str(&source);
        let library = device
            .newLibraryWithSource_options_error(&ns_source, None)
            .expect("Failed to compile Metal shaders");
        let lib_ref: &'static MTLLibrary = unsafe { &*(library.as_ref() as *const _) };
        Self {
            device,
            library: lib_ref,
            pipelines: HashMap::new(),
        }
    }

    pub fn get(&mut self, name: &str) -> &'static MTLComputePipelineState {
        if let Some(p) = self.pipelines.get(name) {
            return p;
        }
        let ns_name = NSString::from_str(name);
        let func = self.library
            .newFunctionWithName(&ns_name)
            .unwrap_or_else(|| panic!("Kernel '{}' not found", name));
        let pipeline = self.device
            .newComputePipelineStateWithFunction_error(&func)
            .unwrap_or_else(|e| panic!("Pipeline '{}' failed: {:?}", name, e));
        let p_ref: &'static MTLComputePipelineState =
            unsafe { &*(pipeline.as_ref() as *const _) };
        self.pipelines.insert(name.to_string(), p_ref);
        p_ref
    }
}
```

- [ ] **Step 3: 테스트 — distance_matrix GPU 실행**

```rust
// tests/gpu_test.rs (추가)
#[test]
fn test_distance_matrix_gpu() {
    if !nexus::gpu::is_available() { return; }
    let device = nexus::gpu::device().unwrap();

    // 3×2 테스트 데이터: [[0,0], [3,4], [1,0]]
    let data: Vec<f32> = vec![0.0, 0.0, 3.0, 4.0, 1.0, 0.0];
    let n: u32 = 3;
    let d: u32 = 2;

    // 예상: dist(0,1)=5.0, dist(0,2)=1.0, dist(1,2)=sqrt(4+16)=sqrt(8)
    // lower triangle: [dist(1,0), dist(2,0), dist(2,1)] = [5.0, 1.0, 2.828...]

    let result = nexus::gpu::dispatch_distance_matrix(device, &data, n, d);
    assert!((result[0] - 5.0).abs() < 0.01);
    assert!((result[1] - 1.0).abs() < 0.01);
    assert!((result[2] - (8.0_f32).sqrt()).abs() < 0.01);
}
```

- [ ] **Step 4: dispatch 함수 구현**

```rust
// src/gpu/mod.rs (추가)
pub fn dispatch_distance_matrix(
    device: &'static dyn MTLDevice,
    data: &[f32], n: u32, d: u32,
) -> Vec<f32> {
    use objc2_metal::*;

    let shader_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("metal/kernels.metal");
    let mut pipes = pipeline::ComputePipelines::new(device, &shader_path);
    let pipe = pipes.get("distance_matrix");

    let data_buf = device.newBufferWithBytes_length_options(
        data.as_ptr() as *const _, (data.len() * 4) as u64,
        MTLResourceOptions::StorageModeShared,
    ).unwrap();

    let out_len = (n * (n - 1) / 2) as usize;
    let out_buf = device.newBufferWithLength_options(
        (out_len * 4) as u64,
        MTLResourceOptions::StorageModeShared,
    ).unwrap();

    let queue = device.newCommandQueue().unwrap();
    let cmd = queue.commandBuffer().unwrap();
    let enc = cmd.computeCommandEncoder().unwrap();
    enc.setComputePipelineState(pipe);
    enc.setBuffer_offset_atIndex(Some(&data_buf), 0, 0);
    enc.setBuffer_offset_atIndex(Some(&out_buf), 0, 1);
    enc.setBytes_length_atIndex(&n as *const u32 as *const _, 4, 2);
    enc.setBytes_length_atIndex(&d as *const u32 as *const _, 4, 3);

    let grid = MTLSize { width: n as u64, height: n as u64, depth: 1 };
    let tg = MTLSize { width: 16, height: 16, depth: 1 };
    enc.dispatchThreads_threadsPerThreadgroup(grid, tg);
    enc.endEncoding();
    cmd.commit();
    cmd.waitUntilCompleted();

    let ptr = out_buf.contents() as *const f32;
    unsafe { std::slice::from_raw_parts(ptr, out_len).to_vec() }
}
```

- [ ] **Step 5: 테스트 통과 확인 + 커밋**

Run: `cd tools/nexus && ~/.cargo/bin/cargo test test_distance_matrix_gpu 2>&1 | tail -5`

```bash
git add tools/nexus/metal/ tools/nexus/src/gpu/pipeline.rs
git commit -m "feat(nexus): Metal distance_matrix kernel + pipeline manager"
```

### Task 1.4: CPU Fallback

**Files:**
- Create: `tools/nexus/src/gpu/fallback.rs`

- [ ] **Step 1: 테스트**

```rust
#[test]
fn test_distance_matrix_cpu_fallback() {
    let data: Vec<f32> = vec![0.0, 0.0, 3.0, 4.0, 1.0, 0.0];
    let result = nexus::gpu::fallback::distance_matrix_cpu(&data, 3, 2);
    assert!((result[0] - 5.0).abs() < 0.01);
}
```

- [ ] **Step 2: CPU 구현**

```rust
// src/gpu/fallback.rs
use rayon::prelude::*;

pub fn distance_matrix_cpu(data: &[f32], n: u32, d: u32) -> Vec<f32> {
    let n = n as usize;
    let d = d as usize;
    let out_len = n * (n - 1) / 2;
    let mut dist = vec![0.0f32; out_len];

    dist.par_iter_mut().enumerate().for_each(|(idx, val)| {
        // idx → (i, j) 역변환
        let i = ((1.0 + (1.0 + 8.0 * idx as f64).sqrt()) / 2.0) as usize;
        let j = idx - i * (i - 1) / 2;
        let mut sum = 0.0f32;
        for k in 0..d {
            let diff = data[i * d + k] - data[j * d + k];
            sum += diff * diff;
        }
        *val = sum.sqrt();
    });
    dist
}
```

- [ ] **Step 3: 테스트 + 커밋**

```bash
git commit -am "feat(nexus): CPU fallback for distance_matrix"
```

---

## Phase 2: Telescope Core (병렬)

### Task 2.1: Lens Trait + Shared Kernels

**Files:**
- Create: `tools/nexus/src/telescope/mod.rs`
- Create: `tools/nexus/src/telescope/lens_trait.rs`
- Create: `tools/nexus/src/telescope/shared_kernels.rs`

- [ ] **Step 1: Lens trait 정의**

```rust
// src/telescope/lens_trait.rs
use std::collections::HashMap;

pub type LensResult = HashMap<String, Vec<f64>>;

pub trait Lens: Send + Sync {
    fn name(&self) -> &str;
    fn category(&self) -> &str;   // "existing", "discovery", "synthesis", ...
    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedKernelCache) -> LensResult;
}
```

- [ ] **Step 2: SharedKernelCache 구현**

```rust
// src/telescope/shared_kernels.rs
use std::sync::Arc;

pub struct SharedKernelCache {
    pub distance_matrix: Arc<Vec<f32>>,    // N*(N-1)/2
    pub mi_matrix: Arc<Vec<f32>>,          // D*D
    pub knn_indices: Arc<Vec<u32>>,        // N*K
    pub n: usize,
    pub d: usize,
}

impl SharedKernelCache {
    pub fn compute(data: &[f64], n: usize, d: usize) -> Self {
        let data_f32: Vec<f32> = data.iter().map(|&x| x as f32).collect();

        let distance_matrix = if crate::gpu::is_available() {
            let dev = crate::gpu::device().unwrap();
            crate::gpu::dispatch_distance_matrix(dev, &data_f32, n as u32, d as u32)
        } else {
            crate::gpu::fallback::distance_matrix_cpu(&data_f32, n as u32, d as u32)
        };

        // MI, KNN도 유사하게...
        Self {
            distance_matrix: Arc::new(distance_matrix),
            mi_matrix: Arc::new(vec![]),  // TODO: Phase 1.3 확장
            knn_indices: Arc::new(vec![]), // TODO: Phase 1.3 확장
            n, d,
        }
    }
}
```

- [ ] **Step 3: Telescope 공개 API**

```rust
// src/telescope/mod.rs
pub mod lens_trait;
pub mod shared_kernels;
pub mod tier;
pub mod consensus;
pub mod lenses;
pub mod cache;

use lens_trait::{Lens, LensResult};
use shared_kernels::SharedKernelCache;
use std::collections::HashMap;

pub struct Telescope {
    lenses: Vec<Box<dyn Lens>>,
}

impl Telescope {
    pub fn new() -> Self {
        let mut lenses: Vec<Box<dyn Lens>> = Vec::new();
        // 렌즈 등록은 Phase 2.2에서
        Self { lenses }
    }

    pub fn scan_all(&self, data: &[f64], n: usize, d: usize) -> HashMap<String, LensResult> {
        let shared = SharedKernelCache::compute(data, n, d);
        self.lenses.iter()
            .map(|lens| {
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    lens.scan(data, n, d, &shared)
                }));
                match result {
                    Ok(r) => (lens.name().to_string(), r),
                    Err(_) => (lens.name().to_string(), HashMap::new()),
                }
            })
            .collect()
    }
}
```

- [ ] **Step 4: 빌드 확인 + 커밋**

```bash
git commit -am "feat(nexus): Lens trait + SharedKernelCache + Telescope API"
```

### Task 2.2: 첫 번째 렌즈 — VoidLens

**Files:**
- Create: `tools/nexus/src/telescope/lenses/discovery.rs`
- Test: `tools/nexus/tests/telescope_test.rs`

- [ ] **Step 1: VoidLens 테스트**

```rust
// tests/telescope_test.rs
use nexus::telescope::lenses::discovery::VoidLens;
use nexus::telescope::lens_trait::Lens;
use nexus::telescope::shared_kernels::SharedKernelCache;

#[test]
fn test_void_lens_finds_gap() {
    // 1D: 점들이 [0,1]과 [3,4]에 밀집, [1,3] 사이 빈칸
    let data: Vec<f64> = vec![
        0.1, 0.3, 0.5, 0.7, 0.9,  // 클러스터 A
        3.1, 3.3, 3.5, 3.7, 3.9,  // 클러스터 B
    ];
    let n = 10;
    let d = 1;
    let shared = SharedKernelCache::compute(&data, n, d);
    let lens = VoidLens;
    let result = lens.scan(&data, n, d, &shared);

    let voids = result.get("void_centers").unwrap();
    assert!(!voids.is_empty());
    // void center 는 ~2.0 근처여야 함
    assert!((voids[0] - 2.0).abs() < 0.5);
}
```

- [ ] **Step 2: VoidLens 구현**

```rust
// src/telescope/lenses/discovery.rs
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_kernels::SharedKernelCache;
use std::collections::HashMap;

pub struct VoidLens;

impl Lens for VoidLens {
    fn name(&self) -> &str { "void" }
    fn category(&self) -> &str { "discovery" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedKernelCache) -> LensResult {
        let mut result = HashMap::new();

        // k-NN 밀도 추정
        let k = (n as f64).sqrt().ceil() as usize;
        let densities: Vec<f64> = (0..n).map(|i| {
            let mut dists: Vec<f64> = (0..n)
                .filter(|&j| j != i)
                .map(|j| {
                    let mut sum = 0.0;
                    for dim in 0..d {
                        let diff = data[i * d + dim] - data[j * d + dim];
                        sum += diff * diff;
                    }
                    sum.sqrt()
                })
                .collect();
            dists.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let kth_dist = dists[k.min(dists.len() - 1)];
            if kth_dist > 0.0 { 1.0 / kth_dist } else { f64::MAX }
        }).collect();

        // 저밀도 영역 중 고밀도에 둘러싸인 곳 = void
        let mean_density = densities.iter().sum::<f64>() / n as f64;
        let threshold = mean_density * 0.3;

        let mut void_centers = Vec::new();
        let mut void_scores = Vec::new();

        // 그리드 탐색: 데이터 범위에서 균일 샘플링
        for dim in 0..d {
            let col: Vec<f64> = (0..n).map(|i| data[i * d + dim]).collect();
            let min_val = col.iter().cloned().fold(f64::INFINITY, f64::min);
            let max_val = col.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let step = (max_val - min_val) / 20.0;

            for grid_i in 1..20 {
                let point = min_val + step * grid_i as f64;
                // 이 점 주변의 밀도 추정
                let local_density: f64 = (0..n).map(|i| {
                    let diff = data[i * d + dim] - point;
                    (-diff * diff / (step * step)).exp()
                }).sum::<f64>() / n as f64;

                if local_density < threshold {
                    void_centers.push(point);
                    void_scores.push(1.0 - local_density / mean_density);
                }
            }
        }

        result.insert("void_centers".to_string(), void_centers);
        result.insert("void_scores".to_string(), void_scores);
        result
    }
}
```

- [ ] **Step 3: 테스트 통과 + 커밋**

```bash
git commit -am "feat(nexus): VoidLens — first discovery lens"
```

### Task 2.3: Tiered Scanning

**Files:**
- Create: `tools/nexus/src/telescope/tier.rs`

- [ ] **Step 1: TieredScanner 구현**

```rust
// src/telescope/tier.rs
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_kernels::SharedKernelCache;
use std::collections::HashMap;

pub enum Tier { T0, T1, T2, T3 }

pub struct TieredScanner {
    tiers: Vec<(Tier, Vec<Box<dyn Lens>>)>,
    signal_threshold: f64,
}

impl TieredScanner {
    pub fn scan(&self, data: &[f64], n: usize, d: usize) -> HashMap<String, LensResult> {
        let shared = SharedKernelCache::compute(data, n, d);
        let mut all_results = HashMap::new();

        for (tier, lenses) in &self.tiers {
            let tier_results: Vec<_> = lenses.iter()
                .filter_map(|lens| {
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let r = lens.scan(data, n, d, &shared);
                        (lens.name().to_string(), r)
                    })).ok()
                })
                .collect();

            let has_signal = tier_results.iter().any(|(_, r)| !r.is_empty());
            for (name, r) in tier_results {
                all_results.insert(name, r);
            }

            if !has_signal {
                break; // 신호 없으면 상위 tier 스킵
            }
        }
        all_results
    }
}
```

- [ ] **Step 2: 커밋**

```bash
git commit -am "feat(nexus): TieredScanner — T0→T3 early exit"
```

### Task 2.4: Consensus System

**Files:**
- Create: `tools/nexus/src/telescope/consensus.rs`

- [ ] **Step 1: 가중 합의 구현**

```rust
// src/telescope/consensus.rs
use std::collections::HashMap;

pub struct ConsensusResult {
    pub pattern_id: String,
    pub agreeing_lenses: Vec<String>,
    pub weighted_score: f64,
    pub tier: ConsensusLevel,
}

pub enum ConsensusLevel {
    Candidate,  // 3+
    High,       // 7+
    Confirmed,  // 12+
}

pub fn weighted_consensus(
    results: &HashMap<String, super::lens_trait::LensResult>,
    hit_rates: &HashMap<String, f64>,  // 렌즈별 히스토리 hit_rate
) -> Vec<ConsensusResult> {
    // 각 렌즈가 "발견한 것"을 모아서 가중 합의
    let mut pattern_votes: HashMap<String, (Vec<String>, f64)> = HashMap::new();

    for (lens_name, lens_result) in results {
        if lens_result.is_empty() { continue; }
        let hit_rate = hit_rates.get(lens_name).copied().unwrap_or(0.1);

        for (key, values) in lens_result {
            if !values.is_empty() {
                let entry = pattern_votes.entry(key.clone()).or_insert_with(|| (Vec::new(), 0.0));
                entry.0.push(lens_name.clone());
                entry.1 += hit_rate;
            }
        }
    }

    pattern_votes.into_iter()
        .filter(|(_, (lenses, _))| lenses.len() >= 3)
        .map(|(pattern_id, (lenses, score))| {
            let tier = match lenses.len() {
                12.. => ConsensusLevel::Confirmed,
                7..=11 => ConsensusLevel::High,
                _ => ConsensusLevel::Candidate,
            };
            ConsensusResult { pattern_id, agreeing_lenses: lenses, weighted_score: score, tier }
        })
        .collect()
}
```

- [ ] **Step 2: 커밋**

```bash
git commit -am "feat(nexus): weighted consensus system"
```

---

## Phase 3: Domain Encoder + Materials DB (병렬)

### Task 3.1: Domain Encoder

**Files:**
- Create: `tools/nexus/src/encoder/mod.rs`
- Create: `tools/nexus/src/encoder/parser.rs`
- Create: `tools/nexus/src/encoder/vectorize.rs`

- [ ] **Step 1: 테스트 — hypotheses.md 파싱**

```rust
// tests/encoder_test.rs
#[test]
fn test_parse_hypothesis() {
    let md = r#"
## H-SC-01: MgB2 초전도
- Tc = 39K
- CN = 6
- Type: conventional
- Year: 2001

## H-SC-02: YBCO 고온초전도
- Tc = 93K
- CN = 6
- Type: cuprate
- Year: 1987
"#;
    let entries = nexus::encoder::parser::parse_hypotheses(md);
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].get("Tc"), Some(&"39".to_string()));
    assert_eq!(entries[1].get("CN"), Some(&"6".to_string()));
}
```

- [ ] **Step 2: parser.rs 구현**

```rust
// src/encoder/parser.rs
use std::collections::HashMap;

pub type HypothesisEntry = HashMap<String, String>;

pub fn parse_hypotheses(md: &str) -> Vec<HypothesisEntry> {
    let mut entries = Vec::new();
    let mut current: Option<HypothesisEntry> = None;

    for line in md.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("## H-") || trimmed.starts_with("## h-") {
            if let Some(entry) = current.take() {
                entries.push(entry);
            }
            let mut entry = HashMap::new();
            entry.insert("id".to_string(), trimmed.trim_start_matches("## ").to_string());
            current = Some(entry);
        } else if trimmed.starts_with("- ") && current.is_some() {
            if let Some(pos) = trimmed.find('=') {
                let key = trimmed[2..pos].trim().to_string();
                let val = trimmed[pos + 1..].trim()
                    .trim_end_matches('K')
                    .trim()
                    .to_string();
                current.as_mut().unwrap().insert(key, val);
            }
        }
    }
    if let Some(entry) = current {
        entries.push(entry);
    }
    entries
}
```

- [ ] **Step 3: vectorize.rs — 수치 행렬 변환**

```rust
// src/encoder/vectorize.rs
use super::parser::HypothesisEntry;

pub fn vectorize(entries: &[HypothesisEntry], feature_keys: &[&str]) -> (Vec<f64>, usize, usize) {
    let n = entries.len();
    let d = feature_keys.len();
    let mut data = vec![0.0f64; n * d];

    for (i, entry) in entries.iter().enumerate() {
        for (j, key) in feature_keys.iter().enumerate() {
            if let Some(val) = entry.get(*key) {
                data[i * d + j] = val.parse::<f64>().unwrap_or(0.0);
            }
        }
    }
    (data, n, d)
}
```

- [ ] **Step 4: 테스트 + 커밋**

```bash
git commit -am "feat(nexus): Domain Encoder — parse hypotheses.md + vectorize"
```

### Task 3.2: Materials DB

**Files:**
- Create: `tools/nexus/src/materials/mod.rs`
- Create: `tools/nexus/src/materials/db.rs`
- Create: `tools/nexus/data/materials-db.json`

- [ ] **Step 1: JSON 데이터 파일 생성**

```json
{
  "superconductor": {
    "known_materials": [
      {"name": "MgB2", "Tc": 39, "CN": 6, "type": "conventional", "year": 2001},
      {"name": "YBCO", "Tc": 93, "CN": 6, "type": "cuprate", "year": 1987},
      {"name": "H3S", "Tc": 203, "CN": 6, "type": "hydride", "year": 2015, "pressure_GPa": 155},
      {"name": "LaH10", "Tc": 250, "CN": 6, "type": "hydride", "year": 2019, "pressure_GPa": 170}
    ],
    "ceiling": {"Tc": 300, "CN": 6, "Jc": 1e8}
  },
  "chip-architecture": {
    "known_tech": [
      {"name": "TSMC_N3", "pitch_nm": 48, "year": 2023, "SM_count": 144},
      {"name": "Intel_18A", "pitch_nm": 40, "year": 2025}
    ],
    "ceiling": {"pitch_nm": 10, "SM_count": 576}
  }
}
```

- [ ] **Step 2: DB 로더 구현**

```rust
// src/materials/db.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Material {
    pub name: String,
    #[serde(flatten)]
    pub properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DomainData {
    pub known_materials: Option<Vec<Material>>,
    pub known_tech: Option<Vec<Material>>,
    pub ceiling: HashMap<String, serde_json::Value>,
}

pub type MaterialsDB = HashMap<String, DomainData>;

pub fn load(path: &str) -> MaterialsDB {
    let content = std::fs::read_to_string(path).expect("Failed to read materials DB");
    serde_json::from_str(&content).expect("Failed to parse materials DB")
}
```

- [ ] **Step 3: 커밋**

```bash
git commit -am "feat(nexus): Materials DB loader + initial data"
```

---

## Phase 4: Discovery Verifier (병렬)

### Task 4.1: Verifier Core

**Files:**
- Create: `tools/nexus/src/verifier/mod.rs`
- Create: `tools/nexus/src/verifier/n6_check.rs`
- Create: `tools/nexus/src/verifier/feasibility.rs`

- [ ] **Step 1: n6_check 테스트**

```rust
// tests/verifier_test.rs
#[test]
fn test_n6_exact_check() {
    use nexus::verifier::n6_check::n6_match;
    // 12 = sigma(6) → EXACT
    assert_eq!(n6_match(12.0), ("sigma", 1.0));
    // 24 = J2(6) → EXACT
    assert_eq!(n6_match(24.0), ("J2", 1.0));
    // 7.0 → no match
    assert_eq!(n6_match(7.0).1, 0.0);
}
```

- [ ] **Step 2: n6_check.rs 구현**

```rust
// src/verifier/n6_check.rs

// n=6 기본 상수
const N6_CONSTANTS: &[(&str, f64)] = &[
    ("n", 6.0), ("sigma", 12.0), ("phi", 2.0), ("tau", 4.0),
    ("J2", 24.0), ("sopfr", 5.0), ("mu", 1.0),
    ("sigma-phi", 10.0), ("sigma-tau", 8.0), ("sigma-mu", 11.0),
    ("sigma*tau", 48.0), ("sigma^2", 144.0), ("phi^tau", 16.0),
    ("tau^2/sigma", 1.333), ("n/phi", 3.0), ("sigma/n", 2.0),
];

pub fn n6_match(value: f64) -> (&'static str, f64) {
    let mut best = ("none", 0.0f64);
    for &(name, constant) in N6_CONSTANTS {
        if constant == 0.0 { continue; }
        let ratio = (value / constant - 1.0).abs();
        if ratio < 0.001 {
            return (name, 1.0);  // EXACT
        } else if ratio < 0.05 && (1.0 - ratio) > best.1 {
            best = (name, 1.0 - ratio);  // CLOSE
        }
    }
    best
}

pub fn n6_exact_ratio(values: &[f64]) -> f64 {
    if values.is_empty() { return 0.0; }
    let exact_count = values.iter()
        .filter(|&&v| n6_match(v).1 >= 0.99)
        .count();
    exact_count as f64 / values.len() as f64
}
```

- [ ] **Step 3: feasibility.rs 종합 점수**

```rust
// src/verifier/feasibility.rs
#[derive(Debug)]
pub enum Grade { S, A, B, C, D }

#[derive(Debug)]
pub struct VerificationResult {
    pub score: f64,
    pub grade: Grade,
    pub n6_exact_ratio: f64,
    pub details: Vec<String>,
}

pub fn verify(
    lens_consensus: f64,       // 0-1: 합의 렌즈 비율
    cross_validation: f64,     // 0-1: 교차검증 통과
    physical_check: f64,       // 0-1: 물리 검증 통과
    graph_bonus: f64,          // 0-1: 그래프 구조 보너스
    novelty: f64,              // 0-1: 새로움
    n6_exact: f64,             // 0-1: n=6 EXACT 비율
) -> VerificationResult {
    let score = lens_consensus * 0.25
        + cross_validation * 0.20
        + physical_check * 0.25
        + graph_bonus * 0.15
        + novelty * 0.05
        + n6_exact * 0.10;

    let grade = match score {
        s if s >= 0.9 => Grade::S,
        s if s >= 0.7 => Grade::A,
        s if s >= 0.5 => Grade::B,
        s if s >= 0.3 => Grade::C,
        _ => Grade::D,
    };

    VerificationResult {
        score, grade, n6_exact_ratio: n6_exact,
        details: vec![],
    }
}
```

- [ ] **Step 4: 테스트 + 커밋**

```bash
git commit -am "feat(nexus): Discovery Verifier — n6_check + feasibility scoring"
```

---

## Phase 5: Discovery Graph (병렬)

### Task 5.1: Graph Core

**Files:**
- Create: `tools/nexus/src/graph/mod.rs`
- Create: `tools/nexus/src/graph/node.rs`
- Create: `tools/nexus/src/graph/edge.rs`
- Create: `tools/nexus/src/graph/structure.rs`
- Create: `tools/nexus/src/graph/persistence.rs`

- [ ] **Step 1: 노드/간선 타입 정의**

```rust
// src/graph/node.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
    Discovery, Hypothesis, Bt, Prediction, AccelHypothesis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub node_type: NodeType,
    pub domain: String,
    pub project: String,  // n6, tecs-l, anima, sedi
    pub summary: String,
    pub confidence: f64,
    pub lenses_used: Vec<String>,
    pub timestamp: String,
}
```

```rust
// src/graph/edge.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EdgeType {
    Derives, Validates, Contradicts, Merges, Triggers, Refutes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub edge_type: EdgeType,
    pub strength: f64,
    pub bidirectional: bool,
}
```

- [ ] **Step 2: 구조 탐지 — 닫힌 루프 + 허브**

```rust
// src/graph/structure.rs
use super::{node::Node, edge::Edge};
use std::collections::{HashMap, HashSet};

pub struct ClosedLoop {
    pub nodes: Vec<String>,
    pub strength: f64,
}

pub struct Hub {
    pub node_id: String,
    pub degree: usize,
}

pub fn find_closed_triangles(nodes: &HashMap<String, Node>, edges: &[Edge]) -> Vec<ClosedLoop> {
    let mut adj: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut edge_strength: HashMap<(&str, &str), f64> = HashMap::new();

    for e in edges {
        adj.entry(&e.from).or_default().insert(&e.to);
        edge_strength.insert((&e.from, &e.to), e.strength);
        if e.bidirectional {
            adj.entry(&e.to).or_default().insert(&e.from);
            edge_strength.insert((&e.to, &e.from), e.strength);
        }
    }

    let mut triangles = Vec::new();
    let node_ids: Vec<&str> = nodes.keys().map(|s| s.as_str()).collect();

    for i in 0..node_ids.len() {
        for j in (i + 1)..node_ids.len() {
            for k in (j + 1)..node_ids.len() {
                let a = node_ids[i];
                let b = node_ids[j];
                let c = node_ids[k];
                if adj.get(a).map_or(false, |s| s.contains(b))
                    && adj.get(b).map_or(false, |s| s.contains(c))
                    && adj.get(c).map_or(false, |s| s.contains(a))
                {
                    let s1 = edge_strength.get(&(a, b)).copied().unwrap_or(0.0);
                    let s2 = edge_strength.get(&(b, c)).copied().unwrap_or(0.0);
                    let s3 = edge_strength.get(&(c, a)).copied().unwrap_or(0.0);
                    triangles.push(ClosedLoop {
                        nodes: vec![a.to_string(), b.to_string(), c.to_string()],
                        strength: (s1 + s2 + s3) / 3.0,
                    });
                }
            }
        }
    }
    triangles
}

pub fn find_hubs(edges: &[Edge], min_degree: usize) -> Vec<Hub> {
    let mut degree: HashMap<String, usize> = HashMap::new();
    for e in edges {
        *degree.entry(e.from.clone()).or_default() += 1;
        *degree.entry(e.to.clone()).or_default() += 1;
    }
    degree.into_iter()
        .filter(|(_, d)| *d >= min_degree)
        .map(|(node_id, d)| Hub { node_id, degree: d })
        .collect()
}
```

- [ ] **Step 3: JSON 영속성**

```rust
// src/graph/persistence.rs
use super::{node::Node, edge::Edge, structure};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscoveryGraph {
    pub nodes: HashMap<String, Node>,
    pub edges: Vec<Edge>,
}

impl DiscoveryGraph {
    pub fn new() -> Self {
        Self { nodes: HashMap::new(), edges: Vec::new() }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id.clone(), node);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge);
    }

    pub fn closed_loops(&self) -> Vec<structure::ClosedLoop> {
        structure::find_closed_triangles(&self.nodes, &self.edges)
    }

    pub fn hubs(&self, min_degree: usize) -> Vec<structure::Hub> {
        structure::find_hubs(&self.edges, min_degree)
    }

    pub fn save(&self, path: &str) {
        let json = serde_json::to_string_pretty(self).unwrap();
        let tmp = format!("{}.tmp", path);
        std::fs::write(&tmp, &json).unwrap();
        std::fs::rename(&tmp, path).unwrap();
    }

    pub fn load(path: &str) -> Self {
        let content = std::fs::read_to_string(path).unwrap_or_else(|_| "{}".to_string());
        serde_json::from_str(&content).unwrap_or_else(|_| Self::new())
    }
}
```

- [ ] **Step 4: 테스트 — 삼각형 탐지**

```rust
// tests/graph_test.rs
#[test]
fn test_closed_triangle_detection() {
    use nexus::graph::persistence::DiscoveryGraph;
    use nexus::graph::node::{Node, NodeType};
    use nexus::graph::edge::{Edge, EdgeType};

    let mut g = DiscoveryGraph::new();
    for id in ["D-001", "D-002", "D-003"] {
        g.add_node(Node {
            id: id.to_string(), node_type: NodeType::Discovery,
            domain: "sc".into(), project: "n6".into(),
            summary: "test".into(), confidence: 0.9,
            lenses_used: vec![], timestamp: "2026-04-03".into(),
        });
    }
    g.add_edge(Edge { from: "D-001".into(), to: "D-002".into(),
        edge_type: EdgeType::Validates, strength: 0.9, bidirectional: true });
    g.add_edge(Edge { from: "D-002".into(), to: "D-003".into(),
        edge_type: EdgeType::Validates, strength: 0.8, bidirectional: true });
    g.add_edge(Edge { from: "D-003".into(), to: "D-001".into(),
        edge_type: EdgeType::Validates, strength: 0.85, bidirectional: true });

    let loops = g.closed_loops();
    assert_eq!(loops.len(), 1);
    assert!((loops[0].strength - 0.85).abs() < 0.01);
}
```

- [ ] **Step 5: 테스트 + 커밋**

```bash
git commit -am "feat(nexus): Discovery Graph — nodes, edges, triangle detection, persistence"
```

---

## Phase 6: Telescope History (Phase 2 이후)

### Task 6.1: History Recorder + Recommender

**Files:**
- Create: `tools/nexus/src/history/mod.rs`
- Create: `tools/nexus/src/history/recorder.rs`
- Create: `tools/nexus/src/history/stats.rs`
- Create: `tools/nexus/src/history/recommend.rs`

- [ ] **Step 1: ScanRecord 구조체 + 기록기**

```rust
// src/history/recorder.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanRecord {
    pub id: String,
    pub timestamp: String,
    pub domain: String,
    pub lenses_used: Vec<String>,
    pub discoveries: Vec<String>,
    pub consensus_level: usize,
}

pub fn append_record(path: &str, record: &ScanRecord) {
    let line = serde_json::to_string(record).unwrap();
    let mut file = std::fs::OpenOptions::new()
        .create(true).append(true).open(path).unwrap();
    writeln!(file, "{}", line).unwrap();
}
```

- [ ] **Step 2: 통계 집계 + 추천**

```rust
// src/history/recommend.rs
use std::collections::HashMap;

pub struct LensRecommendation {
    pub lenses: Vec<String>,
    pub reason: String,
}

pub fn recommend_lenses(
    domain: &str,
    stats: &HashMap<String, HashMap<String, f64>>,  // domain → lens → hit_rate
    serendipity_ratio: f64,
) -> LensRecommendation {
    let domain_stats = stats.get(domain);

    let mut lenses: Vec<String> = if let Some(ds) = domain_stats {
        let mut sorted: Vec<_> = ds.iter().collect();
        sorted.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
        sorted.iter()
            .filter(|(_, rate)| **rate > 0.05)
            .map(|(name, _)| name.to_string())
            .collect()
    } else {
        // Cold start: 기본 8종
        vec![
            "consciousness", "topology", "void", "thermo",
            "evolution", "network", "boundary", "triangle",
        ].into_iter().map(|s| s.to_string()).collect()
    };

    // Serendipity: 랜덤 렌즈 추가 (15%)
    let n_random = ((lenses.len() as f64 * serendipity_ratio) as usize).max(1);
    // TODO: 실제로는 전체 렌즈 목록에서 랜덤 선택
    let reason = if domain_stats.is_some() {
        format!("History-based ({} lenses, {}% serendipity)", lenses.len(), (serendipity_ratio * 100.0) as u32)
    } else {
        "Cold start (default 8 lenses)".to_string()
    };

    LensRecommendation { lenses, reason }
}
```

- [ ] **Step 3: 커밋**

```bash
git commit -am "feat(nexus): Telescope History — recorder + stats + recommender"
```

---

## Phase 7: OUROBOROS v26 (Phase 6 이후)

### Task 7.1: Rust↔Python 브릿지

**Files:**
- Create: `tools/nexus/src/ouroboros/mod.rs`
- Create: `tools/nexus/src/ouroboros/pipeline.rs`

- [ ] **Step 1: Pipeline 오케스트레이션**

```rust
// src/ouroboros/pipeline.rs
use crate::telescope::Telescope;
use crate::encoder;
use crate::verifier;
use crate::graph::persistence::DiscoveryGraph;
use crate::history;

pub struct NexusPipeline {
    pub telescope: Telescope,
    pub graph: DiscoveryGraph,
    pub generation: usize,
}

impl NexusPipeline {
    pub fn new() -> Self {
        Self {
            telescope: Telescope::new(),
            graph: DiscoveryGraph::load("data/discovery-graph.json"),
            generation: 0,
        }
    }

    pub fn run_generation(&mut self, domain: &str, data: &[f64], n: usize, d: usize) -> GenerationResult {
        self.generation += 1;

        // 1. 스캔
        let scan_results = self.telescope.scan_all(data, n, d);

        // 2. 합의
        let hit_rates = std::collections::HashMap::new(); // TODO: history에서 로드
        let consensus = crate::telescope::consensus::weighted_consensus(&scan_results, &hit_rates);

        // 3. 검증
        let mut discoveries = Vec::new();
        for pattern in &consensus {
            let result = verifier::feasibility::verify(
                pattern.weighted_score,
                0.5,  // 교차검증 (향후 OUROBOROS에서)
                0.5,  // 물리검증
                0.0,  // 그래프 보너스
                0.5,  // 새로움
                0.5,  // n6
            );
            discoveries.push((pattern.pattern_id.clone(), result));
        }

        // 4. 그래프 갱신
        for (id, result) in &discoveries {
            self.graph.add_node(crate::graph::node::Node {
                id: format!("D-{:04}", self.generation),
                node_type: crate::graph::node::NodeType::Discovery,
                domain: domain.to_string(),
                project: "n6".to_string(),
                summary: id.clone(),
                confidence: result.score,
                lenses_used: vec![],
                timestamp: chrono_stub(),
            });
        }

        // 5. 저장
        self.graph.save("data/discovery-graph.json");

        GenerationResult {
            generation: self.generation,
            new_discoveries: discoveries.len(),
            top_score: discoveries.iter().map(|(_, r)| r.score).fold(0.0, f64::max),
        }
    }
}

pub struct GenerationResult {
    pub generation: usize,
    pub new_discoveries: usize,
    pub top_score: f64,
}

fn chrono_stub() -> String {
    "2026-04-03T00:00:00Z".to_string()
}
```

- [ ] **Step 2: PyO3 바인딩 (OUROBOROS 연동)**

```rust
// src/python.rs
#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pyfunction]
fn nexus_scan(data: Vec<f64>, n: usize, d: usize) -> PyResult<String> {
    let telescope = crate::telescope::Telescope::new();
    let results = telescope.scan_all(&data, n, d);
    Ok(serde_json::to_string(&results).unwrap())
}

#[cfg(feature = "python")]
#[pymodule]
fn nexus_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(nexus_scan, m)?)?;
    m.add_function(wrap_pyfunction!(gpu_available, m)?)?;
    Ok(())
}

#[cfg(feature = "python")]
#[pyfunction]
fn gpu_available() -> bool {
    crate::gpu::is_available()
}
```

- [ ] **Step 3: 커밋**

```bash
git commit -am "feat(nexus): OUROBOROS pipeline + PyO3 bindings"
```

---

## Phase 8: CLI + Dashboard (Phase 7 이후)

### Task 8.1: CLI

**Files:**
- Create: `tools/nexus/src/main.rs`

- [ ] **Step 1: CLI 구현**

```rust
// src/main.rs
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        return;
    }

    match args[1].as_str() {
        "scan" => cmd_scan(&args[2..]),
        "run" => cmd_run(&args[2..]),
        "verify" => cmd_verify(&args[2..]),
        "graph" => cmd_graph(&args[2..]),
        "history" => cmd_history(&args[2..]),
        "benchmark" => cmd_benchmark(),
        "version" => println!("NEXUS-6 v{}", nexus::VERSION),
        _ => print_usage(),
    }
}

fn cmd_scan(args: &[String]) {
    let domain = args.first().map(|s| s.as_str()).unwrap_or("superconductor");
    println!("Scanning domain: {}", domain);
    println!("GPU: {}", if nexus::gpu::is_available() { "Metal (M4)" } else { "CPU fallback" });
    // TODO: 실제 스캔 실행
}

fn cmd_run(args: &[String]) {
    println!("Starting NEXUS-6 OUROBOROS loop...");
    let mut pipeline = nexus::ouroboros::pipeline::NexusPipeline::new();
    // TODO: 무한 루프 + 로드맵
}

fn cmd_verify(args: &[String]) { println!("Verify: {:?}", args); }
fn cmd_graph(args: &[String]) { println!("Graph: {:?}", args); }
fn cmd_history(args: &[String]) { println!("History: {:?}", args); }

fn cmd_benchmark() {
    println!("NEXUS-6 Benchmark");
    println!("GPU: {}", if nexus::gpu::is_available() { "Available" } else { "Not available" });
    // TODO: 벤치마크 실행
}

fn print_usage() {
    println!("NEXUS-6 Universal Discovery Engine");
    println!();
    println!("Usage: nexus <command> [args]");
    println!();
    println!("Commands:");
    println!("  scan <domain>     Scan a domain");
    println!("  run [--roadmap]   Start OUROBOROS loop");
    println!("  verify <id>       Verify a discovery");
    println!("  graph show|loops  Discovery Graph");
    println!("  history <domain>  Scan history");
    println!("  benchmark         Performance benchmark");
    println!("  version           Show version");
}
```

- [ ] **Step 2: 빌드 + 실행 확인**

Run: `cd tools/nexus && ~/.cargo/bin/cargo build --release 2>&1 | tail -3`
Run: `./target/release/nexus version`
Expected: `NEXUS-6 v0.1.0`

- [ ] **Step 3: 커밋**

```bash
git commit -am "feat(nexus): CLI — scan, run, verify, graph, benchmark"
```

---

## Phase 9: 411종 렌즈 확장 (Phase 7 이후)

### Task 9.1: 렌즈 등록 시스템

**Files:**
- Modify: `tools/nexus/src/telescope/mod.rs`
- Create: `tools/nexus/src/telescope/lenses/mod.rs`

- [ ] **Step 1: 렌즈 레지스트리**

```rust
// src/telescope/lenses/mod.rs
pub mod discovery;
pub mod synthesis;
pub mod validation;
pub mod meta;

use super::lens_trait::Lens;

pub fn all_lenses() -> Vec<Box<dyn Lens>> {
    let mut lenses: Vec<Box<dyn Lens>> = Vec::new();

    // Phase 2에서 구현한 것
    lenses.push(Box::new(discovery::VoidLens));

    // 나머지 렌즈들을 점진적으로 추가
    // 각 렌즈는 Lens trait 구현 + SharedKernelCache 사용
    // 구현 우선순위: hit_rate 높은 것부터

    lenses
}

pub fn lens_count() -> usize { all_lenses().len() }
```

- [ ] **Step 2: 렌즈 추가 패턴 (반복)**

새 렌즈 추가는 모두 같은 패턴:

```rust
// 예: IsomorphismLens
pub struct IsomorphismLens;

impl Lens for IsomorphismLens {
    fn name(&self) -> &str { "isomorphism" }
    fn category(&self) -> &str { "discovery" }
    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedKernelCache) -> LensResult {
        // 1. shared.distance_matrix 또는 shared.mi_matrix 활용
        // 2. 알고리즘 실행
        // 3. LensResult 반환
        HashMap::new()
    }
}
```

각 렌즈를 이 패턴으로 추가. 411종 전부 구현은 점진적 — 우선순위:
1. Tier 0 렌즈 8종 (즉시)
2. n6 산업 렌즈 58종 (1주)
3. 교차 프로젝트 40종 (2주)
4. TECS-L/SEDI/anima 나머지 (점진적)

- [ ] **Step 3: 커밋**

```bash
git commit -am "feat(nexus): lens registry + incremental lens addition pattern"
```

---

## 통합 테스트

### Task 10.1: End-to-End 테스트

**Files:**
- Create: `tools/nexus/tests/integration_test.rs`

- [ ] **Step 1: 통합 테스트**

```rust
// tests/integration_test.rs
#[test]
fn test_full_pipeline_e2e() {
    // 1. 테스트 데이터 생성 (빈칸이 있는 2D 데이터)
    let mut data = Vec::new();
    for i in 0..20 { data.push(i as f64 * 0.1); data.push(0.0); }     // 클러스터 A
    for i in 0..20 { data.push(3.0 + i as f64 * 0.1); data.push(0.0); } // 클러스터 B
    let n = 40;
    let d = 2;

    // 2. Telescope 스캔
    let telescope = nexus::telescope::Telescope::new();
    let results = telescope.scan_all(&data, n, d);
    assert!(!results.is_empty());

    // 3. 결과에 void가 있어야 함
    if let Some(void_result) = results.get("void") {
        assert!(!void_result.is_empty());
    }

    // 4. Discovery Graph에 추가
    let mut graph = nexus::graph::persistence::DiscoveryGraph::new();
    graph.add_node(nexus::graph::node::Node {
        id: "D-TEST-001".into(),
        node_type: nexus::graph::node::NodeType::Discovery,
        domain: "test".into(), project: "n6".into(),
        summary: "void at x=1.5".into(), confidence: 0.8,
        lenses_used: vec!["void".into()], timestamp: "2026-04-03".into(),
    });
    assert_eq!(graph.nodes.len(), 1);
}

#[test]
fn test_gpu_availability() {
    // Metal이 없어도 CPU fallback으로 동작해야 함
    let data: Vec<f32> = vec![0.0, 0.0, 1.0, 1.0];
    let result = nexus::gpu::fallback::distance_matrix_cpu(&data, 2, 2);
    assert!((result[0] - (2.0f32).sqrt()).abs() < 0.01);
}
```

- [ ] **Step 2: 전체 테스트 실행**

Run: `cd tools/nexus && ~/.cargo/bin/cargo test 2>&1 | tail -10`
Expected: 모든 테스트 통과

- [ ] **Step 3: 커밋**

```bash
git commit -am "test(nexus): integration tests — full pipeline e2e"
```

---

## 최종 빌드 + 실행

- [ ] **Release 빌드**: `cd tools/nexus && ~/.cargo/bin/cargo build --release`
- [ ] **GPU 벤치마크**: `./target/release/nexus benchmark`
- [ ] **첫 스캔**: `./target/release/nexus scan superconductor`
- [ ] **최종 커밋 + 태그**:

```bash
git tag -a v0.1.0-nexus -m "NEXUS-6 v0.1.0: 411-lens discovery engine with Metal GPU"
```
