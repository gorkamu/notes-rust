#![allow(warnings)]

mod application;
mod domain;

use application::use_cases::find_by_id_note_use_case::FindByIdNoteUseCase;
use domain::repositories::note_repository::NoteRepository;
use crate::application::use_cases::create_new_note_use_case::CreateNewNoteUseCase;
use crate::application::use_cases::update_note_use_case::UpdateNoteUseCase;
use crate::application::use_cases::delete_note_use_case::DeletedNoteUseCase;
use crate::application::use_cases::notes::find_by_title::FindByTitle;

fn main() {
    let title: String = String::from("El chinao picarral");
    let content: String = String::from("Como estamos jeje");

    let note_repository: NoteRepository = NoteRepository::new();

    // let create_use_case: CreateNewNoteUseCase = CreateNewNoteUseCase::new(&note_repository);
    // match create_use_case.execute(&title, &content) {
    //     Ok(_) => println!("[+] OK!"),
    //     Err(e) => eprintln!("[+] Error creating note: {}", e),
    // }

    // let find_use_case: FindByIdNoteUseCase = FindByIdNoteUseCase::new(&note_repository);
    // match find_use_case.execute(1) {
    //     Ok(_) => println!("[+] OK!"),
    //     Err(e) => eprintln!("[+] Error creating note: {}", e),
    // }

    // let update_use_case: UpdateNoteUseCase = UpdateNoteUseCase::new(&note_repository);
    // match update_use_case.execute(1, &title, &content) {
    //     Ok(_) => println!("[+] Note updated successfully!"),
    //     Err(e) => eprintln!("[+] Error updating note: {}", e),
    // }

    // let delete_use_case: DeletedNoteUseCase = DeletedNoteUseCase::new(&note_repository);
    // match delete_use_case.execute(1) {
    //     Ok(_) => println!("[+] Note deleted successfully!"),
    //     Err(e) => eprintln!("[+] Error deleting note: {}", e),  
    // }

    let find_by_title_use_case: FindByTitle = FindByTitle::new(&note_repository);
    match find_by_title_use_case.execute(&title) {
        Ok(note) => println!("[+] Note found: {:?}", note),
        Err(e) => eprintln!("[+] Error finding note by title: {}", e),
    }

}
