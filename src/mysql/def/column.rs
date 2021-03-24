use super::Type;

#[derive(Debug, sea_query::Iden)]
/// Ref: https://dev.mysql.com/doc/refman/8.0/en/information-schema-columns-table.html
pub enum ColumnFields {
    TableCatalog,
    TableSchema,
    TableName,
    ColumnName,
    OrdinalPosition,
    ColumnDefault,
    IsNullable,
    DataType,
    CharacterMaximumLength,
    CharacterOctetLength,
    NumericPrecision,
    NumericScale,
    DatetimePrecision,
    CharacterSetName,
    CollationName,
    ColumnType,
    ColumnKey,
    Extra,
    Privileges,
    ColumnComment,
    GenerationExpression,
    SrsId,
}

#[derive(Debug)]
pub struct ColumnInfo {
    /// The name of the column
    pub name: String,
    /// The type of the column with additional definitions, e.g. precision, length
    pub col_type: ColumnType,
    /// Can this column contains null
    pub null: bool,
    /// Is this column indexed
    pub key: ColumnKey,
    /// Default value expression for this column, if any
    pub default: Option<ColumnDefault>,
    /// Extra definitions for this column, e.g. auto_increment
    pub extra: ColumnExtra,
    /// The generation expression if this is a generated column 
    pub expression: Option<String>,
    /// User comments
    pub comment: String,
}

pub type ColumnType = Type;

#[derive(Debug, PartialEq)]
pub enum ColumnKey {
    /// This column is not the first column of any key
    NotKey,
    /// This column is part of the primary key
    Primary,
    /// This column is the first column of a unique key
    Unique,
    /// This column is the first column of a non-unique key
    Multiple,
}

#[derive(Debug)]
pub struct ColumnDefault {
    /// default value expression
    pub expr: String,
}

#[derive(Debug, Default, PartialEq)]
pub struct ColumnExtra {
    /// Auto increment
    pub auto_increment: bool,
    /// Only applies to timestamp or datetime
    pub on_update_current_timestamp: bool,
    /// This is a generated column 
    pub generated: bool,
    /// This column has a default value expression
    pub default_generated: bool,
}