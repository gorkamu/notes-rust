use crate::{
    application::use_cases::notes::{
        create::CreateNewNote,
        delete::{self, DeletedNote},
        find_all::{self, FindAll},
    },
    domain::repositories::note_repository::NoteRepository,
};
use ansi_term::Colour;
use inquire::{
    Confirm, Editor, Select, Text,
    ui::{Color, RenderConfig, Styled},
};

pub struct DeletedNoteAction;

impl DeletedNoteAction {
    pub fn execute() -> bool {
        let note_repository: NoteRepository = NoteRepository::new();
        let find_all = FindAll::new(&note_repository);
        let notes = match find_all.execute() {
            Ok(notes) => notes,
            Err(e) => {
                return false;
            }
        };

        // Map notes into a vector of formatted strings for the Select component
        let options: Vec<String> = notes
            .iter()
            .map(|note| format!("{} - {}", note.id.unwrap_or_default(), note.title))
            .collect();

        // Prompt the user to select a note
        let selected_note = Select::new("Select a note to delete:", options).prompt();

        let selected_note = match selected_note {
            Ok(selection) => selection,
            Err(_) => {
                return false;
            }
        };

        // Extract the id from the selected note
        let id_str = selected_note.split(" - ").next().unwrap_or_default();

        let id: i64 = match id_str.parse() {
            Ok(parsed_id) => parsed_id,
            Err(_) => {
                return false;
            }
        };

        let message: String = format!("Are you sure you want to delete the note with ID: {}", id);
        let confirm = Confirm::new(&message).with_default(false).prompt().unwrap();

        if !confirm {
            return false;
        }

        let delete_note = DeletedNote::new(&note_repository);
        if let Err(err) = delete_note.execute(id) {
            println!("{} Failed to delete note: {}", Colour::Red.paint(">"), err);
        }

        return true;
    }
}
