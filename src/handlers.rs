use teloxide::utils::command::BotCommands;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

use crate::database::client_subscription::{get_all_client_subscriptions, insert_client_subscription, remove_manga_from_subscription, ClientSubscription};
use crate::{Command, State};
use crate::manga_info_getter::{get_current_chapter, search_for_manga};
use crate::data_types;
use crate::database::client::insert_client_in_database;
use crate::database::manga::{insert_manga_in_database, Manga, VecManga};

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let _ = insert_client_in_database(msg.chat.id.to_string());
    bot.send_message(msg.chat.id, "User aded to database. Type /help to see avaible commands").await?;
    dialogue.exit().await?;
    Ok(())
}

pub async fn help(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
    dialogue.exit().await?;
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
            bot.send_message(msg.chat.id, "Type the name of manga you want to add").await?;
            dialogue.update(crate::State::ReceiveSearch).await?;
            return Ok(());
        }
    };

    let manga_resp = search_for_manga(manga_title.to_string()).await;

    match manga_resp {
        Ok(resp) => {
            let mut avaible_mangas: VecManga = VecManga::new();

            if resp.data.len() == 0 {
                bot.send_message(msg.chat.id, "No manga found").await?;
                dialogue.exit().await?;
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
                avaible_mangas.mangas.push(Manga {
                    manga_id: manga.id,
                    name: manga_title,
                    current_chapter: manga.attributes.chapter.unwrap_or(String::new())
                });
            }

            bot.send_message(msg.chat.id, found).await?;
            bot.send_message(msg.chat.id, "Type the number of the manga you want do add to your list or 0 to do nothing").await?;
            dialogue.update(crate::State::ReceiveMangaIndex { avaible_mangas }).await?;
        },
        Err(_) => {
            bot.send_message(msg.chat.id, "Failed to search manga").await?;
            dialogue.exit().await?;
        },
    }

    Ok(())
}

pub async fn receive_manga_index(bot: Bot, dialogue: MyDialogue, avaible_mangas: VecManga, msg: Message) -> HandlerResult {
    let text = match msg.text() {
        Some(text) => text,
        None => {
            bot.send_message(msg.chat.id, "Type the number of manga you want to add").await?;
            dialogue.update(crate::State::ReceiveMangaIndex { avaible_mangas }).await?;
            return Ok(());
        }
    };

    let index_to_remove = match text.trim().parse::<usize>() {
        Ok(index) => index,
        Err(_) => {
            bot.send_message(msg.chat.id, "Please enter a valid number").await?;
            dialogue.update(crate::State::ReceiveMangaIndex { avaible_mangas }).await?;
            return Ok(());
        }
    };

    if index_to_remove == 0 {
        bot.send_message(msg.chat.id, "Quiting withount doing anything").await?;
        dialogue.exit().await?;
        return Ok(());
    }

    match avaible_mangas.mangas.get(index_to_remove - 1) {
        Some(manga) => {
            let current_chapter_info = get_current_chapter(manga.manga_id.to_string()).await;

            if let Ok(current_chapter_info) = current_chapter_info {
                let _ = insert_manga_in_database(manga.manga_id.to_string(),  manga.name.to_string(),current_chapter_info.number);
                let _ = insert_client_subscription(manga.manga_id.to_string(), msg.chat.id.to_string());
                bot.send_message(msg.chat.id, "Manga inserted").await?;
            }
        },
        None => {
            bot.send_message(msg.chat.id, "Type the correct number of manga").await?;
            dialogue.update(crate::State::ReceiveMangaIndex { avaible_mangas }).await?;
        }
    };

    dialogue.exit().await?;

    Ok(())
}

pub async fn list(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let subscriptions = get_all_client_subscriptions(msg.chat.id.to_string());

    match subscriptions {
        Ok(subscriptions) => {
            let mut found = String::from("Manga in your list: \n");
            let mut subscription_index = 1;

            for subscription in &subscriptions {
                found.push_str(&format!("{} - {}\n", subscription_index, subscription.manga_name.clone().unwrap()));
                subscription_index += 1;
            }

            bot.send_message(msg.chat.id, found).await?;
            bot.send_message(msg.chat.id, "Type the number of the manga you want to remove or 0 to do nothing").await?;
            dialogue.update(crate::State::ReceiveMangaToRemoveFromList { subscriptions }).await?;
        },
        Err(_) => {
            bot.send_message(msg.chat.id, "No manga found in your list").await?;
            dialogue.exit().await?;
        },
    }

    Ok(())
}


pub async fn receive_manga_to_remove_from_list(bot: Bot, dialogue: MyDialogue, subscriptions: Vec<ClientSubscription>, msg: Message) -> HandlerResult {
    let text = match msg.text() {
        Some(text) => text,
        None => {
            bot.send_message(msg.chat.id, "Type the number of manga you want to remove").await?;
            dialogue.update(crate::State::ReceiveMangaToRemoveFromList { subscriptions }).await?;
            return Ok(());
        }
    };

    let index_to_remove = match text.trim().parse::<usize>() {
        Ok(index) => index,
        Err(_) => {
            bot.send_message(msg.chat.id, "Please enter a valid number").await?;
            dialogue.update(crate::State::ReceiveMangaToRemoveFromList { subscriptions }).await?;
            return Ok(());
        }
    };

    if index_to_remove == 0 {
        bot.send_message(msg.chat.id, "Quiting withount doing anything").await?;
        dialogue.exit().await?;
        return Ok(());
    }

    match subscriptions.get(index_to_remove - 1) {
        Some(subscription) => {
            let _ = remove_manga_from_subscription(subscription.clone().manga_id, subscription.clone().client_id);
            let _ = bot.send_message(msg.chat.id, "Manga removed from list").await?;
        },
        None => {
            bot.send_message(msg.chat.id, "Type the correct number of manga").await?;
            dialogue.update(crate::State::ReceiveMangaToRemoveFromList { subscriptions }).await?;
        }
    };

    dialogue.exit().await?;

    Ok(())
}
