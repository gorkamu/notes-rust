use crate::domain::{entities::note::Note, repositories::note_repository::NoteRepository};

pub struct DeletedNote<'a> {
    note_repository: &'a NoteRepository,
}

///
/// The `DeletedNote` struct provides a use case for deleting a note.
/// It encapsulates the logic for deleting a note and interacting with the `NoteRepository`.
/// This use case is responsible for validating the input, ensuring that the note exists, and handling any errors
/// that may occur during the deletion process, such as invalid input or database errors.
///
impl<'a> DeletedNote<'a> {
    /// 
    /// Creates a new instance of `DeletedNote`.
    /// 
    /// # Arguments
    /// * `note_repository`: A reference to an instance of `NoteRepository` to interact with the note storage.
    /// 
    /// # Returns
    /// A new `DeletedNote` instance.
    /// 
    /// # Example
    /// ```
    /// let note_repository = NoteRepository::new();
    /// let delete_use_case = DeletedNote::new(&note_repository);
    /// ```
    /// 
    pub fn new(note_repository: &'a NoteRepository) -> Self {
        DeletedNote { note_repository }
    }

    ///
    /// Executes the use case to delete a note.
    /// 
    /// # Arguments
    /// * `id`: The ID of the note to be deleted. Must be greater than 0.
    /// 
    /// # Returns
    /// * `Ok(())`: If the note is successfully deleted.
    /// * `Err(String)`: If there is an error during the deletion process, such as invalid input or note not found.
    /// 
    /// # Example
    /// ```
    /// let id = 1; // ID of the note to delete
    /// match delete_use_case.execute(id) {
    ///     Ok(()) => println!("Note deleted successfully."),
    ///     Err(err) => println!("Failed to delete note: {}", err),
    /// }
    /// ```
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