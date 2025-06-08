use crate::domain::{entities::note::Note, repositories::note_repository::NoteRepository};

pub struct DeletedNote<'a> {
    note_repository: &'a NoteRepository,
}

///
/// The `DeletedNoteUseCase` struct provides a use case for deleting a note.
/// It encapsulates the logic for deleting a note and interacting with the `NoteRepository`.
/// This use case is responsible for validating the input and ensuring that the note exists before attempting to delete it.
/// It also handles any errors that may occur during the deletion process, such as invalid input or note not found.
///
impl<'a> DeletedNote<'a> {
    /// 
    /// Creates a new instance of `DeletedNoteUseCase`.
    /// # Arguments
    /// * `note_repository`: An instance of `NoteRepository` to interact with the note storage.
    /// # Returns
    /// A new `DeletedNoteUseCase` instance.
    /// 
    pub fn new(note_repository: &'a NoteRepository) -> Self {
        DeletedNote { note_repository }
    }

    ///
    /// Executes the use case to delete a note.
    /// # Arguments
    /// * `id`: The ID of the note to be deleted.
    /// # Returns
    /// * `Ok(())`: If the note is successfully deleted.
    /// * `Err(String)`: If there is an error during the deletion process, such as invalid input or note not found.
    /// 
    pub fn execute(&self, id: i64) -> Result<(), String> {
        if id <= 0 {
            return Err("Invalid note ID".to_string());
        }

        match self.note_repository.delete(id) {
            Ok(()) => {
                println!("[+] Deleted Note: {}", id);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }
}
