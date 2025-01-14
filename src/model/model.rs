use std::collections::HashMap;

use ratatui::widgets::ListState;

use crate::model::event::{AppEvent, EventListener, InputField};
use crate::model::traits::{IModel, StateQuery};
use crate::model::CurrentScreen;
use crate::model::CurrentlyEditing;
pub struct Model {
    pub key_input: String,
    pub value_input: String,
    pub pairs: HashMap<String, String>,
    pub current_screen: CurrentScreen,
    pub current_editing: Option<CurrentlyEditing>,
    pub should_exit: bool,
    pub should_print: bool,
    pub list_state: Option<ListState>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            key_input: String::new(),
            value_input: String::new(),
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
            current_editing: None,
            should_exit: false,
            should_print: false,
            list_state: None,
        }
    }

    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.current_editing {
            match edit_mode {
                CurrentlyEditing::Key => self.current_editing = Some(CurrentlyEditing::Value),
                CurrentlyEditing::Value => self.current_editing = Some(CurrentlyEditing::Key),
            }
        } else {
            self.current_editing = Some(CurrentlyEditing::Key)
        }
    }

    pub fn save_key_value(&mut self) {
        self.pairs
            .insert(self.key_input.clone(), self.value_input.clone());
        self.key_input = String::new();
        self.value_input = String::new();
    }

    pub fn print_json(&self) -> serde_json::Result<()> {
        let json = serde_json::to_string(&self.pairs)?;
        println!("{}", json);

        Ok(())
    }
}

impl EventListener for Model {
    fn on_event(&mut self, event: AppEvent) {
        println!("Model received event: {:?}", event);

        match event {
            AppEvent::ScreenChanged(screen) => {
                println!("Model 收到 ScreenChanged 事件: {:?}", screen);
                self.current_screen = screen;
            }
            AppEvent::EditingStateChanged(editing_state) => {
                self.current_editing = editing_state;
            }
            AppEvent::InputChanged(value, field) => match field {
                InputField::Key => self.key_input.push_str(&value),
                InputField::Value => self.value_input.push_str(&value),
            },
            AppEvent::SaveRequested => {
                self.save_key_value();
            }
            AppEvent::ExitRequested(should_exit) => {
                self.should_exit = should_exit;
                self.should_print = should_exit;
            }
        }
    }
}

impl StateQuery for Model {
    fn get_editing_state(&self) -> Option<CurrentlyEditing> {
        self.current_editing.clone()
    }
}

impl IModel for Model {
    fn should_exit(&self) -> bool {
        self.should_exit
    }
}
