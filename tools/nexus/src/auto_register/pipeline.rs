use serde::{Deserialize, Serialize};

use super::classifier::{self, ClassifiedDiscovery};
use super::notifier::{self, Notification};
use super::registrar::{self, RegistrationResult};
use super::sync_trigger::{self, SyncResult};

/// Full pipeline result containing all stages.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PipelineResult {
    pub classified: Vec<ClassifiedDiscovery>,
    pub registered: Vec<RegistrationResult>,
    pub sync_commands: Vec<String>,
    pub notifications: Vec<Notification>,
}

/// The auto-registration pipeline.
///
/// Given raw discovery strings + their n6 scores, runs:
/// 1. Classification (type inference)
/// 2. Registration (backend routing)
/// 3. Sync trigger determination
/// 4. Notification generation
pub struct AutoRegisterPipeline;

impl AutoRegisterPipeline {
    /// Process a batch of raw discoveries through the full pipeline.
    pub fn process(
        raw_discoveries: &[String],
        n6_scores: &[f64],
        source: &str,
    ) -> PipelineResult {
        assert_eq!(
            raw_discoveries.len(),
            n6_scores.len(),
            "discoveries and scores must have equal length"
        );

        // 1. Classify each discovery
        let classified: Vec<ClassifiedDiscovery> = raw_discoveries
            .iter()
            .zip(n6_scores.iter())
            .map(|(raw, &score)| classifier::classify(raw, score, source))
            .collect();

        // 2. Register each classified discovery
        let registered: Vec<RegistrationResult> = classified
            .iter()
            .map(|d| registrar::register(d))
            .collect();

        // 3. Determine sync commands
        let sync_result: SyncResult = sync_trigger::trigger_sync(&registered);
        let sync_commands: Vec<String> = sync_result
            .commands
            .iter()
            .map(|c| c.command.clone())
            .collect();

        // 4. Generate notifications
        let notifications = notifier::notify(&registered);

        PipelineResult {
            classified,
            registered,
            sync_commands,
            notifications,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_empty() {
        let result = AutoRegisterPipeline::process(&[], &[], "test");
        assert!(result.classified.is_empty());
        assert!(result.registered.is_empty());
        assert!(result.notifications.is_empty());
    }

    #[test]
    fn test_pipeline_single_constant() {
        let discoveries = vec!["Found sigma = 12.0 in physics".to_string()];
        let scores = vec![1.0];
        let result = AutoRegisterPipeline::process(&discoveries, &scores, "test");
        assert_eq!(result.classified.len(), 1);
        assert_eq!(result.registered.len(), 1);
        assert!(result.registered[0].success);
        assert!(result.registered[0]
            .registered_to
            .contains(&"math_atlas".to_string()));
    }

    #[test]
    fn test_pipeline_mixed_batch() {
        let discoveries = vec![
            "Value 12.0 found".to_string(),
            "BT-200 cross-domain physics energy resonance".to_string(),
            "Some random observation about cats".to_string(),
        ];
        let scores = vec![1.0, 0.9, 0.1];
        let result = AutoRegisterPipeline::process(&discoveries, &scores, "evolve");
        assert_eq!(result.classified.len(), 3);
        assert_eq!(result.registered.len(), 3);
        // All should succeed (registration is planning-only)
        assert!(result.registered.iter().all(|r| r.success));
        // Should have notifications for each
        assert_eq!(result.notifications.len(), 3);
    }

    #[test]
    fn test_pipeline_sync_commands_generated() {
        let discoveries = vec!["sigma = 12.0 constant".to_string()];
        let scores = vec![1.0];
        let result = AutoRegisterPipeline::process(&discoveries, &scores, "scan");
        // math_atlas registration should trigger atlas sync
        assert!(!result.sync_commands.is_empty());
    }
}
