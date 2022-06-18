use std::sync::{Arc, RwLock};

use log::debug;
use serde_json::{json, Value};

use crate::behaviour::entity::comparison::string_comparison_properties::StringComparisonProperties;
use crate::behaviour::entity::comparison::StringComparisonFunction;
use crate::frp::Stream;
use crate::model::{PropertyInstanceGetter, PropertyInstanceSetter, ReactiveEntityInstance};
use crate::reactive::entity::expression::{Expression, ExpressionValue, OperatorPosition};
use crate::reactive::entity::gate::Gate;
use crate::reactive::entity::operation::Operation;
use crate::reactive::entity::Disconnectable;

pub type StringComparisonExpressionValue = ExpressionValue<String>;

/// Generic implementation of comparison_gates operations with two inputs (LHS,RHS) and one result.
///
/// The implementation is realized using reactive streams.
pub struct StringComparison<'a> {
    pub lhs: RwLock<Stream<'a, StringComparisonExpressionValue>>,

    pub rhs: RwLock<Stream<'a, StringComparisonExpressionValue>>,

    pub f: StringComparisonFunction,

    pub internal_result: RwLock<Stream<'a, bool>>,

    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl StringComparison<'_> {
    pub fn new(e: Arc<ReactiveEntityInstance>, f: StringComparisonFunction) -> StringComparison<'static> {
        let lhs = e
            .properties
            .get(StringComparisonProperties::LHS.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .map(|v| match v.as_str() {
                Some(lhs_str) => (OperatorPosition::LHS, String::from(lhs_str)),
                None => (OperatorPosition::LHS, StringComparisonProperties::LHS.default_value()),
            });
        let rhs = e
            .properties
            .get(StringComparisonProperties::RHS.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .map(|v| -> StringComparisonExpressionValue {
                match v.as_str() {
                    Some(rhs_str) => (OperatorPosition::RHS, String::from(rhs_str)),
                    None => (OperatorPosition::RHS, StringComparisonProperties::RHS.default_value()),
                }
            });

        let expression = lhs.merge(&rhs).fold(
            Expression::new(StringComparisonProperties::LHS.default_value(), StringComparisonProperties::RHS.default_value()),
            |old_state, (o, value)| match *o {
                OperatorPosition::LHS => old_state.lhs(String::from(value.clone())),
                OperatorPosition::RHS => old_state.rhs(String::from(value.clone())),
            },
        );

        // The internal result
        let internal_result = expression.map(move |e| f(e.lhs.clone(), e.rhs.clone()));

        let handle_id = e.properties.get(StringComparisonProperties::RESULT.as_ref()).unwrap().id.as_u128();

        let string_comparison = StringComparison {
            lhs: RwLock::new(lhs),
            rhs: RwLock::new(rhs),
            f,
            internal_result: RwLock::new(internal_result),
            entity: e.clone(),
            handle_id,
        };

        // Connect the internal result with the stream of the result property
        string_comparison.internal_result.read().unwrap().observe_with_handle(
            move |v| {
                debug!("Setting result of string comparison: {}", v);
                e.set(StringComparisonProperties::RESULT.to_string(), json!(*v));
            },
            handle_id,
        );

        string_comparison
    }

    /// TODO: extract to trait "Named"
    /// TODO: unit test
    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for StringComparison<'_> {
    /// TODO: Add guard: disconnect only if actually connected
    fn disconnect(&self) {
        debug!("Disconnect string comparison {} {}", self.type_name(), self.handle_id);
        self.internal_result.read().unwrap().remove(self.handle_id);
    }
}

impl Operation for StringComparison<'_> {
    fn lhs(&self, value: Value) {
        self.entity.set(StringComparisonProperties::LHS.as_ref(), value);
    }

    fn result(&self) -> Value {
        self.entity.get(StringComparisonProperties::RESULT.as_ref()).unwrap()
    }
}

impl Gate for StringComparison<'_> {
    fn rhs(&self, value: Value) {
        self.entity.set(StringComparisonProperties::RHS.as_ref(), value);
    }
}

/// Automatically disconnect streams on destruction
impl Drop for StringComparison<'_> {
    fn drop(&mut self) {
        debug!("Drop comparison gate");
        self.disconnect();
    }
}
