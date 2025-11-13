use nom::{IResult, bytes::complete::tag, multi::separated_list1};

use crate::parser::query_parser::parse_tree::{InsertQuery, Queries, Query, SelectQuery};

pub mod insert_parser;
pub mod parse_tree;
pub mod select_parser;

impl Queries {
    pub fn parse(input: &str) -> IResult<&str, Queries> {
        let (input, queries) = separated_list1(tag(";"), Query::parse)(input)?;
        Ok((input, Queries { queries }))
    }
}

impl Query {
    pub fn parse(input: &str) -> IResult<&str, Query> {
        // alt((SelectQuery::parse, InsertQuery::parse))(input)
        let select_query_attempt = SelectQuery::parse(input);
        if select_query_attempt.is_ok() {
            return select_query_attempt
                .map(|(input, select_query)| (input, Query::Select(select_query)));
        }

        let insert_query_attempt = InsertQuery::parse(input);
        if insert_query_attempt.is_ok() {
            return insert_query_attempt
                .map(|(input, insert_query)| (input, Query::Insert(insert_query)));
        };

        // Err(Err::Error((input, nom::error::ErrorKind::Tag)))
        panic!("Query not recognized!");
    }
}

// #[cfg(test)]
// mod test {
//     use std::vec;

//     use super::*;
// }
