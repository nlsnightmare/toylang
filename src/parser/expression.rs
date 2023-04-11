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
    ArrayAssignment {
        identifier: String,
        index: Box<Expression>,
        value: Box<Expression>,
    },
    ArrayIndexing {
        identifier: Box<Expression>,
        index: Box<Expression>,
    },
    Return(Box<Expression>),
    Addition(BinaryExpression),
    Subtraction(BinaryExpression),
    Multiplication(BinaryExpression),
    LessEquals(BinaryExpression),
    LessThan(BinaryExpression),
    GreaterThan(BinaryExpression),
    GreaterEquals(BinaryExpression),
    Division(BinaryExpression),
    Or(BinaryExpression),
    And(BinaryExpression),
    BoolNegation(Box<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

impl BinaryExpression {
    pub fn new(left: Expression, right: Expression) -> BinaryExpression {
        Self {
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}
