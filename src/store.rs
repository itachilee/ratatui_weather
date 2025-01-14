use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

use crate::{
    model::model::Model,
    reducer::{reducer, Action},
};

pub struct Dispatcher {
    pub state: Arc<Mutex<Model>>,
    tx: mpsc::Sender<Action>,
}

impl Dispatcher {
    pub fn new(state: Arc<Mutex<Model>>) -> Self {
        let (tx, rx) = mpsc::channel();
        let state_clone = Arc::clone(&state);

        thread::spawn(move || {
            while let Ok(action) = rx.recv() {
                let mut state = state_clone.lock().unwrap();
                reducer(&mut *state, action);
            }
        });

        Dispatcher { state, tx }
    }

    pub fn dispatch(&self, action: Action) {
        self.tx.send(action).unwrap();
    }
}
