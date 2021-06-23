use crate::{
    parser::Parser,
    postgres::{
        def::*,
        parser::yes_or_no_to_bool,
        query::{constraints, ColumnQueryResult},
    },
    Name,
};

use std::convert::TryFrom;

pub fn parse_column_query_reulst(
    result: ColumnQueryResult,

    // A Vec of Vecs as one column can be a part of several constraints, and each constraint has
    // one or more elements
    unique_results: Vec<Vec<constraints::UniqueQueryResult>>,
) -> ColumnInfo {
    ColumnInfo {
        name: result.column_name.clone(),
        col_type: parse_column_type(&result),
        default: ColumnExpression::from_option_string(result.column_default),
        generated: ColumnExpression::from_option_string(result.column_generated),
        not_null: NotNull::from_bool(yes_or_no_to_bool(&result.is_nullable)),
        unique: parse_unique_query_result_vecs(unique_results),
    }
}

fn parse_unique_query_result_vecs(
    results: Vec<Vec<constraints::UniqueQueryResult>>,
) -> Option<Vec<Unique>> {
    let mut out = Vec::new();
    for result in results {
        if let Ok(unique) = super::constraints::parse_unique_query_results(result) {
            out.push(unique);
        }
    }

    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}

pub fn parse_column_type(result: &ColumnQueryResult) -> ColumnType {
    let mut parser_type = Parser::new(&result.column_type);

    if parser_type.curr().is_none() {
        return Type::Unknown(String::default());
    }

    let ctype = if let Some(word) = parser_type.next_if_unquoted_any() {
        parse_type_name(word.as_str())
    } else {
        parse_type_name("")
    };

    if ctype.has_numeric_attr() {
        parse_numeric_attributes(
            result.numeric_precision,
            result.numeric_precision_radix,
            result.numeric_scale,
            ctype,
        )
    } else {
        ctype
    }
}

pub fn parse_type_name(type_name: &str) -> Type {
    match type_name.to_lowercase().as_str() {
        "smallint" | "int2" => Type::SmallInt,
        "integer" | "int" | "int4" => Type::Integer,
        "bigint" | "int8" => Type::BigInt,
        "decimal" => Type::Decimal(ArbitraryPrecisionNumericAttr::default()),
        "numeric" => Type::Numeric(ArbitraryPrecisionNumericAttr::default()),
        "real" | "float4" => Type::Real,
        "double precision" | "double" | "float8" => Type::DoublePrecision,
        "smallserial" | "serial2" => Type::SmallSerial,
        "serial" | "serial4" => Type::Serial,
        "bigserial" | "serial8" => Type::BigSerial,

        _ => Type::Unknown(format!("{} is unknown or unimplemented", type_name)),
    }
}

pub fn parse_numeric_attributes(
    num_precision: Option<i32>,
    num_precision_radix: Option<i32>,
    num_scale: Option<i32>,
    mut ctype: ColumnType,
) -> ColumnType {
    let numeric_precision: Option<u16> = match num_precision {
        None => None,
        Some(num) => match u16::try_from(num) {
            Ok(num) => Some(num),
            Err(e) => None,
        },
    };
    let numeric_precision_radix: Option<u16> = match num_precision_radix {
        None => None,
        Some(num) => match u16::try_from(num) {
            Ok(num) => Some(num),
            Err(e) => None,
        },
    };
    let numeric_scale: Option<u16> = match num_scale {
        None => None,
        Some(num) => match u16::try_from(num) {
            Ok(num) => Some(num),
            Err(e) => None,
        },
    };

    match ctype {
        Type::Decimal(ref mut attr) | Type::Numeric(ref mut attr) => {
            attr.precision = numeric_precision;
            attr.scale = numeric_scale;
        }
        _ => panic!("parse_numeric_attributes(_) received a type other than Decimal or Numeric"),
    };

    ctype
}
