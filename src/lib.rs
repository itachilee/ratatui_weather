pub mod api;
pub mod json;
pub mod mediator;
pub mod observer;
pub mod ui;
// pub mod redis_db;
pub mod predule {
    pub use ratatui::prelude::*;
    pub use ratatui::*;
}
