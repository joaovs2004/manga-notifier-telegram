use rusqlite::{Connection, Result};
use std::error::Error;

#[derive(Debug)]
pub struct Manga {
    pub manga_id: String,
    pub current_chapter: String
}

fn create_client_subscription_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS client_subscription (
            manga_id TEXT NOT NULL,
            client_id TEXT NOT NULL,
            FOREIGN KEY(manga_id) REFERENCES manga(id),
            FOREIGN KEY(client_id) REFERENCES client(telegram_id),
        )",
        (), // empty list of parameters.
    )?;

    Ok(())
}

pub async fn insert_manga_in_manga_list(conn: &Connection, manga_id: String, current_chapter: String) -> Result<()> {
    let _ = create_client_subscription_table(&conn);

    conn.execute(
        "INSERT INTO manga VALUES ((?1), (?2) )",
        (manga_id, current_chapter),
    )?;

    println!("Manga inserted");

    Ok(())
}

pub async fn remove_manga_from_manga_list(conn: &Connection, manga_id: String, current_chapter: String) -> Result<()> {
    let _ = create_client_subscription_table(&conn);

    conn.execute(
        "DELETE FROM manga WHERE manga_id ((?1), (?2) )",
        (manga_id, current_chapter),
    )?;

    println!("Manga inserted");

    Ok(())
}

pub fn get_current_chapter_from_manga_database(conn: &Connection, manga_id: String) -> Result<String, Box<dyn Error>> {
    let _ = create_client_subscription_table(conn);

    let mut stmt = conn.prepare("SELECT * FROM manga WHERE manga_id=(?1)")?;

    let manga = stmt.query_row([manga_id], |row| {
        Ok(Manga {
            manga_id: row.get(0)?,
            current_chapter: row.get(1)?
        })
    });

    let current_chapter = manga?.current_chapter;

    Ok(current_chapter)
}
