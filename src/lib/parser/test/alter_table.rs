#![cfg(test)]
use crate::lib::ast::ddl::{AlterTableDropColumn, AlterTableRenameColumn};
use crate::lib::ast::predule::{
    AlterTableAddColumn, AlterTableQuery, AlterTableRenameTo, Column, DataType, TableName,
};
use crate::lib::parser::predule::{Parser, ParserContext};

#[test]
pub fn alter_table_rename_1() {
    let text = r#"
        ALTER TABLE foo RENAME TO bar;
    "#
    .to_owned();

    let mut parser = Parser::new(text).unwrap();

    let expected = AlterTableQuery::builder()
        .set_table(TableName {
            table_name: "foo".to_owned(),
            database_name: None,
        })
        .set_action(AlterTableRenameTo { name: "bar".into() }.into())
        .build();

    assert_eq!(
        parser.parse(ParserContext::default()).unwrap(),
        vec![expected],
    );
}

#[test]
pub fn alter_table_add_column_1() {
    let text = r#"
        ALTER TABLE foo ADD COLUMN name varchar(100);
    "#
    .to_owned();

    let mut parser = Parser::new(text).unwrap();

    let expected = AlterTableQuery::builder()
        .set_table(TableName {
            table_name: "foo".to_owned(),
            database_name: None,
        })
        .set_action(
            AlterTableAddColumn {
                column: Column::builder()
                    .set_name("name".to_owned())
                    .set_data_type(DataType::Varchar(100))
                    .build(),
            }
            .into(),
        )
        .build();

    assert_eq!(
        parser.parse(ParserContext::default()).unwrap(),
        vec![expected],
    );
}

#[test]
pub fn alter_table_add_column_2() {
    let text = r#"
        ALTER TABLE foo ADD name varchar(100);
    "#
    .to_owned();

    let mut parser = Parser::new(text).unwrap();

    let expected = AlterTableQuery::builder()
        .set_table(TableName {
            table_name: "foo".to_owned(),
            database_name: None,
        })
        .set_action(
            AlterTableAddColumn {
                column: Column::builder()
                    .set_name("name".to_owned())
                    .set_data_type(DataType::Varchar(100))
                    .build(),
            }
            .into(),
        )
        .build();

    assert_eq!(
        parser.parse(ParserContext::default()).unwrap(),
        vec![expected],
    );
}

#[test]
pub fn alter_table_rename_column_1() {
    let text = r#"
        ALTER TABLE foo RENAME COLUMN name TO name_1;
    "#
    .to_owned();

    let mut parser = Parser::new(text).unwrap();

    let expected = AlterTableQuery::builder()
        .set_table(TableName {
            table_name: "foo".to_owned(),
            database_name: None,
        })
        .set_action(
            AlterTableRenameColumn {
                from_name: "name".into(),
                to_name: "name_1".into(),
            }
            .into(),
        )
        .build();

    assert_eq!(
        parser.parse(ParserContext::default()).unwrap(),
        vec![expected],
    );
}

#[test]
pub fn alter_table_rename_column_2() {
    let text = r#"
        ALTER TABLE foo RENAME name TO name_1;
    "#
    .to_owned();

    let mut parser = Parser::new(text).unwrap();

    let expected = AlterTableQuery::builder()
        .set_table(TableName {
            table_name: "foo".to_owned(),
            database_name: None,
        })
        .set_action(
            AlterTableRenameColumn {
                from_name: "name".into(),
                to_name: "name_1".into(),
            }
            .into(),
        )
        .build();

    assert_eq!(
        parser.parse(ParserContext::default()).unwrap(),
        vec![expected],
    );
}

#[test]
pub fn alter_table_drop_column_1() {
    let text = r#"
        ALTER TABLE foo DROP COLUMN name;
    "#
    .to_owned();

    let mut parser = Parser::new(text).unwrap();

    let expected = AlterTableQuery::builder()
        .set_table(TableName {
            table_name: "foo".to_owned(),
            database_name: None,
        })
        .set_action(
            AlterTableDropColumn {
                column_name: "name".into(),
            }
            .into(),
        )
        .build();

    assert_eq!(
        parser.parse(ParserContext::default()).unwrap(),
        vec![expected],
    );
}
