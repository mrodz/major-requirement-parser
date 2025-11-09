use anyhow::{Context, Result};
use pest::Parser;

use crate::parser::{MQLParser, Rule};

mod parser;
mod test;

const EXAMPLE_3: &str = r#"SELECT 1 FROM CLASS(MATH 2250) : "must take MATH 2250";"#;

fn main() -> Result<()> {
    let mut result = MQLParser::parse(Rule::file, EXAMPLE_3)
        .map_err(|e| e.renamed_rules(parser::renamed_rules_impl))?;

    let parsed_mql_file =
        MQLParser::parse_file(result.next().context("File should have a child rule")?)?;

    dbg!(parsed_mql_file);

    Ok(())
}
