use nom::{
    IResult,
    bytes::{complete::tag, streaming::tag_no_case},
    character::streaming::{alpha1, alphanumeric1, i32, multispace0, space1},
    combinator::opt,
    multi::separated_list1,
};

use crate::parser::query_parser::parse_tree::{
    ColumnIdentifier, ColumnStatement, Expr, FromStatement, Function, Literal, Operator,
    SelectQuery, SelectStatement, TableStatement, WhereStatement,
};

impl SelectQuery {
    pub fn parse(input: &str) -> IResult<&str, SelectQuery> {
        let (input, _) = multispace0(input)?;
        let (input, _) = tag_no_case("select")(input)?;
        let (input, _) = space1(input)?;
        let (input, _) = multispace0(input)?;

        let (input, select_statement) = SelectStatement::parse(input)?;

        let (input, _) = multispace0(input)?;
        let (input, _) = tag_no_case("from")(input)?;
        let (input, _) = space1(input)?;
        let (input, _) = multispace0(input)?;

        let (input, from_statement) = FromStatement::parse(input)?;

        // let (input, _) = space1(input)?;
        // let (input, _) = multispace0(input)?;

        // let (input, where_statement) = opt(WhereStatement::parse)(input)?;

        let (input, _) = multispace0(input)?;

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
    pub fn parse(input: &str) -> IResult<&str, SelectStatement> {
        let (input, columns) = separated_list1(tag(","), ColumnStatement::parse)(input)?;
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
    pub fn parse(input: &str) -> IResult<&str, ColumnStatement> {
        let (input, _) = multispace0(input)?;
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

impl TableStatement {
    pub fn parse(input: &str) -> IResult<&str, TableStatement> {
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

impl Function {
    pub fn parse(input: &str) -> IResult<&str, Function> {
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

impl FromStatement {
    pub fn parse(input: &str) -> IResult<&str, FromStatement> {
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

impl Literal {
    pub fn parse(input: &str) -> IResult<&str, Literal> {
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

impl ColumnIdentifier {
    fn parse(input: &str) -> IResult<&str, ColumnIdentifier> {
        let (input, _) = multispace0(input)?;
        let (input, column_name) = alphanumeric1(input)?;
        let (input, _) = multispace0(input)?;
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

impl WhereStatement {
    fn parse(input: &str) -> IResult<&str, WhereStatement> {
        let (input, _) = tag_no_case("where")(input)?;
        let (input, _) = space1(input)?;
        let (input, _) = multispace0(input)?;

        let (input, expr) = Expr::parse(input)?;

        Ok((input, WhereStatement { expr }))
    }
}

impl Expr {
    fn parse(input: &str) -> IResult<&str, Expr> {
        // todo!()
        let (input, left) = ColumnStatement::parse(input)?;
        let (input, _) = space1(input)?;
        let (input, operator) = tag("=")(input)?;
        let (input, _) = space1(input)?;
        let (input, right) = ColumnStatement::parse(input)?;

        Ok((
            input,
            Expr {
                left,
                operator: Operator::Equal,
                right,
            },
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;
    use std::vec;

    // #[test]
    #[rstest]
    #[case("SELECT col1, col2 FROM table1;")]
    #[case("select col1, col2 FROM table1;")]
    #[case("SELECT col1, col2 from table1;")]
    #[case("select col1, col2 from table1;")]
    #[case(" select col1, col2 from table1;")]
    #[case("     select col1, col2 from table1;")]
    #[case("  select  col1,  col2  from  table1;")]
    #[case("  select  \ncol1,  col2  \nfrom  \ntable1;")]
    #[case("  select  \r\ncol1,  col2  \r\nfrom  \r\ntable1;")]
    #[case("  select  \tcol1,  col2  \tfrom  \ttable1;")]
    fn select_works(#[case] input: &str) {
        let (remaind, queries) = SelectQuery::parse(input).unwrap();
        assert_eq!(remaind, "");
        assert_eq!(
            queries,
            SelectQuery {
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
            }
        );
    }

    #[rstest]
    #[case("SELECTcol1, col2 FROM table1;")]
    fn failed(#[case] input: &str) {
        let res = SelectQuery::parse(input);
        assert!(res.is_err());
    }

    #[rstest]
    #[case(
        r#"
        SELECT col1, col2, col3
        FROM table1
        where col1 = col2;
        "#
    )]
    fn select_where_works(#[case] input: &str) {
        use crate::parser::query_parser::parse_tree::Operator;

        let (remainder, queries) = SelectQuery::parse(input).unwrap();
        assert_eq!(remainder, "");
        assert_eq!(
            queries,
            SelectQuery {
                select_statement: SelectStatement {
                    columns: vec![
                        ColumnStatement::ColumnStatement(ColumnIdentifier {
                            table_name: None,
                            column_name: "col1".to_string()
                        }),
                        ColumnStatement::ColumnStatement(ColumnIdentifier {
                            table_name: None,
                            column_name: "col2".to_string()
                        }),
                        ColumnStatement::ColumnStatement(ColumnIdentifier {
                            table_name: None,
                            column_name: "col3".to_string()
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
                where_statement: Some(WhereStatement {
                    expr: Expr {
                        left: ColumnStatement::ColumnStatement(ColumnIdentifier {
                            table_name: None,
                            column_name: "col1".to_string()
                        }),
                        operator: Operator::Equal,
                        right: ColumnStatement::ColumnStatement(ColumnIdentifier {
                            table_name: None,
                            column_name: "col2".to_string()
                        })
                    }
                }),
                order_by_statement: None,
                group_by_statement: None,
                having_statement: None,
                limit_statement: None
            }
        )
    }
}
