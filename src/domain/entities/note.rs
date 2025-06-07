use chrono::prelude::*;

/// Represents a note in the system.
/// A note consists of a title, content, and a timestamp indicating when it was created.
/// The title must be non-empty and up to 100 characters, while the content must be non-empty and up to 1000 characters.
/// The `id` field is optional and can be set when the note is created or updated.
#[derive(Debug, Clone)]
pub struct Note {
    /// Unique identifier for the note, optional for creation
    pub(crate) id: Option<i64>,

    /// Title of the note, must be non-empty and up to 100 characters
    pub(crate) title: String,

    /// Content of the note, must be non-empty and up to 1000 characters
    pub(crate) content: String,

    /// Timestamp of when the note was created, automatically set to the current UTC time
    pub(crate) created_at: DateTime<Utc>,

    /// Timestamp of when the note was last updated, automatically set to the current UTC time
    pub(crate) updated_at: DateTime<Utc>,
}

impl Note {
    /// Creates a new Note instance with the provided title and content.
    ///
    /// # Arguments
    /// * `title` - The title of the note, must be non-empty and up to 100 characters.
    /// * `content` - The content of the note, must be non-empty and up to 1000 characters.
    /// # Returns
    /// A new `Note` instance with the current UTC timestamp.
    /// # Examples
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

        Note {
            id: None,
            title,
            content,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Creates a new `Note` instance from primitive values.
    /// # Arguments
    /// * `id` - The unique identifier for the note, can be `None` for new notes.
    /// * `title` - The title of the note, must be non-empty and up to 100 characters.
    /// * `content` - The content of the note, must be non-empty and up to 1000 characters.
    /// * `created_at` - The timestamp of when the note was created, typically set to the current UTC time.
    /// * `updated_at` - The timestamp of when the note was last updated, typically set to the current UTC time.
    /// # Returns
    /// A new `Note` instance with the provided values.
    /// # Examples
    /// ```
    /// let note = Note::from_primitives(
    ///    1,
    ///   String::from("My First Note"),
    ///   String::from("This is the content of my first note."),
    ///   Utc::now(),
    ///   Utc::now(),
    /// );
    /// assert_eq!(note.get_id(), Some(1));
    /// assert_eq!(note.get_title(), "My First Note");
    /// assert_eq!(note.get_content(), "This is the content of my first note.");
    /// ```
    /// # Errors
    /// Panics if the title is empty, exceeds 100 characters, or if the content is empty or exceeds 1000 characters.
    pub fn from_primitives(
        id: i64,
        title: String,
        content: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Note {
            id: Option::Some(id),
            title,
            content,
            created_at,
            updated_at,
        }
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
    pub fn get_id(&self) -> Option<i64> {
        self.id
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

    /// Get the last updated timestamp of the note
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// The last updated timestamp of the note as a `DateTime<Utc>`.
    /// # Examples
    /// ```
    /// let mut note = Note::create(
    ///     String::from("My First Note"),
    ///     String::from("This is the content of my first note."),
    /// );
    /// let updated_at = note.get_updated_at();
    /// assert!(updated_at <= Utc::now());
    /// ```
    /// # Note
    /// The `updated_at` field is set to the current UTC time when the note is created.
    /// It can be updated later by modifying the note.
    ///
    pub fn get_updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    /// Set the ID of the note.
    /// # Arguments
    /// * `value` - The new ID for the note, must be a valid i64.
    /// # Examples
    /// ```
    /// let mut note = Note::create(
    ///     String::from("My First Note"),  
    ///    String::from("This is the content of my first note."),   
    /// );
    /// note.set_id(1);
    /// assert_eq!(note.get_id(), Some(1));
    /// ```
    /// # Note
    /// This method is typically used when the note is created or updated in a database.
    /// It sets the `id` field to the provided value.
    /// It is important to ensure that the ID is unique within the context of the application.
    ///
    /// # Panics
    /// This method will panic if the provided value is not a valid i64.
    /// It is important to ensure that the ID meets the constraints of the application.
    ///
    pub fn set_id(&mut self, value: i64) {
        self.id = Some(value);
    }

    /// Set the title of the note.
    /// # Arguments
    /// * `title` - The new title for the note, must be non-empty and up to 100 characters.
    /// # Examples
    /// ```
    /// let mut note = Note::create(
    ///    String::from("My First Note"),
    ///   String::from("This is the content of my first note."),
    /// );
    /// note.set_title(String::from("Updated Note Title"));
    /// assert_eq!(note.get_title(), "Updated Note Title");
    /// ```
    /// # Errors
    /// Panics if the title is empty or exceeds 100 characters.
    ///
    /// # Note
    /// This method updates the `updated_at` field to the current UTC time when the title is changed.
    /// It is typically used when the title of the note is modified.
    ///
    /// # Panics
    /// This method will panic if the title is empty or exceeds 100 characters.
    /// It is important to ensure that the title meets these constraints to maintain the integrity of the note.
    ///
    pub fn set_title(&mut self, title: String) {
        if title.is_empty() {
            panic!("Title cannot be empty");
        }

        if title.len() > 100 {
            panic!("Title cannot exceed 100 characters");
        }

        self.title = title;
        self.updated_at = Utc::now();
    }

    /// Set the content of the note
    /// # Arguments
    /// * `content` - The new content for the note, must be non-empty and up to 1000 characters.
    /// # Examples
    /// ```
    /// let mut note = Note::create(
    ///     String::from("My First Note"),
    ///     String::from("This is the content of my first note."),
    /// );
    /// note.set_content(String::from("Updated content for my first note."));
    /// assert_eq!(note.get_content(), "Updated content for my first note.");
    /// ```
    /// # Errors
    /// Panics if the content is empty or exceeds 1000 characters.
    /// # Note
    /// This method updates the `updated_at` field to the current UTC time when the content is changed.
    /// It is typically used when the content of the note is modified.
    pub fn set_content(&mut self, content: String) {
        if content.is_empty() {
            panic!("Content cannot be empty");
        }

        if content.len() > 1000 {
            panic!("Content cannot exceed 1000 characters");
        }

        self.content = content;
        self.updated_at = Utc::now();
    }

    /// Update the `updated_at` timestamp to the current UTC time.
    /// # Arguments
    /// * `value` - The new timestamp to set for the `updated_at` field.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut note = Note::create(
    ///    String::from("My First Note"),
    ///     String::from("This is the content of my first note."),
    /// );
    /// let new_time = Utc::now();
    /// note.set_updated_at(new_time);
    /// assert_eq!(note.get_updated_at(), new_time);
    /// ```
    /// # Note
    /// This method is typically used to update the `updated_at` field when the note is modified.
    /// It sets the `updated_at` field to the provided value.
    pub fn set_updated_at(&mut self, value: DateTime<Utc>) {
        self.updated_at = value;
    }
}
