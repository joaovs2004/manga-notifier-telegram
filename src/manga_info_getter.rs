use reqwest::header::USER_AGENT;
use std::error::Error;

use crate::data_types::manga_types::{LastChapterInfo, Manga, Cover};

pub async fn search_for_manga(title: String) -> Result<Manga, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!("https://api.mangadex.org/manga?title={}", title.replace(" ", "%20"));

    let client = reqwest::Client::new();

    let resp = client.
        get(url)
        .header(USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36")
        .send()
        .await?;

    let resp = resp.json::<Manga>().await?;

    Ok(resp)
}

pub async fn get_manga_cover_art(cover_art_id: String) -> Result<String, Box<dyn Error>> {
    let url = format!("https://api.mangadex.org/cover/{}", cover_art_id);

    let client = reqwest::Client::new();

    let resp = client
        .get(url)
        .header(USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36")
        .send()
        .await?
        .json::<Cover>().await?;

    Ok(resp.data.attributes.fileName)
}

pub async fn get_current_chapter(manga_id: String) -> Result<LastChapterInfo, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!("https://api.mangadex.org/manga/{}/feed?translatedLanguage[]=pt-br&limit=96&includes[]=scanlation_group&includes[]=user&order[volume]=desc&order[chapter]=desc&offset=0&contentRating[]=safe&contentRating[]=suggestive&contentRating[]=erotica&contentRating[]=pornographic", manga_id);

    let client = reqwest::Client::new();

    let resp = client.
        get(url)
        .header(USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36")
        .send()
        .await?
        .json::<Manga>()
        .await?;

    let current_chapter_info = match resp.data.first() {
        Some(data) => {
            Ok(LastChapterInfo {
                number: data.attributes.chapter.clone().unwrap(),
                id: data.id.clone()
            })
        },
        None => Err("Error trying to get last chapter")
    }?;

    Ok(current_chapter_info)
}
