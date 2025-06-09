use crate::domain::entities::note::Note;
use crate::domain::repositories::note_repository::NoteRepository;

pub struct UpdateNote<'a> {
    note_repository: &'a NoteRepository,
}

///
/// The `UpdateNote` struct provides a use case for updating an existing note.
/// It interacts with the `NoteRepository` to perform the update operation and handle any errors related to the note's existence or input validation.
/// This use case is part of the application layer, which orchestrates the interaction between the domain entities and the user interface or other application components.
///
impl<'a> UpdateNote<'a> {
    /// 
    /// Creates a new instance of `UpdateNote`.
    /// 
    /// # Arguments
    /// * `note_repository`: A reference to an instance of `NoteRepository` to interact with the note storage.
    /// 
    /// # Returns
    /// A new `UpdateNote` instance.
    /// 
    /// # Example
    /// ```
    /// let note_repository = NoteRepository::new();
    /// let update_use_case = UpdateNote::new(&note_repository);
    /// ```
    /// 
    pub fn new(note_repository: &'a NoteRepository) -> Self {
        UpdateNote { note_repository }
    }

    /// 
    /// Executes the use case to update a note.
    /// 
    /// # Arguments
    /// * `id`: The ID of the note to be updated. Must be greater than 0.
    /// * `title`: A reference to a `String` containing the new title for the note. Must not be empty and cannot exceed 100 characters.
    /// * `content`: A reference to a `String` containing the new content for the note. Must not be empty and cannot exceed 1000 characters.
    /// 
    /// # Returns
    /// * `Ok(Note)`: If the note is successfully updated.
    /// * `Err(String)`: If there is an error during the update process, such as invalid input or note not found.
    /// 
    /// # Errors
    /// * Returns an error if the `id` is less than or equal to 0.
    /// * Returns an error if the `title` or `content` is empty.
    /// * Panics if the `title` exceeds 100 characters or the `content` exceeds 1000 characters.
    /// * Returns an error if the note with the specified `id` does not exist.
    /// * Returns an error if there is an issue updating the note in the repository.
    /// 
    /// # Example
    /// ```
    /// let id = 1;
    /// let title = String::from("Updated Title");
    /// let content = String::from("Updated content for the note.");
    /// match update_use_case.execute(id, &title, &content) {
    ///     Ok(note) => println!("Note updated successfully: {:?}", note),
    ///     Err(err) => println!("Failed to update note: {}", err),
    /// }
    /// ```
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
        note.set_updated_at(chrono::Utc::now());

        match self.note_repository.update(note) {
            updated_note => {
                println!("[+] Note updated successfully with id: {}", id);
                updated_note
            }
            e => Err(format!("Error updating note: {:?}", e)),
        }
    }
}