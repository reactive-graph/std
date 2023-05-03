use rand::Rng;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde_json::json;
use serde_json::Value;

use crate::model::*;
use crate::model_random::PseudoNumberGeneratorProperties::SEED;
use crate::model_result::ResultNumberU64Properties::RESULT;
use crate::model_runtime::ActionProperties::TRIGGER;
use crate::reactive::*;

entity_behaviour!(
    RandomU64Pseudo,
    RandomU64PseudoFactory,
    RandomU64PseudoFsm,
    RandomU64PseudoBehaviourTransitions,
    RandomU64PseudoValidator
);

behaviour_validator!(RandomU64PseudoValidator, ReactiveEntityInstance, TRIGGER.as_ref(), SEED.as_ref(), RESULT.as_ref());

impl BehaviourInit<ReactiveEntityInstance> for RandomU64PseudoBehaviourTransitions {}

impl BehaviourConnect<ReactiveEntityInstance> for RandomU64PseudoBehaviourTransitions {
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

impl BehaviourShutdown<ReactiveEntityInstance> for RandomU64PseudoBehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for RandomU64PseudoBehaviourTransitions {}

fn random(rng: &mut ChaCha8Rng) -> Value {
    json!(rng.gen::<u64>())
}
