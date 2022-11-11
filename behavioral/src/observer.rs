use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Event {
    Load,
    Save,
}

pub type Subscriber = fn(file_path: String);

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

    pub fn notify(&self, event_type: Event, file_path: String) {
        let listeners = self.events.get(&event_type).unwrap();
        for listener in listeners {
            listener(file_path.clone());
        }
    }
}


#[derive(Default)]
pub struct Editor {
    publisher: Publisher,
    file_path: String,
}

impl Editor {
    pub fn events(&mut self) -> &mut Publisher {
        &mut self.publisher
    }

    pub fn load(&mut self, path: String) {
        self.file_path = path.clone();
        self.publisher.notify(Event::Load, path);
    }

    pub fn save(&self) {
        self.publisher.notify(Event::Save, self.file_path.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_observer() {
        let mut editor = Editor::default();

        editor.events().subscribe(Event::Load, |file_path| {
            let log = "~/dev/minor/design-pattern/behavioral/src/text.txt".to_string();
            println!("Save log to {}: Load file {}", log, file_path);
        });

        editor.events().subscribe(Event::Save, save_listener);

        editor.load("test1.txt".into());
        editor.load("test2.txt".into());
        editor.save();

        editor.events().unsubscribe(Event::Save, save_listener);
        editor.save();
    }

    fn save_listener(file_path: String) {
        let email = "admin@example.com".to_string();
        println!("Email to {}: Save file {}", email, file_path);
    }
}
