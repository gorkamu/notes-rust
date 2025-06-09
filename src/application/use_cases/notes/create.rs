use crate::domain::{entities::note::Note, repositories::note_repository::NoteRepository};

pub struct CreateNewNote<'a>  {
    note_repository: &'a NoteRepository,
}

///
/// The `CreateNewNote` struct provides a use case for creating a new note.
/// It encapsulates the logic for creating a note and interacting with the `NoteRepository`.
///
impl<'a> CreateNewNote<'a> {
    /// 
    /// Creates a new instance of `CreateNewNote`.
    /// 
    /// # Arguments
    /// * `note_repository`: A reference to an instance of `NoteRepository` to interact with the note storage.
    /// 
    /// # Returns
    /// A new `CreateNewNote` instance.
    /// 
    /// # Example
    /// ```
    /// let note_repository = NoteRepository::new();
    /// let create_use_case = CreateNewNote::new(&note_repository);
    /// ```
    /// 
    pub fn new(note_repository: &'a NoteRepository) -> Self {
        CreateNewNote { note_repository }
    }

    /// 
    /// Executes the use case to create a new note.
    /// 
    /// # Arguments
    /// * `title`: A reference to a `String` containing the title of the note to be created.
    /// * `content`: A reference to a `String` containing the content of the note to be created.
    /// 
    /// # Returns
    /// A `Result` containing the created `Note` on success, or an error message on failure.
    /// 
    /// # Example
    /// ```
    /// let title = String::from("My Note");
    /// let content = String::from("This is the content of my note.");
    /// let result = create_use_case.execute(&title, &content);
    /// match result {
    ///     Ok(note) => println!("Note created with ID: {}", note.get_id().unwrap()),
    ///     Err(err) => println!("Failed to create note: {}", err),
    /// }
    /// ```
    /// 
    pub fn execute(&self, title: &String, content: &String) -> Result<Note, String> {
        let mut note = Note::create(title, content);

        let id: i64 = self
            .note_repository
            .save(&note)
            .map_err(|e| e.to_string())?;

        println!("[+] Note created with id: '{}'", id);

        note.set_id(id);

        Ok(note)
    }
}