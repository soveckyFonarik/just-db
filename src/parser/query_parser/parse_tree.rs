// use std::panic;

// use nom::{
//     Err, IResult,
//     branch::alt,
//     bytes::complete::tag,
//     character::streaming::{alpha1, alphanumeric1, i32, space1},
//     multi::separated_list1,
// };

/// *Queries*:
/// select col1,col2 from table1;
/// insert into table1 (col1,col2 ) values (1, 'valStr');
#[derive(Debug, PartialEq)]
pub struct Queries {
    pub queries: Vec<Query>,
}

#[derive(Debug, PartialEq)]
pub enum Query {
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
pub struct SelectQuery {
    pub select_statement: SelectStatement,
    pub from_statement: FromStatement,
    pub where_statement: Option<WhereStatement>,
    pub order_by_statement: Option<OrderByStatement>,
    pub group_by_statement: Option<GroupByStatement>,
    pub having_statement: Option<HavingStatement>,
    pub limit_statement: Option<LimitStatement>,
}

/// *InsertQuery*
/// insert into <table_name> (<columns>) values (<values>)
#[derive(Debug, PartialEq)]
pub struct InsertQuery {
    pub table_name: String,
    // TODO: design this
    pub columns: Vec<String>,
    // TODO: design this
    pub values: Vec<String>,
}

/// *DeleteQuery*
/// delete from <table_name> where <where_statement>
#[derive(Debug, PartialEq)]
pub struct DeleteQuery {
    pub table_name: String,
    pub where_statement: Option<WhereStatement>,
}

/// *UpdateQuery*
/// update <table_name> set <set_statement> where <where_statement>
#[derive(Debug, PartialEq)]
pub struct UpdateQuery {
    pub table_name: String,
    pub set_statement: SetStatement,
    pub where_statement: Option<WhereStatement>,
}

/// *CreateTableQuery*
/// create table <table_name> (<columns_definition>) <constraints>
#[derive(Debug, PartialEq)]
pub struct CreateTableQuery {
    pub table_name: String,
    // FIX
    pub columns_definition: Vec<String>,
    pub constraints: Vec<String>,
}

/// *DropTableQuery*
/// drop table <table_name>
#[derive(Debug, PartialEq)]
pub struct DropTableQuery {
    pub table_name: String,
}

/// *AlterTableQuery*
/// alter table <table_name> <action>
#[derive(Debug, PartialEq)]
pub struct AlterTableQuery {
    pub table_name: String,
    pub action: String,
}

/// *SelectStatement*:
#[derive(Debug, PartialEq)]
pub struct SelectStatement {
    pub columns: Vec<ColumnStatement>,
    pub distinct: bool,
}

#[derive(Debug, PartialEq)]
/// t1,t2, schema.t3, join t4 on t1.id = t4.id
pub struct FromStatement {
    pub tables: Vec<TableStatement>,
    pub joins: Vec<JoinStatement>,
}

#[derive(Debug, PartialEq)]
pub struct TableStatement {
    pub table_name: String,
    pub alias: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct JoinStatement {
    pub table_name: String,
    // TODO: design this
    pub join_type: String,
    pub on: Vec<Expr>,
}

#[derive(Debug, PartialEq)]
pub struct Expr {
    pub left: ColumnStatement,
    pub operator: Operator,
    pub right: ColumnStatement,
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Equal,
    NotEqual,
}

#[derive(Debug, PartialEq)]
pub struct WhereStatement {
    pub expr: Expr,
}

#[derive(Debug, PartialEq)]
pub struct OrderByStatement {
    pub columns: Vec<ColumnStatement>,
    pub order: Order,
}

#[derive(Debug, PartialEq)]
pub enum Order {
    Asc,
    Desc,
}

#[derive(Debug, PartialEq)]
pub struct LimitStatement {
    pub limit: i32,
}

#[derive(Debug, PartialEq)]
pub struct GroupByStatement {
    pub columns: Vec<ColumnStatement>,
}

#[derive(Debug, PartialEq)]
pub struct HavingStatement {
    pub expt: Vec<Expr>,
}

#[derive(Debug, PartialEq)]
pub struct SetStatement {
    // TODO: Design this
    pub columns: Vec<String>,
}

/// *ColumnStatement*
/// might be:
/// col1, tableName.col1, 11, 'tlalala', count(*)
#[derive(Debug, PartialEq)]
pub enum ColumnStatement {
    ColumnStatement(ColumnIdentifier),
    ColumnStatementLiteral(Literal),
    ColumnStatementFunction(Function),
}

#[derive(Debug, PartialEq)]
pub struct ColumnIdentifier {
    pub table_name: Option<String>,
    pub column_name: String,
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Integer(i32),
    String(String),
    Float(f32),
    Boolean(bool),
}

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub arguments: Vec<ColumnStatement>,
}

// #[cfg(test)]
// mod test {
//     use std::vec;

//     use super::*;
// }
