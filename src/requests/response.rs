use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    pub hits: Vec<Hit>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hit {
    pub highlights: Vec<Value>,
    pub index: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub result: Result,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Result {
    #[serde(rename = "_type")]
    pub type_field: String,
    pub annotation_count: i64,
    pub api_path: String,
    pub artist_names: String,
    pub full_title: String,
    pub header_image_thumbnail_url: String,
    pub header_image_url: String,
    pub id: i64,
    pub instrumental: bool,
    pub lyrics_owner_id: i64,
    pub lyrics_state: String,
    pub lyrics_updated_at: i64,
    pub path: String,
    pub primary_artist_names: String,
    pub pyongs_count: i64,
    pub relationships_index_url: String,
    pub release_date_components: ReleaseDateComponents,
    pub release_date_for_display: String,
    pub release_date_with_abbreviated_month_for_display: String,
    pub song_art_image_thumbnail_url: String,
    pub song_art_image_url: String,
    pub stats: Stats,
    pub title: String,
    pub title_with_featured: String,
    pub updated_by_human_at: i64,
    pub url: String,
    pub featured_artists: Option<Vec<FeaturedArtist>>,
    pub primary_artist: PrimaryArtist,
    pub primary_artists: Vec<PrimaryArtist2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReleaseDateComponents {
    pub year: i64,
    pub month: i64,
    pub day: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stats {
    pub unreviewed_annotations: i64,
    pub concurrents: Option<i64>,
    pub hot: bool,
    pub pageviews: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeaturedArtist {
    #[serde(rename = "_type")]
    pub type_field: String,
    pub api_path: String,
    pub header_image_url: String,
    pub id: i64,
    pub image_url: String,
    pub index_character: String,
    pub is_meme_verified: bool,
    pub is_verified: bool,
    pub name: String,
    pub slug: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrimaryArtist {
    #[serde(rename = "_type")]
    pub type_field: String,
    pub api_path: String,
    pub header_image_url: String,
    pub id: i64,
    pub image_url: String,
    pub index_character: String,
    pub is_meme_verified: bool,
    pub is_verified: bool,
    pub name: String,
    pub slug: String,
    pub url: String,
    pub iq: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrimaryArtist2 {
    #[serde(rename = "_type")]
    pub type_field: String,
    pub api_path: String,
    pub header_image_url: String,
    pub id: i64,
    pub image_url: String,
    pub index_character: String,
    pub is_meme_verified: bool,
    pub is_verified: bool,
    pub name: String,
    pub slug: String,
    pub url: String,
    pub iq: Option<i64>,
}
