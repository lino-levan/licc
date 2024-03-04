// pub enum CType {
//     Void,
//     Char,
//     SignedChar,
//     UnsignedChar,
//     Short,
//     UnsignedShort,
//     Int,
//     UnsignedInt,
//     Long,
//     UnsignedLong,
//     LongLong,
//     UnsignedLongLong,
//     Float,
//     Double,
//     LongDouble,
// }

// pub struct StructField {
//     name: String,
//     field_type: Type,
// }

// pub struct StructType {
//     fields: Vec<StructField>,
// }

// pub struct PointerType {
//     pointer_type: Box<Type>,
// }

// pub enum Type {
//     StructType(StructType),
//     PointerType(PointerType),
//     CType(CType),
// }

// pub struct FunctionParameter {
//     name: String,
//     parameter_type: Type,
// }

// pub struct FunctionType {
//     parameters: Vec<FunctionParameter>,
//     return_type: Type,
// }

// pub struct TypeDefiniton {
//     name: String,
//     definition_type: Type,
// }

// pub struct VariableDeclaration {
//     name: String,
//     variable_type: Type,
//     value: Option<Expression>,
// }

// pub struct FunctionDeclaration {
//     name: String,
//     parameters: Vec<FunctionParameter>,
//     return_type: Type,
//     body: Vec<Statement>,
// }

// pub enum TopLevelStatement {
//     TypeDefiniton(TypeDefiniton),
//     VariableDeclaration(VariableDeclaration),
//     FunctionDeclaration(FunctionDeclaration),
//     Struct(Struct),
//     Enum(Enum),
//     Directive(Directive),
// }

// pub struct Module {
//     statements: Vec<TopLevelStatement>,
// }
