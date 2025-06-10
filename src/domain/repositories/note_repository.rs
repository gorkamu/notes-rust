use crate::domain::entities::note::Note;
use chrono::prelude::*;
use rusqlite::{Connection, Result, params}; // Asegúrate de que esta ruta sea correcta

///
/// The `NoteRepository` struct provides an interface for interacting with a SQLite database to manage notes.
/// It allows for saving, updating, deleting, and finding notes by their ID.
/// 
pub struct NoteRepository {
    connection: Connection,
}

impl NoteRepository {
    /// 
    /// Creates a new instance of `NoteRepository`.
    /// This function initializes the SQLite database connection and creates the `notes` table if it does not exist.
    /// # Returns
    /// A new `NoteRepository` instance with an established connection to the SQLite database.
    ///
    /// # Errors
    /// This function will panic if there is an error opening the database or creating the table.
    ///
    /// # Example
    /// ```
    /// let note_repository = NoteRepository::new();
    /// ```
    /// 
    pub fn new() -> Self {
        let connection = Connection::open("notes-rust.db")
            .map_err(|err| format!("Error al abrir la base de datos: {}", err))
            .unwrap();

        connection.execute(
            "CREATE TABLE IF NOT EXISTS notes (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        ).expect("Error al crear la tabla");

        NoteRepository { connection }
    }
}

impl NoteRepository {
    ///
    /// Returns a reference to the SQLite connection used by the repository.
    /// This function allows access to the underlying database connection for executing raw SQL queries if needed.
    /// # Returns
    /// A reference to the `Connection` object.
    ///
    /// # Example
    /// ```
    /// let connection = note_repository.connection();
    /// ```
    ///     
    pub fn connection(&self) -> &Connection {
        &self.connection
    }

    ///
    /// Saves a new note to the SQLite database.
    /// # Arguments
    /// * `note`: A reference to the `Note` object to be saved.
    /// # Returns
    /// * `Ok(i64)`: The ID of the newly created note if the operation is successful.
    /// * `Err(String)`: An error message if there is an issue saving the note, such as a database error.
    /// 
    pub fn save(&self, note: &Note) -> Result<i64, String> {
        self.connection
            .execute(
                "INSERT INTO notes (title, content, created_at, updated_at) VALUES (?1, ?2, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
                params![
                    note.get_title(), 
                    note.get_content()
                ],
            )
            .map_err(|err| format!("Error al guardar la nota: {}", err))?;

        let id = self.connection.last_insert_rowid();
        println!("[+] Saved note to SQLite: {:?} (id: {})", note, id);

        Ok(id)
    }

    pub fn find_all(&self) -> Option<Vec<Note>> {
        let mut stmt = self
            .connection
            .prepare(
                "SELECT 
                        id,
                        title, 
                        content,
                        created_at,
                        updated_at
                    FROM notes;",
            )
            .map_err(|err| format!("Error al preparar la consulta: {}", err))
            .ok()?;

        let notes = stmt
            .query_map([], |row| {
                let id: i64 = row.get(0)?;
                let title: String = row.get(1)?;
                let content: String = row.get(2)?;
                let created_at: String = row.get(3)?;
                let updated_at: String = row.get(4)?;

                let updated_at_date = updated_at.parse::<DateTime<Utc>>()
                    .map_err(|_| format!("Error parsing updated_at: {}", updated_at))
                    .unwrap_or_else(|_| Utc::now());
                let created_at_date = created_at.parse::<DateTime<Utc>>()
                    .map_err(|_| format!("Error parsing created_at: {}", created_at))
                    .unwrap_or_else(|_| Utc::now());

                    
                Ok(Note {
                        id: Option::Some(id),
                        title: title,
                        content: content,
                        created_at: created_at_date,
                        updated_at: updated_at_date,
                    })
            })
            .map_err(|err| format!("Error al buscar las notas: {}", err))
            .ok()?;

        let notes_vec: Vec<Note> = notes.filter_map(Result::ok).collect();

        if notes_vec.is_empty() {
            // println!("No notes found.");
            None
        } else {
            // println!("[+] Found {} notes in SQLite", notes_vec.len());
            Some(notes_vec)
        }
    }

    /// 
    /// Updates an existing note in the SQLite database.
    /// # Arguments
    /// * `note`: A `Note` object containing the updated information.
    /// # Returns
    /// * `Ok(Note)`: The updated `Note` object if the operation is successful.
    /// * `Err(String)`: An error message if there is an issue updating the note, such as a database error.
    ///
    pub fn update(&self, note: Note) -> Result<Note, String> {
        self.connection
            .execute(
                "UPDATE notes SET title = ?1, content = ?2, updated_at = CURRENT_TIMESTAMP WHERE id = ?3",
                params![
                    note.get_title(), 
                    note.get_content(), 
                    note.get_id()
                ],
            )
            .map_err(|err| format!("Error al actualizar la nota: {}", err))?;

        println!("[+] Updated note in SQLite: {:?}", note.get_id());
        Ok(note.clone())
    }

    /// 
    /// Deletes a note from the SQLite database by its ID.
    /// # Arguments
    /// * `id`: The ID of the note to be deleted.
    /// # Returns
    /// * `Ok(())`: If the note is successfully deleted.
    /// * `Err(String)`: An error message if there is an issue deleting the note, such as a database error.
    /// 
    pub fn delete(&self, id: i64) -> Result<(), String> {
        self.connection
            .execute("DELETE FROM notes WHERE id = ?1", params![id])
            .map_err(|err| format!("Error al eliminar la nota: {}", err))?;

        Ok(())
    }

    /// 
    /// Finds a note by its ID in the SQLite database.
    /// # Arguments
    /// * `id`: The ID of the note to be found.
    /// # Returns
    /// * `Option<Note>`: An `Option` containing the `Note` if found, or `None` if no note with the given ID exists.
    /// 
    pub fn find_by_id(&self, id: i64) -> Option<Note> {
        let mut stmt = self
            .connection
            .prepare(
                "SELECT 
                        id,
                        title, 
                        content,
                        created_at,
                        updated_at
                    FROM notes 
                    WHERE id = ?1
                    LIMIT 1;",
            )
            .map_err(|err| format!("Error al preparar la consulta: {}", err))
            .ok()?;

        let result = stmt
            .query_row(params![id], |row| {
                let id: i64 = row.get(0)?;
                let title: String = row.get(1)?;
                let content: String = row.get(2)?;
                let created_at: String = row.get(3)?;
                let updated_at: String = row.get(4)?;

                let updated_at_date = updated_at.parse::<DateTime<Utc>>()
                    .map_err(|_| format!("Error parsing updated_at: {}", updated_at))
                    .unwrap_or_else(|_| Utc::now());
                let created_at_date = created_at.parse::<DateTime<Utc>>()
                    .map_err(|_| format!("Error parsing created_at: {}", created_at))
                    .unwrap_or_else(|_| Utc::now());

                    
                Ok(Note {
                        id: Option::Some(id),
                        title: title,
                        content: content,
                        created_at: created_at_date,
                        updated_at: updated_at_date,
                    })
            })
            .map_err(|err| format!("Error al buscar la nota: {}", err))
            .ok();

        match result {
            Some(note) => {
                Some(note)
            }
            _ => {
                None
            }
        }
    }

    ///
    /// Finds notes by their title in the SQLite database.
    /// # Arguments
    /// * `title`: The title or partial title of the notes to be found.
    /// # Returns
    /// * `Option<Vec<Note>>`: An `Option` containing a vector of `Note` objects if found, or `None` if no notes match the given title.
    ///
    /// This function performs a case-insensitive search for the title in the notes table.
    /// It uses a SQL query with a `LIKE` clause to match the title, allowing for partial matches.
    /// If notes with the specified title are found, it returns them wrapped in `Some(Vec<Note>)`.
    /// If no notes are found, it returns `None`.
    ///
    pub fn find_by_title(&self, title: &str) -> Option<Vec<Note>> {
        let mut stmt = self
            .connection
            .prepare(
                "SELECT 
                        id,
                        title, 
                        content,
                        created_at,
                        updated_at
                    FROM notes 
                    WHERE title LIKE '%' || ?1 || '%';",
            )
            .map_err(|err| format!("Error al preparar la consulta: {}", err))
            .ok()?;

        let notes = stmt
            .query_map(params![title], |row| {
                let id: i64 = row.get(0)?;
                let title: String = row.get(1)?;
                let content: String = row.get(2)?;
                let created_at: String = row.get(3)?;
                let updated_at: String = row.get(4)?;

                let updated_at_date = updated_at.parse::<DateTime<Utc>>()
                    .map_err(|_| format!("Error parsing updated_at: {}", updated_at))
                    .unwrap_or_else(|_| Utc::now());
                let created_at_date = created_at.parse::<DateTime<Utc>>()
                    .map_err(|_| format!("Error parsing created_at: {}", created_at))
                    .unwrap_or_else(|_| Utc::now());

                Ok(Note {
                    id: Option::Some(id),
                    title: title,
                    content: content,
                    created_at: created_at_date,
                    updated_at: updated_at_date,
                })
            })
            .map_err(|err| format!("Error al buscar las notas por título: {}", err))
            .ok()?;

        let notes_vec: Vec<Note> = notes.filter_map(Result::ok).collect();

        if notes_vec.is_empty() {
            None
        } else {
            Some(notes_vec)
        }
    }
}
