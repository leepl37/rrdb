use crate::lib::ast::predule::SQLExpression;

#[derive(Clone, Debug, PartialEq)]
pub struct ParenthesesExpression {
    pub expression: SQLExpression,
}

impl Into<SQLExpression> for ParenthesesExpression {
    fn into(self) -> SQLExpression {
        SQLExpression::Parentheses(Box::new(self))
    }
}
