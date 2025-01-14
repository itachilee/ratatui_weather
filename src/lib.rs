pub mod json;
pub mod model;
pub mod redis_db;
// pub mod reducer;
// pub mod store;
pub mod ui;

pub mod predule {
    pub use crate::ui::*;
    pub use ratatui::prelude::*;
    pub use ratatui::*;
}
