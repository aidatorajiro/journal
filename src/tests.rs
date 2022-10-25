#[cfg(test)]
mod migration {
    use std::{fs, path::Path};

    use crate::migration::facebook::{facebook_parse_comments, CommentsV2};

    #[test]
    fn facebook() {
        let r = fs::read_to_string(Path::new("/Users/aidatorajiro/Downloads/facebook-kawarusosu/comments_and_reactions/comments.json")).unwrap();
        let x1 = facebook_parse_comments(&r).unwrap();
        let x2 = serde_json::to_string(&x1).unwrap();
        let x3: CommentsV2 = serde_json::from_str(&x2).unwrap();
        assert_eq!(x1, x3);
    }
}