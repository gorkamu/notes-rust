use crate::domain::{entities::note::Note, repositories::note_repository::NoteRepository};

pub struct FindAll<'a> {
    note_repository: &'a NoteRepository,
}

impl<'a> FindAll<'a> {
    /// Creates a new instance of `FindAll`.
    ///
    /// # Arguments
    /// * `note_repository`: A reference to an instance of `NoteRepository` to interact with the note storage.
    ///
    /// # Returns
    /// A new `FindAll` instance.
    ///
    pub fn new(note_repository: &'a NoteRepository) -> Self {
        FindAll { note_repository }
    }

    /// Executes the use case to find all notes.
    ///
    /// # Returns
    /// A `Result` containing a vector of found `Note` instances if successful, or an error message if no notes are found.
    ///
    pub fn execute(&self) -> Result<Vec<Note>, String> {
        match self.note_repository.find_all() {
            Some(notes) => {                
                Ok(notes)
            }
            None => Err("No notes found".to_string()),
        }
    }
}