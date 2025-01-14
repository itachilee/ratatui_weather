use crate::model::CurrentScreen;
use crate::model::CurrentlyEditing;
use std::collections::HashMap;
pub trait Storage {
    fn save(&mut self, key: String, value: String);
    fn load(&self) -> HashMap<String, String>;
}

pub trait Screen {
    fn get_current(&self) -> CurrentScreen;
    fn change_to(&mut self, screen: CurrentScreen);
}

pub trait StateQuery {
    fn get_editing_state(&self) -> Option<CurrentlyEditing>;
    // 其他需要查询的状态...
}

pub trait IModel {
    fn should_exit(&self) -> bool;
}
