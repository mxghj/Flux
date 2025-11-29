mod core;
use crate::{core::apps::indexer::indexing, ui::app::run_ui};
mod ui;
// Main function that run parser
fn main() -> iced::Result
{
    let apps = indexing().unwrap_or_default();
    run_ui(apps)
}
