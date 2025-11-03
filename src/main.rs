use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::streaming::{alpha1, alphanumeric1, i32, space1},
    multi::separated_list1,
};

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
struct Query {
    select: Option<SelectQuery>,
    insert: Option<InsertQuery>,
}

#[derive(Debug, PartialEq)]
struct SelectQuery {
    table: String,
    columns: Vec<String>,
}

#[derive(Debug, PartialEq)]
struct InsertQuery {
    table: String,
    values: Vec<Value>,
    columns: Vec<String>,
}

#[derive(Debug, PartialEq)]
struct Value {
    i_value: Option<i32>,
    s_value: Option<String>,
}

// insert into t1(col1, col2) values (1,2)
// select col1, col2 from t1;
// &str -> select_querty {columns:}
fn parse(input: &str) -> IResult<&str, Query> {
    alt((
        //Query {selectQuery}
        parse_select,
        //Query {InsertQuery}
        parse_insert,
    ))(input)
}

fn parse_select(input: &str) -> IResult<&str, Query> {
    let (input, _) = tag("select")(input)?;
    let (input, _) = space1(input)?;
    let (input, columns) = parse_column_list(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("from")(input)?;
    let (input, _) = space1(input)?;
    let (input, table) = alphanumeric1(input)?;
    // ;
    let (input, _) = tag(";")(input)?;
    Ok((
        input,
        Query {
            select: Some(SelectQuery {
                table: table.to_string(),
                columns: columns.iter().map(|s| s.to_string()).collect(),
            }),
            insert: None,
        },
    ))
}

fn parse_column_list(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag(", "), alphanumeric1)(input)
}

fn parse_values_list(input: &str) -> IResult<&str, Vec<Value>> {
    separated_list1(tag(", "), parse_value)(input)
}

fn parse_value(input: &str) -> IResult<&str, Value> {
    let res = i32(input);
    if res.is_ok() {
        return res.map(|(input, i_value)| {
            (
                input,
                Value {
                    i_value: Some(i_value),
                    s_value: None,
                },
            )
        });
    }
    let (input, _) = tag("'")(input)?;
    let (input, s) = alpha1(input)?;
    let (input, _) = tag("'")(input)?;
    Ok((
        input,
        Value {
            s_value: Some(s.to_string()),
            i_value: None,
        },
    ))

    // Err(nom::Err::Error((
    //     "No value found",
    //     nom::error::ErrorKind::Tag,
    // )))
}

// "insert into table1 (col1, col2) values (123, 'sdf');"
fn parse_insert(input: &str) -> IResult<&str, Query> {
    let (input, _) = tag("insert")(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("into")(input)?;
    let (input, _) = space1(input)?;
    let (input, table) = alphanumeric1(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, columns) = parse_column_list(input)?;
    let (input, _) = tag(")")(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("values")(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, values) = parse_values_list(input)?;
    let (input, _) = tag(")")(input)?;
    let (input, _) = tag(";")(input)?;
    // TODO:
    Ok((
        input,
        Query {
            insert: Some(InsertQuery {
                table: table.to_string(),
                columns: columns.iter().map(|s| s.to_string()).collect(),
                values: values,
            }),
            select: None,
        },
    ))
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    #[test]
    fn select_works() {
        let (remaind, query) = parse("select col1, col2 from table1;").unwrap();
        assert_eq!(remaind, "");
        assert_eq!(
            Query {
                insert: None,
                select: Some(SelectQuery {
                    table: "table1".to_string(),
                    columns: vec!["col1".to_string(), "col2".to_string()],
                }),
            },
            query
        );
    }

    #[test]
    fn inset_works() {
        let (remaind, query) =
            parse("insert into table1 (col1, col2) values (123, 'sdf');").unwrap();
        assert_eq!(remaind, "");
        assert_eq!(
            Query {
                select: None,
                insert: Some(InsertQuery {
                    table: "table1".to_string(),
                    columns: vec!["col1".to_string(), "col2".to_string()],
                    values: vec![
                        Value {
                            i_value: Some(123),
                            s_value: None,
                        },
                        Value {
                            i_value: None,
                            s_value: Some("sdf".to_string()),
                        }
                    ]
                }),
            },
            query
        );
    }

    #[test]
    fn update_works() {}
}
