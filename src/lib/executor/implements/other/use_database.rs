use std::error::Error;

use crate::lib::ast::other::UseDatabaseQuery;
use crate::lib::executor::predule::{
    ExecuteColumn, ExecuteColumnType, ExecuteField, ExecuteResult, ExecuteRow, Executor,
};

impl Executor {
    pub async fn use_databases(
        &self,
        query: UseDatabaseQuery,
    ) -> Result<ExecuteResult, Box<dyn Error>> {
        Ok(ExecuteResult {
            columns: (vec![ExecuteColumn {
                name: "desc".into(),
                data_type: ExecuteColumnType::String,
            }]),
            rows: (vec![ExecuteRow {
                fields: vec![ExecuteField::String(format!(
                    "database changed: {}",
                    query.database_name
                ))],
            }]),
        })
    }
}
