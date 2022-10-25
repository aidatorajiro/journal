pub mod facebook {
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    pub struct CommentsV2 {
        comments_v2: Vec<CommentsV2Entry>
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    pub struct CommentsV2Entry {
        timestamp: u64,
        data: Option<Vec<CommentsV2EntryData>>,
        title: String
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    pub struct CommentsV2EntryData {
        comment: CommentsV2EntryDataComment
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    pub struct CommentsV2EntryDataComment {
        timestamp: u64,
        comment: String,
        author: String
    }

    pub fn facebook_parse_comments (data: &String) -> Result<CommentsV2, serde_json::Error> {
        serde_json::from_str(data)
    }
}