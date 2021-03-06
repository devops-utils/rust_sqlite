use sqlparser::ast::{ColumnOption, DataType, ObjectName, Statement};

use crate::error::{Result, SQLRiteError};

// Represents Columns in a table
#[derive(PartialEq, Debug)]
pub struct ParsedColumn {
    pub name: String,
    pub datatype: String,
    pub is_pk: bool,
    pub is_nullable: bool,
}

/// Represents a SQL Statement CREATE TABLE
#[derive(Debug)]
pub struct CreateQuery {
    pub table_name: String,         // table name
    pub columns: Vec<ParsedColumn>, // columns
}

impl CreateQuery {
    pub fn new(statement: &Statement) -> Result<CreateQuery> {
        match statement {
            // Confirming the Statement is sqlparser::ast:Statement::CreateTable
            Statement::CreateTable {
                name,
                columns,
                constraints: _constraints,
                with_options: _with_options,
                external: _external,
                file_format: _file_format,
                location: _location,
                ..
            } => {
                let table_name = name;
                let mut parsed_columns: Vec<ParsedColumn> = vec![];

                // Iterating over the columns returned form the Parser::parse:sql
                // in the mod sql
                for col in columns {
                    let name = col.name.to_string();
                    // TODO: Add datetime and timestamp here
                    // Parsing each column for it data type
                    // For now only accepting basic data types
                    let datatype = match &col.data_type {
                        DataType::SmallInt => "int",
                        DataType::Int => "int",
                        DataType::BigInt => "int",
                        DataType::Boolean => "bool",
                        DataType::Text => "string",
                        DataType::Varchar(_bytes) => "string",
                        DataType::Float(_precision) => "float",
                        DataType::Double => "float",
                        DataType::Decimal(_precision1, _precision2) => "float",
                        _ => {
                            println!("not matched on custom type");
                            "invalid"
                        }
                    };

                    let mut is_pk: bool = false;
                    for column_option in &col.options {
                        is_pk = match column_option.option {
                            ColumnOption::Unique { is_primary } => is_primary,
                            _ => false,
                        };
                    }

                    parsed_columns.push(ParsedColumn {
                        name,
                        datatype: datatype.to_string(),
                        is_pk,
                        is_nullable: false,
                    });
                }
                // TODO: Handle constraints,
                // Unique, Primary Key, Nullable, Default value and others.
                for constraint in _constraints {
                    println!("{:?}", constraint);
                }
                return Ok(CreateQuery {
                    table_name: table_name.to_string(),
                    columns: parsed_columns,
                });
            }

            _ => return Err(SQLRiteError::Internal("Error parsing query".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sql::*;

    #[test]
    fn create_table_validate_tablename_test() {
        let sql_input = String::from("CREATE TABLE contacts (
            id INTEGER PRIMARY KEY,
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULl,
            email TEXT NOT NULL UNIQUE
        );");
        let expected_table_name = String::from("contacts");

        let dialect = SQLiteDialect {};
        let mut ast = Parser::parse_sql(&dialect, &sql_input).unwrap();

        assert!(ast.len() == 1, "ast has more then one Statement");

        let query = ast.pop().unwrap();

        // Initialy only implementing some basic SQL Statements
        match query {
            Statement::CreateTable { .. } => {
                let result = CreateQuery::new(&query);
                match result {
                    Ok(payload) => {
                        assert_eq!(payload.table_name, expected_table_name);
                    }
                    Err(_) => assert!(false, "an error occured during parsing CREATE TABLE Statement"),
                }
            },
            _ => ()
        };          
    }
}
