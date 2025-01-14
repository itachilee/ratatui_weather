pub mod editor;
pub mod event;
pub mod mediator;
pub mod model;
pub mod reducer;
pub mod screen;
pub mod storage;
pub mod traits;

use editor::EditorState;
use screen::ScreenManager;
use storage::StorageManager;
#[derive(Clone, Debug)]
pub enum CurrentlyEditing {
    Key,
    Value,
}
#[derive(Clone, PartialEq, Debug)]
pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

pub struct AppModel {
    screen_manager: ScreenManager,
    editor_state: EditorState,
    storage_manager: StorageManager,
}
