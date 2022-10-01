use std::sync::Arc;
use std::sync::RwLock;

use log::debug;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

use crate::behaviour::entity::gate::function::NumericGateFunction;
use crate::behaviour::entity::gate::properties::NumericGateProperties;
use crate::frp::Stream;
use crate::model::{PropertyInstanceGetter, PropertyInstanceSetter, ReactiveEntityInstance};
use crate::reactive::entity::expression::{Expression, ExpressionValue, OperatorPosition};
use crate::reactive::entity::gate::Gate;
use crate::reactive::entity::operation::Operation;
use crate::reactive::entity::Disconnectable;

pub const NUMERIC_GATE: &str = "numeric_gate";

pub type NumericExpressionValue = ExpressionValue<f64>;

/// Generic implementation of numeric_gates operations with two inputs (LHS,RHS) and one result.
///
/// The implementation is realized using reactive streams.
pub struct NumericGate<'a> {
    pub lhs: RwLock<Stream<'a, NumericExpressionValue>>,

    pub rhs: RwLock<Stream<'a, NumericExpressionValue>>,

    pub f: NumericGateFunction<f64>,

    pub internal_result: RwLock<Stream<'a, f64>>,

    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl NumericGate<'_> {
    pub fn new(e: Arc<ReactiveEntityInstance>, f: NumericGateFunction<f64>) -> NumericGate<'static> {
        let lhs_initial = e
            .as_f64(NumericGateProperties::LHS.as_ref())
            .unwrap_or_else(|| NumericGateProperties::LHS.default_value());
        let lhs = e
            .properties
            .get(NumericGateProperties::LHS.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .map(move |v| match v.as_f64() {
                Some(b) => (OperatorPosition::LHS, b),
                None => (OperatorPosition::LHS, lhs_initial),
            });
        let rhs_initial = e
            .as_f64(NumericGateProperties::RHS.as_ref())
            .unwrap_or_else(|| NumericGateProperties::RHS.default_value());
        let rhs = e
            .properties
            .get(NumericGateProperties::RHS.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .map(move |v| -> NumericExpressionValue {
                match v.as_f64() {
                    Some(b) => (OperatorPosition::RHS, b),
                    None => (OperatorPosition::RHS, rhs_initial),
                }
            });

        let expression = lhs
            .merge(&rhs)
            .fold(Expression::new(lhs_initial, rhs_initial), |old_state, (o, value)| match *o {
                OperatorPosition::LHS => old_state.lhs(*value),
                OperatorPosition::RHS => old_state.rhs(*value),
            });

        let internal_result = expression.map(move |e| f(e.lhs, e.rhs));

        let handle_id = Uuid::new_v4().as_u128();

        let numeric_gate = NumericGate {
            lhs: RwLock::new(lhs),
            rhs: RwLock::new(rhs),
            f,
            internal_result: RwLock::new(internal_result),
            entity: e.clone(),
            handle_id,
        };

        // Initial calculation
        e.set(NumericGateProperties::RESULT.as_ref(), json!(f(lhs_initial, rhs_initial)));

        // Connect the internal result with the stream of the result property
        numeric_gate.internal_result.read().unwrap().observe_with_handle(
            move |v| {
                debug!("Setting result of {}: {}", NUMERIC_GATE, v);
                e.set(NumericGateProperties::RESULT.to_string(), json!(*v));
            },
            handle_id,
        );

        numeric_gate
    }

    /// TODO: extract to trait "Named"
    /// TODO: unit test
    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for NumericGate<'_> {
    fn disconnect(&self) {
        self.internal_result.read().unwrap().remove(self.handle_id);
    }
}

impl Operation for NumericGate<'_> {
    fn lhs(&self, value: Value) {
        self.entity.set(NumericGateProperties::LHS.as_ref(), value);
    }

    fn result(&self) -> Value {
        self.entity.get(NumericGateProperties::RESULT.as_ref()).unwrap()
    }
}

impl Gate for NumericGate<'_> {
    fn rhs(&self, value: Value) {
        self.entity.set(NumericGateProperties::RHS.as_ref(), value);
    }
}

impl Drop for NumericGate<'_> {
    fn drop(&mut self) {
        self.disconnect();
    }
}
