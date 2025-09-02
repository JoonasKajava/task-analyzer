use nom::{
    bytes::complete::{take_until, take_while, take_while1},
    IResult, Parser,
};

#[derive(Debug, PartialEq)]
struct TaskParser<'a> {
    start: &'a [u8],
    end: &'a [u8],
    jira: &'a [u8],
}

impl<'a> TaskParser<'a> {
    fn parse(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let is_valid_time = |c: u8| c.is_ascii_digit() || c == b':';

        let start = take_while1(is_valid_time);

        let time_padding = take_while(|c: u8| c == b' ' || c == b'-');

        let end = take_while(is_valid_time);

        let padding = take_until("JIRA:");

        let jira_tag = take_while1(|c| c != b' ');

        let (input, (start, _, end, _, jira)) =
            (start, time_padding, end, padding, jira_tag).parse(input)?;

        Ok((
            input,
            TaskParser {
                start,
                end,
                jira
            }
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::backend::parsing::task_parser::TaskParser;

    #[test]
    fn test_parse() {
        let result = TaskParser::parse("08:00 - 11:00 JIRA:ABC-123 Some work on task".as_bytes()).unwrap();
        assert_eq!(result.1, TaskParser {
            start: "08:00".as_bytes(),
            end: "11:00".as_bytes(),
            jira: "JIRA:ABC-123".as_bytes(),
        })
    }
    #[test]
    fn test_parse2() {
        let result = TaskParser::parse("08:00 - JIRA:ABC-123 Some work on task".as_bytes()).unwrap();
        assert_eq!(result.1, TaskParser {
            start: "08:00".as_bytes(),
            end: "".as_bytes(),
            jira: "JIRA:ABC-123".as_bytes(),
        })
    }

    #[test]
    fn test_parse3() {
        let result = TaskParser::parse("08:00 JIRA:ABC-123 Some work on task".as_bytes()).unwrap();
        assert_eq!(result.1, TaskParser {
            start: "08:00".as_bytes(),
            end: "".as_bytes(),
            jira: "JIRA:ABC-123".as_bytes(),
        })
    }

    #[test]
    fn test_parse4() {
        let result = TaskParser::parse("08:00  Some work on JIRA:ABC-123 task".as_bytes()).unwrap();
        assert_eq!(result.1, TaskParser {
            start: "08:00".as_bytes(),
            end: "".as_bytes(),
            jira: "JIRA:ABC-123".as_bytes(),
        })
    }
}
