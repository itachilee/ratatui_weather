use crate::model::event::AppEvent;
use crate::model::event::EventListener;

pub trait Mediator {
    fn notify(&mut self, event: AppEvent);
    fn register_listener(&mut self, listener: Box<dyn EventListener>);
}

pub struct AppMediator {
    listeners: Vec<Box<dyn EventListener>>,
}

impl AppMediator {
    pub fn new() -> Self {
        Self {
            listeners: Vec::new(),
        }
    }
}

impl Mediator for AppMediator {
    fn notify(&mut self, event: AppEvent) {
        for listener in &mut self.listeners {
            listener.on_event(event.clone());
        }
    }

    fn register_listener(&mut self, listener: Box<dyn EventListener>) {
        self.listeners.push(listener);
    }
}
