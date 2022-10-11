use std::sync::Arc;
use std::sync::RwLock;

use log::debug;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

use crate::behaviour::entity::gate::function::ArithmeticGateFunction;
use crate::behaviour::entity::gate::properties::ArithmeticGateProperties;
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

pub type ArithmeticExpressionValue = ExpressionValue<f64>;

/// Generic implementation of arithmetic_gates operations with two inputs (LHS,RHS) and one result.
///
/// The implementation is realized using reactive streams.
pub struct ArithmeticGate<'a> {
    pub lhs: RwLock<Stream<'a, ArithmeticExpressionValue>>,

    pub rhs: RwLock<Stream<'a, ArithmeticExpressionValue>>,

    pub f: ArithmeticGateFunction<f64>,

    pub internal_result: RwLock<Stream<'a, f64>>,

    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl ArithmeticGate<'_> {
    pub fn new(e: Arc<ReactiveEntityInstance>, f: ArithmeticGateFunction<f64>) -> Result<ArithmeticGate<'static>, BehaviourCreationError> {
        let lhs = e.properties.get(ArithmeticGateProperties::LHS.as_ref()).ok_or(BehaviourCreationError)?;
        let rhs = e.properties.get(ArithmeticGateProperties::RHS.as_ref()).ok_or(BehaviourCreationError)?;
        let result = e.properties.get(ArithmeticGateProperties::RESULT.as_ref()).ok_or(BehaviourCreationError)?;

        let lhs_stream = lhs.stream.read().unwrap().map(|v| match v.as_f64() {
            Some(f) => (OperatorPosition::LHS, f),
            None => (OperatorPosition::LHS, ArithmeticGateProperties::LHS.default_value()),
        });
        let rhs_stream = rhs.stream.read().unwrap().map(|v| -> ArithmeticExpressionValue {
            match v.as_f64() {
                Some(f) => (OperatorPosition::RHS, f),
                None => (OperatorPosition::RHS, ArithmeticGateProperties::RHS.default_value()),
            }
        });

        let expression = lhs_stream.merge(&rhs_stream).fold(
            Expression::new(ArithmeticGateProperties::LHS.default_value(), ArithmeticGateProperties::RHS.default_value()),
            |old_state, (o, value)| match *o {
                OperatorPosition::LHS => old_state.lhs(*value),
                OperatorPosition::RHS => old_state.rhs(*value),
            },
        );

        // The internal result
        let internal_result = expression.map(move |e| f(e.lhs, e.rhs));

        let handle_id = Uuid::new_v4().as_u128();

        let arithmetic_gate = ArithmeticGate {
            lhs: RwLock::new(lhs_stream),
            rhs: RwLock::new(rhs_stream),
            f,
            internal_result: RwLock::new(internal_result),
            entity: e.clone(),
            handle_id,
        };

        // Initial calculation
        if let Some(lhs) = lhs.as_f64() {
            if let Some(rhs) = rhs.as_f64() {
                result.set(json!(f(lhs, rhs)));
            }
        }

        // Connect the internal result with the stream of the result property
        let entity = e.clone();
        arithmetic_gate.internal_result.read().unwrap().observe_with_handle(
            move |v| {
                debug!("Setting result of arithmetic gate: {}", v);
                entity.set(ArithmeticGateProperties::RESULT.to_string(), json!(*v));
            },
            handle_id,
        );

        Ok(arithmetic_gate)
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for ArithmeticGate<'_> {
    fn disconnect(&self) {
        self.internal_result.read().unwrap().remove(self.handle_id);
    }
}

impl Operation for ArithmeticGate<'_> {
    fn lhs(&self, value: Value) {
        self.entity.set(ArithmeticGateProperties::LHS.as_ref(), value);
    }

    fn result(&self) -> Value {
        self.entity.get(ArithmeticGateProperties::RESULT.as_ref()).unwrap()
    }
}

impl Gate for ArithmeticGate<'_> {
    fn rhs(&self, value: Value) {
        self.entity.set(ArithmeticGateProperties::RHS.as_ref(), value);
    }
}

impl Drop for ArithmeticGate<'_> {
    fn drop(&mut self) {
        self.disconnect();
    }
}
