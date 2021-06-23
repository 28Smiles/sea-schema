use super::{InformationSchema, SchemaQueryBuilder};
use crate::sqlx_types::{postgres::PgRow, Row};
use sea_query::{Expr, Iden, Query, SelectStatement};
use std::rc::Rc;

#[derive(Debug, sea_query::Iden)]
/// Ref: https://www.postgresql.org/docs/13/infoschema-table-constraints.html
pub enum TableConstraintsField {
    ConstriantSchema,
    ConstraintName,
    TableSchema,
    TableName,
    ConstraintType,
    IsDeferrable,
    InitiallyDeferred,
}

#[derive(Debug, Default)]
pub struct TableConstraintQueryResult {
    constraint_schema: String,
    constraint_name: Option<String>,
    constraint_type: String,
    is_deferrable: String,
    initially_deferred: String,
}

impl SchemaQueryBuilder {
    pub fn query_constriants(schema: Rc<dyn Iden>, table: Rc<dyn Iden>) -> SelectStatement {
        Query::select()
            .columns(vec![
                TableConstraintsField::ConstriantSchema,
                TableConstraintsField::ConstraintName,
                TableConstraintsField::ConstraintType,
                TableConstraintsField::IsDeferrable,
                TableConstraintsField::InitiallyDeferred,
            ])
            .from((
                InformationSchema::Schema,
                InformationSchema::TableConstraints,
            ))
            .and_where(Expr::col(TableConstraintsField::TableSchema).eq(schema.to_string()))
            .and_where(Expr::col(TableConstraintsField::TableName).eq(table.to_string()))
            .take()
    }
}

#[cfg(feature = "sqlx-postres")]
impl From<&PgRow> for TableConstraintQueryResult {
    fn from(row: &PgRow) -> Self {
        Self {
            constraint_schema: row.get(0),
            constraint_name: row.get(1),
            constraint_type: row.get(2),
            is_deferrable: row.get(3),
            initially_deferred: row.get(4),
        }
    }
}

#[cfg(not(feature = "sqlx-postres"))]
impl From<&PgRow> for TableConstraintQueryResult {
    fn from(row: &PgRow) -> Self {
        Self::default()
    }
}
