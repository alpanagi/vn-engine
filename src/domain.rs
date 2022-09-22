use std::collections::VecDeque;

use bevy::prelude::*;

use crate::interpreter::Command;
use crate::script_loader::Script;

#[derive(Clone)]
pub struct Screen {
    pub background: String,
    pub speaker: Option<String>,
    pub text: String,
}

pub struct GameState {
    pub unprocessed_scripts: Vec<Handle<Script>>,
    pub commands: VecDeque<Command>,
    pub history: Vec<(Command, Screen)>,
}
impl Default for GameState {
    fn default() -> Self {
        GameState {
            unprocessed_scripts: vec![],
            commands: VecDeque::new(),
            history: vec![],
        }
    }
}

impl GameState {
    pub fn current_screen(&self) -> Option<Screen> {
        self.history.last().map(|x| x.1.clone())
    }

    pub fn forward(&mut self) {
        let screen = self.current_screen().unwrap_or(Screen {
            background: String::new(),
            speaker: Some(String::new()),
            text: String::new(),
        });

        if let Some(command) = self.commands.pop_front() {
            self.history
                .push((command.clone(), self.run_command(&command, &screen)));
        }
    }

    fn run_command(&self, command: &Command, screen: &Screen) -> Screen {
        match command {
            Command::DisplayBackground { image_path } => Screen {
                background: image_path.clone(),
                speaker: screen.speaker.clone(),
                text: screen.text.clone(),
            },
            Command::Text { speaker, text } => Screen {
                background: screen.background.clone(),
                speaker: speaker.clone(),
                text: text.clone(),
            },
        }
    }

    pub fn backwards(&mut self) {
        if let Some(history_event) = self.history.last() {
            return {
                self.commands.insert(0, history_event.0.clone());
                self.history.pop();
            };
        }
    }
}
