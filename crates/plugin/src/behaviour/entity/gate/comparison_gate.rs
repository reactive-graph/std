use std::sync::Arc;
use std::sync::RwLock;

use log::debug;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

use crate::behaviour::entity::gate::properties::ComparisonGateProperties;
use crate::behaviour::entity::gate::ComparisonGateFunction;
use crate::frp::Stream;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::expression::Expression;
use crate::reactive::entity::expression::ExpressionValue;
use crate::reactive::entity::expression::OperatorPosition;
use crate::reactive::entity::gate::Gate;
use crate::reactive::entity::operation::Operation;
use crate::reactive::entity::Disconnectable;
use crate::reactive::BehaviourCreationError;

pub type ComparisonExpressionValue = ExpressionValue<Value>;

/// Generic implementation of comparison_gates operations with two inputs (LHS,RHS) and one result.
///
/// The implementation is realized using reactive streams.
pub struct ComparisonGate<'a> {
    pub lhs: RwLock<Stream<'a, ComparisonExpressionValue>>,

    pub rhs: RwLock<Stream<'a, ComparisonExpressionValue>>,

    pub f: ComparisonGateFunction,

    pub internal_result: RwLock<Stream<'a, bool>>,

    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl ComparisonGate<'_> {
    pub fn new(e: Arc<ReactiveEntityInstance>, f: ComparisonGateFunction) -> Result<ComparisonGate<'static>, BehaviourCreationError> {
        let lhs = e.properties.get(ComparisonGateProperties::LHS.as_ref()).ok_or(BehaviourCreationError)?;
        let rhs = e.properties.get(ComparisonGateProperties::RHS.as_ref()).ok_or(BehaviourCreationError)?;
        let result = e.properties.get(ComparisonGateProperties::RESULT.as_ref()).ok_or(BehaviourCreationError)?;

        let lhs_stream = lhs.stream.read().unwrap().map(|v| (OperatorPosition::LHS, v.clone()));
        let rhs_stream = rhs
            .stream
            .read()
            .unwrap()
            .map(|v| -> ComparisonExpressionValue { (OperatorPosition::RHS, v.clone()) });

        let expression = lhs_stream.merge(&rhs_stream).fold(
            Expression::new(ComparisonGateProperties::LHS.default_value(), ComparisonGateProperties::RHS.default_value()),
            |old_state, (o, value)| match *o {
                OperatorPosition::LHS => old_state.lhs(value.clone()),
                OperatorPosition::RHS => old_state.rhs(value.clone()),
            },
        );

        // The internal result
        let internal_result = expression.map(move |e| f(e.lhs.clone(), e.rhs.clone()));

        let handle_id = Uuid::new_v4().as_u128();

        let comparison_gate = ComparisonGate {
            lhs: RwLock::new(lhs_stream),
            rhs: RwLock::new(rhs_stream),
            f,
            internal_result: RwLock::new(internal_result),
            entity: e.clone(),
            handle_id,
        };

        // Initial calculation
        result.set(json!(f(lhs.get(), rhs.get())));

        // Connect the internal result with the stream of the result property
        let entity = e.clone();
        comparison_gate.internal_result.read().unwrap().observe_with_handle(
            move |v| {
                debug!("Setting result of comparison gate: {}", v);
                entity.set(ComparisonGateProperties::RESULT.to_string(), json!(*v));
            },
            handle_id,
        );

        Ok(comparison_gate)
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for ComparisonGate<'_> {
    fn disconnect(&self) {
        self.internal_result.read().unwrap().remove(self.handle_id);
    }
}

impl Operation for ComparisonGate<'_> {
    fn lhs(&self, value: Value) {
        self.entity.set(ComparisonGateProperties::LHS.as_ref(), value);
    }

    fn result(&self) -> Value {
        self.entity.get(ComparisonGateProperties::RESULT.as_ref()).unwrap()
    }
}

impl Gate for ComparisonGate<'_> {
    fn rhs(&self, value: Value) {
        self.entity.set(ComparisonGateProperties::RHS.as_ref(), value);
    }
}

impl Drop for ComparisonGate<'_> {
    fn drop(&mut self) {
        self.disconnect();
    }
}
