#![allow(warnings)]

mod application;
mod domain;
mod infrastructure;

use domain::repositories::note_repository::NoteRepository;

use application::use_cases::notes::create::CreateNewNote;
use application::use_cases::notes::find_by_id::FindById;
use application::use_cases::notes::find_by_title::FindByTitle;
use application::use_cases::notes::update::UpdateNote;
use application::use_cases::notes::delete::DeletedNote;

use infrastructure::ui::icli::presenter::Presenter;

fn main() {
    let presenter = Presenter::new();
    presenter.execute();
}
