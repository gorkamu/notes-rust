use inquire::{
    error::InquireResult, ui::{Color, RenderConfig, Styled}, Confirm, Editor, InquireError, Select, Text
};

use crate::infrastructure::ui::icli::actions::notes::{create::CreateNoteAction, delete::DeletedNoteAction, find::FindNoteAction};

enum ActionOptions {
    Create,
    Find,
    Update,
    Delete,
}

pub struct Presenter;

impl Presenter {
    pub fn new() -> Self {
        Presenter
    }

    pub fn execute(&self) {
        self.render_banner();

        let options: Vec<&str> = vec![
            "Create a new note",
            "Search by title",
            "Update a note",
            "Delete a note",
        ];

        let ans: Result<&str, InquireError> = Select::new("What do you want to do?", options.clone()).prompt();

        match ans {
            Ok(choice) => {
                match self.map_choice_to_action(choice, &options) {
                    Some(ActionOptions::Create) => CreateNoteAction::execute(),
                    Some(ActionOptions::Find) => FindNoteAction::execute(),
                    Some(ActionOptions::Update) => false,
                    Some(ActionOptions::Delete) => DeletedNoteAction::execute(),
                    None => false,
                };
            }
            Err(_) => println!("There was an error, please try again."),
        }
    }

    fn map_choice_to_action(&self, choice: &str, options: &[&str]) -> Option<ActionOptions> {
        match options.iter().position(|&opt| opt == choice) {
            Some(0) => Some(ActionOptions::Create),
            Some(1) => Some(ActionOptions::Find),
            Some(2) => Some(ActionOptions::Update),
            Some(3) => Some(ActionOptions::Delete),
            _ => None,
        }
    }

    fn render_banner(&self) {
        Presenter::clear_terminal();

        // Print the banner
        println!(r"

  _   _           _                  _ _   _____    _ _   _    _         _   
 | \ | |         | |                ( | ) |  __ \  ( | ) | |  | |       | |  
 |  \| |   ___   | |_    ___   ___   V V  | |__) |  V V  | |  | |  ___  | |_ 
 | . ` |  / _ \  | __|  / _ \ / __|       |  _  /        | |  | | / __| | __|
 | |\  | | (_) | | |_  |  __/ \__ \       | | \ \        | |__| | \__ \ | |_ 
 |_| \_|  \___/   \__|  \___| |___/       |_|  \_\        \____/  |___/  \__|
                                                                                                                                                                  
        ");
    }

    fn clear_terminal() {
        if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
            std::process::Command::new("clear")
                .status()
                .expect("Failed to clear terminal");
        } else if cfg!(target_os = "windows") {
            std::process::Command::new("cmd")
                .args(&["/C", "cls"])
                .status()
                .expect("Failed to clear terminal");
        }
    }
}