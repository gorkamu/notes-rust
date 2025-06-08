use crate::domain::entities::note::Note;
use crate::domain::repositories::note_repository::NoteRepository;

pub struct UpdateNoteUseCase<'a> {
    note_repository: &'a NoteRepository,
}

///
/// The `UpdateNoteUseCase` struct provides a use case for updating an existing note.
/// It interacts with the `NoteRepository` to perform the update operation and handle any errors related to the note's existence or input validation.
/// This use case is part of the application layer, which orchestrates the interaction between the domain entities and the user interface or other application components.
///
impl<'a> UpdateNoteUseCase<'a> {
    /// 
    /// Creates a new instance of `UpdateNoteUseCase`.
    /// # Arguments
    /// * `note_repository`: An instance of `NoteRepository` to interact with the note storage.
    /// # Returns
    /// A new `UpdateNoteUseCase` instance.
    /// 
    pub fn new(note_repository: &'a NoteRepository) -> Self {
        UpdateNoteUseCase { note_repository }
    }

    /// 
    /// Executes the use case to update a note.
    /// # Arguments
    /// * `id`: The ID of the note to be updated.
    /// * `title`: The new title for the note.
    /// * `content`: The new content for the note.
    /// # Returns
    /// * `Ok(Note)`: If the note is successfully updated.
    /// * `Err(String)`: If there is an error during the update process, such as invalid input or note not found.
    /// 
    pub fn execute(&self, id: i64, title: &String, content: &String) -> Result<Note, String> {
        if id <= 0 {
            return Err("Invalid note ID".to_string());
        }

        if title.is_empty() || content.is_empty() {
            return Err("Title and content cannot be empty".to_string());
        }

        if title.len() > 100 {
            panic!("Title cannot exceed 100 characters");
        }

        if content.len() > 1000 {
            panic!("Content cannot exceed 1000 characters");
        }

        let mut note = match self.note_repository.find_by_id(id) {
            Some(note) => {
                println!("[+] Note found with id: {}", id);
                note
            }
            None => return Err(format!("Note with id {} not found", id)),
        };

        note.set_title(title.clone());
        note.set_content(content.clone());      
        note.set_updated_at(
            chrono::Utc::now(),
        );  

        match self.note_repository.update(note) {
            updated_note => {
                println!("[+] Note updated successfully with id: {}", id);
                updated_note
            }
            e => Err(format!("Error updating note: {:?}", e)),
        }
    }
}