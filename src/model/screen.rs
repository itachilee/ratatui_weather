use crate::model::traits::Screen;
use crate::model::CurrentScreen;
pub struct ScreenManager {
    current: CurrentScreen,
}

impl ScreenManager {
    pub fn new() -> Self {
        Self {
            current: CurrentScreen::Main,
        }
    }
}

impl Screen for ScreenManager {
    fn get_current(&self) -> CurrentScreen {
        // self.current
        panic!("not implemented");
    }

    fn change_to(&mut self, screen: CurrentScreen) {
        self.current = screen;
    }
}
