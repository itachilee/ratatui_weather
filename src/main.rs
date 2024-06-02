use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{
        self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use event::{Event, KeyCode, KeyEventKind};
use ratatui::{
    backend::{self, Backend, CrosstermBackend},
    Terminal,
};
use ratatui_waether::{model::Model, predule::*};
use ratatui_waether::{
    model::{CurrentScreen, CurrentlyEditing},
    reducer::{Action, EditAction},
    store::Dispatcher,
};
fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    enable_raw_mode()?;
    let mut stderr = std::io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let state = Arc::new(Mutex::new(Model::new()));
    let dispatcher = Dispatcher::new(Arc::clone(&state));
    run_app(&mut terminal, &state, &dispatcher);

    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // if let Ok(do_print) = res {
    //     if do_print {
    //         app.print_json()?;
    //     }
    // } else if let Err(err) = res {
    //     println!("{err:?}");
    // }

    Ok(())
}

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    state: &Arc<Mutex<Model>>,
    dispatcher: &Dispatcher,
    // ) -> std::io::Result<bool> {
) {
    loop {
        {
            let state = state.lock().unwrap();
            if state.should_exit {
                break;
            }
            terminal.draw(|f| ui(f, &*state)).unwrap();
        }

        if event::poll(Duration::from_millis(200)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                let state = state.lock().unwrap();
                if key.kind == event::KeyEventKind::Release {
                    continue;
                }

                match state.current_screen {
                    CurrentScreen::Main => match key.code {
                        KeyCode::Char('e') => dispatcher.dispatch(Action::ChangeToEditMode),

                        KeyCode::Char('q') => dispatcher.dispatch(Action::ChangeToExitMode),
                        // app.current_screen = CurrentScreen::Exiting;
                        _ => {}
                    },
                    CurrentScreen::Editing if key.kind == KeyEventKind::Press => match key.code {
                        KeyCode::Enter => {
                            dispatcher.dispatch(Action::EditMode(EditAction::Enter));
                        }
                        KeyCode::Backspace => {
                            dispatcher.dispatch(Action::EditMode(EditAction::Backspace));
                        }
                        KeyCode::Esc => {
                            dispatcher.dispatch(Action::EditMode(EditAction::Esc));
                        }
                        KeyCode::Tab => {
                            dispatcher.dispatch(Action::EditMode(EditAction::Tab));
                        }
                        KeyCode::Char(value) => {
                            dispatcher.dispatch(Action::EditMode(EditAction::Char(value)));
                        }
                        _ => {}
                    },
                    CurrentScreen::Exiting => dispatcher.dispatch(Action::ExitMode(key.code)),
                    _ => {}
                }
            }
        }
    }
}
