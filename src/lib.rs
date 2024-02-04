#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod customized;
pub use app::TemplateApp;
pub use customized::toggle_switch;
