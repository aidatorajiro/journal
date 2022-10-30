use serde_derive::Deserialize;
use serde_derive::Serialize;

pub type Root = Vec<Root2>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root2 {
    pub timestamp: i64,
    #[serde(default)]
    pub attachments: Vec<Attachment>,
    pub data: Vec<Daum2>,
    pub title: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attachment {
    pub data: Vec<Daum>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Daum {
    pub external_context: Option<ExternalContext>,
    pub media: Option<Media>,
    pub text: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExternalContext {
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Media {
    pub uri: String,
    pub creation_timestamp: i64,
    pub media_metadata: MediaMetadata,
    pub title: String,
    pub description: Option<String>,
    pub thumbnail: Option<Thumbnail>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MediaMetadata {
    pub photo_metadata: Option<PhotoMetadata>,
    pub video_metadata: Option<VideoMetadata>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhotoMetadata {
    pub exif_data: Vec<ExifDaum>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExifDaum {
    pub upload_ip: String,
    pub taken_timestamp: i64,
    pub iso: Option<i64>,
    pub focal_length: Option<String>,
    pub modified_timestamp: Option<i64>,
    pub camera_make: Option<String>,
    pub camera_model: Option<String>,
    pub exposure: Option<String>,
    pub f_stop: Option<String>,
    pub orientation: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoMetadata {
    pub exif_data: Vec<ExifDaum2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExifDaum2 {
    pub upload_ip: String,
    pub upload_timestamp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Thumbnail {
    pub uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Daum2 {
    pub post: Option<String>,
    pub update_timestamp: Option<i64>,
}
