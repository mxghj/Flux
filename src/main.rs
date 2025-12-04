mod core;
use crate::{core::apps::indexer::indexing, toml_files::{Config, read_theme, settings}, ui::app::run_ui};
mod ui;
mod toml_files;
fn main() -> iced::Result
{
    let apps = indexing().unwrap_or_default();
    let config: Config = settings().unwrap_or(
        Config {
            using: "Stryde-Dark".to_string(),
            antialiasing: false,
            list_text_size: 16,
            input_text_size: 18,
            app_width: 774.0,
            app_height: 500.0,
            icon_size: 37,
            show_apps: true,
            close_on_launch: true
        }
    );
    // Get settings if get any errors put the default one
    let theme = read_theme(&config.using).unwrap_or(
        iced::Theme::custom(
            "Stryde-Dark".to_string(),
            iced::theme::Palette {
                background: iced::Color::from_rgb(0.063, 0.063, 0.071),
                text: iced::Color::WHITE,
                primary: iced::Color::from_rgb(0.055, 0.122, 0.165),
                success: iced::Color::from_rgb(0.306, 0.306, 0.318),
                danger: iced::Color::from_rgb(25.0/255.0, 25.0/255.0, 28.0/255.0),
            },
        )
    );
    // Get theme if get any errors put the default one
    run_ui(apps, config, theme)
}
