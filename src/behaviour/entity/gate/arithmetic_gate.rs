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
    pub fn new(e: Arc<ReactiveEntityInstance>, f: ArithmeticGateFunction<f64>) -> ArithmeticGate<'static> {
        let lhs_initial = e
            .as_f64(ArithmeticGateProperties::LHS.as_ref())
            .unwrap_or_else(|| ArithmeticGateProperties::LHS.default_value());
        let lhs = e
            .properties
            .get(ArithmeticGateProperties::LHS.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .map(move |v| match v.as_f64() {
                Some(b) => (OperatorPosition::LHS, b),
                None => (OperatorPosition::LHS, lhs_initial),
            });
        let rhs_initial = e
            .as_f64(ArithmeticGateProperties::RHS.as_ref())
            .unwrap_or_else(|| ArithmeticGateProperties::RHS.default_value());
        let rhs = e
            .properties
            .get(ArithmeticGateProperties::RHS.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .map(move |v| -> ArithmeticExpressionValue {
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

        // The internal result
        let internal_result = expression.map(move |e| f(e.lhs, e.rhs));

        let handle_id = Uuid::new_v4().as_u128();

        let arithmetic_gate = ArithmeticGate {
            lhs: RwLock::new(lhs),
            rhs: RwLock::new(rhs),
            f,
            internal_result: RwLock::new(internal_result),
            entity: e.clone(),
            handle_id,
        };

        // Initial calculation
        e.set(ArithmeticGateProperties::RESULT.as_ref(), json!(f(lhs_initial, rhs_initial)));

        // Connect the internal result with the stream of the result property
        arithmetic_gate.internal_result.read().unwrap().observe_with_handle(
            move |v| {
                debug!("Setting result of arithmetic gate: {}", v);
                e.set(ArithmeticGateProperties::RESULT.to_string(), json!(*v));
            },
            handle_id,
        );

        arithmetic_gate
    }

    /// TODO: extract to trait "Named"
    /// TODO: unit test
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
