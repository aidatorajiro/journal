#[cfg(test)]
mod migration {
    use std::{fs, path::Path};

    use crate::typedef::migration::facebook::comments::CommentsV2;

    fn facebook_parse_comments (data: &String) -> Result<CommentsV2, serde_json::Error> {
        serde_json::from_str(data)
    }

    #[test]
    fn facebook() {
        let r = fs::read_to_string(Path::new("/Users/aidatorajiro/Downloads/facebook-kawarusosu/comments_and_reactions/comments.json")).unwrap();
        let x1 = facebook_parse_comments(&r).unwrap();
        println!("{:?}", x1);
    }
}