use sea_query::{Condition, Expr, IntoTableRef, Query, SelectStatement, SimpleExpr};

use super::query::{InformationSchema as Schema, TablesFields};
use super::Postgres;
use crate::probe::SchemaProbe;

impl SchemaProbe for Postgres {
    fn get_current_schema() -> SimpleExpr {
        Expr::cust("CURRENT_SCHEMA()")
    }

    fn query_tables() -> SelectStatement {
        let (expr, tbl_ref, condition) = (
            Expr::col(TablesFields::TableName),
            (Schema::Schema, Schema::Tables).into_table_ref(),
            Condition::all()
                .add(
                    Expr::expr(Self::get_current_schema())
                        .equals(Schema::Tables, TablesFields::TableSchema),
                )
                .add(Expr::col(TablesFields::TableType).eq("BASE TABLE")),
        );
        Query::select()
            .expr_as(expr, TablesFields::TableName)
            .from(tbl_ref)
            .cond_where(condition)
            .take()
    }
}
