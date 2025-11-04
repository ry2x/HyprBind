#[derive(PartialEq, Eq, Clone, Copy)]
pub enum SortColumn {
    Keybind,
    Description,
    Command,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum SortState {
    Ascending,
    Descending,
    None,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Theme {
    Dark,
    Light,
}

pub struct ColumnVisibility {
    pub keybind: bool,
    pub description: bool,
    pub command: bool,
}

impl Default for ColumnVisibility {
    fn default() -> Self {
        Self {
            keybind: true,
            description: true,
            command: false,
        }
    }
}
