use crate::domain::entities::note::Note;
use crate::domain::repositories::note_repository::NoteRepository;

pub struct UpdateNoteUseCase {
    note_repository: NoteRepository,
}

impl UpdateNoteUseCase {
    pub fn new(note_repository: NoteRepository) -> Self {
        UpdateNoteUseCase { note_repository }
    }

    pub fn execute(&self, id: i64, title: String, content: String) -> Result<Note, String> {
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

        note.set_title(title);
        note.set_content(content);      
        note.set_updated_at(
            chrono::Utc::now(),
        );  

        match self.note_repository.update(note) {
            updated_note => {
                println!("[+] Note updated successfully with id: {}", id);
                updated_note
            }
            e => Err(format!("Error updating note: {:?}", e)),
        }
    }
}