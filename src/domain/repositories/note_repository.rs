use crate::domain::entities::note::Note;
use chrono::prelude::*;
use rusqlite::{Connection, Result, params}; // Asegúrate de que esta ruta sea correcta

pub struct NoteRepository {
    connection: Connection,
}

impl NoteRepository {
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
    pub fn connection(&self) -> &Connection {
        &self.connection
    }

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

    pub fn update(&self, note: &Note) -> Result<(), String> {
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

        println!("[+] Updated note in SQLite: {:?}", note);
        Ok(())
    }

    pub fn delete(&self, id: i64) -> Result<(), String> {
        self.connection
            .execute("DELETE FROM notes WHERE id = ?1", params![id])
            .map_err(|err| format!("Error al eliminar la nota: {}", err))?;

        println!("[+] Deleted note with ID: {}", id);
        Ok(())
    }


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
                println!("Found note with ID {}: {:?}", id, note);
                Some(note)
            }
            _ => {
                println!("No note found with ID: {}", id);
                None
            }
        }
    }
}
