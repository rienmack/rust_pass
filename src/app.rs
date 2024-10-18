use std::collections::HashMap;

pub enum CurrentScreen {
    Main,
    Editing,
    Login,
    Exiting,
}

pub enum CurrentlyEditing {
    Key,
    Value,
}

pub struct App {
    pub key_input: String,
    pub value_input: String,
    pub pairs: HashMap<String, String>,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
}

impl App {
    pub fn new() -> App {
        App {
            key_input: String::new(),
            value_input: String::new(),
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Login,
            currently_editing: None,
        }
    }

    pub fn save_key_value(&mut self) {
        self.pairs
            .insert(self.key_input.clone(), self.value_input.clone());

        self.key_input = String::new();
        self.value_input = String::new();
        self.currently_editing = None;
    }

    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            self.currently_editing = match edit_mode {
                CurrentlyEditing::Key => Some(CurrentlyEditing::Value),
                CurrentlyEditing::Value => Some(CurrentlyEditing::Key),
            }
        } else {
            self.currently_editing = Some(CurrentlyEditing::Key);
        }
    }
}
