use crate::domain::entities::note::Note;
use crate::domain::repositories::note_repository::NoteRepository;

pub struct DeletedNoteUseCase {
    note_repository: NoteRepository,
}

impl DeletedNoteUseCase {
    pub fn new(note_repository: NoteRepository) -> Self {
        DeletedNoteUseCase { note_repository }
    }

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
