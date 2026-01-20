use crate::hyprland::{KeyBindEntry, SearchOptions};
use crate::ui::{SortColumn, SortState};
use std::cmp::Ordering;

pub fn filter_and_sort(
    entries: &[KeyBindEntry],
    search_query: &str,
    search_options: &SearchOptions,
    sort_column: SortColumn,
    sort_state: SortState,
) -> Vec<KeyBindEntry> {
    let mut filtered: Vec<_> = entries
        .iter()
        .filter(|e| matches_search(e, search_query, search_options))
        .cloned()
        .collect();

    if sort_state != SortState::None {
        apply_sort(&mut filtered, sort_column);
        if sort_state == SortState::Descending {
            filtered.reverse();
        }
    }

    filtered
}

fn matches_search(entry: &KeyBindEntry, query: &str, options: &SearchOptions) -> bool {
    if query.is_empty() {
        return true;
    }

    entry.matches(query, options)
}

fn apply_sort(entries: &mut [KeyBindEntry], sort_column: SortColumn) {
    match sort_column {
        SortColumn::Description => {
            entries.sort_by(|a, b| a.description.cmp(&b.description));
        }
        SortColumn::Keybind => {
            entries.sort_by(|a, b| {
                let mod_cmp = a.modifiers.cmp(&b.modifiers);
                if mod_cmp == Ordering::Equal {
                    a.key.cmp(&b.key)
                } else {
                    mod_cmp
                }
            });
        }
        SortColumn::Command => {
            entries.sort_by(|a, b| a.command.cmp(&b.command));
        }
    }
}

pub fn next_sort_state(
    current_column: SortColumn,
    clicked_column: SortColumn,
    current_state: SortState,
) -> (SortColumn, SortState) {
    if current_column == clicked_column {
        let next_state = match current_state {
            SortState::Ascending => SortState::Descending,
            SortState::Descending => SortState::None,
            SortState::None => SortState::Ascending,
        };
        (clicked_column, next_state)
    } else {
        (clicked_column, SortState::Ascending)
    }
}
