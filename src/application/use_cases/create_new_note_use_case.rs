use crate::domain::entities::note::Note;
use crate::domain::repositories::note_repository::NoteRepository;

pub struct CreateNewNoteUseCase {
    note_repository: NoteRepository,
}

impl CreateNewNoteUseCase {
    pub fn new(note_repository: NoteRepository) -> Self {
        CreateNewNoteUseCase { note_repository }
    }

    pub fn execute(&self, title: String, content: String) -> Result<Note, String> {
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
