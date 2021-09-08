use std::sync::{Arc, RwLock};

use log::debug;
use serde_json::{json, Value};

use crate::behaviour::entity::gate::function::ArithmeticGateFunction;
use crate::behaviour::entity::gate::properties::ArithmeticGateProperties;
use crate::frp::Stream;
use crate::model::{PropertyInstanceGetter, PropertyInstanceSetter, ReactiveEntityInstance};
use crate::reactive::entity::expression::{Expression, ExpressionValue, OperatorPosition};
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
    pub fn new<'a>(
        e: Arc<ReactiveEntityInstance>,
        f: ArithmeticGateFunction<f64>,
    ) -> ArithmeticGate<'static> {
        let lhs = e
            .properties
            .get(ArithmeticGateProperties::LHS.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .map(|v| match v.as_f64() {
                Some(b) => (OperatorPosition::LHS, b),
                None => (
                    OperatorPosition::LHS,
                    ArithmeticGateProperties::LHS.default_value(),
                ),
            });
        let rhs = e
            .properties
            .get(ArithmeticGateProperties::RHS.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .map(|v| -> ArithmeticExpressionValue {
                match v.as_f64() {
                    Some(b) => (OperatorPosition::RHS, b),
                    None => (
                        OperatorPosition::RHS,
                        ArithmeticGateProperties::RHS.default_value(),
                    ),
                }
            });

        let expression = lhs.merge(&rhs).fold(
            Expression::new(
                ArithmeticGateProperties::LHS.default_value(),
                ArithmeticGateProperties::RHS.default_value(),
            ),
            |old_state, (o, value)| match *o {
                OperatorPosition::LHS => old_state.lhs(*value),
                OperatorPosition::RHS => old_state.rhs(*value),
            },
        );

        // The internal result
        let internal_result = expression.map(move |e| f(e.lhs, e.rhs));

        // TODO: handle result based on outbound property id and inbound property id
        let handle_id = e
            .properties
            .get(ArithmeticGateProperties::RESULT.as_ref())
            .unwrap()
            .id
            .as_u128();

        let arithmetic_gate = ArithmeticGate {
            lhs: RwLock::new(lhs),
            rhs: RwLock::new(rhs),
            f,
            internal_result: RwLock::new(internal_result),
            entity: e.clone(),
            handle_id,
        };

        // Connect the internal result with the stream of the result property
        arithmetic_gate
            .internal_result
            .read()
            .unwrap()
            .observe_with_handle(
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
    /// TODO: Add guard: disconnect only if actually connected
    fn disconnect(&self) {
        debug!(
            "Disconnect arithmetic gate {} {}",
            self.type_name(),
            self.handle_id
        );
        self.internal_result.read().unwrap().remove(self.handle_id);
    }
}

impl Operation for ArithmeticGate<'_> {
    fn lhs(&self, value: Value) {
        self.entity
            .set(ArithmeticGateProperties::LHS.as_ref(), value);
    }

    fn result(&self) -> Value {
        self.entity
            .get(ArithmeticGateProperties::RESULT.as_ref())
            .unwrap()
            .clone()
    }
}

impl Gate for ArithmeticGate<'_> {
    fn rhs(&self, value: Value) {
        self.entity
            .set(ArithmeticGateProperties::RHS.as_ref(), value);
    }
}

/// Automatically disconnect streams on destruction
impl Drop for ArithmeticGate<'_> {
    fn drop(&mut self) {
        debug!("Drop arithmetic gate");
        self.disconnect();
    }
}
