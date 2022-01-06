use std::convert::AsRef;
use std::sync::{Arc, RwLock};

use crate::model::PropertyInstanceSetter;
use crate::reactive::BehaviourCreationError;
use crate::reactive::{Expression, ExpressionValue, OperatorPosition};
use log::trace;
use serde_json::{json, Value};

use crate::behaviour::entity::ArrayPushProperties;
use crate::frp::Stream;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;

pub const ARRAY_PUSH: &'static str = "array_push";

pub type ArrayPushExpressionValue = ExpressionValue<Value>;

pub struct ArrayPush<'a> {
    pub lhs: RwLock<Stream<'a, ArrayPushExpressionValue>>,

    pub rhs: RwLock<Stream<'a, ArrayPushExpressionValue>>,

    pub internal_result: RwLock<Stream<'a, Value>>,

    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl ArrayPush<'_> {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>) -> Result<ArrayPush<'static>, BehaviourCreationError> {
        let lhs = e
            .properties
            .get(ArrayPushProperties::ARRAY.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .map(|v| (OperatorPosition::LHS, v.clone()));
        let rhs = e
            .properties
            .get(ArrayPushProperties::VALUE.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .map(|v| (OperatorPosition::RHS, v.clone()));

        let expression = lhs.merge(&rhs).fold(
            Expression::new(ArrayPushProperties::ARRAY.default_value(), ArrayPushProperties::VALUE.default_value()),
            |old_state, (o, value)| match *o {
                OperatorPosition::LHS => old_state.lhs(value.clone()),
                OperatorPosition::RHS => old_state.rhs(value.clone()),
            },
        );

        // The internal result
        let internal_result = expression.map(move |e| push_array(e.lhs.clone(), e.rhs.clone()));

        // TODO: handle result based on outbound property id and inbound property id
        let handle_id = e.properties.get(ArrayPushProperties::RESULT.as_ref()).unwrap().id.as_u128();

        let array_push = ArrayPush {
            lhs: RwLock::new(lhs),
            rhs: RwLock::new(rhs),
            internal_result: RwLock::new(internal_result),
            entity: e.clone(),
            handle_id,
        };

        // Connect the internal result with the stream of the result property
        array_push.internal_result.read().unwrap().observe_with_handle(
            move |v| {
                e.set(ArrayPushProperties::RESULT.to_string(), json!(*v));
            },
            handle_id,
        );

        Ok(array_push)
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for ArrayPush<'_> {
    fn disconnect(&self) {
        trace!("Disconnecting {} with id {}", ARRAY_PUSH, self.entity.id);
        self.internal_result.read().unwrap().remove(self.handle_id);
    }
}

/// Automatically disconnect streams on destruction
impl Drop for ArrayPush<'_> {
    fn drop(&mut self) {
        self.disconnect();
    }
}

fn push_array(array: Value, value: Value) -> Value {
    match array.as_array() {
        Some(array) => {
            let mut array = array.clone();
            array.push(value);
            Value::Array(array)
        }
        None => ArrayPushProperties::RESULT.default_value(),
    }
}
