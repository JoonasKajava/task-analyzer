use chrono::{NaiveDateTime, NaiveTime};

use markdown::mdast::{ListItem, Node, Paragraph, Text};
use regex::Regex;
use thiserror::Error;
pub struct ObsidianParser;

// Temp
#[derive(Debug, Default, PartialEq)]
pub struct ActivityEntry {
    pub key: String,
    pub sub_key: Option<String>,
    pub start_time: NaiveTime,
    pub end_time: Option<NaiveTime>,
}

#[derive(Debug, Error)]
pub enum ObsidianParseError {
    #[error("Failed to parse markdown")]
    MarkdownParseError,
    #[error("Failed to parse using regex")]
    RegexParseError,
}
//
impl ObsidianParser {

    fn collect_text(node: &Node) -> String {
        match node {
            Node::Text(Text { value, .. }) => value.clone(),
            Node::Paragraph(Paragraph { children, .. }) | Node::ListItem(ListItem {children, ..}) => children
                .iter()
                .map(ObsidianParser::collect_text)
                .collect::<Vec<String>>()
                .join(""),
            _ => String::new(),
        }
    }

    fn parse_task(listitem: &Node) -> Option<ActivityEntry> {
        let text = ObsidianParser::collect_text(listitem);
        let reg = Regex::new(r"(\d{2}:\d{2})(?: - )?(?:(\d{2}:\d{2})?).+?(JIRA:.+?)\s").expect("TODO");

        let captures = reg.captures(&text)?;

        let start = captures.get(1)?;
        let end = captures.get(2)?;
        let key = captures.get(3)?;

        let start = NaiveTime::parse_from_str(start.as_str(), "%R").ok()?;
        let end = NaiveTime::parse_from_str(end.as_str(), "%R").ok();

        Some(ActivityEntry {
            key:  key.as_str().to_string(),
            sub_key: None,
            start_time: start,
            end_time: end,
        })
    }

    fn find_tasks(node: &Node, tasks: &mut Vec<ActivityEntry>) {
        match node {
            Node::ListItem(_) => {
                if let Some(activity) = ObsidianParser::parse_task(node) {
                    tasks.push(activity);
                }
            },
            _ => {
                if let Some(children) = node.children() {
                    children
                        .iter()
                        .for_each(|child| ObsidianParser::find_tasks(child, tasks))
                }
            }
        }
    }

    pub fn parse(input: impl AsRef<str>) -> Result<Vec<ActivityEntry>, ObsidianParseError> {
        let mdast = markdown::to_mdast(input.as_ref(), &markdown::ParseOptions::default())
            .map_err(|_| ObsidianParseError::MarkdownParseError)?;

        let mut tasks = vec![];

        ObsidianParser::find_tasks(&mdast, &mut tasks);

        Ok(tasks)
    }
}
