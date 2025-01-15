use std::collections::HashMap;

#[derive(Default)]
pub struct Editor {
    publisher: Publisher,
    file_path: String,
}

impl Editor {
    fn events(&mut self) -> &mut Publisher {
        &mut self.publisher
    }

    fn load(&mut self, path: String) {
        self.file_path = path.clone();
        self.publisher.notify(Event::Load, path);
    }

    fn save(&mut self) {
        self.publisher.notify(Event::Save, self.file_path.clone());
    }
}

#[derive(Eq, Hash, PartialEq, Clone)]
enum Event {
    Load,
    Save,
}

pub type Subscriber = fn(String);

#[derive(Default)]
pub struct Publisher {
    events: HashMap<Event, Vec<Subscriber>>,
}

impl Publisher {
    pub fn subscribe(&mut self, event_type: Event, listener: Subscriber) {
        self.events.entry(event_type.clone()).or_default();
        self.events.get_mut(&event_type).unwrap().push(listener);
    }

    pub fn unsubscribe(&mut self, event_type: Event, listener: Subscriber) {
        self.events
            .get_mut(&event_type)
            .unwrap()
            .retain(|&x| x != listener);
    }

    fn notify(&mut self, event_type: Event, file_path: String) {
        let listeners = self.events.get(&event_type).unwrap();
        for listener in listeners {
            listener(file_path.clone());
        }
    }
}

fn test_observer() {
    let mut editor = Editor::default();

    editor.events().subscribe(Event::Load, load_listener);

    editor.events().subscribe(Event::Save, save_listener);

    editor.load("test1.txt".into());
    editor.load("test2.txt".into());
    editor.save();

    editor.events().unsubscribe(Event::Save, save_listener);
    editor.save();
}

fn load_listener(file_path: String) {
    let log = "/path/to/log/file.txt".to_string();
    println!("Save log to {}: Load file {}", log, file_path);
}

fn save_listener(file_path: String) {
    let email = "admin@example.com".to_string();
    println!("Email to {}: Save file {}", email, file_path);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        test_observer();
    }
}
