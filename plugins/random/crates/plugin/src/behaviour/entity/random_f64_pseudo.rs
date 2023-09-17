use inexor_rgf_behaviour::entity_behaviour;
use inexor_rgf_behaviour_api::behaviour_validator;
use inexor_rgf_behaviour_api::prelude::*;
use inexor_rgf_graph::prelude::*;
use inexor_rgf_model_runtime::ActionProperties::TRIGGER;
use inexor_rgf_reactive::ReactiveEntity;
use rand::Rng;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

use inexor_rgf_model_random::PseudoNumberGeneratorProperties::SEED;
use inexor_rgf_model_result::ResultNumberF64Properties::RESULT;

entity_behaviour!(
    RandomF64Pseudo,
    RandomF64PseudoFactory,
    RandomF64PseudoFsm,
    RandomF64PseudoBehaviourTransitions,
    RandomF64PseudoValidator
);

behaviour_validator!(RandomF64PseudoValidator, Uuid, ReactiveEntity, TRIGGER.as_ref(), SEED.as_ref(), RESULT.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for RandomF64PseudoBehaviourTransitions {}

impl BehaviourConnect<Uuid, ReactiveEntity> for RandomF64PseudoBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let Some(seed) = self.reactive_instance.as_u64(SEED) else {
            return Err(BehaviourConnectFailed {});
        };
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(TRIGGER.as_ref(), move |trigger: &Value| {
            if !trigger.as_bool().unwrap_or(false) {
                return;
            }
            reactive_instance.set(RESULT, random(&mut rng));
        });
        Ok(())
    }
}

impl BehaviourShutdown<Uuid, ReactiveEntity> for RandomF64PseudoBehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for RandomF64PseudoBehaviourTransitions {}

fn random(rng: &mut ChaCha8Rng) -> Value {
    json!(rng.gen::<f64>())
}
