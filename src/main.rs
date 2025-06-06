mod domain;

use crate::domain::entities::note::Note;
use crate::domain::repositories::note_repository::NoteRepository;


fn main() {
    let note = Note::create(
        String::from("My First Note"),
        String::from("This is the content of my first note."),
    );

    let repository = NoteRepository::new();
    repository.save(&note).expect("Failed to save note");    
}
