use chrono::{DateTime, Local, Utc};



struct ObsidianParser;

// Temp
#[derive(Debug, Default)]
struct ActivityEntry {
    pub key: String,
    pub sub_key: Option<String>,
    pub start_time: DateTime<Local>,
    pub end_time: DateTime<Local>
}

impl ObsidianParser {
    fn parse(&self, input: impl AsRef<str>) -> Result<Vec<ActivityEntry>, ()> {
        Ok(vec![ActivityEntry::default()])
    }
}
