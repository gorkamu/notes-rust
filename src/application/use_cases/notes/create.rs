use crate::domain::{entities::note::Note, repositories::note_repository::NoteRepository};

pub struct CreateNewNote<'a>  {
    note_repository: &'a NoteRepository,
}

///
/// The `CreateNewNoteUseCase` struct provides a use case for creating a new note.
/// It encapsulates the logic for creating a note and interacting with the `NoteRepository`.
/// 
impl <'a> CreateNewNote<'a>  {
    /// 
    /// Creates a new instance of `CreateNewNoteUseCase`.
    /// # Arguments
    /// * `note_repository`: An instance of `NoteRepository` to interact with the note storage.
    /// # Returns
    /// A new `CreateNewNoteUseCase` instance.
    /// # Example
    /// ```
    /// let note_repository = NoteRepository::new();
    /// let create_use_case = CreateNewNoteUseCase::new(note_repository);
    /// ```
    /// 
    pub fn new(note_repository: &'a NoteRepository) -> Self {
        CreateNewNote { note_repository }
    }

    /// 
    /// Executes the use case to create a new note.
    /// # Arguments
    /// * `title`: The title of the note to be created.
    /// * `content`: The content of the note to be created.
    /// # Returns
    /// A `Result` containing the created `Note` on success, or an error message on failure.    
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
