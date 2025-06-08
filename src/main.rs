#![allow(warnings)]

mod application;
mod domain;

use domain::repositories::note_repository::NoteRepository;

use application::use_cases::notes::create::CreateNewNote;
use application::use_cases::notes::find_by_id::FindById;
use application::use_cases::notes::find_by_title::FindByTitle;
use application::use_cases::notes::update::UpdateNote;
use application::use_cases::notes::delete::DeletedNote;

fn main() {    
    let note_repository: NoteRepository = NoteRepository::new();

    let title: String = String::from("El chinao picarral");
    let content: String = String::from("Como estamos jeje");

    // let create: CreateNewNote = CreateNewNote::new(&note_repository);
    // match create.execute(&title, &content) {
    //     Ok(_) => println!("[+] OK!"),
    //     Err(e) => eprintln!("[+] Error creating note: {}", e),
    // }

    // let find_by_id: FindById = FindById::new(&note_repository);
    // match find_by_id.execute(1) {
    //     Ok(_) => println!("[+] OK!"),
    //     Err(e) => eprintln!("[+] Error creating note: {}", e),
    // }

    // let update: UpdateNote = UpdateNote::new(&note_repository);
    // match update.execute(1, &title, &content) {
    //     Ok(_) => println!("[+] Note updated successfully!"),
    //     Err(e) => eprintln!("[+] Error updating note: {}", e),
    // }

    // let delete: DeletedNote = DeletedNote::new(&note_repository);
    // match delete.execute(1) {
    //     Ok(_) => println!("[+] Note deleted successfully!"),
    //     Err(e) => eprintln!("[+] Error deleting note: {}", e),  
    // }

    let find_by_title: FindByTitle = FindByTitle::new(&note_repository);
    match find_by_title.execute(&title) {
        Ok(note) => println!("[+] Note found: {:?}", note),
        Err(e) => eprintln!("[+] Error finding note by title: {}", e),
    }

}
