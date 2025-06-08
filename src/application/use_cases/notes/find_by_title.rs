use crate::domain::{entities::note::Note, repositories::note_repository::NoteRepository};

pub struct FindByTitle<'a> {
    note_repository: &'a NoteRepository,
}

impl<'a> FindByTitle<'a> {
    /// 
    /// Creates a new instance of `FindByTitle`.
    /// # Arguments
    /// * `note_repository`: An instance of `NoteRepository` to interact with the note storage.
    /// # Returns
    /// A new `FindByTitle` instance.
    /// 
    pub fn new(note_repository: &'a NoteRepository) -> Self {
        FindByTitle { note_repository }
    }

    /// 
    /// Executes the use case to find notes by title.
    /// # Arguments
    /// * `title`: The title of the notes to be found.
    /// # Returns
    /// A `Result` containing a vector of `Note` on success, or an error message on failure.
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