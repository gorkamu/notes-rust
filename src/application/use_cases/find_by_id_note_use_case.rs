use crate::domain::entities::note::Note;
use crate::domain::repositories::note_repository::NoteRepository;

pub struct FindByIdNoteUseCase {
    note_repository: NoteRepository,
}

///
/// The `FindByIdNoteUseCase` struct provides a use case for finding a note by its ID.
/// It interacts with the `NoteRepository` to retrieve the note and handle any errors related to the ID or note existence.
///
/// This use case is part of the application layer, which orchestrates the interaction between the domain entities and the user interface or other application components.
///
impl FindByIdNoteUseCase {
    /// 
    /// Creates a new instance of `FindByIdNoteUseCase`.
    /// # Arguments
    /// * `note_repository` - An instance of `NoteRepository` to interact with the note storage.
    /// # Returns
    /// A new instance of `FindByIdNoteUseCase`.    
    /// 
    pub fn new(note_repository: NoteRepository) -> Self {
        FindByIdNoteUseCase { note_repository }
    }

    /// 
    /// Executes the use case to find a note by its ID.
    /// # Arguments
    /// * `id` - The ID of the note to find.
    /// # Returns
    /// A `Result` containing the found `Note` if successful, or an error message if the note is not found or the ID is invalid.
    /// # Errors
    /// Returns an error if the ID is less than or equal to zero, or if the note with the specified ID does not exist.
    /// 
    pub fn execute(&self, id: i64) -> Result<Note, String> {
        if id <= 0 {
            return Err("Invalid note ID".to_string());
        }

        match self.note_repository.find_by_id(id) {
            Some(note) => {
                println!("[+] Note: {:?}", note);
                Ok(note)
            }
            None => Err(format!("Note with id {} not found", id)),
        }
    }
}
