#[derive(Debug, sea_query::Iden)]
/// Ref: https://dev.mysql.com/doc/refman/8.0/en/information-schema-key-column-usage-table.html
pub enum KeyColumnUsage {
    ConstraintSchema,
    ConstraintName,
    TableSchema,
    TableName,
    ColumnName,
    OrdinalPosition,
    PositionInUniqueConstraint,
    ReferencedTableSchema,
    ReferencedTableName,
    ReferencedColumnName,
}

#[derive(Debug, sea_query::Iden)]
/// Ref: https://dev.mysql.com/doc/refman/8.0/en/information-schema-referential-constraints-table.html
pub enum ReferentialConstraints {
    ConstraintSchema,
    ConstraintName,
    UniqueConstraintSchema,
    UniqueConstraintName,
    UpdateRule,
    DeleteRule,
    TableName,
    ReferencedTableName,
}