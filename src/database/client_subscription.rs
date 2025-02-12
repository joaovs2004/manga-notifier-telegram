use rusqlite::{Connection, Result};

#[derive(Debug, Clone)]
pub struct ClientSubscription {
    pub manga_id: String,
    pub client_id: String,
    pub manga_name: Option<String>
}

fn create_client_subscription_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS client_subscription (
            manga_id TEXT NOT NULL,
            client_id TEXT NOT NULL,
            FOREIGN KEY(manga_id) REFERENCES manga(id),
            FOREIGN KEY(client_id) REFERENCES client(telegram_id)
        )",
        (),
    )?;

    Ok(())
}

pub fn insert_client_subscription(manga_id: String, client_id: String) -> Result<()> {
    let conn = Connection::open("./database.db3")?;

    let _ = create_client_subscription_table(&conn);

    let client_subscription = get_client_subscription(&conn, manga_id.clone(), client_id.clone());

    if let Err(_) = client_subscription {
        conn.execute(
            "INSERT INTO client_subscription VALUES ((?1), (?2) )",
            (manga_id, client_id),
        )?;
    }

    println!("Manga inserted");

    let _ = conn.close();

    Ok(())
}

pub fn remove_manga_from_subscription(manga_id: String, client_id: String) -> Result<()> {
    let conn = Connection::open("./database.db3")?;

    let _ = create_client_subscription_table(&conn);

    conn.execute(
        "DELETE FROM client_subscription WHERE manga_id=(?1) AND client_id=(?2)",
        (manga_id, client_id),
    )?;

    println!("Manga inserted");

    Ok(())
}

pub fn get_client_subscription(conn: &Connection, manga_id: String, client_id: String) -> Result<ClientSubscription> {
    let _ = create_client_subscription_table(conn);

    let mut stmt = conn.prepare("SELECT * FROM client_subscription WHERE manga_id=(?1) AND client_id=(?2)")?;

    let manga_subscription = stmt.query_row([manga_id, client_id], |row| {
        Ok(ClientSubscription {
            manga_id: row.get(0)?,
            client_id: row.get(1)?,
            manga_name: None
        })
    });

    let manga_subscription = manga_subscription?;

    Ok(manga_subscription)
}

pub fn get_all_client_subscriptions(client_id: String) -> Result<Vec<ClientSubscription>> {
    let conn = Connection::open("./database.db3")?;

    let _ = create_client_subscription_table(&conn);

    let mut stmt = conn.prepare(
        "SELECT manga_id, client_id, manga.name FROM client_subscription JOIN manga ON client_subscription.manga_id=manga.id WHERE client_id=(?1)"
    )?;

    let manga_subscription = stmt.query_map([client_id], |row| {
        Ok(ClientSubscription {
            manga_id: row.get(0)?,
            client_id: row.get(1)?,
            manga_name: row.get(2)?
        })
    });

    let manga_subscription = manga_subscription?;

    let mut manga_subscriptions: Vec<ClientSubscription> = Vec::new();

    for manga in manga_subscription {
        manga_subscriptions.push(manga?);
    }

    Ok(manga_subscriptions)
}
