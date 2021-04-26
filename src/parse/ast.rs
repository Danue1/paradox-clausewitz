#[derive(Debug, PartialEq)]
pub struct Ast {
    pub flavor: Flavor,
    pub encoding: Encoding,
    pub value_list: Vec<Value>,
}

#[derive(Debug, PartialEq)]
pub enum Flavor {
    Ck3, // Utf8
    Eu4, // Windows1252
}

#[derive(Debug, PartialEq)]
pub enum Encoding {
    Text,
    Binary,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    // key
    // @key
    Key(Key),
    // key = value
    // key = @value
    // @key = value
    // @key = @value
    KeyExpression(KeyExpression),
    // key = { value }
    // key = { @value }
    // key = tag { value }
    // key = tag { @value }
    // @key = { value }
    // @key = { @value }
    // @key = tag { value }
    // @key = tag { @value }
    KeyExpressionList(KeyExpressionList),
}

#[derive(Debug, PartialEq)]
pub struct Key {
    pub path: Path,
    pub is_variable: bool,
}

#[derive(Debug, PartialEq)]
pub struct KeyExpression {
    pub key: Key,
    pub expression: Box<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct KeyExpressionList {
    pub key: Key,
    pub tag: Option<Scalar>,
    pub expression_list: Vec<Expression>,
}

#[derive(Debug, PartialEq)]
pub enum Path {
    // foo
    Scalar(Scalar),
    // foo.bar
    Field(Scalar, Box<Path>),
    // foo:bar
    Parent(Scalar, Box<Path>),
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    // foo
    Value(Value),
    // foo < bar
    Binary(Key, Operator, Key),
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    GreaterThan,        // >
    GreaterThanOrEqual, // >=
    LessThan,           // <
    LessThanOrEqual,    // <=
}

#[derive(Debug, PartialEq, Clone)]
pub enum Scalar {
    Boolean(Boolean),
    Integer(i64),
    Decimal(f64),
    Datetime(Datetime),
    String(String),
    Ident(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Datetime {
    pub year: u16,
    pub month: u8,
    pub date: u8,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Boolean {
    True,
    False,

    Yes,
    No,
}

impl From<Boolean> for bool {
    fn from(boolean: Boolean) -> Self {
        match boolean {
            Boolean::True | Boolean::Yes => true,
            Boolean::False | Boolean::No => false,
        }
    }
}
