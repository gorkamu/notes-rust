use crate::{
    FindByTitle,
    application::use_cases::notes::find_by_id::{self, FindById},
    domain::repositories::note_repository::NoteRepository,
};
use ansi_term::Colour;
use inquire::{
    Confirm, Editor, Select, Text,
    ui::{Color, RenderConfig, Styled},
};

pub struct FindNoteAction;

impl FindNoteAction {
    pub fn execute() -> bool {
        let note_repository: NoteRepository = NoteRepository::new();
        let find_by_title = FindByTitle::new(&note_repository);
        let find_by_id = FindById::new(&note_repository);

        let title = Text::new("Title:").prompt();
        let title = match title {
            Ok(title) => title,
            Err(_) => {
                return false;
            }
        };

        let result = find_by_title.execute(&title);

        if let Err(err) = result {
            return false;
        }

        let notes = result.unwrap();

        if notes.is_empty() {
            return false;
        }

        // Map notes into a vector of formatted strings for the Select component
        let options: Vec<String> = notes
            .iter()
            .map(|note| format!("{} - {}", note.id.unwrap_or_default(), note.title))
            .collect();

        // Prompt the user to select a note
        let selected_note = Select::new("Select a note:", options).prompt();

        let selected_note = match selected_note {
            Ok(selection) => selection,
            Err(_) => {
                return false;
            }
        };

        // Extract the id from the selected note
        let id_str = selected_note.split(" - ").next().unwrap_or_default();

        let id: i64 = match id_str.parse() {
            Ok(parsed_id) => parsed_id,
            Err(_) => {
                return false;
            }
        };

        // Execute the find by ID use case
        let current_note = find_by_id.execute(id);
        if let Err(err) = current_note {
            return false;
        }

        let note = current_note.unwrap();

        // Display note content on inquire Editor
        let content = Editor::new("Content:")
            .with_render_config(FindNoteAction::description_render_config())
            .with_predefined_text(&note.content)
            .prompt();

        return true;
    }

    /// Provides a custom render configuration for the description editor.
    ///
    /// This configuration customizes the appearance of the editor's prompt
    /// when the user cancels the input.
    ///
    /// # Returns
    /// A `RenderConfig` instance with the desired customization.
    fn description_render_config() -> RenderConfig<'static> {
        RenderConfig::default()
            .with_canceled_prompt_indicator(Styled::new("<skipped>").with_fg(Color::DarkYellow))
    }
}
