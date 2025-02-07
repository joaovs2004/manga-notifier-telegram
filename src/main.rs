use data_types::manga_types::LastChapterInfo;
use rusqlite::{Connection, Result};
use teloxide::{prelude::*, repls::CommandReplExt, dispatching::dialogue::InMemStorage, utils::command::BotCommands,types::{InlineKeyboardButton, InlineKeyboardMarkup, InputFile}};
use std::{thread, time};
use manga_info_getter::{get_current_chapter, search_for_manga, get_manga_cover_art};
use database::client::{get_clients, insert_client_in_database};
use database::manga::{insert_manga_in_database, update_manga_in_database, get_current_chapter_from_manga_database};
use url::Url;

pub mod manga_info_getter;
pub mod database;
pub mod data_types;

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceiveFullName,
    ReceiveAge {
        full_name: String,
    },
    ReceiveLocation {
        full_name: String,
        age: u8,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let bot = Bot::new("");

    tokio::spawn(Command::repl(bot.clone(), answer));

    let time_to_sleep = time::Duration::from_secs(15);

    loop {
        // let conn = Connection::open("./database.db3")?;

        // let clients = get_clients(&conn);

        // let current_chapter_info = get_current_chapter().await;

        // if let (Ok(clients), Ok(current_chapter_info)) = (clients, current_chapter_info) {
        //     let current_chapter_in_db = get_current_chapter_from_manga_database(&conn, "801513ba-a712-498c-8f57-cae55b38cc92".into());

        //     match current_chapter_in_db {
        //         Ok(current_chapter_db) => {
        //             if !current_chapter_info.number.eq(&current_chapter_db) {
        //                 let _ = update_manga_in_database(&conn, "801513ba-a712-498c-8f57-cae55b38cc92".into(), current_chapter_info.number.clone()).await;
        //                 let _ = alert_users(bot.clone(), clients, current_chapter_info).await;
        //             }
        //         },
        //         Err(_) => {
        //             let _ = insert_manga_in_database(&conn, "801513ba-a712-498c-8f57-cae55b38cc92".into(), current_chapter_info.number.clone()).await;
        //             let _ = alert_users(bot.clone(), clients, current_chapter_info).await;
        //         }
        //     }
        // }

        // conn.close();

        thread::sleep(time_to_sleep);
    }
}

async fn alert_users(bot: Bot, clients: Vec<String>, current_chapter_info: LastChapterInfo) {
    let message = format!("Chapter {} released. Link to read: https://mangadex.org/chapter/{}", current_chapter_info.number, current_chapter_info.id);

    for client in clients {
        let _ = bot.send_message(client, message.clone()).await;
    }
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Shows the available commands")]
    Help,
    #[command(description = "Adds the user to the database, when a new chapter is released the user is notified")]
    Start,
    #[command(description = "List your mangas")]
    List,
    #[command(description = "Add manga to your list")]
    Add,
    #[command(description = "Remove manga from your list")]
    Remove,
    #[command(description = "Search for manga in mangadex")]
    Search(String),
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Start => {
            let _ = insert_client_in_database(msg.chat.id.to_string());
            bot.send_message(msg.chat.id, "User aded to database. Type /help to see avaible commands").await?
        },
        Command::List => todo!(),
        Command::Add => todo!(),
        Command::Remove => todo!(),
        Command::Search(title) => {
            let manga_resp = search_for_manga(title).await.unwrap();

            if manga_resp.data.len() == 0 {
                bot.send_message(msg.chat.id, "No manga found").await?
            } else {
                for manga in manga_resp.data {
                    let mut manga_title = String::new();
                    let cover_art: Vec<&data_types::manga_types::Relationship> = manga.relationships.iter().filter(|x| x.typ == "cover_art").collect();
                    let mut cover_art_id = String::new();

                    match cover_art.first() {
                        Some(relation) => cover_art_id = relation.id.clone(),
                        None => println!("No cover art found"),
                    };

                    let cover_art_id = get_manga_cover_art(cover_art_id).await.unwrap();
                    let cover_url = format!("https://uploads.mangadex.org/covers/{}/{}.256.jpg", manga.id, cover_art_id);

                    match manga.attributes.title {
                        data_types::manga_types::Title::TitleString(title) => {
                            manga_title = title;
                        },
                        data_types::manga_types::Title::Object(child_title) => {
                            manga_title = child_title.en;
                        },
                    };


                    let keyboard = InlineKeyboardMarkup::new([[
                        InlineKeyboardButton::callback("Add", "add"),
                    ]]);
                    bot.send_photo(msg.chat.id, InputFile::url(Url::parse(&cover_url).unwrap()))
                        .caption(manga_title)
                        .reply_markup(keyboard)
                        .await?;
                    //bot.send_message(msg.chat.id, manga_title)
                    //    .reply_markup(keyboard)
                    //    .await?;
                }

                bot.send_message(msg.chat.id, "Found manga")
                    .await?
            }
        },
    };

    Ok(())
}
