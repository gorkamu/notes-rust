use crate::domain::entities::note::Note;
use crate::domain::repositories::note_repository::NoteRepository;

pub struct FindById<'a> {
    note_repository: &'a NoteRepository,
}

///
/// The `FindById` struct provides a use case for finding a note by its ID.
/// It interacts with the `NoteRepository` to retrieve the note and handle any errors related to the ID or note existence.
///
/// This use case is part of the application layer, which orchestrates the interaction between the domain entities and the user interface or other application components.
///
impl<'a> FindById<'a> {
    /// 
    /// Creates a new instance of `FindById`.
    /// 
    /// # Arguments
    /// * `note_repository` - A reference to an instance of `NoteRepository` to interact with the note storage.
    /// 
    /// # Returns
    /// A new instance of `FindById`.
    /// 
    /// # Example
    /// ```
    /// let note_repository = NoteRepository::new();
    /// let find_by_id_use_case = FindById::new(&note_repository);
    /// ```
    /// 
    pub fn new(note_repository: &'a NoteRepository) -> Self {
        FindById { note_repository }
    }

    /// 
    /// Executes the use case to find a note by its ID.
    /// 
    /// # Arguments
    /// * `id` - The ID of the note to find. Must be greater than 0.
    /// 
    /// # Returns
    /// A `Result` containing the found `Note` if successful, or an error message if the note is not found or the ID is invalid.
    /// 
    /// # Errors
    /// * Returns an error if the ID is less than or equal to zero.
    /// * Returns an error if the note with the specified ID does not exist.
    /// 
    /// # Example
    /// ```
    /// let id = 1; // ID of the note to find
    /// match find_by_id_use_case.execute(id) {
    ///     Ok(note) => println!("Found note: {:?}", note),
    ///     Err(err) => println!("Failed to find note: {}", err),
    /// }
    /// ```
    /// 
    pub fn execute(&self, id: i64) -> Result<Note, String> {
        if id <= 0 {
            return Err("Invalid note ID".to_string());
        }

        match self.note_repository.find_by_id(id) {
            Some(note) => {
                Ok(note)
            }
            None => Err(format!("Note with id {} not found", id)),
        }
    }
}