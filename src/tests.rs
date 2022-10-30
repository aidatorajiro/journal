#[cfg(test)]
mod migration {
    use std::{fs, path::Path};

    use crate::typedef::migration::facebook::comments::Root;

    fn facebook_parse_comments (data: &String) -> Result<Root, serde_json::Error> {
        serde_json::from_str(data)
    }

    #[test]
    fn facebook() {
        let r = fs::read_to_string(Path::new("/Users/aidatorajiro/Downloads/facebook-kawarusosu/comments_and_reactions/comments.json")).unwrap();
        let x1 = facebook_parse_comments(&r).unwrap();
    }
}