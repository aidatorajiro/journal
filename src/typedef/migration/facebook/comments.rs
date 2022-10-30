use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    pub comments_v2: Vec<CommentsV2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommentsV2 {
    pub timestamp: i64,
    #[serde(default)]
    pub data: Vec<Daum>,
    pub title: String,
    #[serde(default)]
    pub attachments: Vec<Attachment>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Daum {
    pub comment: Comment,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Comment {
    pub timestamp: i64,
    pub comment: String,
    pub author: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attachment {
    pub data: Vec<Daum2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Daum2 {
    pub external_context: Option<ExternalContext>,
    pub media: Option<Media>,
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
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MediaMetadata {
    pub photo_metadata: PhotoMetadata,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhotoMetadata {
    pub exif_data: Vec<ExifDaum>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExifDaum {
    pub upload_ip: String,
    pub taken_timestamp: i64,
}
