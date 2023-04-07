pub type AST = Vec<Box<Expression>>;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    VariableDecleration {
        name: String,
        value: Box<Expression>,
    },
    VariableAssignment {
        name: String,
        value: Box<Expression>,
    },
    FunctionDefinition {
        name: String,
        arguments: Vec<String>,
        body: AST,
    },
    FunctionCall {
        name: String,
        arguments: AST,
    },
    IfCondition {
        condition: Box<Expression>,
        body: AST,
    },
    WhileLoop {
        condition: Box<Expression>,
        body: AST,
    },
    String(String),
    Number(f64),
    Variable(String),
    Bool(bool),
    Array(Vec<Box<Expression>>),
    ArrayIndexing {
        array: Box<Expression>,
        index: Box<Expression>,
    },
    Return(Box<Expression>),
    Addition {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Subtraction {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Multiplication {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    LessEquals {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    LessThan {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    GreaterThan {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    GreaterEquals {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Division {
        left: Box<Expression>,
        right: Box<Expression>,
    },
}
