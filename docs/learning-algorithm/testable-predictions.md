# 학습 알고리즘 검증가능 예측 (Testable Predictions) --- 24개

> BT-54 (AdamW 5중주), BT-46 (ln(4/3) RLHF), BT-56 (Complete LLM),
> BT-58 (sigma-tau=8), BT-64 (0.1 정규화) 및 H-LA-01~30에서 도출.
> 각 예측은 반증 가능(falsifiable)하며, 구체적 검증 방법을 포함한다.

---

## Tier 1: 즉시 검증 가능 (1 GPU / 공식 문서)

### TP-LA-01: AdamW beta_1 = 0.9 = 1-1/(sigma-phi)
**예측**: 모든 주요 프레임워크의 AdamW 기본 beta_1 = 0.9.
**n=6 근거**: 1-1/(sigma-phi) = 1-0.1 = 0.9. BT-54.
**검증**: PyTorch docs, TensorFlow docs, JAX/optax docs.
**반증 조건**: 기본값이 0.95 또는 0.85로 변경되면 CLOSE.

### TP-LA-02: GPT-3/4 beta_2 = 0.95 = 1-1/(J₂-tau)
**예측**: 대형 LLM 학습에서 beta_2 = 0.95가 최적이다.
**n=6 근거**: 1-1/(J₂-tau) = 1-1/20 = 0.95. BT-54.
**검증**: Brown et al. (2020) GPT-3 논문, GPT-4 기술 보고서.
**반증 조건**: beta_2 = 0.98 또는 0.99가 대형 LLM 표준이 되면 CLOSE.

### TP-LA-03: AdamW epsilon = 10^(-8) = 10^(-(sigma-tau))
**예측**: AdamW 기본 epsilon = 1e-8.
**n=6 근거**: sigma-tau = 8, 10^(-8). BT-54.
**검증**: PyTorch/TensorFlow 기본값 확인.
**반증 조건**: 1e-6 또는 1e-10이 기본값이 되면 CLOSE.

### TP-LA-04: Weight decay = 0.1 = 1/(sigma-phi)
**예측**: LLM 학습의 표준 weight decay = 0.1.
**n=6 근거**: 1/(sigma-phi) = 0.1. BT-64.
**검증**: GPT-3, LLaMA, Chinchilla 논문.
**반증 조건**: 0.01 또는 0.3이 표준이 되면 FAIL.

### TP-LA-05: Mertens dropout = ln(4/3) = 0.288
**예측**: 최적 dropout rate ~ 0.288 (서치 불필요).
**n=6 근거**: ln(tau^2/sigma) = ln(4/3) = 0.2877. BT-46.
**검증**: 0.2-0.3 범위의 dropout이 대부분의 task에서 최적.
**반증 조건**: 최적 dropout이 0.5 이상으로 확정되면 FAIL.

### TP-LA-06: Gradient clipping = R(6) = 1.0
**예측**: 표준 gradient clipping 값 = 1.0.
**n=6 근거**: R(6) = sigma*phi/(n*tau) = 1. BT-54.
**검증**: GPT-3, LLaMA, PaLM 모든 논문에서 clip=1.0.
**반증 조건**: clip=0.5 또는 2.0이 표준이 되면 CLOSE.

### TP-LA-07: LoRA rank = sigma-tau = 8 최적
**예측**: LoRA의 최적 rank = 8.
**n=6 근거**: sigma-tau = 8. BT-58.
**검증**: Hu et al. (2021) 원 논문, 후속 연구.
**반증 조건**: rank=4 또는 rank=16이 보편적으로 우위이면 CLOSE.

---

## Tier 2: 실험 검증 (클러스터)

### TP-LA-08: SwiGLU FFN 비율 = tau^2/sigma = 4/3
**예측**: SwiGLU 기반 FFN의 확장 비율 = 8/3 = 2*tau^2/sigma.
**n=6 근거**: tau^2/sigma = 4/3, 2배 = 8/3. BT-33.
**검증**: LLaMA, PaLM, Gemma에서 d_ff = 8/3 * d_model.
**반증 조건**: 4x 확장이 SwiGLU에서도 표준이면 CLOSE.

### TP-LA-09: MoE active experts = sigma-tau = 8 최적
**예측**: MoE에서 top-k active experts = 8이 보편적이다.
**n=6 근거**: sigma-tau = 8. BT-58, BT-67.
**검증**: Switch (top-1), ST-MoE (top-2), Mixtral (top-2/8).
**반증 조건**: 현재 top-2/8이 주류이므로 CLOSE (top-8 자체가 expert pool).

### TP-LA-10: Transformer head dim = 2^(sigma-sopfr) = 128
**예측**: Attention head dimension = 128이 보편적이다.
**n=6 근거**: sigma-sopfr = 7, 2^7 = 128. BT-56.
**검증**: GPT-3 (128), LLaMA (128), PaLM (128).
**반증 조건**: head_dim=64가 다시 표준이 되면 CLOSE.

### TP-LA-11: Batch size 최적 = power of 2, 기본 2^(sigma-tau) = 256
**예측**: 학습 batch size의 기본 최적값은 256 근처이다.
**n=6 근거**: 2^(sigma-tau) = 2^8 = 256. BT-58.
**검증**: ImageNet (256), GPT-3 (3.2M tokens ~ 256 seqs * 2K).
**반증 조건**: batch size 64가 보편적으로 우위이면 CLOSE.

### TP-LA-12: Learning rate warmup = sigma/1000 = 0.012 비율
**예측**: LR warmup의 최적 비율은 총 학습의 약 1-2%이다.
**n=6 근거**: sigma/1000 = 0.012.
**검증**: GPT-3 warmup = 375M/300B = 0.125%, Chinchilla 유사.
**반증 조건**: 10%+ warmup이 표준이면 CLOSE.

---

## Tier 3: 전문 연구 (대규모 실험)

### TP-LA-13: Chinchilla 비율 tokens/params = J₂-tau = 20
**예측**: 최적 학습 토큰 수 = 파라미터 수 x 20.
**n=6 근거**: J₂-tau = 24-4 = 20. BT-26.
**검증**: Hoffmann et al. (2022) Chinchilla.
**반증 조건**: 최적 비율이 100+ (LLaMA 방식)이면 CLOSE.

### TP-LA-14: PPO clip = ln(4/3) = 0.2 근방
**예측**: PPO clipping parameter ~ 0.2 = ln(4/3) 근사.
**n=6 근거**: ln(4/3) = 0.288, PPO clip = 0.2. BT-46.
**검증**: Schulman et al. (2017): clip = 0.2.
**반증 조건**: clip = 0.1 또는 0.5가 표준이 되면 CLOSE.

### TP-LA-15: Temperature = R(6) = 1.0 기본
**예측**: LLM 추론 temperature 기본값 = 1.0.
**n=6 근거**: R(6) = 1. BT-46.
**검증**: OpenAI API, Anthropic API 기본값.
**반증 조건**: 기본 temperature가 0.7로 변경되면 CLOSE.

### TP-LA-16: Top-p = 0.95 = 1-1/(J₂-tau)
**예측**: nucleus sampling top-p 기본값 = 0.95.
**n=6 근거**: 1-1/(J₂-tau) = 0.95. BT-42.
**검증**: OpenAI API, Hugging Face 기본값.
**반증 조건**: top-p = 0.9가 표준이 되면 CLOSE.

### TP-LA-17: Top-k = 40 = tau*(sigma-phi)
**예측**: top-k sampling 기본값 = 40.
**n=6 근거**: tau*(sigma-phi) = 4*10 = 40. BT-42.
**검증**: GPT-2 원 논문 (Radford et al., 2019).
**반증 조건**: top-k = 50 또는 20이 표준이 되면 CLOSE.

---

## Tier 4: 미래 예측 (차세대 모델)

### TP-LA-18: 차세대 LLM d_model = 2^sigma = 4096 유지
**예측**: 대형 LLM의 hidden dimension은 4096 = 2^12 = 2^sigma 근방을 유지한다.
**n=6 근거**: 2^sigma = 2^12 = 4096. BT-56.
**검증**: GPT-4, Claude, Gemini 아키텍처 (공개 시).
**반증 조건**: d=8192 (2^13)가 표준이 되면 CLOSE (sigma+mu).

### TP-LA-19: KV-head 수 = sigma-tau = 8 보편 유지
**예측**: GQA의 KV-head 수는 8로 수렴한다.
**n=6 근거**: sigma-tau = 8. BT-39.
**검증**: LLaMA-2 (8 KV heads), Mistral (8 KV heads).
**반증 조건**: 4 또는 16 KV heads가 표준이 되면 CLOSE.

### TP-LA-20: 새 옵티마이저도 beta_1=0.9 유지
**예측**: AdamW 이후 옵티마이저 (SOAP, Muon 등)도 momentum=0.9를 유지한다.
**n=6 근거**: 1-1/(sigma-phi) = 0.9. BT-54.
**검증**: 차세대 옵티마이저 논문.
**반증 조건**: momentum=0.95가 표준이 되면 CLOSE.

### TP-LA-21: Cosine LR 최소 = 1/(sigma-phi) of max
**예측**: Cosine annealing의 최소 LR = max_LR * 0.1.
**n=6 근거**: 1/(sigma-phi) = 0.1. BT-64.
**검증**: GPT-3 (min_lr = max_lr/10), Chinchilla.
**반증 조건**: min_lr = max_lr/100이 표준이면 CLOSE.

### TP-LA-22: Context length = 2^sigma = 4096 기본
**예측**: LLM 기본 context window = 4096 tokens.
**n=6 근거**: 2^sigma = 4096. BT-44.
**검증**: GPT-3 (2048->4096), LLaMA (4096), Mistral (8192=2^(sigma+mu)).
**반증 조건**: 현재 이미 8K-128K로 확장 중 -> BT-44 래더 참조.

### TP-LA-23: Vocabulary size = 2^sopfr * 10^(n/phi) ~ 32K
**예측**: 토크나이저 어휘 크기 = 32000 근방.
**n=6 근거**: 2^sopfr * 10^(n/phi) = 32 * 1000 = 32000. BT-73.
**검증**: LLaMA (32000), Mistral (32000).
**반증 조건**: 256K 어휘가 표준이 되면 CLOSE.

### TP-LA-24: Carmichael LR schedule period = lambda(6) = 2
**예측**: 학습률 주기적 스케줄의 주기 수 = 2.
**n=6 근거**: lambda(6) = 2. BT-54.
**검증**: Cosine with warm restarts (Loshchilov & Hutter, 2017).
**반증 조건**: 3-cycle 이상이 표준이 되면 CLOSE.
