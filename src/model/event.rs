use crate::model::CurrentScreen;

use crate::model::CurrentlyEditing;

#[derive(Clone, Debug)]
pub enum AppEvent {
    ScreenChanged(CurrentScreen),
    EditingStateChanged(Option<CurrentlyEditing>),
    InputChanged(String, InputField),
    SaveRequested,
    ExitRequested(bool),
    // ... 其他事件
}
#[derive(Clone, Debug)]
pub enum InputField {
    Key,
    Value,
}

pub trait EventListener {
    fn on_event(&mut self, event: AppEvent);
}
