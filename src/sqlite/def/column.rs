use super::{DefaultType, Type};
use sea_query::{
    foreign_key::ForeignKeyAction as SeaQueryForeignKeyAction, Alias, Index, IndexCreateStatement,
};
use std::num::ParseIntError;

#[allow(unused_imports)]
use crate::sqlx_types::{sqlite::SqliteRow, Row};

/// An SQLite column definition
#[derive(Debug, PartialEq, Clone)]
pub struct ColumnInfo {
    pub cid: i32,
    pub name: String,
    pub r#type: Type,
    pub not_null: bool,
    pub default_value: DefaultType,
    pub primary_key: bool,
}

#[cfg(feature = "sqlx-sqlite")]
impl ColumnInfo {
    /// Map an [SqliteRow] into a column definition type [ColumnInfo]
    pub fn to_column_def(row: &SqliteRow) -> Result<ColumnInfo, ParseIntError> {
        let col_not_null: i8 = row.get(3);
        let is_pk: i8 = row.get(5);
        let default_value: &str = row.get(4);
        Ok(ColumnInfo {
            cid: row.get(0),
            name: row.get(1),
            r#type: Type::to_type(row.get(2))?,
            not_null: col_not_null != 0,
            default_value: if default_value == "NULL" {
                DefaultType::Null
            } else if default_value.is_empty() {
                DefaultType::Unspecified
            } else {
                let value = default_value.to_owned().replace('\'', "");

                if let Ok(is_int) = value.parse::<i32>() {
                    DefaultType::Integer(is_int)
                } else if let Ok(is_float) = value.parse::<f32>() {
                    DefaultType::Float(is_float)
                } else {
                    DefaultType::String(value)
                }
            },
            primary_key: is_pk != 0,
        })
    }
}

#[cfg(not(feature = "sqlx-sqlite"))]
impl ColumnInfo {
    pub fn to_column_def(_: &SqliteRow) -> Result<ColumnInfo, ParseIntError> {
        i32::from_str_radix("", 10)?;
        unimplemented!()
    }
}

/// Maps the index and all columns in the index which is the result of queries
/// `PRAGMA index_list(table_name)` and
/// `SELECT * FROM sqlite_master where name = 'index_name'`
#[derive(Debug, Default, Clone)]
pub struct IndexInfo {
    /// Is it a SQLindex
    pub r#type: String,
    pub index_name: String,
    pub table_name: String,
    pub unique: bool,
    pub origin: String,
    pub partial: i32,
    pub columns: Vec<String>,
}

impl IndexInfo {
    /// Write all the discovered index into a [IndexCreateStatement]
    pub fn write(&self) -> IndexCreateStatement {
        let mut new_index = Index::create();
        new_index
            .name(&self.index_name)
            .table(Alias::new(&self.table_name));

        if self.unique {
            new_index.unique();
        }

        self.columns.iter().for_each(|column| {
            new_index.col(Alias::new(column));
        });

        new_index
    }
}

/// Maps the index all columns as a result of using query
/// `PRAGMA index_list(table_name)`
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub(crate) struct PartialIndexInfo {
    pub(crate) seq: i32,
    pub(crate) name: String,
    pub(crate) unique: bool,
    pub(crate) origin: String,
    pub(crate) partial: i32,
}

#[cfg(feature = "sqlx-sqlite")]
impl From<&SqliteRow> for PartialIndexInfo {
    fn from(row: &SqliteRow) -> Self {
        let is_unique: i8 = row.get(2);
        Self {
            seq: row.get(0),
            name: row.get(1),
            unique: is_unique != 0,
            origin: row.get(3),
            partial: row.get(4),
        }
    }
}

#[cfg(not(feature = "sqlx-sqlite"))]
impl From<&SqliteRow> for PartialIndexInfo {
    fn from(_: &SqliteRow) -> Self {
        Self::default()
    }
}

/// Maps all the columns in an index as a result of using query
/// `SELECT * FROM sqlite_master where name = 'index_name'`
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub(crate) struct IndexedColumns {
    pub(crate) r#type: String,
    pub(crate) name: String,
    pub(crate) table: String,
    pub(crate) root_page: i32,
    pub(crate) indexed_columns: Vec<String>,
}

#[cfg(feature = "sqlx-sqlite")]
impl From<&SqliteRow> for IndexedColumns {
    fn from(row: &SqliteRow) -> Self {
        let indexed_columns_new: String = row.get(4);
        let split_at_on = indexed_columns_new.split("ON").collect::<Vec<_>>();
        let split_at_open_bracket = split_at_on[1].trim().split('(').collect::<Vec<_>>();
        let columns_to_index = split_at_open_bracket[1]
            .replace(')', "")
            .split(',')
            .map(|column| column.trim().replace(['`', '"'], ""))
            .collect::<Vec<String>>();

        Self {
            r#type: row.get(0),
            name: row.get(1),
            table: row.get(2),
            root_page: row.get(3),
            indexed_columns: columns_to_index,
        }
    }
}

#[cfg(not(feature = "sqlx-sqlite"))]
impl From<&SqliteRow> for IndexedColumns {
    fn from(_: &SqliteRow) -> Self {
        Self::default()
    }
}

/// Confirms if a table's primary key is set to autoincrement as a result of using query
/// `SELECT COUNT(*) from sqlite_sequence where name = 'table_name';
#[derive(Debug, Default, Clone)]
pub(crate) struct PrimaryKeyAutoincrement(pub(crate) u8);

#[cfg(feature = "sqlx-sqlite")]
impl From<&SqliteRow> for PrimaryKeyAutoincrement {
    fn from(row: &SqliteRow) -> Self {
        Self(row.get(0))
    }
}

#[cfg(not(feature = "sqlx-sqlite"))]
impl From<&SqliteRow> for PrimaryKeyAutoincrement {
    fn from(_: &SqliteRow) -> Self {
        Self::default()
    }
}

/// Indexes the foreign keys
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct ForeignKeysInfo {
    pub(crate) id: i32,
    pub(crate) seq: i32,
    pub(crate) table: String,
    pub(crate) from: String,
    pub(crate) to: String,
    pub(crate) on_update: ForeignKeyAction,
    pub(crate) on_delete: ForeignKeyAction,
    pub(crate) r#match: MatchAction,
}

#[cfg(feature = "sqlx-sqlite")]
impl From<&SqliteRow> for ForeignKeysInfo {
    fn from(row: &SqliteRow) -> Self {
        Self {
            id: row.get(0),
            seq: row.get(1),
            table: row.get(2),
            from: row.get(3),
            to: row.get(4),
            on_update: {
                let op: &str = row.get(5);
                op.into()
            },
            on_delete: {
                let op: &str = row.get(6);
                op.into()
            },
            r#match: {
                let op: &str = row.get(7);
                op.into()
            },
        }
    }
}

#[cfg(not(feature = "sqlx-sqlite"))]
impl From<&SqliteRow> for ForeignKeysInfo {
    fn from(_: &SqliteRow) -> Self {
        Self::default()
    }
}

/// Indexes the actions performed on the foreign keys of a table
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ForeignKeyAction {
    NoAction,
    Restrict,
    SetNull,
    SetDefault,
    Cascade,
}

impl Default for ForeignKeyAction {
    fn default() -> Self {
        Self::NoAction
    }
}

impl From<&str> for ForeignKeyAction {
    fn from(action: &str) -> Self {
        match action {
            "NO ACTION" => Self::NoAction,
            "RESTRICT" => Self::Restrict,
            "SET NULL" => Self::SetNull,
            "SET DEFAULT" => Self::SetDefault,
            "CASCADE" => Self::Cascade,
            _ => Self::NoAction,
        }
    }
}

impl ForeignKeyAction {
    pub(crate) fn to_seaquery_foreign_key_action(&self) -> SeaQueryForeignKeyAction {
        match self {
            Self::NoAction => SeaQueryForeignKeyAction::NoAction,
            Self::Restrict => SeaQueryForeignKeyAction::Restrict,
            Self::SetNull => SeaQueryForeignKeyAction::SetNull,
            Self::SetDefault => SeaQueryForeignKeyAction::SetDefault,
            Self::Cascade => SeaQueryForeignKeyAction::Cascade,
        }
    }
}

/// Maps to the SQLite `MATCH` actions
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MatchAction {
    Simple,
    Partial,
    Full,
    None,
}

impl Default for MatchAction {
    fn default() -> Self {
        Self::None
    }
}

impl From<&str> for MatchAction {
    fn from(action: &str) -> Self {
        match action {
            "MATCH SIMPLE" => Self::Simple,
            "MATCH PARTIAL" => Self::Partial,
            "MATCH FULL" => Self::Full,
            "MATCH NONE" => Self::None,
            _ => Self::None,
        }
    }
}
