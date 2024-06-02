use crossterm::event::KeyCode;

use crate::model::{CurrentScreen, CurrentlyEditing, Model};

pub enum EditAction {
    Char(char),
    Tab,
    Enter,
    Backspace,
    Esc,
}

pub enum Action {
    ChangeToMainMode,
    MainMode,
    ChangeToEditMode,
    EditMode(EditAction),
    ChangeToExitMode,
    ExitMode(KeyCode),
}

pub fn reducer(state: &mut Model, action: Action) {
    match action {
        Action::ChangeToMainMode => {
            state.current_screen = CurrentScreen::Main;
        }
        Action::ChangeToEditMode => {
            state.current_screen = CurrentScreen::Editing;
            state.current_editing = Some(CurrentlyEditing::Key);
        }
        Action::ChangeToExitMode => {
            state.current_screen = CurrentScreen::Exiting;
        }
        Action::MainMode => {}
        Action::EditMode(action) => match action {
            EditAction::Tab => {
                state.toggle_editing();
            }
            EditAction::Backspace => {
                if let Some(editing) = &state.current_editing {
                    match editing {
                        CurrentlyEditing::Key => {
                            state.key_input.pop();
                        }
                        CurrentlyEditing::Value => {
                            state.value_input.pop();
                        }
                    }
                }
            }
            EditAction::Char(value) => {
                if let Some(editing) = &state.current_editing {
                    match editing {
                        CurrentlyEditing::Key => {
                            state.key_input.push(value);
                        }
                        CurrentlyEditing::Value => {
                            state.value_input.push(value);
                        }
                    }
                }
            }
            EditAction::Enter => {
                if let Some(editing) = &state.current_editing {
                    match editing {
                        CurrentlyEditing::Key => {
                            state.current_editing = Some(CurrentlyEditing::Key)
                        }
                        CurrentlyEditing::Value => {
                            // app.current_editing = Some(CurrentlyEditing::Value);

                            state.save_key_value();
                            state.current_editing = None;
                            state.current_screen = CurrentScreen::Main;
                        }
                    }
                }
            }
            EditAction::Esc => {
                state.current_screen = CurrentScreen::Main;
                state.current_editing = None;
            }
        },
        Action::ExitMode(key) => match key {
            KeyCode::Char('y') => {
                state.should_exit = true;
            }
            KeyCode::Char('n') | KeyCode::Char('q') => {
                state.should_exit = false;
            }
            _ => {}
        },
    }
}
