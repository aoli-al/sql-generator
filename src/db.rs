/// The base type for what fills in a sudoku square. An enum instead of a raw integer/wrapper struct for ~*~ type safety ~*~
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DTYPE {
    INT,
    FLOAT,
}

/// The type of an index into a board
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Field {
    pub name: String,
    pub dtype: DTYPE,
}


/// The base type for a sudoku board. I *think* Rust should automatically lay out the doubly-nested
/// array in an efficient fashion?
#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub fields: Vec<Field>,
}


impl DTYPE {
    /// Constructs a data type from string
    pub fn from_s(s: String) -> Option<Self> {
        match s.as_str() {
            "int" => Some(DTYPE::INT),
            "float" => Some(DTYPE::FLOAT),
            _ => None,
        }
    }
}