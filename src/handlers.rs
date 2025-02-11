use teloxide::utils::command::BotCommands;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

use crate::{Command, State};
use crate::manga_info_getter::search_for_manga;
use crate::data_types;
use crate::database::client::insert_client_in_database;

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub async fn start(bot: Bot, _dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let _ = insert_client_in_database(msg.chat.id.to_string());
    bot.send_message(msg.chat.id, "User aded to database. Type /help to see avaible commands").await?;
    Ok(())
}

pub async fn help(bot: Bot, _dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
    Ok(())
}

pub async fn search(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Type the name of the manga you want to search").await?;
    dialogue.update(crate::State::ReceiveSearch).await?;

    Ok(())
}

pub async fn receive_search(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let manga_title = match msg.text() {
        Some(text) => text,
        None => {
            bot.send_message(msg.chat.id, "Type the number of manga you want to add").await?;
            return Ok(());
        }
    };

    let manga_resp = search_for_manga(manga_title.to_string()).await;

    match manga_resp {
        Ok(resp) => {
            let mut avaible_mangas_ids: Vec<String> = Vec::new();

            if resp.data.len() == 0 {
                bot.send_message(msg.chat.id, "No manga found").await?;
                return Ok(());
            }

            let mut found = String::from("Manga Found: \n");
            let mut manga_index = 1;

            for manga in resp.data {
                let manga_title = match manga.attributes.title {
                    data_types::manga_types::Title::TitleString(title) => title,
                    data_types::manga_types::Title::Object(child_title) => child_title.en
                };

                found.push_str(&format!("{} - {}\n", manga_index, manga_title));
                manga_index += 1;
                avaible_mangas_ids.push(manga.id);
            }

            bot.send_message(msg.chat.id, found).await?;
            bot.send_message(msg.chat.id, "Type the number of the manga you want do add to your list").await?;
            dialogue.update(crate::State::ReceiveMangaIndex { avaible_mangas_id: avaible_mangas_ids }).await?;
        },
        Err(_) => {
            bot.send_message(msg.chat.id, "Failed to search manga").await?;
        },
    }

    Ok(())
}

pub async fn receive_manga_index(bot: Bot, _dialogue: MyDialogue, avaible_mangas: Vec<String>, msg: Message) -> HandlerResult {
    let text = match msg.text() {
        Some(text) => text,
        None => {
            bot.send_message(msg.chat.id, "Type the number of manga you want to add").await?;
            return Ok(());
        }
    };

    let index_to_remove = match text.trim().parse::<usize>() {
        Ok(index) => index,
        Err(_) => {
            bot.send_message(msg.chat.id, "Please enter a valid number").await?;
            return Ok(());
        }
    };

    match avaible_mangas.get(index_to_remove - 1) {
        Some(manga) => {
            println!("{}", manga);
        }
        None => {
            bot.send_message(msg.chat.id, "Type the correct number of manga").await?;
        }
    }

    Ok(())
}
