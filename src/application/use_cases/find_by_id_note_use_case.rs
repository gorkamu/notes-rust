use crate::domain::entities::note::Note;
use crate::domain::repositories::note_repository::NoteRepository;

pub struct FindByIdNoteUseCase {
    note_repository: NoteRepository,
}

impl FindByIdNoteUseCase {
    pub fn new(note_repository: NoteRepository) -> Self {
        FindByIdNoteUseCase { note_repository }
    }

    pub fn execute(&self, id: i64) -> Result<Note, String> {
        if id <= 0 {
            return Err("Invalid note ID".to_string());
        }

        match self.note_repository.find_by_id(id) {
            Some(note) => {
                println!("[+] Note found with id: {}", id);
                Ok(note)
            }
            None => Err(format!("Note with id {} not found", id)),
        }
    }
}
