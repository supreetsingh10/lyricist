use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    pub hits: Vec<Hit>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hit {
    pub highlights: Option<Vec<Value>>,
    pub index: Option<String>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub result: Result,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Result {
    #[serde(rename = "_type")]
    pub type_field: Option<String>,
    pub annotation_count: Option<i64>,
    pub api_path: Option<String>,
    pub artist_names: Option<String>,
    pub full_title: Option<String>,
    pub header_image_thumbnail_url: Option<String>,
    pub header_image_url: Option<String>,
    pub id: Option<i64>,
    pub instrumental: bool,
    pub lyrics_owner_id: Option<i64>,
    pub lyrics_state: Option<String>,
    pub lyrics_updated_at: Option<i64>,
    pub path: Option<String>,
    pub primary_artist_names: Option<String>,
    pub pyongs_count: Option<i64>,
    pub relationships_index_url: Option<String>,
    pub release_date_components: Option<ReleaseDateComponents>,
    pub release_date_for_display: Option<String>,
    pub release_date_with_abbreviated_month_for_display: Option<String>,
    pub song_art_image_thumbnail_url: Option<String>,
    pub song_art_image_url: Option<String>,
    pub stats: Option<Stats>,
    pub title: Option<String>,
    pub title_with_featured: Option<String>,
    pub updated_by_human_at: Option<i64>,
    pub url: Option<String>,
    pub featured_artists: Option<Vec<FeaturedArtist>>,
    pub primary_artist: Option<PrimaryArtist>,
    pub primary_artists: Vec<PrimaryArtist2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReleaseDateComponents {
    pub year: Option<i64>,
    pub month: Option<i64>,
    pub day: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stats {
    pub unreviewed_annotations: Option<i64>,
    pub concurrents: Option<i64>,
    pub hot: bool,
    pub pageviews: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeaturedArtist {
    #[serde(rename = "_type")]
    pub type_field: Option<String>,
    pub api_path: Option<String>,
    pub header_image_url: Option<String>,
    pub id: Option<i64>,
    pub image_url: Option<String>,
    pub index_character: Option<String>,
    pub is_meme_verified: bool,
    pub is_verified: bool,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrimaryArtist {
    #[serde(rename = "_type")]
    pub type_field: Option<String>,
    pub api_path: Option<String>,
    pub header_image_url: Option<String>,
    pub id: Option<i64>,
    pub image_url: Option<String>,
    pub index_character: Option<String>,
    pub is_meme_verified: bool,
    pub is_verified: bool,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub url: Option<String>,
    pub iq: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrimaryArtist2 {
    #[serde(rename = "_type")]
    pub type_field: Option<String>,
    pub api_path: Option<String>,
    pub header_image_url: Option<String>,
    pub id: Option<i64>,
    pub image_url: Option<String>,
    pub index_character: Option<String>,
    pub is_meme_verified: bool,
    pub is_verified: bool,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub url: Option<String>,
    pub iq: Option<i64>,
}
