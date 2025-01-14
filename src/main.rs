use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use ratatui_weather::model::{
    mediator,
    reducer::{Action, EditAction, ListAction},
    CurrentScreen,
};
use ratatui_weather::{
    model::{mediator::AppMediator, model::Model, reducer::ActionReducer},
    predule::*,
};

use ratatui_weather::model::mediator::Mediator;
fn main() -> std::io::Result<()> {
    println!("starting weather app...");

    let mut terminal = ratatui::init();

    let mut mediator = Box::new(AppMediator::new());
    let model = Model::new();
    let model = Box::new(model);
    mediator.register_listener(model.clone());
    let mut reducer = ActionReducer::new(mediator, &model);

    let res = run_app(&mut terminal, &mut reducer);
    ratatui::restore();

    Ok(())
}

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    reducer: &mut ActionReducer,
) -> std::io::Result<bool> {
    loop {
        {
            let state = reducer.get_model();
            let state = state.as_ref();
            if state.should_exit {
                return Ok(state.should_print);
            }
            terminal.draw(|f| ui(f, state))?;
        }

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                println!("Received key event: {:?}", key); // 添加日志
                if key.kind == event::KeyEventKind::Release {
                    continue;
                }

                let current_screen = &reducer.get_model().current_screen;
                match current_screen {
                    CurrentScreen::Main => match key.code {
                        KeyCode::Char('e') => {
                            println!("Triggering edit screen"); // 添加日志
                            reducer.reduce(Action::ChangeScreen(CurrentScreen::Editing));
                        }
                        KeyCode::Char('q') => {
                            reducer.reduce(Action::ChangeScreen(CurrentScreen::Exiting));
                        }
                        KeyCode::Down => reducer.reduce(Action::MainMode(ListAction::Down)),
                        _ => {}
                    },
                    CurrentScreen::Editing if key.kind == KeyEventKind::Press => match key.code {
                        KeyCode::Enter => {
                            reducer.reduce(Action::EditMode(EditAction::Enter));
                        }
                        KeyCode::Backspace => {
                            reducer.reduce(Action::EditMode(EditAction::Backspace));
                        }
                        KeyCode::Esc => {
                            reducer.reduce(Action::EditMode(EditAction::Esc));
                        }
                        KeyCode::Tab => {
                            reducer.reduce(Action::EditMode(EditAction::Tab));
                        }
                        KeyCode::Char(value) => {
                            reducer.reduce(Action::EditMode(EditAction::Char(value)));
                        }
                        _ => {}
                    },
                    CurrentScreen::Exiting => reducer.reduce(Action::ExitMode(key.code)),
                    _ => {}
                }
            }
        }
    }
}
