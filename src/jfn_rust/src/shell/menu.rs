//! The edit menu, raised through the platform's own menu host.

use jfn_platform_abi::{App, ChannelValue, LogicalPoint};

use crate::shell::actor::{Target, Work};
use crate::shell::fields::Snapshot;
use crate::shell::lang::Strings;

/// The edit menu's items, in order.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Item {
    Undo,
    Redo,
    Cut,
    Copy,
    Paste,
    SelectAll,
}

impl Item {
    pub const ALL: [Item; 6] = [
        Item::Undo,
        Item::Redo,
        Item::Cut,
        Item::Copy,
        Item::Paste,
        Item::SelectAll,
    ];

    pub fn command(self) -> jfn_input::EditCommand {
        use jfn_input::EditCommand as E;
        match self {
            Item::Undo => E::Undo,
            Item::Redo => E::Redo,
            Item::Cut => E::Cut,
            Item::Copy => E::Copy,
            Item::Paste => E::Paste,
            Item::SelectAll => E::SelectAll,
        }
    }

    pub fn label(self, strings: &Strings) -> &'static str {
        match self {
            Item::Undo => strings.undo,
            Item::Redo => strings.redo,
            Item::Cut => strings.cut,
            Item::Copy => strings.copy,
            Item::Paste => strings.paste,
            Item::SelectAll => strings.select_all,
        }
    }

    /// Enabled exactly when it would change something; Paste always.
    pub fn enabled(self, field: &Snapshot) -> bool {
        let edit = field.edit_state();
        match self {
            Item::Undo => edit.undo,
            Item::Redo => edit.redo,
            Item::Cut => edit.cut,
            Item::Copy => edit.copy,
            Item::Paste => true,
            Item::SelectAll => edit.select_all,
        }
    }
}

/// Raises the edit menu for `field` at `anchor`, in window coordinates. A
/// selection posts [`crate::shell::actor::Work::EditAt`] naming `field`; a dismissal
/// posts nothing. No accelerator text is drawn.
pub fn open_edit(app: &App, field: &Snapshot, anchor: LogicalPoint, strings: &Strings) {
    let sender = crate::shell::shell_sender();
    let items = Item::ALL
        .into_iter()
        .map(|item| {
            let id = field.id.clone();
            let command = item.command();
            jfn_platform_abi::Item {
                label: item.label(strings).to_owned(),
                enabled: item.enabled(field),
                value: ChannelValue::new(
                    sender.clone(),
                    Work::EditAt {
                        field: Target::Named(id),
                        command,
                    },
                ),
            }
        })
        .collect();
    app.open_context_menu(app.new_menu(anchor).add_items(items));
}
