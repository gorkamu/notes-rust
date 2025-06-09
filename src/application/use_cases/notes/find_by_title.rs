use crate::domain::{entities::note::Note, repositories::note_repository::NoteRepository};

pub struct FindByTitle<'a> {
    note_repository: &'a NoteRepository,
}

///
/// The `FindByTitle` struct provides a use case for finding notes by their title.
/// It interacts with the `NoteRepository` to retrieve notes that match the given title and handles any errors related to the input or note existence.
///
/// This use case is part of the application layer, which orchestrates the interaction between the domain entities and the user interface or other application components.
///
impl<'a> FindByTitle<'a> {
    /// 
    /// Creates a new instance of `FindByTitle`.
    /// 
    /// # Arguments
    /// * `note_repository`: A reference to an instance of `NoteRepository` to interact with the note storage.
    /// 
    /// # Returns
    /// A new `FindByTitle` instance.
    /// 
    /// # Example
    /// ```
    /// let note_repository = NoteRepository::new();
    /// let find_by_title_use_case = FindByTitle::new(&note_repository);
    /// ```
    /// 
    pub fn new(note_repository: &'a NoteRepository) -> Self {
        FindByTitle { note_repository }
    }

    /// 
    /// Executes the use case to find notes by title.
    /// 
    /// # Arguments
    /// * `title`: A reference to a `String` containing the title of the notes to be found. Must not be empty.
    /// 
    /// # Returns
    /// A `Result` containing the found `Note` if successful, or an error message if no notes are found or the title is invalid.
    /// 
    /// # Errors
    /// * Returns an error if the `title` is empty.
    /// * Returns an error if no notes with the specified title are found.
    /// 
    /// # Example
    /// ```
    /// let title = String::from("Meeting Notes");
    /// match find_by_title_use_case.execute(&title) {
    ///     Ok(note) => println!("Found note: {:?}", note),
    ///     Err(err) => println!("Failed to find note: {}", err),
    /// }
    /// ```
    /// 
    pub fn execute(&self, title: &String) -> Result<Note, String> {
        if title.is_empty() {
            return Err("Title cannot be empty".to_string());
        }

        match self.note_repository.find_by_title(&title) {
            Some(note) => {
                println!("[+] Note: {:?}", note);
                Ok(note)
            }
            None => Err(format!("Note with title `{:?}` not found", &title)),
        }
    }
}