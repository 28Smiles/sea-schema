use std::rc::Rc;
#[cfg(feature="sqlx-mysql")]
use sqlx::{Row, mysql::MySqlRow};
use sea_query::{Expr, Iden, Order, Query, SelectStatement};
use crate::mysql::def::*;
use super::SchemaQuery;

#[derive(Debug)]
pub struct ColumnQueryResult {
    pub column_name: String,
    pub column_type: String,
    pub is_nullable: String,
    pub column_key: String,
    pub column_default: Option<String>,
    pub extra: String,
    pub generation_expression: Option<String>,
    pub column_comment: String,
}

impl SchemaQuery {
    pub fn query_columns(&self, schema: Rc<dyn Iden>, table: Rc<dyn Iden>) -> SelectStatement {
        Query::select()
            .columns(vec![
                ColumnFields::ColumnName,
                ColumnFields::ColumnType,
                ColumnFields::IsNullable,
                ColumnFields::ColumnKey,
                ColumnFields::ColumnDefault,
                ColumnFields::Extra,
                ColumnFields::GenerationExpression,
                ColumnFields::ColumnComment,
            ])
            .from_schema(InformationSchema::Schema, InformationSchema::Columns)
            .and_where(Expr::col(ColumnFields::TableSchema).eq(schema.to_string()))
            .and_where(Expr::col(ColumnFields::TableName).eq(table.to_string()))
            .order_by(ColumnFields::OrdinalPosition, Order::Asc)
            .take()
    }
}

#[cfg(feature="sqlx-mysql")]
impl From<&MySqlRow> for ColumnQueryResult {
    fn from(row: &MySqlRow) -> Self {
        Self {
            column_name: row.get(0),
            column_type: row.get(1),
            is_nullable: row.get(2),
            column_key: row.get(3),
            column_default: row.get(4),
            extra: row.get(5),
            generation_expression: row.get(6),
            column_comment: row.get(7),
        }
    }
}
