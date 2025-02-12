use data_types::manga_types::LastChapterInfo;
use dptree::case;
use rusqlite::{Connection, Result};
use teloxide::{prelude::*, dispatching::dialogue::InMemStorage, utils::command::BotCommands};
use std::{thread, time};
use manga_info_getter::get_current_chapter;
use database::{client_subscription::{get_client_subscriptions_by_manga, ClientSubscription}, manga::{get_all_manga_from_database, VecManga}};
use database::manga::{update_manga_in_database, get_current_chapter_from_manga_database};
use handlers::{help, list, receive_manga_index, receive_manga_to_remove_from_list, receive_search, search, start};

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
    ReceiveMangaToRemoveFromList {
        subscriptions: Vec<ClientSubscription>
    },
    Search,
    ReceiveSearch,
    ReceiveMangaIndex {
        avaible_mangas: VecManga
    },
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "Shows the available commands")]
    Help,
    #[command(description = "Adds the user to the database")]
    Start,
    #[command(description = "List the mangas in your list and remove any if desired")]
    List,
    #[command(description = "Search for manga in mangadex and add the manga you want to your list")]
    Search
}

#[tokio::main]
async fn main() -> Result<()> {
    let bot = Bot::from_env();

    tokio::spawn(spawn(bot.clone()));

    let time_to_sleep = time::Duration::from_secs(60*60);
    let conn = Connection::open("./database.db3")?;

    loop {
        let mangas = get_all_manga_from_database(&conn);

        match mangas {
            Ok(mangas) => {
                for manga in mangas {
                    let current_chapter_info = get_current_chapter(manga.clone().manga_id).await;
                    let current_chapter_in_db = get_current_chapter_from_manga_database(&conn, manga.clone().manga_id);

                    if let (Ok(current_chapter_in_db), Ok(current_chapter_info)) = (current_chapter_in_db, current_chapter_info) {
                        if !current_chapter_info.number.eq(&current_chapter_in_db) {
                            let _ = update_manga_in_database(&conn, manga.clone().manga_id, current_chapter_info.number.clone());
                            let _ = alert_users(bot.clone(), manga.clone().manga_id, current_chapter_info).await;
                        }
                    }
                }
            },
            Err(_) => {
                println!("Error when getting mangas in database");
            },
        }

        thread::sleep(time_to_sleep);
    }
}

async fn spawn(bot: Bot) {
    Dispatcher::builder(
        bot.clone(),
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
            .branch(dptree::case![State::ReceiveMangaIndex { avaible_mangas }].endpoint(receive_manga_index))
            .branch(dptree::case![State::ReceiveMangaToRemoveFromList { subscriptions }].endpoint(receive_manga_to_remove_from_list))
    )
    .dependencies(dptree::deps![InMemStorage::<State>::new()])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}

async fn alert_users(bot: Bot, manga_id: String, current_chapter_info: LastChapterInfo) -> Result<()> {
    let clients = get_client_subscriptions_by_manga(manga_id)?;

    for client in clients {
        let message = format!("Chapter {} released. Link to read: https://mangadex.org/chapter/{}", current_chapter_info.number, current_chapter_info.id);
        let _ = bot.send_message(client.client_id, message.clone()).await;
    }

    Ok(())
}
