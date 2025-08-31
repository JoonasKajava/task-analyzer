

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::backend::parsing::obsidian::obsidian::ObsidianParser;


    #[test]
    fn test_simple() -> Result<()> {
        let simple = include_str!("./obsidian_parsing_test_files/simple.md");

        println!("{:?}", ObsidianParser::parse(simple));

        Ok(())
    }
}
