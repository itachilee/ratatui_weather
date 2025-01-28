use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::Backend,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
// use ratatui_weather::redis_db;
use tokio::{sync::mpsc, task};

#[derive(Debug, Clone)]
struct AppState {
    message: String,
    data: Option<String>,
    count: u32,
}

impl AppState {
    fn new() -> Self {
        AppState {
            message: "Press Enter to fetch data...".to_string(),
            data: None,
            count: 0,
        }
    }
}

static mut COUNT: i32 = 0;
async fn fetch_data() -> String {
    tokio::time::sleep(Duration::from_secs(2)).await;
    unsafe {
        COUNT += 1;
    }
    format!("demo data count: {}", unsafe { COUNT })
}

async fn app<B: Backend>(
    terminal: &mut Terminal<B>,
    state: Arc<Mutex<AppState>>,
    tx: mpsc::Sender<()>,
) -> std::io::Result<()> {
    loop {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    continue;
                }
                match key.code {
                    KeyCode::Enter => {
                        let tx = tx.clone();
                        let state = state.clone();
                        task::spawn(async move {
                            let data = fetch_data().await;

                            {
                                let mut state = state.lock().unwrap();
                                // println!("data: {}", data.clone());

                                state.data = Some(data);
                                state.message = "Data fetched successfully!".to_string();
                                // println!("data: {:?}", state);
                                state.count += 1;
                            }
                            tx.send(()).await.ok();
                            // let state = state.lock().unwrap();
                            // println!("send {}", state.count);
                        });
                    }
                    KeyCode::Char('q') => return Ok(()), // Quit
                    _ => {}
                }
            }
        }

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
                .split(f.area());

            let state = state.lock().unwrap();
            let paragraph = Paragraph::new(state.message.clone());
            let data_block = Paragraph::new(
                state
                    .data
                    .clone()
                    .unwrap_or_else(|| "No data yet.".to_string()),
            )
            .block(Block::default().borders(Borders::ALL).title("Data"));

            f.render_widget(paragraph, chunks[0]);
            f.render_widget(data_block, chunks[1]);
        })?;
    }
}

pub async fn run() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;

    let state = Arc::new(Mutex::new(AppState::new()));
    let (tx, mut rx) = mpsc::channel(1);

    let ui_state = state.clone();
    // let ui_tx = tx.clone();

    tokio::spawn(async move {
        let mut count = 0;
        while rx.recv().await.is_some() {
            count += 1;

            // println!("recv {}", count);
            // let _ = ui_tx.send(()).await;
        }
    });

    let res = app(&mut terminal, ui_state, tx.clone()).await;

    ratatui::restore();

    res
}
