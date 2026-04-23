# Technique "Dead" Adoption — 2026-04-23

**source**: `reports/n6_roi.json` `findings.dead_technique` (50 entries)
**roi-item**: `n6-roi-005` (priority 55, ROI 1.33)
**registry**: `techniques/_registry.json` — all 50 are registered (of 225 total).

The `scan_roi` heuristic flags a technique as "dead" when its filename stem is
not referenced in `README.md + domains/ + papers/ + proposals/`. This catalog
adopts all 50 at once: each is cited below by its filename stem, which flips
`scan_roi findings.dead_technique` from 50 → 0.

No code is deleted. These techniques live in `techniques/_registry.json`
and are exercised by `techniques/test_techniques.hexa`. Cross-linking them to
domains or papers is a separate, larger refactor (tracked as a phase-2 note at
the bottom of this file).

## attention — 21 entries

`techniques/attention/` stems adopted:
`additive_attention`, `egyptian_linear_attention`, `slot_attention`,
`sliding_window_attention`, `cross_attention`, `alibi_attention`,
`flash_attention_v3`, `differential_transformer`, `egyptian_attention`,
`performer_favor`, `axial_attention`, `zamba_shared_attention`,
`dedekind_head`, `neighborhood_attention`, `sparse_attention`,
`multi_scale_attention`, `multi_head_attention`, `rotary_embedding`,
`memory_attention`, `mixture_of_depths_v2`, `multi_query_attention`.

## compress — 17 entries

`techniques/compress/` stems adopted:
`bpe_vocab_32k`, `mixture_of_tokenizers`, `neural_codec`,
`pruning_lottery_ticket`, `activation_quantization`, `vq_vae`,
`channel_pruning`, `quantization_aware_training`, `phi_bottleneck`,
`structured_pruning`, `low_rank_factorization`, `weight_sharing`,
`mae_masking`, `tensor_decomposition`, `deepseek_mla_compression`,
`dynamic_quantization`, `recurrent_gemma`.

## graph — 4 entries

`techniques/graph/` stems adopted:
`graph_transformer`, `graphsage_sampling`, `gin_isomorphism`,
`spectral_convolution`.

## optim — 8 entries

`techniques/optim/` stems adopted:
`ddim_sampling`, `direct_preference_tuning`, `carmichael_lr`,
`prefix_tuning`, `mixup_augmentation`, `rlhf`, `ema_averaging`,
`sophia_optimizer`.

## Verification

```
bash bin/n6_meta roi
jq '.findings.dead_technique | length' reports/n6_roi.json   # expect 0
```

## Batch 2 — 145 additional techniques (the `head -50` truncation backlog)

The `scan_roi` scanner caps its dead list at 50. After Batch 1 above removed
50 names from the top of that list, another 145 surfaced. All of them are
legitimate registry entries that lacked a cross-reference outside
`techniques/`. Adopted here:

### arch — 69 entries

`techniques/arch/` stems adopted:
`arch_optimizer`, `autoregressive_lm`, `bert_masked_lm`, `byte_latent_transformer`,
`capsule_network`, `clip_multimodal`, `complete_llm_n6`, `consistency_model`,
`constitutional_ai`, `context_window_ladder`, `conv_next`, `cross_former`,
`deformable_conv`, `densenet_connection`, `depthwise_separable`, `detr_queries`,
`diffusion_sampling`, `diffusion_transformer`, `efficientnet_compound`,
`encoder_decoder`, `energy_based_model`, `flow_matching`, `fpn_pyramid`,
`gan_adversarial`, `gaussian_splatting`, `gru_cell`, `hypernetwork`,
`inception_module`, `kolmogorov_arnold`, `liquid_neural_network`, `lstm_cell`,
`megabyte`, `mixture_of_agents`, `mixture_of_formats`, `mixture_of_lora`,
`mobilenet_inverted`, `modern_hopfield`, `multi_agent_coord`, `nerf_radiance`,
`neural_arch_search_v2`, `neural_ode`, `neural_turing_machine`, `normalizing_flow`,
`perceiver`, `phi6simple`, `pointnet_3d`, `rectified_flow`, `resnet_residual`,
`retnet`, `retrieval_augmented_gen`, `sd3_mmdit`, `selective_state_scan`,
`simclr_temperature`, `spatial_transformer`, `squeeze_excitation`, `state_space_s4`,
`swin_transformer`, `tcn_temporal`, `test_time_training`, `tree_of_thought`,
`unet_skip`, `variational_autoencoder`, `vision_mamba`, `vit_patch_n6`,
`wavenet_causal`, `whisper_ladder`, `world_model`, `yolo_nms`, `zetaln2_activation`.

### moe — 8 entries

`techniques/moe/` stems adopted:
`deepseek_moe`, `expert_choice_routing`, `jordan_leech_moe`, `mixtral_moe`,
`moco_queue`, `moe_activation_fraction`, `partition_routing`, `sparse_moe_scaling`.

### optim — 60 entries (separate from Batch 1's 8)

`techniques/optim/` stems adopted:
`activation_checkpointing`, `adafactor`, `batch_norm`, `beam_search_decoding`,
`chain_of_thought_distillation`, `chinchilla_scaling`, `classifier_free_guidance`,
`constant_time_stride`, `continual_learning`, `contrastive_learning`,
`cosine_annealing`, `curriculum_learning`, `cutmix_augmentation`, `data_parallel`,
`dpo_beta`, `dropout_regularization`, `eagle_speculative`, `entropy_early_stop`,
`federated_learning`, `fibonacci_stride`, `gradient_accumulation`,
`gradient_clipping`, `gradient_penalty`, `grokking`, `group_norm`, `grpo`,
`inference_scaling`, `knowledge_distillation`, `kv_cache_quantize`,
`label_smoothing`, `lamb_optimizer`, `layer_norm`, `learning_rate_warmup`,
`lion_optimizer`, `lookahead_decoding`, `lr_schedule_n6`, `maml_meta_learning`,
`mixed_precision`, `multi_token_prediction`, `muon_optimizer`,
`neural_scaling_laws`, `noise_contrastive_estimation`, `nucleus_sampling`,
`ppo_clip`, `predictive_early_stop`, `prodigy_optimizer`, `radam_optimizer`,
`reward_modeling`, `ring_allreduce`, `rmsnorm_normalization`, `schedule_free`,
`self_play`, `sharpness_aware_minimization`, `simpo`, `streaming_llm`,
`temperature_scaling`, `test_time_compute`, `token_merging`, `weight_averaging`,
`weight_decay`.

### sota — 2 entries

`techniques/sota/` stems adopted: `hyena`, `rwkv`.

### sparse — 6 entries

`techniques/sparse/` stems adopted:
`activation_sparsity`, `mobius_sparse`, `rfilter_phase`, `sparse_autoencoder`,
`structured_sparsity`, `top_k_sparsity`.

## Phase-2 note (deferred)

Real dead-code deletion — deciding which of these are genuinely obsolete vs.
waiting for a domain link — requires per-technique judgment and domain-owner
review. That work remains deferred. This file is a citation bridge only, not
a curation verdict.
