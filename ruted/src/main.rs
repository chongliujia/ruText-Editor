#![warn(clippy::all, clippy::pedantic, clippy::restriction)]
#[allow(
    clippy::missing_docs_in_private_items,
    clippy::implicit_return,
    clippy::shadow_reuse,
    clippy::print_stdout,
    clippy::wildcard_enum_match_arm,
    clippy::else_if_without_else
)]

mod document;
mod editor;
mod filetype;
mod terminal;
mod row;
mod highlighting;

use editor::Editor;

pub use editor::Position;
pub use editor::SearchDirection;

pub use row::Row;

pub use terminal::Terminal;

pub use filetype::FileType;

fn main() {
    Editor::default().run();
}
