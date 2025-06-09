use inquire::{ui::{Color, RenderConfig, Styled}, Confirm, Editor, Text};

use crate::{application::use_cases::notes::create::CreateNewNote, domain::repositories::note_repository::NoteRepository};

/// Represents the action of creating a new note through the CLI.
pub struct CreateNoteAction;

impl CreateNoteAction {
    /// Executes the process of creating a new note.
    /// 
    /// This method prompts the user for the note's title and content, confirms the action,
    /// and then attempts to save the note using the application logic.
    /// 
    /// # Returns
    /// - `true` if the note was successfully created.
    /// - `false` if the process was canceled or an error occurred.
    pub fn execute() -> bool {
        let title = Text::new("Title:").prompt();
        let title = match title {
            Ok(title) => title,
            Err(_) => {
                println!("An error occurred when asking for the title, try again later.");
                return false;
            }
        };

        let content = Editor::new("Content:")
            .with_formatter(&|submission| {
                let char_count = submission.chars().count();
                if char_count == 0 {
                    String::from("<skipped>")
                } else if char_count <= 20 {
                    submission.into()
                } else {
                    let mut substr: String = submission.chars().take(17).collect();
                    substr.push_str("...");
                    substr
                }
            })
            .with_render_config(CreateNoteAction::description_render_config())
            .prompt();

        let content = match content {
            Ok(content) => content,
            Err(_) => {
                println!("An error occurred when asking for the content, try again later.");
                return false;
            }
        };

        let confirm = Confirm::new("Save")
            .with_default(false)
            .prompt().unwrap();

        if !confirm {
            println!("Note creation canceled.");
            return false;
        }

        CreateNoteAction::create_note(&title, &content);

        return true;
    }

    /// Creates a new note using the provided title and content.
    /// 
    /// This method interacts with the `NoteRepository` and the `CreateNewNote` use case
    /// to persist the note.
    /// 
    /// # Arguments
    /// - `title`: The title of the note.
    /// - `content`: The content of the note.
    fn create_note(title: &String, content: &String) -> () {
        let note_repository: NoteRepository = NoteRepository::new();
        let create_use_case: CreateNewNote = CreateNewNote::new(&note_repository);

        if let Err(err) = create_use_case.execute(&title, &content) {
            println!("Failed to create note: {}", err);
        }
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