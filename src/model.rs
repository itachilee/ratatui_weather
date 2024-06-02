use std::collections::HashMap;

pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

pub enum CurrentlyEditing {
    Key,
    Value,
}
pub struct Model {
    pub key_input: String,
    pub value_input: String,
    pub pairs: HashMap<String, String>,
    pub current_screen: CurrentScreen,
    pub current_editing: Option<CurrentlyEditing>,
    pub should_exit: bool,
    pub should_print: bool,
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
