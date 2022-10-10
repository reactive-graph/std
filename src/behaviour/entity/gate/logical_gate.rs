use std::sync::Arc;
use std::sync::RwLock;

use log::debug;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

use crate::behaviour::entity::gate::properties::LogicalGateProperties;
use crate::behaviour::entity::gate::LogicalGateFunction;
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

pub type LogicalExpressionValue = ExpressionValue<bool>;

/// Generic implementation of logical_gates operations with two inputs (LHS,RHS) and one result.
///
/// The implementation is realized using reactive streams.
pub struct LogicalGate<'a> {
    pub lhs: RwLock<Stream<'a, LogicalExpressionValue>>,

    pub rhs: RwLock<Stream<'a, LogicalExpressionValue>>,

    pub f: LogicalGateFunction,

    pub internal_result: RwLock<Stream<'a, bool>>,

    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl LogicalGate<'_> {
    pub fn new(e: Arc<ReactiveEntityInstance>, f: LogicalGateFunction) -> Result<LogicalGate<'static>, BehaviourCreationError> {
        let lhs = e.properties.get(LogicalGateProperties::LHS.as_ref()).ok_or(BehaviourCreationError)?;
        let rhs = e.properties.get(LogicalGateProperties::RHS.as_ref()).ok_or(BehaviourCreationError)?;

        let lhs_stream = lhs.stream.read().unwrap().map(|v| match v.as_bool() {
            Some(b) => (OperatorPosition::LHS, b),
            None => (OperatorPosition::LHS, LogicalGateProperties::LHS.default_value()),
        });
        let rhs_stream = rhs.stream.read().unwrap().map(|v| -> LogicalExpressionValue {
            match v.as_bool() {
                Some(b) => (OperatorPosition::RHS, b),
                None => (OperatorPosition::RHS, LogicalGateProperties::RHS.default_value()),
            }
        });

        let expression = lhs_stream.merge(&rhs_stream).fold(
            Expression::new(LogicalGateProperties::LHS.default_value(), LogicalGateProperties::RHS.default_value()),
            |old_state, (o, value)| match *o {
                OperatorPosition::LHS => old_state.lhs(*value),
                OperatorPosition::RHS => old_state.rhs(*value),
            },
        );

        let internal_result = expression.map(move |e| f(e.lhs, e.rhs));

        let handle_id = Uuid::new_v4().as_u128();

        let logical_gate = LogicalGate {
            lhs: RwLock::new(lhs_stream),
            rhs: RwLock::new(rhs_stream),
            f,
            internal_result: RwLock::new(internal_result),
            entity: e.clone(),
            handle_id,
        };

        if let Some(lhs) = lhs.as_bool() {
            if let Some(rhs) = rhs.as_bool() {
                e.set(LogicalGateProperties::RESULT, Value::Bool(f(lhs, rhs)));
            }
        }

        // Connect the internal result with the stream of the result property
        let entity = e.clone();
        logical_gate.internal_result.read().unwrap().observe_with_handle(
            move |v| {
                debug!("Setting result of logical gate: {}", v);
                entity.set(LogicalGateProperties::RESULT.as_ref(), json!(*v));
            },
            handle_id,
        );

        Ok(logical_gate)
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for LogicalGate<'_> {
    fn disconnect(&self) {
        self.internal_result.read().unwrap().remove(self.handle_id);
    }
}

impl Operation for LogicalGate<'_> {
    fn lhs(&self, value: Value) {
        self.entity.set(LogicalGateProperties::LHS.as_ref(), value);
    }

    fn result(&self) -> Value {
        self.entity.get(LogicalGateProperties::RESULT.as_ref()).unwrap()
    }
}

impl Gate for LogicalGate<'_> {
    fn rhs(&self, value: Value) {
        self.entity.set(LogicalGateProperties::RHS.as_ref(), value);
    }
}

impl Drop for LogicalGate<'_> {
    fn drop(&mut self) {
        self.disconnect();
    }
}
