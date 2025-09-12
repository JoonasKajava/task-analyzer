use chrono::NaiveTime;

#[derive(Debug, Default, PartialEq)]
pub struct ActivityEntry {
    pub key: String,
    pub sub_key: Option<String>,
    pub start_time: NaiveTime,
    pub end_time: Option<NaiveTime>,
}
