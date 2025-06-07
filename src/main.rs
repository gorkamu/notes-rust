#![allow(warnings)]

mod application;
mod domain;

use application::use_cases::find_by_id_note_use_case::FindByIdNoteUseCase;
use domain::repositories::note_repository::NoteRepository;

fn main() {
    let title: String = String::from("Hola jeje");
    let content: String = String::from("Esta es la primera nota :)");

    let note_repository: NoteRepository = NoteRepository::new();
    // let create_use_case: CreateNewNoteUseCase = CreateNewNoteUseCase::new(note_repository);

    // match create_use_case.execute(title, content) {
    //     Ok(_) => println!("[+] OK!"),
    //     Err(e) => eprintln!("[+] Error creating note: {}", e),
    // }

    let find_use_case: FindByIdNoteUseCase = FindByIdNoteUseCase::new(note_repository);

    match find_use_case.execute(1) {
        Ok(_) => println!("[+] OK!"),
        Err(e) => eprintln!("[+] Error creating note: {}", e),
    }
}
