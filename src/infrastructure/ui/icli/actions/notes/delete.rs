use inquire::{ui::{Color, RenderConfig, Styled}, Confirm, Editor, Select, Text};

use crate::{application::use_cases::notes::{create::CreateNewNote, delete::{self, DeletedNote}, find_all::{self, FindAll}}, domain::repositories::note_repository::NoteRepository};

pub struct DeletedNoteAction;

impl DeletedNoteAction {
    pub fn execute() -> bool {
        let note_repository: NoteRepository = NoteRepository::new();
        let find_all = FindAll::new(&note_repository);
        let notes = match find_all.execute() {
            Ok(notes) => notes,
            Err(e) => {
                println!("Error fetching notes: {}", e);
                return false;
            }
        };

        // Map notes into a vector of formatted strings for the Select component
        let options: Vec<String> = notes.iter()
            .map(|note| format!("{} - {}", note.id.unwrap_or_default(), note.title))
            .collect();

        // Prompt the user to select a note
        let selected_note = Select::new("Select a note to delete:", options)
            .prompt();

        let selected_note = match selected_note {
            Ok(selection) => selection,
            Err(_) => {
                println!("An error occurred while selecting a note.");
                return false;
            }
        };

        // Extract the id from the selected note
        let id_str = selected_note.split(" - ").next().unwrap_or_default();

        let id: i64 = match id_str.parse() {
            Ok(parsed_id) => parsed_id,
            Err(_) => {
                println!("Invalid note ID: {}", id_str);
                return false;
            }
        };

        let message: String = format!("Are you sure you want to delete the note with ID: {}", id);
        let confirm = Confirm::new(&message)
            .with_default(false)
            .prompt().unwrap();

        if !confirm {
            println!("Note deletion canceled.");
            return false;
        }

        let delete_note = DeletedNote::new(&note_repository);
        if let Err(err) = delete_note.execute(id) {
            println!("Failed to delete note: {}", err);
        }
        
        return true;
    }
}