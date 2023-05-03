use rand::Rng;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde_json::json;
use serde_json::Value;

use crate::model::*;
use crate::model_random::PseudoNumberGeneratorProperties::SEED;
use crate::model_result::ResultNumberI64Properties::RESULT;
use crate::model_runtime::ActionProperties::TRIGGER;
use crate::reactive::*;

entity_behaviour!(
    RandomI64Pseudo,
    RandomI64PseudoFactory,
    RandomI64PseudoFsm,
    RandomI64PseudoBehaviourTransitions,
    RandomI64PseudoValidator
);

behaviour_validator!(RandomI64PseudoValidator, ReactiveEntityInstance, TRIGGER.as_ref(), SEED.as_ref(), RESULT.as_ref());

impl BehaviourInit<ReactiveEntityInstance> for RandomI64PseudoBehaviourTransitions {}

impl BehaviourConnect<ReactiveEntityInstance> for RandomI64PseudoBehaviourTransitions {
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

impl BehaviourShutdown<ReactiveEntityInstance> for RandomI64PseudoBehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for RandomI64PseudoBehaviourTransitions {}

fn random(rng: &mut ChaCha8Rng) -> Value {
    json!(rng.gen::<i64>())
}
