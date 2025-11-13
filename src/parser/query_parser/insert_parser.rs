use nom::{
    IResult,
    bytes::complete::tag,
    character::streaming::{alphanumeric1, space1},
    multi::separated_list1,
};

use crate::parser::query_parser::parse_tree::InsertQuery;

impl InsertQuery {
    // "insert into table1 (col1, col2) values (123, 'sdf');"
    pub fn parse(input: &str) -> IResult<&str, InsertQuery> {
        let (input, _) = tag("insert")(input)?;
        let (input, _) = space1(input)?;
        let (input, _) = tag("into")(input)?;
        let (input, _) = space1(input)?;
        let (input, table_name) = alphanumeric1(input)?;
        let (input, _) = space1(input)?;
        let (input, _) = tag("(")(input)?;
        let (input, columns) = Self::parse_column_list(input)?;
        let (input, _) = tag(")")(input)?;
        let (input, _) = space1(input)?;
        let (input, _) = tag("values")(input)?;
        let (input, _) = space1(input)?;
        let (input, _) = tag("(")(input)?;
        let (input, values) = Self::parse_column_list(input)?;
        let (input, _) = tag(")")(input)?;
        let (input, _) = tag(";")(input)?;
        Ok((
            input,
            InsertQuery {
                table_name: table_name.to_string(),
                columns: columns.iter().map(|s| s.to_string()).collect(),
                values: values.iter().map(|s| s.to_string()).collect(),
            },
        ))
    }

    fn parse_column_list(input: &str) -> IResult<&str, Vec<&str>> {
        separated_list1(tag(", "), alphanumeric1)(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::vec;

    #[test]
    fn inset_works() {
        let (remainder, insert_query) =
            InsertQuery::parse("insert into table1 (col1) values (123);").unwrap();
        assert_eq!(remainder, "");
        assert_eq!(
            insert_query,
            InsertQuery {
                table_name: "table1".to_string(),
                columns: vec!["col1".to_string()],
                values: vec!["123".to_string()]
            }
        );
    }

    #[test]
    fn insert_multiple_works() {
        let (remainder, queries) =
            InsertQuery::parse("insert into table1 (col1, col2) values (123, 456);").unwrap();
        assert_eq!(remainder, "");
        assert_eq!(
            queries,
            InsertQuery {
                table_name: "table1".to_string(),
                columns: vec!["col1".to_string(), "col2".to_string()],
                values: vec!["123".to_string(), "456".to_string()]
            }
        );
    }
}
