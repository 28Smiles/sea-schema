use sea_query::{Alias, Iden, SchemaStatement, Table, TableStatement};
use crate::mysql::def::TableDef;

impl TableDef {
    pub fn write(&self, statements: &mut Vec<SchemaStatement>) {
        let mut table = Table::create();
        table.table(Alias::new(self.info.name.as_ref()));
        for col in self.columns.iter() {
            table.col(col.write());
        }
        table.engine(self.info.engine.to_string().as_str());
        table.character_set(self.info.char_set.to_string().as_str());
        table.collate(self.info.collation.to_string().as_str());
        statements.push(SchemaStatement::TableStatement(TableStatement::Create(table)));
    }
}

#[cfg(test)]
mod tests {
    use sea_query::{MysqlQueryBuilder, SchemaStatement, TableStatement};
    use crate::mysql::def::*;

    #[test]
    fn test_1() {
        let mut statements = Vec::new();

        TableDef {
            info: TableInfo {
                name: "actor".to_owned(),
                engine: StorageEngine::InnoDb,
                auto_increment: Some(
                    200,
                ),
                char_set: CharSet::Utf8Mb4,
                collation: Collation::Utf8Mb40900AiCi,
                comment: "".to_owned(),
            },
            columns: vec![
                ColumnInfo {
                    name: "actor_id".to_owned(),
                    col_type: ColumnType::SmallInt(
                        NumericAttr {
                            maximum: None,
                            decimal: None,
                            unsigned: Some(
                                true,
                            ),
                            zero_fill: None,
                        },
                    ),
                    null: false,
                    key: ColumnKey::Primary,
                    default: None,
                    extra: ColumnExtra {
                        auto_increment: true,
                        on_update_current_timestamp: false,
                        generated: false,
                        default_generated: false,
                    },
                    expression: None,
                    comment: "".to_owned(),
                },
                ColumnInfo {
                    name: "first_name".to_owned(),
                    col_type: ColumnType::Varchar(
                        StringAttr {
                            length: Some(
                                45,
                            ),
                            charset: None,
                            collation: None,
                        },
                    ),
                    null: false,
                    key: ColumnKey::NotKey,
                    default: None,
                    extra: ColumnExtra {
                        auto_increment: false,
                        on_update_current_timestamp: false,
                        generated: false,
                        default_generated: false,
                    },
                    expression: None,
                    comment: "".to_owned(),
                },
                ColumnInfo {
                    name: "last_name".to_owned(),
                    col_type: ColumnType::Varchar(
                        StringAttr {
                            length: Some(
                                45,
                            ),
                            charset: None,
                            collation: None,
                        },
                    ),
                    null: false,
                    key: ColumnKey::Multiple,
                    default: None,
                    extra: ColumnExtra {
                        auto_increment: false,
                        on_update_current_timestamp: false,
                        generated: false,
                        default_generated: false,
                    },
                    expression: None,
                    comment: "".to_owned(),
                },
                ColumnInfo {
                    name: "last_update".to_owned(),
                    col_type: ColumnType::Timestamp(
                        TimeAttr {
                            fractional: None,
                        },
                    ),
                    null: false,
                    key: ColumnKey::NotKey,
                    default: Some(
                        ColumnDefault {
                            expr: "CURRENT_TIMESTAMP".to_owned(),
                        },
                    ),
                    extra: ColumnExtra {
                        auto_increment: false,
                        on_update_current_timestamp: true,
                        generated: false,
                        default_generated: true,
                    },
                    expression: None,
                    comment: "".to_owned(),
                },
            ],
            indexes: vec![],
            foreign_keys: vec![],
        }.write(&mut statements);

        assert_eq!(
            if let SchemaStatement::TableStatement(statement) = &statements[0] {
                if let TableStatement::Create(statement) = statement {
                    statement.to_string(MysqlQueryBuilder)
                } else {
                    "".to_owned()
                }
            } else {
                "".to_owned()
            },
            vec![
                "CREATE TABLE `actor` (",
                    "`actor_id` SMALLINT UNSIGNED NOT NULL AUTO_INCREMENT,",
                    "`first_name` VARCHAR(45) NOT NULL,",
                    "`last_name` VARCHAR(45) NOT NULL,",
                    "`last_update` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP",
                ")",
                "ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci",
            ].join(" ")
        );
    }
}