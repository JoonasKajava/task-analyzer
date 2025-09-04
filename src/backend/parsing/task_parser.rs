use chrono::NaiveTime;
use nom::{
    branch::alt,
    bytes::complete::{take_until, take_while, take_while1},
    combinator::{eof, opt},
    sequence::preceded,
    IResult, Parser,
};

use anyhow::{bail, Error, Result};

use crate::backend::parsing::obsidian::obsidian::ActivityEntry;

#[derive(Debug, PartialEq)]
struct TaskParser<'a> {
    start: &'a [u8],
    end: &'a [u8],
    jira: &'a [u8],
}

impl<'a> TaskParser<'a> {
    fn convert_bytes_to_str(bytes: &[u8]) -> Result<&str> {
        Ok(str::from_utf8(bytes)?)
    }

    fn parse(input: &[u8]) -> IResult<&[u8], TaskParser> {
        let is_valid_time = |c: u8| c.is_ascii_digit() || c == b':';

        let start = take_while1(is_valid_time);

        let time_padding = take_while(|c: u8| c == b' ' || c == b'-');

        let end = take_while(is_valid_time);

        let padding = take_until("JIRA:");

        let jira_tag = take_while1(|c| c != b' ');

        let (input, (start, _, end, _, jira)) =
            (start, time_padding, end, padding, jira_tag).parse(input)?;
        Ok((input, TaskParser { jira, start, end }))
    }

    fn parse_activity(input: &[u8]) -> Result<Option<ActivityEntry>> {
        let parse_result = TaskParser::parse(input);

        match parse_result {
            Ok(result) => {
                let start =
                    NaiveTime::parse_from_str(TaskParser::convert_bytes_to_str(result.1.start)?, "%R")?;
                let end =
                    NaiveTime::parse_from_str(TaskParser::convert_bytes_to_str(result.1.end)?, "%R")
                        .ok();

                Ok(Some(ActivityEntry {
                    key: TaskParser::convert_bytes_to_str(result.1.jira)?.to_string(),
                    sub_key: None,
                    start_time: start,
                    end_time: end,
                }))
            }
            Err(err) => match err {
                nom::Err::Error(inner_err) => match inner_err.code {
                    // Parsing was very likely successful, but no JIRA key was found
                    nom::error::ErrorKind::TakeUntil => Ok(None),
                    _ => bail!("Parser error")
                },
                _ => bail!("Parser error")
            },
        }
    }

}

#[cfg(test)]
mod tests {
    use anyhow::{Ok, Result};
    use chrono::NaiveTime;

    use crate::backend::parsing::{obsidian::obsidian::ActivityEntry, task_parser::TaskParser};

    #[test]
    fn test_parse() -> Result<()> {
        let result = TaskParser::parse_activity("08:00 - 11:00 JIRA:ABC-123 Some work on task".as_bytes())?;


        assert_eq!(
            result,
            Some(ActivityEntry {
                key: "JIRA:ABC-123".into(),
                sub_key: None,
                start_time: NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
                end_time: NaiveTime::from_hms_opt(11, 0, 0)
            }),
        );
        Ok(())
    }
    #[test]
    fn test_parse2() -> Result<()> {
        let result = TaskParser::parse_activity("08:00 - JIRA:ABC-123 Some work on task".as_bytes())?;

        assert_eq!(
            result,
            Some(ActivityEntry {
                key: "JIRA:ABC-123".into(),
                sub_key: None,
                start_time: NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
                end_time: None
            }),
        );
        Ok(())
    }

    #[test]
    fn test_parse3() -> Result<()> {
        let result = TaskParser::parse_activity("08:00 JIRA:ABC-123 Some work on task".as_bytes())?;
        assert_eq!(
            result,
            Some(ActivityEntry {
                key: "JIRA:ABC-123".into(),
                sub_key: None,
                start_time: NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
                end_time: None
            }),
        );
        Ok(())
    }

    #[test]
    fn test_parse4() -> Result<()> {
        let result = TaskParser::parse_activity("08:00 Some work on JIRA:ABC-123 task".as_bytes())?;
        assert_eq!(
            result,
            Some(ActivityEntry {
                key: "JIRA:ABC-123".into(),
                sub_key: None,
                start_time: NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
                end_time: None
            }),
        );
        Ok(())
    }

    #[test]
    fn test_parse_without_key() -> Result<()> {
        let result = TaskParser::parse_activity("08:00 Some work on task".as_bytes())?;

        assert!(result.is_none());
        Ok(())
    }
}
