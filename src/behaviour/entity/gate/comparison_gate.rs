use std::sync::{Arc, RwLock};

use log::debug;
use serde_json::{json, Value};

use crate::behaviour::entity::gate::properties::ComparisonGateProperties;
use crate::behaviour::entity::gate::ComparisonGateFunction;
use crate::frp::Stream;
use crate::model::{PropertyInstanceGetter, PropertyInstanceSetter, ReactiveEntityInstance};
use crate::reactive::entity::expression::{Expression, ExpressionValue, OperatorPosition};
use crate::reactive::entity::gate::Gate;
use crate::reactive::entity::operation::Operation;
use crate::reactive::entity::Disconnectable;

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
    pub fn new(e: Arc<ReactiveEntityInstance>, f: ComparisonGateFunction) -> ComparisonGate<'static> {
        let lhs = e
            .properties
            .get(ComparisonGateProperties::LHS.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .map(|v| (OperatorPosition::LHS, v.clone()));
        let rhs = e
            .properties
            .get(ComparisonGateProperties::RHS.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .map(|v| -> ComparisonExpressionValue { (OperatorPosition::RHS, v.clone()) });

        let expression = lhs.merge(&rhs).fold(
            Expression::new(ComparisonGateProperties::LHS.default_value(), ComparisonGateProperties::RHS.default_value()),
            |old_state, (o, value)| match *o {
                OperatorPosition::LHS => old_state.lhs(value.clone()),
                OperatorPosition::RHS => old_state.rhs(value.clone()),
            },
        );

        // The internal result
        let internal_result = expression.map(move |e| f(e.lhs.clone(), e.rhs.clone()));

        // TODO: handle result based on outbound property id and inbound property id
        let handle_id = e.properties.get(ComparisonGateProperties::RESULT.as_ref()).unwrap().id.as_u128();

        let comparison_gate = ComparisonGate {
            lhs: RwLock::new(lhs),
            rhs: RwLock::new(rhs),
            f,
            internal_result: RwLock::new(internal_result),
            entity: e.clone(),
            handle_id,
        };

        // Connect the internal result with the stream of the result property
        comparison_gate.internal_result.read().unwrap().observe_with_handle(
            move |v| {
                debug!("Setting result of comparison gate: {}", v);
                e.set(ComparisonGateProperties::RESULT.to_string(), json!(*v));
            },
            handle_id,
        );

        comparison_gate
    }

    /// TODO: extract to trait "Named"
    /// TODO: unit test
    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for ComparisonGate<'_> {
    /// TODO: Add guard: disconnect only if actually connected
    fn disconnect(&self) {
        debug!("Disconnect comparison gate {} {}", self.type_name(), self.handle_id);
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

/// Automatically disconnect streams on destruction
impl Drop for ComparisonGate<'_> {
    fn drop(&mut self) {
        debug!("Drop comparison gate");
        self.disconnect();
    }
}
