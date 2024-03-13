#[derive(Debug)]
pub enum CType {
    Void,
    Char,
    SignedChar,
    UnsignedChar,
    Short,
    UnsignedShort,
    Int,
    UnsignedInt,
    Long,
    UnsignedLong,
    LongLong,
    UnsignedLongLong,
    Float,
    Double,
    LongDouble,
}

#[derive(Debug)]
pub enum Type {
    CType(CType),
}

#[derive(Debug)]
pub struct FunctionParameter {
    pub name: String,
    pub parameter_type: Type,
}

#[derive(Debug)]
pub struct FunctionDeclaration {
    pub name: String,
    pub parameters: Vec<FunctionParameter>,
    pub return_type: Type,
    pub body: Vec<Statement>
}

#[derive(Debug)]
pub enum TopLevelStatement {
    FunctionDeclaration(FunctionDeclaration),
}

#[derive(Debug)]
pub struct Module {
    pub statements: Vec<TopLevelStatement>,
}

#[derive(Debug)]
pub enum Expression {
    IntegerLiteral(i64)
}

#[derive(Debug)]
pub enum Statement {
    Return(Expression)
}
