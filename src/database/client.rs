use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct Client {
    pub telegram_id: String
}

pub fn create_client_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS client (
            telegram_id TEXT NOT NULL UNIQUE
        )",
        (), // empty list of parameters.
    )?;

    Ok(())
}

pub fn insert_client_in_database(telegram_id: String) -> Result<()> {
    let conn = Connection::open("./database.db3")?;

    let _ = create_client_table(&conn);

    let clients = get_clients(&conn)?;

    if !clients.contains(&telegram_id) {
        let me = Client {
            telegram_id
        };

        conn.execute(
            "INSERT INTO client (telegram_id) VALUES (?1)",
            &[&me.telegram_id],
        )?;

        println!("Client inserted");
    }

    Ok(())
}

pub fn get_clients(conn: &Connection,) -> Result<Vec<String>> {
    let _ = create_client_table(conn);

    let mut stmt = conn.prepare("SELECT telegram_id FROM client")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Client {
            telegram_id: row.get(0)?,
        })
    })?;

    let mut clients_telegram_id = Vec::new();

    for person in person_iter {
        clients_telegram_id.push(person?.telegram_id);
    }

    Ok(clients_telegram_id)
}