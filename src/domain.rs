use std::collections::{HashMap, VecDeque};

use bevy::prelude::*;

use crate::interpreter::Command;
use crate::script_loader::Script;

#[derive(Clone)]
pub struct Screen {
    pub background: String,
    pub speaker: Option<String>,
    pub speaker_color: Color,
    pub text: String,
}

pub type GameData = HashMap<String, HashMap<String, String>>;

pub struct GameState {
    pub data: GameData,
    pub unprocessed_scripts: Vec<Handle<Script>>,
    pub commands: VecDeque<Command>,
    pub history: Vec<(Command, Screen)>,
}
impl Default for GameState {
    fn default() -> Self {
        GameState {
            data: HashMap::new(),
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
            speaker_color: Color::WHITE,
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
                speaker_color: Color::WHITE,
                text: screen.text.clone(),
            },
            Command::CharacterText { speaker, text } => {
                let speaker_name: String = match self.data.get(speaker) {
                    Some(data) => data.get("name").unwrap_or(&speaker).to_owned(),
                    None => speaker.clone(),
                };

                let speaker_color: String = self
                    .data
                    .get(speaker)
                    .and_then(|data| data.get("color"))
                    .map(Clone::clone)
                    .unwrap_or("".to_string());

                let speaker_color: Color = match speaker_color.as_str() {
                    "blue" => Color::BLUE,
                    "red" => Color::RED,
                    _ => Color::WHITE,
                };

                Screen {
                    background: screen.background.clone(),
                    speaker: Some(speaker_name),
                    speaker_color: speaker_color,
                    text: text.clone(),
                }
            }
            Command::Text { speaker, text } => Screen {
                background: screen.background.clone(),
                speaker: speaker.clone(),
                speaker_color: Color::WHITE,
                text: text.clone(),
            },
        }
    }

    pub fn backwards(&mut self) {
        if self.history.len() == 1 {
            return;
        }

        if let Some(history_event) = self.history.pop() {
            return {
                self.commands.push_front(history_event.0.clone());
            };
        }
    }
}
