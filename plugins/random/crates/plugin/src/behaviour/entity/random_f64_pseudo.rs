use rand::Rng;
use rand_chacha::ChaCha8Rng;
use rand_chacha::rand_core::SeedableRng;
use reactive_graph_behaviour_model_api::behaviour_validator;
use reactive_graph_behaviour_model_api::prelude::*;
use reactive_graph_behaviour_model_impl::entity_behaviour;
use reactive_graph_graph::prelude::*;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_runtime_model::ActionProperties::TRIGGER;
use serde_json::Value;
use serde_json::json;
use uuid::Uuid;

use reactive_graph_std_random_model::PseudoNumberGeneratorProperties::SEED;
use reactive_graph_std_result_model::ResultNumberF64Properties::RESULT;

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
    json!(rng.random::<f64>())
}
