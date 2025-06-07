#![allow(warnings)]

mod application;
mod domain;

use application::use_cases::find_by_id_note_use_case::FindByIdNoteUseCase;
use domain::repositories::note_repository::NoteRepository;
use crate::application::use_cases::update_note_use_case::UpdateNoteUseCase;
use crate::application::use_cases::delete_note_use_case::DeletedNoteUseCase;

fn main() {
    let title: String = String::from("---------");
    let content: String = String::from(".....BBBBBB Esta es la primera nota :)");

    let note_repository: NoteRepository = NoteRepository::new();
    // let create_use_case: CreateNewNoteUseCase = CreateNewNoteUseCase::new(note_repository);

    // match create_use_case.execute(title, content) {
    //     Ok(_) => println!("[+] OK!"),
    //     Err(e) => eprintln!("[+] Error creating note: {}", e),
    // }

    // let find_use_case: FindByIdNoteUseCase = FindByIdNoteUseCase::new(note_repository);
    // match find_use_case.execute(1) {
    //     Ok(_) => println!("[+] OK!"),
    //     Err(e) => eprintln!("[+] Error creating note: {}", e),
    // }

    // let update_use_case: UpdateNoteUseCase = UpdateNoteUseCase::new(note_repository);
    // match update_use_case.execute(1, title, content) {
    //     Ok(_) => println!("[+] Note updated successfully!"),
    //     Err(e) => eprintln!("[+] Error updating note: {}", e),
    // }

    let delete_use_case: DeletedNoteUseCase = DeletedNoteUseCase::new(note_repository);
    match delete_use_case.execute(1) {
        Ok(_) => println!("[+] Note deleted successfully!"),
        Err(e) => eprintln!("[+] Error deleting note: {}", e),  
    }
}
