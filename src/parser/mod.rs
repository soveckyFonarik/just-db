use std::panic;

use nom::{
    Err, IResult,
    branch::alt,
    bytes::complete::tag,
    character::streaming::{alpha1, alphanumeric1, i32, space1},
    multi::separated_list1,
};

/// TREE STRUCTRURE

/// *Queries*:
/// select col1,col2 from table1;
/// insert into table1 (col1,col2 ) values (1, 'valStr');
#[derive(Debug, PartialEq)]
struct Queries {
    queries: Vec<Query>,
}

#[derive(Debug, PartialEq)]
enum Query {
    // select
    Select(SelectQuery),
    // insert
    Insert(InsertQuery),
    // delete
    Delete(DeleteQuery),
    // update
    Update(UpdateQuery),
    // create table
    CreateTable(CreateTableQuery),
    // drop table
    DropTable(DropTableQuery),
    // alter table
    AlterTable(AlterTableQuery),
}

// *SeletQuery*
// select <select_statement>
// from <from_statement>
// where <where_statement>
// order by <order_by_statement>
// group by <group_by_statement>
// having <having_statement>
// limit <limit_statement>
#[derive(Debug, PartialEq)]
struct SelectQuery {
    select_statement: SelectStatement,
    from_statement: FromStatement,
    where_statement: Option<WhereStatement>,
    order_by_statement: Option<OrderByStatement>,
    group_by_statement: Option<GroupByStatement>,
    having_statement: Option<HavingStatement>,
    limit_statement: Option<LimitStatement>,
}

/// *InsertQuery*
/// insert into <table_name> (<columns>) values (<values>)
#[derive(Debug, PartialEq)]
struct InsertQuery {
    table_name: String,
    columns: Vec<String>,
    values: Vec<String>,
}

/// *DeleteQuery*
/// delete from <table_name> where <where_statement>
#[derive(Debug, PartialEq)]
struct DeleteQuery {
    table_name: String,
    where_statement: Option<WhereStatement>,
}

/// *UpdateQuery*
/// update <table_name> set <set_statement> where <where_statement>
#[derive(Debug, PartialEq)]
struct UpdateQuery {
    table_name: String,
    set_statement: SetStatement,
    where_statement: Option<WhereStatement>,
}

/// *CreateTableQuery*
/// create table <table_name> (<columns_definition>) <constraints>
#[derive(Debug, PartialEq)]
struct CreateTableQuery {
    table_name: String,
    // FIX
    columns_definition: Vec<String>,
    constraints: Vec<String>,
}

/// *DropTableQuery*
/// drop table <table_name>
#[derive(Debug, PartialEq)]
struct DropTableQuery {
    table_name: String,
}

/// *AlterTableQuery*
/// alter table <table_name> <action>
#[derive(Debug, PartialEq)]
struct AlterTableQuery {
    table_name: String,
    action: String,
}

/// *SelectStatement*:
#[derive(Debug, PartialEq)]
struct SelectStatement {
    columns: Vec<ColumnStatement>,
    distinct: bool,
}

#[derive(Debug, PartialEq)]
/// t1,t2, schema.t3, join t4 on t1.id = t4.id
struct FromStatement {
    tables: Vec<TableStatement>,
    joins: Vec<JoinStatement>,
}

#[derive(Debug, PartialEq)]
struct TableStatement {
    table_name: String,
    alias: Option<String>,
}

#[derive(Debug, PartialEq)]
struct JoinStatement {
    table_name: String,
    // TODO: design this
    join_type: String,
    on: Vec<Condition>,
}

#[derive(Debug, PartialEq)]
struct Condition {
    left: ColumnStatement,
    operator: Operator,
    right: ColumnStatement,
}

#[derive(Debug, PartialEq)]
enum Operator {
    Equal,
    NotEqual,
}

#[derive(Debug, PartialEq)]
struct WhereStatement {
    contitions: Vec<Condition>,
}

#[derive(Debug, PartialEq)]
struct OrderByStatement {
    columns: Vec<ColumnStatement>,
    order: Order,
}

#[derive(Debug, PartialEq)]
enum Order {
    Asc,
    Desc,
}

#[derive(Debug, PartialEq)]
struct LimitStatement {
    limit: i32,
}

#[derive(Debug, PartialEq)]
struct GroupByStatement {
    columns: Vec<ColumnStatement>,
}

#[derive(Debug, PartialEq)]
struct HavingStatement {
    contitions: Vec<Condition>,
}

#[derive(Debug, PartialEq)]
struct SetStatement {
    // TODO: Design this
    columns: Vec<String>,
}

/// *ColumnStatement*
/// might be:
/// col1, tableName.col1, 11, 'tlalala', count(*)
#[derive(Debug, PartialEq)]
enum ColumnStatement {
    ColumnStatement(ColumnIdentifier),
    ColumnStatementLiteral(Literal),
    ColumnStatementFunction(Function),
}

#[derive(Debug, PartialEq)]
struct ColumnIdentifier {
    table_name: Option<String>,
    column_name: String,
}

#[derive(Debug, PartialEq)]
enum Literal {
    Integer(i32),
    String(String),
    Float(f32),
    Boolean(bool),
}

#[derive(Debug, PartialEq)]
struct Function {
    name: String,
    arguments: Vec<ColumnStatement>,
}

// PARSERS

// insert into t1(col1, col2) values (1,2)
// select col1, col2 from t1;
// &str -> select_querty {columns:}
impl Queries {
    fn parse(input: &str) -> IResult<&str, Queries> {
        let (input, queries) = separated_list1(tag(";"), Query::parse)(input)?;
        Ok((input, Queries { queries }))
    }
}

impl Query {
    fn parse(input: &str) -> IResult<&str, Query> {
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

impl SelectQuery {
    fn parse(input: &str) -> IResult<&str, SelectQuery> {
        let (input, _) = tag("select")(input)?;
        let (input, _) = space1(input)?;
        let (input, select_statement) = SelectStatement::parse(input)?;
        let (input, _) = space1(input)?;
        let (input, _) = tag("from")(input)?;
        let (input, _) = space1(input)?;
        let (input, from_statement) = FromStatement::parse(input)?;
        // ;
        let (input, _) = tag(";")(input)?;
        // TODO: FIX
        Ok((
            input,
            SelectQuery {
                select_statement,
                from_statement,
                where_statement: None,
                order_by_statement: None,
                group_by_statement: None,
                having_statement: None,
                limit_statement: None,
            },
        ))
    }
}

impl SelectStatement {
    fn parse(input: &str) -> IResult<&str, SelectStatement> {
        let (input, columns) = separated_list1(tag(", "), ColumnStatement::parse)(input)?;
        Ok((
            input,
            SelectStatement {
                columns,
                //TODO: implement distinct
                distinct: false,
            },
        ))
    }
}

impl ColumnStatement {
    fn parse(input: &str) -> IResult<&str, ColumnStatement> {
        let res = ColumnIdentifier::parse(input);
        if res.is_ok() {
            return res.map(|(input, column_identifier)| {
                (input, ColumnStatement::ColumnStatement(column_identifier))
            });
        }

        let res = Literal::parse(input);
        if res.is_ok() {
            return res
                .map(|(input, literal)| (input, ColumnStatement::ColumnStatementLiteral(literal)));
        }

        let res = Function::parse(input);
        if res.is_ok() {
            return res.map(|(input, function)| {
                (input, ColumnStatement::ColumnStatementFunction(function))
            });
        }

        // alt((ColumnIdentifier||parse, Literal||parse, Function||parse));
        panic!("Column Statement not recognized")
    }
}

impl Literal {
    fn parse(input: &str) -> IResult<&str, Literal> {
        let res = i32(input);
        if res.is_ok() {
            return res.map(|(input, i_value)| (input, Literal::Integer(i_value)));
        }
        let (input, _) = tag("'")(input)?;
        let (input, s) = alpha1(input)?;
        let (input, _) = tag("'")(input)?;

        Ok((input, Literal::String(s.to_string())))
    }
}

impl Function {
    fn parse(input: &str) -> IResult<&str, Function> {
        let (input, name) = alpha1(input)?;
        let (input, _) = tag("(")(input)?;
        let (input, arguments) = separated_list1(tag(", "), ColumnStatement::parse)(input)?;
        let (input, _) = tag(")")(input)?;
        Ok((
            input,
            Function {
                name: name.to_string(),
                arguments,
            },
        ))
    }
}

impl ColumnIdentifier {
    fn parse(input: &str) -> IResult<&str, ColumnIdentifier> {
        let (input, column_name) = alphanumeric1(input)?;
        Ok((
            input,
            ColumnIdentifier {
                // TODO: implemetn table_name.col_name
                table_name: None,
                column_name: column_name.to_string(),
            },
        ))
    }
}

impl FromStatement {
    fn parse(input: &str) -> IResult<&str, FromStatement> {
        let (input, table) = TableStatement::parse(input)?;
        // let (input, _) = space1(input)?;
        // let (input, joins) = separated_list1(tag(" "), JoinStatement::parse)(input)?;
        Ok((
            input,
            FromStatement {
                tables: vec![table],
                // TODO
                joins: vec![],
            },
        ))
    }
}

impl TableStatement {
    fn parse(input: &str) -> IResult<&str, TableStatement> {
        let (input, table_name) = alphanumeric1(input)?;
        Ok((
            input,
            TableStatement {
                table_name: table_name.to_string(),
                // TODO: implements
                alias: None,
            },
        ))
    }
}

impl InsertQuery {
    // "insert into table1 (col1, col2) values (123, 'sdf');"
    fn parse(input: &str) -> IResult<&str, InsertQuery> {
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

// fn parse_value(input: &str) -> IResult<&str, Value> {
//     let res = i32(input);
//     if res.is_ok() {
//         return res.map(|(input, i_value)| {
//             (
//                 input,
//                 Value {
//                     i_value: Some(i_value),
//                     s_value: None,
//                 },
//             )
//         });
//     }
//     let (input, _) = tag("'")(input)?;
//     let (input, s) = alpha1(input)?;
//     let (input, _) = tag("'")(input)?;
//     Ok((
//         input,
//         Value {
//             s_value: Some(s.to_string()),
//             i_value: None,
//         },
//     ))
// }

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    #[test]
    fn select_works() {
        let (remaind, queries) = Queries::parse("select col1, col2 from table1;").unwrap();
        assert_eq!(remaind, "");
        assert_eq!(
            queries,
            Queries {
                queries: vec![Query::Select(SelectQuery {
                    select_statement: SelectStatement {
                        columns: vec![
                            ColumnStatement::ColumnStatement(ColumnIdentifier {
                                table_name: None,
                                column_name: "col1".to_string()
                            }),
                            ColumnStatement::ColumnStatement(ColumnIdentifier {
                                table_name: None,
                                column_name: "col2".to_string()
                            })
                        ],
                        distinct: false
                    },
                    from_statement: FromStatement {
                        tables: vec![TableStatement {
                            table_name: "table1".to_string(),
                            alias: None
                        }],
                        joins: vec![]
                    },
                    where_statement: None,
                    order_by_statement: None,
                    group_by_statement: None,
                    having_statement: None,
                    limit_statement: None
                })]
            }
        );
    }

    #[test]
    fn inset_works() {
        let (remainder, insert_query) =
            Queries::parse("insert into table1 (col1) values (123);").unwrap();
        assert_eq!(remainder, "");
        assert_eq!(
            insert_query,
            Queries {
                queries: vec![Query::Insert(InsertQuery {
                    table_name: "table1".to_string(),
                    columns: vec!["col1".to_string()],
                    values: vec!["123".to_string()]
                })]
            }
        );
    }

    #[test]
    fn insert_multiple_works() {
        let (remainder, queries) =
            Queries::parse("insert into table1 (col1, col2) values (123, 456);").unwrap();
        assert_eq!(remainder, "");
        assert_eq!(
            queries,
            Queries {
                queries: vec![Query::Insert(InsertQuery {
                    table_name: "table1".to_string(),
                    columns: vec!["col1".to_string(), "col2".to_string()],
                    values: vec!["123".to_string(), "456".to_string()]
                })]
            }
        );
    }
    // #[test]
    // fn update_works() {}
}
