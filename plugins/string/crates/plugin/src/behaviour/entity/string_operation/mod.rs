use serde_json::json;
use serde_json::Value;

pub use function::StringOperationFunction;
pub use function::STRING_OPERATIONS;

use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::model_result::ResultStringProperties::RESULT;
use crate::model_string::StringOperationProperties::LHS;
use crate::reactive::behaviour_validator;
use crate::reactive::entity_behaviour;
use crate::reactive::BehaviourConnect;
use crate::reactive::BehaviourConnectFailed;
use crate::reactive::BehaviourDisconnect;
use crate::reactive::BehaviourFsm;
use crate::reactive::BehaviourInit;
use crate::reactive::BehaviourInitializationFailed;
use crate::reactive::BehaviourShutdown;
use crate::reactive::BehaviourTransitions;
use crate::reactive::PropertyObserverContainer;

pub mod function;

entity_behaviour!(
    StringOperation,
    StringOperationFactory,
    StringOperationFsm,
    StringOperationBehaviourTransitions,
    StringOperationValidator,
    f,
    StringOperationFunction
);

behaviour_validator!(StringOperationValidator, ReactiveEntityInstance, LHS.as_ref(), RESULT.as_ref());

impl BehaviourInit<ReactiveEntityInstance> for StringOperationBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        let lhs = self
            .reactive_instance
            .get(LHS)
            .and_then(|lhs| lhs.as_str().map(|lhs| lhs.to_string()))
            .ok_or(BehaviourInitializationFailed {})?;
        let f = self.f;
        self.reactive_instance.set(RESULT, json!(f(lhs)));
        Ok(())
    }
}

impl BehaviourConnect<ReactiveEntityInstance> for StringOperationBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.property_observers.reactive_instance.clone();
        let f = self.f;
        self.property_observers.observe_with_handle(LHS.as_ref(), move |lhs: &Value| {
            if let Some(lhs) = lhs.as_str().map(|lhs| lhs.to_string()) {
                reactive_instance.set(RESULT, json!(f(lhs)));
            }
        });
        Ok(())
    }
}

impl BehaviourShutdown<ReactiveEntityInstance> for StringOperationBehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for StringOperationBehaviourTransitions {}
