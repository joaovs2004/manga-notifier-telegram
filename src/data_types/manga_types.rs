use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SearchedManga {
    pub data: Vec<MangaData>
}

#[derive(Deserialize, Debug)]
pub struct Manga {
    pub data: Vec<MangaData>
}

#[derive(Deserialize, Debug)]
pub struct MangaData {
    pub attributes: MangaAttributes,
    pub id: String,
    pub relationships: Vec<Relationship>
}

#[derive(Deserialize, Debug)]
pub struct MangaAttributes {
    pub chapter: Option<String>,
    pub title: Title
}

#[derive(Deserialize, Debug)]
pub struct Relationship {
    pub id: String,
    #[serde(alias= "type")]
    pub typ: String
}

#[derive(Deserialize, Debug)]
pub struct Cover {
    pub data: CoverData
}

#[derive(Deserialize, Debug)]
pub struct CoverData {
    pub attributes: CoverAttributes
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CoverAttributes {
    pub file_name: String
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
#[allow(unused)]
pub enum Title {
    TitleString(String),
    Object(ChildTitle)
}

#[derive(Deserialize, Debug)]
pub struct ChildTitle {
    pub en: String,
}

#[derive(Deserialize, Debug)]
pub struct LastChapterInfo {
    pub number: String,
    pub id: String
}
