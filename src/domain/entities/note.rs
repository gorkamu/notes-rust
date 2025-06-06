use chrono::prelude::*;


/// Represents a note in the system.
/// A note consists of a title, content, and a timestamp indicating when it was created.
/// The title must be non-empty and up to 100 characters, while the content must be non-empty and up to 1000 characters.
/// The `id` field is optional and can be set when the note is created or updated.
#[derive(Debug)]
pub struct Note {
    /// Unique identifier for the note, optional for creation
    id: Option<String>,

    /// Title of the note, must be non-empty and up to 100 characters
    title: String,

    /// Content of the note, must be non-empty and up to 1000 characters
    content: String,

    /// Timestamp of when the note was created, automatically set to the current UTC time
    created_at: DateTime<Utc>,
}

impl Note {
    /// Creates a new Note instance with the provided title and content.
    ///
    /// # Arguments
    /// * `title` - The title of the note, must be non-empty and up to 100 characters.
    /// * `content` - The content of the note, must be non-empty and up to 1000 characters.
    /// # Returns
    /// A new `Note` instance with the current UTC timestamp.
    /// /// # Examples
    /// ```
    /// let note = Note::create(
    ///     String::from("My First Note"),
    ///     String::from("This is the content of my first note."),
    /// );
    /// assert_eq!(note.get_title(), "My First Note");
    /// assert_eq!(note.get_content(), "This is the content of my first note.");
    /// ```
    /// # Errors
    /// Panics if the title is empty, exceeds 100 characters, or if the content is empty or exceeds 1000 characters.
    pub fn create(title: String, content: String) -> Self {
        if title.is_empty() {
            panic!("Title cannot be empty");
        }

        if content.is_empty() {
            panic!("Content cannot be empty");
        }

        if title.len() > 100 {
            panic!("Title cannot exceed 100 characters");
        }

        if content.len() > 1000 {
            panic!("Content cannot exceed 1000 characters");
        }

        Note { id: None, title, content, created_at: Utc::now() }
    }

    /// Get the ID of the note.
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// A mutable reference to the `Note` instance.
    /// # Examples
    /// ```
    /// let mut note = Note::create(
    ///     String::from("My First Note"),
    ///     String::from("This is the content of my first note."),
    /// );
    /// note.get_id(); // Initially None
    /// assert_eq!(note.get_id(), None);
    /// ```    
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Get the Note title
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// A reference to the title of the note.
    /// # Examples
    /// ```
    /// let note = Note::create(
    ///     String::from("My First Note"),
    ///     String::from("This is the content of my first note."),
    /// );  
    /// assert_eq!(note.get_title(), "My First Note");
    /// ```
    pub fn get_title(&self) -> &String {
        &self.title
    }

    /// Get the content of the note
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// A reference to the content of the note.
    /// # Examples
    /// ```
    /// let note = Note::create(
    ///     String::from("My First Note"),
    ///     String::from("This is the content of my first note."),
    /// );
    /// assert_eq!(note.get_content(), "This is the content of my first note.");
    /// ```
    pub fn get_content(&self) -> &String {
        &self.content
    }

    /// Get the creation timestamp of the note
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// The creation timestamp of the note as a `DateTime<Utc>`.
    /// # Examples
    /// ```
    /// let note = Note::create(
    ///     String::from("My First Note"),
    ///     String::from("This is the content of my first note."),  
    /// );
    /// let created_at = note.get_created_at();
    /// assert!(created_at <= Utc::now());
    /// ```
    pub fn get_created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}