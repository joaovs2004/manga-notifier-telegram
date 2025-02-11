use data_types::manga_types::{LastChapterInfo, Manga};
use dptree::case;
use rusqlite::{Connection, Result};
use teloxide::{prelude::*, repls::CommandReplExt, dispatching::dialogue::InMemStorage, utils::command::BotCommands,types::{InlineKeyboardButton, InlineKeyboardMarkup, InputFile}};
use std::{thread, time};
use manga_info_getter::{get_current_chapter, search_for_manga, get_manga_cover_art};
use database::client::{get_clients, insert_client_in_database};
use database::manga::{insert_manga_in_database, update_manga_in_database, get_current_chapter_from_manga_database};
use handlers::{help, list, receive_manga_index, receive_search, search, start};

pub mod manga_info_getter;
pub mod database;
pub mod data_types;
pub mod handlers;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    Help,
    List,
    Add,
    Remove,
    Search,
    ReceiveSearch,
    ReceiveMangaIndex {
        avaible_mangas_id: Vec<String>
    },
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Command {
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
    Search
}

#[tokio::main]
async fn main() -> Result<()> {
    let bot = Bot::from_env();

    Dispatcher::builder(
        bot,
        Update::filter_message()
            .enter_dialogue::<Message, InMemStorage<State>, State>()
            .branch(
                Update::filter_message()
                    .filter_command::<Command>()
                    .branch(case![Command::Start].endpoint(start))
            )
            .branch(
                Update::filter_message()
                    .filter_command::<Command>()
                    .branch(case![Command::Help].endpoint(help))
            )
            .branch(
                Update::filter_message()
                    .filter_command::<Command>()
                    .branch(case![Command::Search].endpoint(search))
            )
            .branch(
                Update::filter_message()
                    .filter_command::<Command>()
                    .branch(case![Command::List].endpoint(list))
            )
            .branch(dptree::case![State::ReceiveSearch].endpoint(receive_search))
            .branch(dptree::case![State::ReceiveMangaIndex { avaible_mangas_id }].endpoint(receive_manga_index))
    )
    .dependencies(dptree::deps![InMemStorage::<State>::new()])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;

    //tokio::spawn(Command::repl(bot.clone(), answer));

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
