use crate::lib::ast::predule::SQLExpression;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct WhereClause {
    pub expression: Option<Box<SQLExpression>>,
}
