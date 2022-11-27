use crate::model::properties;

properties!(
    IfThenElseProperties,
    (CONDITION, "condition", false),
    (THEN_PAYLOAD, "then_payload", 0),
    (ELSE_PAYLOAD, "else_payload", 0),
    (RESULT, "result", 0)
);
