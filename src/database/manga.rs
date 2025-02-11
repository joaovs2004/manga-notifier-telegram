use rusqlite::{Connection, Result};
use std::error::Error;

#[derive(Debug)]
pub struct Manga {
    pub manga_id: String,
    pub current_chapter: String
}

fn create_manga_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS manga (
            id TEXT NOT NULL PRIMARY KEY,
            current_chapter TEXT NOT NULL
        )",
        (),
    )?;

    Ok(())
}

pub fn insert_manga_in_database(manga_id: String, current_chapter: String) -> Result<()> {
    let conn = Connection::open("./database.db3")?;

    let _ = create_manga_table(&conn);

    let current_chapter_in_db = get_current_chapter_from_manga_database(&conn, manga_id.clone());

    match current_chapter_in_db {
        Ok(current_chapter_db) => {
            if !current_chapter.eq(&current_chapter_db) {
                let _ = update_manga_in_database(&conn, manga_id.into(), current_chapter.clone());
            }
        },
        Err(_) => {
            conn.execute(
                "INSERT INTO manga VALUES ((?1), (?2) )",
                (manga_id, current_chapter),
            )?;
        }
    }

    let _ = conn.close();

    Ok(())
}

pub fn update_manga_in_database(conn: &Connection, manga_id: String, current_chapter: String) -> Result<()> {
    let _ = create_manga_table(&conn);

    conn.execute(
        "UPDATE manga SET current_chapter=(?2) WHERE id = (?1)",
        (manga_id, current_chapter),
    )?;

    println!("Manga update");

    Ok(())
}

pub fn get_all_manga_from_database(conn: &Connection, manga_id: String) -> Result<String, Box<dyn Error>> {
    let _ = create_manga_table(conn);

    let mut stmt = conn.prepare("SELECT id FROM manga")?;

    let result_manga_id = stmt.query_row([manga_id], |row| {
        Ok(row.get(0)?)
    });

    let manga_id = result_manga_id?;

    Ok(manga_id)
}

pub fn get_current_chapter_from_manga_database(conn: &Connection, manga_id: String) -> Result<String, Box<dyn Error>> {
    let _ = create_manga_table(conn);

    let mut stmt = conn.prepare("SELECT * FROM manga WHERE id=(?1)")?;

    let manga = stmt.query_row([manga_id], |row| {
        Ok(Manga {
            manga_id: row.get(0)?,
            current_chapter: row.get(1)?
        })
    });

    let current_chapter = manga?.current_chapter;

    Ok(current_chapter)
}
