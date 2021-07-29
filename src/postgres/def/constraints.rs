#[cfg(feature = "with-serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
/// An enum consisting of all constraints
pub enum Constraint {
    Check(Check),
    NotNull(NotNull),
    Unique(Unique),
    PrimaryKey(PrimaryKey),
    References(References),
    Exclusion(Exclusion),
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
/// A constraint which states that a value must satisfy the following Boolean expression
pub struct Check {
    /// The Boolean expression that must be satisfied
    pub expr: String,
    /// If marked with NO INHERIT, the constraint will not propogate to child tables
    pub no_inherit: bool,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
/// The constraint that a value must not be null
pub struct NotNull;

impl NotNull {
    pub fn from_bool(boolean: bool) -> Option<NotNull> {
        if boolean {
            Some(NotNull)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
/// That each set of values for these columns must be unique across the whole table
pub struct Unique(pub Vec<String>);

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
/// A constraint stating that the given columns act as a unique identifier for rows in the table.
/// This implies that the columns are not null and are unique together
pub struct PrimaryKey(pub Vec<String>);

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
/// A constraint that column references the values appearing in the row of another table
pub struct References {
    pub columns: Vec<String>,
    pub table: String,
    pub foreign_columns: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
/// A constraint that ensures that, if any two rows are compared on the specified columns or
/// expressions using the specified operators, at least one of these operator comparisons returns
/// false or null
pub struct Exclusion {
    pub using: String,
    pub columns: Vec<String>,
    pub operation: String,
}
