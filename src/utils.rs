pub mod utils {
    use std::time::{SystemTime, UNIX_EPOCH};

    /// Returns current timestamp in second.
    pub fn create_timestamp() -> u64 {
        return SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}