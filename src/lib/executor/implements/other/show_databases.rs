use std::error::Error;
use std::io::ErrorKind;

use futures::future::join_all;

use crate::lib::ast::predule::ShowDatabasesQuery;
use crate::lib::errors::predule::ExecuteError;
use crate::lib::executor::predule::{
    DatabaseConfig, ExecuteColumn, ExecuteColumnType, ExecuteField, ExecuteResult, ExecuteRow,
    Executor, StorageEncoder,
};

impl Executor {
    pub async fn show_databases(
        &self,
        _query: ShowDatabasesQuery,
    ) -> Result<ExecuteResult, Box<dyn Error>> {
        let encoder = StorageEncoder::new();

        let base_path = self.get_base_path();

        match std::fs::read_dir(&base_path) {
            Ok(read_dir_result) => {
                let futures = read_dir_result.map(|e| async {
                    match e {
                        Ok(entry) => match entry.file_type() {
                            Ok(file_type) => {
                                if file_type.is_dir() {
                                    let mut path = entry.path();
                                    path.push("database.config");

                                    match tokio::fs::read(path).await {
                                        Ok(result) => {
                                            let database_config: DatabaseConfig =
                                                encoder.decode(result.as_slice()).unwrap();

                                            Some(database_config.database_name)
                                        }
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                }
                            }
                            Err(_) => None,
                        },
                        Err(_) => None,
                    }
                });

                let database_list = join_all(futures).await.into_iter().flatten();

                Ok(ExecuteResult {
                    columns: (vec![ExecuteColumn {
                        name: "database name".into(),
                        data_type: ExecuteColumnType::String,
                    }]),
                    rows: database_list
                        .map(|e| ExecuteRow {
                            fields: vec![ExecuteField::String(e)],
                        })
                        .collect(),
                })
            }
            Err(error) => match error.kind() {
                ErrorKind::NotFound => Err(ExecuteError::boxed("base path not exists")),
                _ => Err(ExecuteError::boxed("database listup failed")),
            },
        }
    }

    pub async fn find_database(&self, database_name: String) -> Result<bool, Box<dyn Error>> {
        let result = self.show_databases(ShowDatabasesQuery {}).await?;

        Ok(result.rows.iter().any(|e| {
            if let ExecuteField::String(name) = &e.fields[0] {
                name == &database_name
            } else {
                false
            }
        }))
    }
}
