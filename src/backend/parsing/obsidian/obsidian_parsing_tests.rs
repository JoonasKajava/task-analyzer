#[cfg(test)]
mod tests {
    use anyhow::Result;
    use chrono::NaiveTime;

    use crate::backend::parsing::obsidian::obsidian::{ActivityEntry, ObsidianParser};

    #[test]
    fn test_simple() -> Result<()> {
        let simple = include_str!("./obsidian_parsing_test_files/simple.md");

        let parse_result = ObsidianParser::parse(simple)?;

        assert_eq!(
            parse_result,
            vec![
                ActivityEntry {
                    key: "JIRA:ABC-123".into(),
                    sub_key: None,
                    start_time: NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
                    end_time: NaiveTime::from_hms_opt(11, 0, 0)
                },
                ActivityEntry {
                    key: "JIRA:XYZ-999".into(),
                    sub_key: None,
                    start_time: NaiveTime::from_hms_opt(8, 20, 0).unwrap(),
                    end_time: NaiveTime::from_hms_opt(11, 43, 0)
                },
                ActivityEntry {
                    key: "JIRA:XYZ-999".into(),
                    sub_key: None,
                    start_time: NaiveTime::from_hms_opt(7, 0, 0).unwrap(),
                    end_time: None
                }
            ]
        );

        println!("{:?}", ObsidianParser::parse(simple));

        Ok(())
    }
}
