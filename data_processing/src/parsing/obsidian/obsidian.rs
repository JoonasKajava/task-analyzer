use chrono::NaiveTime;

use log::error;
use markdown::mdast::{ListItem, Node, Paragraph, Text};

use thiserror::Error;

use crate::task_parser::TaskParser;
pub struct ObsidianParser;


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
            Node::Paragraph(Paragraph { children, .. })
            | Node::ListItem(ListItem { children, .. }) => children
                .iter()
                .map(ObsidianParser::collect_text)
                .collect::<Vec<String>>()
                .join(""),
            _ => String::new(),
        }
    }

    fn parse_task(listitem: &Node) -> Option<ActivityEntry> {
        let text = ObsidianParser::collect_text(listitem);
        match TaskParser::parse_activity(text.as_bytes()) {
            Ok(x) => x,
            Err(e) => {
                println!("{e:?}");
                error!("{e:?}");
                None
            }
        }
    }

    fn find_tasks(node: &Node, tasks: &mut Vec<ActivityEntry>) {
        match node {
            Node::ListItem(_) => {
                if let Some(activity) = ObsidianParser::parse_task(node) {
                    tasks.push(activity);
                }
            }
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
