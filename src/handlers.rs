use teloxide::utils::command::BotCommands;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};
use teloxide::{types::{InlineKeyboardButton, InlineKeyboardMarkup, InputFile}};
use url::Url;

use crate::{MyDialogue, HandlerResult, Command};
use crate::manga_info_getter::{search_for_manga, get_manga_cover_art};
use crate::data_types;
use crate::database::client::insert_client_in_database;

pub async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let _ = insert_client_in_database(msg.chat.id.to_string());
    bot.send_message(msg.chat.id, "User aded to database. Type /help to see avaible commands").await?;
    //dialogue.exit().await?;
    Ok(())
}

pub async fn help(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
    Ok(())
}

pub async fn search(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Type the name of the manga you want to search").await?;
    dialogue.update(crate::State::ReceiveSearch).await?;

    Ok(())
}

pub async fn receive_search(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text() {
        Some(manga_title) => {
            let manga_resp = search_for_manga(manga_title.to_string()).await.unwrap();

            if manga_resp.data.len() == 0 {
                bot.send_message(msg.chat.id, "No manga found").await?;
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
                        InlineKeyboardButton::callback("Add to list", "add"),
                    ]]);
                    bot.send_photo(msg.chat.id, InputFile::url(Url::parse(&cover_url).unwrap()))
                        .caption(manga_title)
                        .reply_markup(keyboard)
                        .await?;
                }
            }
        },
        None => {
            bot.send_message(msg.chat.id, "Send the manga name").await?;
        }
    };
    Ok(())
}

pub async fn receive_manga_id(bot: Bot, dialogue: MyDialogue, callback: CallbackQuery) -> HandlerResult {
    println!("{}", callback.data.unwrap());
    dialogue.exit().await?;
    Ok(())
}
