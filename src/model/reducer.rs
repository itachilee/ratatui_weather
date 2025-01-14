use crate::model::event::{AppEvent, InputField};
use crate::model::mediator::Mediator;
use crate::model::traits::StateQuery;
use crate::model::CurrentScreen;
use crate::model::CurrentlyEditing;
use crossterm::event::KeyCode;

use super::model::Model;
#[derive(Debug)]
pub enum EditAction {
    Char(char),
    Tab,
    Enter,
    Backspace,
    Esc,
}

#[derive(Debug)]
pub enum ListAction {
    Up,
    Down,
    Edit,
}

#[derive(Debug)]
pub enum Action {
    ChangeScreen(CurrentScreen),
    MainMode(ListAction),
    EditMode(EditAction),
    ExitMode(KeyCode),
}

pub struct ActionReducer {
    mediator: Box<dyn Mediator>,
    model: Box<Model>,
}

impl ActionReducer {
    pub fn new(mediator: Box<dyn Mediator>, model: Box<Model>) -> Self {
        Self { mediator, model }
    }

    pub fn get_model(&self) -> &Box<Model> {
        &self.model
    }

    pub fn reduce(&mut self, action: Action) {
        println!("Reducing action: {:?}", action); // 添加日志

        match action {
            Action::ChangeScreen(screen) => {
                self.mediator
                    .notify(AppEvent::ScreenChanged(screen.clone()));

                println!("通知主页变更 {:?}", screen);
                if screen.clone() == CurrentScreen::Editing {
                    println!("通知编辑状态变更 {:?}", screen);
                    self.mediator
                        .notify(AppEvent::EditingStateChanged(Some(CurrentlyEditing::Key)));
                }
            }

            Action::EditMode(action) => match action {
                EditAction::Tab => {
                    if let Some(current_editing) = self.model.get_editing_state() {
                        self.mediator.notify(AppEvent::EditingStateChanged(Some(
                            match current_editing {
                                CurrentlyEditing::Key => CurrentlyEditing::Value,
                                CurrentlyEditing::Value => CurrentlyEditing::Key,
                            },
                        )));
                    }
                }

                EditAction::Char(value) => {
                    if let Some(current_editing) = self.model.get_editing_state() {
                        self.mediator.notify(AppEvent::InputChanged(
                            value.to_string(),
                            match current_editing {
                                CurrentlyEditing::Key => InputField::Key,
                                CurrentlyEditing::Value => InputField::Value,
                            },
                        ));
                    }
                    // // 通知输入更新
                    // self.mediator.notify(AppEvent::InputChanged(
                    //     value.to_string(),
                    //     match current_editing {
                    //         CurrentlyEditing::Key => InputField::Key,
                    //         CurrentlyEditing::Value => InputField::Value,
                    //     },
                    // ));
                }

                EditAction::Enter => {
                    if let Some(current_editing) = self.model.get_editing_state() {
                        if let CurrentlyEditing::Value = current_editing {
                            self.mediator.notify(AppEvent::SaveRequested);
                            self.mediator
                                .notify(AppEvent::ScreenChanged(CurrentScreen::Main));
                            self.mediator.notify(AppEvent::EditingStateChanged(None));
                        }
                    }

                    // if let Some(CurrentlyEditing::Value) = current_editing {
                    //     self.mediator.notify(AppEvent::SaveRequested);
                    //     self.mediator
                    //         .notify(AppEvent::ScreenChanged(CurrentScreen::Main));
                    //     self.mediator.notify(AppEvent::EditingStateChanged(None));
                    // }
                }
                EditAction::Backspace => {}
                EditAction::Esc => {} // ... 其他 EditAction 处理
            },

            Action::ExitMode(key) => match key {
                KeyCode::Char('y') => {
                    self.mediator.notify(AppEvent::ExitRequested(true));
                }
                KeyCode::Char('n') | KeyCode::Char('q') => {
                    self.mediator.notify(AppEvent::ExitRequested(false));
                }
                _ => {}
            },

            Action::MainMode(list_action) => match list_action {
                // ListAction::Down => state.,
                _ => {}
            },
            // ... 其他 Action 处理
        }
    }
}
