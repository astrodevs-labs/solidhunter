use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StateMutability {
    #[serde(rename = "payable")]
    Payable,

    #[serde(rename = "pure")]
    Pure,

    #[serde(rename = "nonpayable")]
    NonPayable,

    #[serde(rename = "view")]
    View,
}

pub type SourceLocation = String;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mutability {
    #[serde(rename = "mutable")]
    Mutable,

    #[serde(rename = "immutable")]
    Immutable,

    #[serde(rename = "constant")]
    Constant,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageLocation {
    #[serde(rename = "default")]
    Default,

    #[serde(rename = "storage")]
    Storage,

    #[serde(rename = "memory")]
    Memory,

    #[serde(rename = "calldata")]
    Calldata,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Visibility {
    #[serde(rename = "public")]
    Public,

    #[serde(rename = "external")]
    External,

    #[serde(rename = "internal")]
    Internal,

    #[serde(rename = "private")]
    Private,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssignmentOperator {
    #[serde(rename = "=")]
    Equal,

    #[serde(rename = "+=")]
    PlusEqual,

    #[serde(rename = "-=")]
    MinusEqual,

    #[serde(rename = "*=")]
    StarEqual,

    #[serde(rename = "/=")]
    SlashEqual,

    #[serde(rename = "%=")]
    PercentEqual,

    #[serde(rename = "|=")]
    PipeEqual,

    #[serde(rename = "&=")]
    AmpersandEqual,

    #[serde(rename = "^=")]
    CaretEqual,

    #[serde(rename = ">>=")]
    RightShiftEqual,

    #[serde(rename = "<<=")]
    LeftShiftEqual,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinaryOperator {
    #[serde(rename = "+")]
    Plus,

    #[serde(rename = "-")]
    Minus,

    #[serde(rename = "*")]
    Star,

    #[serde(rename = "/")]
    Slash,

    #[serde(rename = "%")]
    Percent,

    #[serde(rename = "**")]
    DoubleStar,

    #[serde(rename = "&&")]
    DoubleAmpersand,

    #[serde(rename = "||")]
    DoublePipe,

    #[serde(rename = "!=")]
    ExclamationEqual,

    #[serde(rename = "==")]
    DoubleEqual,

    #[serde(rename = "<")]
    LessThan,

    #[serde(rename = "<=")]
    LessThanOrEqual,

    #[serde(rename = ">")]
    GreaterThan,

    #[serde(rename = ">=")]
    GreaterThanOrEqual,

    #[serde(rename = "^")]
    Caret,

    #[serde(rename = "&")]
    Ampersand,

    #[serde(rename = "|")]
    Pipe,

    #[serde(rename = "<<")]
    LeftShift,

    #[serde(rename = ">>")]
    RightShift,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnaryOperator {
    #[serde(rename = "!")]
    Exclamation,

    #[serde(rename = "-")]
    Minus,

    #[serde(rename = "+")]
    Plus,

    #[serde(rename = "++")]
    DoublePlus,

    #[serde(rename = "--")]
    DoubleMinus,

    #[serde(rename = "++")]
    PlusPlus,

    #[serde(rename = "--")]
    MinusMinus,

    #[serde(rename = "delete")]
    Delete,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FunctionCallKind {
    #[serde(rename = "functionCall")]
    FunctionCall,

    #[serde(rename = "structConstructorCall")]
    StructConstructorCall,

    #[serde(rename = "typeConversion")]
    TypeConversion,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LiteralKind {
    #[serde(rename = "number")]
    Number,

    #[serde(rename = "string")]
    String,

    #[serde(rename = "hexString")]
    HexString,

    #[serde(rename = "unicodeString")]
    UnicodeString,

    #[serde(rename = "bool")]
    Bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FunctionDefinitionKind {
    #[serde(rename = "function")]
    Function,

    #[serde(rename = "constructor")]
    Constructor,

    #[serde(rename = "fallback")]
    Fallback,

    #[serde(rename = "receive")]
    Receive,

    #[serde(rename = "modifier")]
    Modifier,

    #[serde(rename = "event")]
    Event,

    #[serde(rename = "freeFunction")]
    FreeFunction
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TypeName {
    ArrayTypeName(Box<ArrayTypeName>),
    ElementaryTypeName(Box<ElementaryTypeName>),
    FunctionTypeName(Box<FunctionTypeName>),
    Mapping(Box<Mapping>),
    UserDefinedTypeName(Box<UserDefinedTypeName>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Expression {
    Assignment(Box<Assignment>),
    BinaryOperation(Box<BinaryOperation>),
    Conditional(Box<Conditional>),
    ElementaryTypeNameExpression(Box<ElementaryTypeNameExpression>),
    FunctionCall(Box<FunctionCall>),
    FunctionCallOptions(Box<FunctionCallOptions>),
    Identifier(Box<Identifier>),
    IdentifierPath(Box<IdentifierPath>),
    IndexAccess(Box<IndexAccess>),
    IndexRangeAccess(Box<IndexRangeAccess>),
    Literal(Box<Literal>),
    MemberAccess(Box<MemberAccess>),
    NewExpression(Box<NewExpression>),
    TupleExpression(Box<TupleExpression>),
    UnaryOperation(Box<UnaryOperation>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Statement {
    Block(Box<Block>),
    Break(Box<Break>),
    Continue(Box<Continue>),
    DoWhileStatement(Box<DoWhileStatement>),
    EmitStatement(Box<EmitStatement>),
    ExpressionStatement(Box<ExpressionStatement>),
    ForStatement(Box<ForStatement>),
    IfStatement(Box<IfStatement>),
    PlaceholderStatement(Box<PlaceholderStatement>),
    Return(Box<Return>),
    RevertStatement(Box<RevertStatement>),
    TryStatement(Box<TryStatement>),
    UncheckedBlock(Box<UncheckedBlock>),
    VariableDeclarationStatement(Box<VariableDeclarationStatement>),
    WhileStatement(Box<WhileStatement>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeType {
    // Expressions
    Assignment,
    BinaryOperation,
    Conditional,
    ElementaryTypeNameExpression,
    FunctionCall,
    FunctionCallOptions,
    Identifier,
    IdentifierPath,
    IndexAccess,
    IndexRangeAccess,
    Literal,
    MemberAccess,
    NewExpression,
    TupleExpression,
    UnaryOperation,

    // Statements
    Block,
    Break,
    Continue,
    DoWhileStatement,
    EmitStatement,
    ExpressionStatement,
    ForStatement,
    IfStatement,
    InlineAssembly,
    PlaceholderStatement,
    Return,
    RevertStatement,
    TryStatement,
    UncheckedBlock,
    VariableDeclarationStatement,
    VariableDeclaration,
    WhileStatement,

    // Definitions
    ContractDefinition,
    FunctionDefinition,
    EventDefinition,
    ErrorDefinition,
    ModifierDefinition,
    StructDefinition,
    EnumDefinition,
    EnumValue,
    UserDefinedValueTypeDefinition,

    // Directives
    PragmaDirective,
    ImportDirective,
    UsingForDirective,

    // Misc
    SourceUnit,
    InheritanceSpecifier,
    ElementaryTypeName,
    FunctionTypeName,
    ParameterList,
    TryCatchClause,
    ModifierInvocation,
    StructuredDocumentation,
    UserDefinedTypeName,

    /// An unknown AST node type.
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnumDefinition {
    id: usize,
    src: SourceLocation,
    name: String,
    #[serde(rename = "nameLocation", skip_serializing_if = "Option::is_none")]
    name_location: Option<String>,
    #[serde(rename = "canonicalName")]
    canonical_name: String,
    members: Vec<EnumValue>,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnumValue {
    id: usize,
    src: SourceLocation,
    name: String,
    #[serde(rename = "nameLocation", skip_serializing_if = "Option::is_none")]
    name_location: Option<String>,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeDescriptions {
    #[serde(rename = "typeIdentifier", skip_serializing_if = "Option::is_none")]
    type_identifier: Option<String>,
    #[serde(rename = "typeString", skip_serializing_if = "Option::is_none")]
    type_string: Option<String>
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructuredDocumentation {
    id: usize,
    src: SourceLocation,
    text: String,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructureFunction {
    function: IdentifierPath
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SourceUnitChildNodes {
    ContractDefinition(Box<ContractDefinition>),
    StructDefinition(Box<StructDefinition>),
    EnumDefinition(Box<EnumDefinition>),
    ErrorDefinition(Box<ErrorDefinition>),
    UsingForDirective(Box<UsingForDirective>),
    PragmaDirective(Box<PragmaDirective>),
    ImportDirective(Box<ImportDirective>),
    Other(String)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceUnit {
    id: usize,
    src: SourceLocation,
    #[serde(rename = "absolutePath")]
    absolute_path: String,
    #[serde(rename = "exportedSymbols")]
    exported_symbols: Option<HashMap<String, Vec<String>>>,
    #[serde(rename = "license", skip_serializing_if = "Option::is_none")]
    license: Option<String>,
    #[serde(rename = "nodes")]
    nodes: Vec<SourceUnitChildNodes>,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContractDefinitionChildNodes {
    FunctionDefinition(Box<FunctionDefinition>),
    ModifierDefinition(Box<ModifierDefinition>),
    StructDefinition(Box<StructDefinition>),
    UserDefinedValueTypeDefinition(Box<UserDefinedValueTypeDefinition>),
    UsingForDirective(Box<UsingForDirective>),
    VariableDeclaration(Box<VariableDeclaration>),
    EnumDefinition(Box<EnumDefinition>),
    ErrorDefinition(Box<ErrorDefinition>),
    EventDefinition(Box<EventDefinition>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractKind {
    #[serde(rename = "contract")]
    Contract,
    #[serde(rename = "interface")]
    Interface,
    #[serde(rename = "library")]
    Library,
    Other(String)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContractDefinition {
    id: usize,
    src: SourceLocation,
    name: String,
    #[serde(rename = "nameLocation", skip_serializing_if = "Option::is_none")]
    name_location: Option<String>,
    #[serde(rename = "abstract")]
    is_abstract: bool,
    #[serde(rename = "baseContracts")]
    base_contracts: Vec<InheritanceSpecifier>,
    #[serde(rename = "canonicalName", skip_serializing_if = "Option::is_none")]
    canonical_name: Option<String>,
    #[serde(rename = "contractDependencies")]
    contract_dependencies: Vec<usize>,
    #[serde(rename = "contractKind")]
    contract_kind: ContractKind,
    #[serde(rename = "documentation", skip_serializing_if = "Option::is_none")]
    documentation: Option<StructuredDocumentation>,
    #[serde(rename = "fullyImplemented")]
    is_fully_implemented: Option<bool>,
    #[serde(rename = "linearizedBaseContracts")]
    linearized_base_contracts: Option<Vec<usize>>,
    #[serde(rename = "nodes")]
    nodes: Vec<ContractDefinitionChildNodes>,
    #[serde(rename = "scope")]
    scope: Option<usize>,
    #[serde(rename = "usedErrors")]
    used_errors: Vec<usize>,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BaseName {
    UserDefinedTypeName(UserDefinedTypeName),
    IdentifierPath(IdentifierPath),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InheritanceSpecifier {
    id: usize,
    src: SourceLocation,
    #[serde(rename = "arguments")]
    arguments: Option<Vec<Expression>>,
    #[serde(rename = "baseName")]
    base_name: BaseName,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Assignment {
    id: usize,
    src: SourceLocation,
    #[serde(rename = "argumentTypes", skip_serializing_if = "Option::is_none")]
    argument_types: Option<Vec<TypeDescriptions>>,
    #[serde(rename = "isConstant")]
    is_constant: Option<bool>,
    #[serde(rename = "isLValue")]
    is_l_value: Option<bool>,
    #[serde(rename = "isPure")]
    is_pure: Option<bool>,
    #[serde(rename = "lValueRequested")]
    l_value_requested: Option<bool>,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
    #[serde(rename = "leftHandSide")]
    left_hand_side: Expression,
    operator: AssignmentOperator,
    #[serde(rename = "rightHandSide")]
    right_hand_side: Expression,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BinaryOperation {
    id: usize,
    src: SourceLocation,
    #[serde(rename = "argumentTypes", skip_serializing_if = "Option::is_none")]
    argument_types: Option<Vec<TypeDescriptions>>,
    #[serde(rename = "isConstant")]
    is_constant: bool,
    #[serde(rename = "isLValue")]
    is_l_value: bool,
    #[serde(rename = "isPure")]
    is_pure: bool,
    #[serde(rename = "lValueRequested")]
    l_value_requested: bool,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
    #[serde(rename = "commonType")]
    common_type: TypeDescriptions,
    #[serde(rename = "leftExpression")]
    left_expression: Expression,
    operator: BinaryOperator,
    #[serde(rename = "rightExpression")]
    right_expression: Expression,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Conditional {
    id: usize,
    src: SourceLocation,
    #[serde(rename = "argumentTypes", skip_serializing_if = "Option::is_none")]
    argument_types: Option<Vec<TypeDescriptions>>,
    #[serde(rename = "isConstant")]
    is_constant: bool,
    #[serde(rename = "isLValue")]
    is_l_value: bool,
    #[serde(rename = "isPure")]
    is_pure: bool,
    #[serde(rename = "lValueRequested")]
    l_value_requested: bool,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
    #[serde(rename = "condition")]
    condition: Expression,
    #[serde(rename = "falseExpression")]
    false_expression: Expression,
    #[serde(rename = "trueExpression")]
    true_expression: Expression,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ElementaryTypeNameExpression {
    id: usize,
    src: SourceLocation,
    #[serde(rename = "argumentTypes", skip_serializing_if = "Option::is_none")]
    argument_types: Option<Vec<TypeDescriptions>>,
    #[serde(rename = "isConstant")]
    is_constant: bool,
    #[serde(rename = "isLValue")]
    is_l_value: bool,
    #[serde(rename = "isPure")]
    is_pure: bool,
    #[serde(rename = "lValueRequested")]
    l_value_requested: bool,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
    #[serde(rename = "typeName")]
    type_name: ElementaryTypeName,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ElementaryTypeName {
    id: usize,
    src: SourceLocation,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
    name: String,
    #[serde(rename = "stateMutability")]
    state_mutability: Option<StateMutability>,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunctionCall {
    id: usize,
    src: SourceLocation,
    #[serde(rename = "argumentTypes", skip_serializing_if = "Option::is_none")]
    argument_types: Option<Vec<TypeDescriptions>>,
    #[serde(rename = "isConstant")]
    is_constant: Option<bool>,
    #[serde(rename = "isLValue")]
    is_l_value: Option<bool>,
    #[serde(rename = "isPure")]
    is_pure: Option<bool>,
    #[serde(rename = "lValueRequested")]
    l_value_requested: Option<bool>,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
    #[serde(rename = "arguments")]
    arguments: Vec<Expression>,
    #[serde(rename = "expression")]
    expression: Expression,
    kind: Option<FunctionCallKind>,
    names: Vec<String>,
    #[serde(rename = "tryCall")]
    try_call: bool,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunctionCallOptions {
    id: usize,
    src: SourceLocation,
    #[serde(rename = "argumentTypes", skip_serializing_if = "Option::is_none")]
    argument_types: Option<Vec<TypeDescriptions>>,
    #[serde(rename = "isConstant")]
    is_constant: bool,
    #[serde(rename = "isLValue")]
    is_l_value: bool,
    #[serde(rename = "isPure")]
    is_pure: bool,
    #[serde(rename = "lValueRequested")]
    l_value_requested: bool,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
    expression: Expression,
    names: Vec<String>,
    options: Vec<Expression>,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Identifier {
    id: usize,
    src: SourceLocation,
    #[serde(rename = "argumentTypes", skip_serializing_if = "Option::is_none")]
    argument_types: Option<Vec<TypeDescriptions>>,
    name: String,
    #[serde(rename = "overloadedDeclarations")]
    overloaded_declarations: Vec<usize>,
    #[serde(rename = "referencedDeclaration")]
    referenced_declaration: Option<usize>,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexAccess {
    id: usize,
    src: SourceLocation,
    #[serde(rename = "argumentTypes", skip_serializing_if = "Option::is_none")]
    argument_types: Option<Vec<TypeDescriptions>>,
    #[serde(rename = "isConstant")]
    is_constant: bool,
    #[serde(rename = "isLValue")]
    is_l_value: bool,
    #[serde(rename = "isPure")]
    is_pure: bool,
    #[serde(rename = "lValueRequested")]
    l_value_requested: bool,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
    #[serde(rename = "baseExpression")]
    base_expression: Expression,
    #[serde(rename = "indexExpression", skip_serializing_if = "Option::is_none")]
    index_expression: Option<Expression>,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexRangeAccess {
    id: usize,
    src: SourceLocation,
    #[serde(rename = "argumentTypes", skip_serializing_if = "Option::is_none")]
    argument_types: Option<Vec<TypeDescriptions>>,
    #[serde(rename = "isConstant")]
    is_constant: bool,
    #[serde(rename = "isLValue")]
    is_l_value: bool,
    #[serde(rename = "isPure")]
    is_pure: bool,
    #[serde(rename = "lValueRequested")]
    l_value_requested: bool,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
    #[serde(rename = "baseExpression")]
    base_expression: Expression,
    #[serde(rename = "endExpression", skip_serializing_if = "Option::is_none")]
    end_expression: Option<Expression>,
    #[serde(rename = "startExpression", skip_serializing_if = "Option::is_none")]
    start_expression: Option<Expression>,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Literal {
    id: usize,
    src: SourceLocation,
    #[serde(rename = "argumentTypes", skip_serializing_if = "Option::is_none")]
    argument_types: Option<Vec<TypeDescriptions>>,
    #[serde(rename = "isConstant")]
    is_constant: Option<bool>,
    #[serde(rename = "isLValue")]
    is_l_value: Option<bool>,
    #[serde(rename = "isPure")]
    is_pure: Option<bool>,
    #[serde(rename = "lValueRequested")]
    l_value_requested: Option<bool>,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
    #[serde(rename = "hexValue")]
    hex_value: String,
    kind: LiteralKind,
    //#[serde(rename = "subdenomination", skip_serializing_if = "Option::is_none")]
    //subdenomination: Option<Subdenomination>,
    value: Option<String>,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemberAccess {
    id: usize,
    src: SourceLocation,
    #[serde(rename = "argumentTypes", skip_serializing_if = "Option::is_none")]
    argument_types: Option<Vec<TypeDescriptions>>,
    #[serde(rename = "isConstant")]
    is_constant: Option<bool>,
    #[serde(rename = "isLValue")]
    is_l_value: Option<bool>,
    #[serde(rename = "isPure")]
    is_pure: Option<bool>,
    #[serde(rename = "lValueRequested")]
    l_value_requested: Option<bool>,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
    #[serde(rename = "expression")]
    expression: Expression,
    #[serde(rename = "memberLocation")]
    member_location: SourceLocation,
    #[serde(rename = "memberName")]
    member_name: String,
    #[serde(rename = "referencedDeclaration")]
    referenced_declaration: Option<usize>,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewExpression {
    id: usize,
    src: SourceLocation,
    #[serde(rename = "argumentTypes", skip_serializing_if = "Option::is_none")]
    argument_types: Option<Vec<TypeDescriptions>>,
    #[serde(rename = "isConstant")]
    is_constant: bool,
    #[serde(rename = "isLValue")]
    is_l_value: bool,
    #[serde(rename = "isPure")]
    is_pure: bool,
    #[serde(rename = "lValueRequested")]
    l_value_requested: bool,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
    #[serde(rename = "typeName")]
    type_name: TypeName,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArrayTypeName {
    id: usize,
    src: SourceLocation,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
    #[serde(rename = "baseType")]
    base_type: TypeName,
    length: Option<Expression>,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunctionTypeName {
    id : usize,
    src: SourceLocation,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
    #[serde(rename = "parameterTypes")]
    parameter_types: ParameterList,
    #[serde(rename = "returnParameterTypes")]
    return_parameter_types: ParameterList,
    #[serde(rename = "stateMutability")]
    state_mutability: StateMutability,
    visibility: Visibility,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParameterList {
    id: usize,
    src: SourceLocation,
    parameters: Vec<VariableDeclaration>,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VariableDeclaration {
    id: usize,
    src: SourceLocation,
    name: String,
    #[serde(rename = "nameLocation")]
    name_location: Option<String>,
    #[serde(rename = "baseFunctions", skip_serializing_if = "Option::is_none")]
    base_functions: Option<Vec<usize>>,
    #[serde(rename = "constant")]
    is_constant: bool,
    documentation: Option<StructuredDocumentation>,
    #[serde(rename = "functionSelector")]
    function_selector: Option<String>,
    indexed: Option<bool>,
    mutability: Mutability,
    overrides: Option<OverrideSpecifier>,
    scope: Option<usize>,
    #[serde(rename = "stateVariable")]
    state_variable: bool,
    #[serde(rename = "storageLocation")]
    storage_location: StorageLocation,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
    #[serde(rename = "typeName", skip_serializing_if = "Option::is_none")]
    type_name: Option<TypeName>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    value: Option<Expression>,
    visibility: Visibility,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OverridesEnum {
    UserDefinedTypeName(Vec<UserDefinedTypeName>),
    Identifier(Vec<IdentifierPath>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OverrideSpecifier {
    id: usize,
    src: SourceLocation,
    overrides: OverridesEnum,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserDefinedTypeName {
    id: usize,
    src: SourceLocation,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
    //#[serde(rename = "contractScope")]
    //contract_scope: Option<???>,
    name: Option<String>,
    #[serde(rename = "pathNode")]
    path_node: Option<IdentifierPath>,
    #[serde(rename = "referencedDeclaration")]
    referenced_declaration: Option<usize>,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentifierPath {
    id: usize,
    src: SourceLocation,
    name: String,
    #[serde(rename = "nameLocations")]
    name_locations: Option<Vec<String>>,
    #[serde(rename = "referencedDeclaration")]
    referenced_declaration: Option<usize>,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Mapping {
    id: usize,
    src: SourceLocation,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
    #[serde(rename = "keyType")]
    key_type: TypeName,
    #[serde(rename = "valueType")]
    value_type: TypeName,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TupleExpression {
    id: usize,
    src: SourceLocation,
    #[serde(rename = "argumentTypes")]
    argument_types: Option<Vec<TypeDescriptions>>,
    #[serde(rename = "isConstant")]
    is_constant: bool,
    #[serde(rename = "isLValue")]
    is_l_value: bool,
    #[serde(rename = "isPure")]
    is_pure: bool,
    #[serde(rename = "lValueRequested")]
    l_value_requested: bool,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
    components: Vec<Expression>,
    #[serde(rename = "isInlineArray")]
    is_inline_array: bool,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnaryOperation {
    id: usize,
    src: SourceLocation,
    #[serde(rename = "argumentTypes")]
    argument_types: Option<Vec<TypeDescriptions>>,
    #[serde(rename = "isConstant")]
    is_constant: bool,
    #[serde(rename = "isLValue")]
    is_l_value: bool,
    #[serde(rename = "isPure")]
    is_pure: bool,
    #[serde(rename = "lValueRequested")]
    l_value_requested: bool,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
    #[serde(rename = "operator")]
    operator: UnaryOperator,
    prefix: bool,
    #[serde(rename = "subExpression")]
    sub_expression: Expression,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorDefinition {
    id: usize,
    src: SourceLocation,
    name: String,
    #[serde(rename = "nameLocation")]
    name_location: String,
    documentation: Option<StructuredDocumentation>,
    #[serde(rename = "errorSelector")]
    error_selector: Option<String>,
    parameters: Option<ParameterList>,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventDefinition {
    id: usize,
    src: SourceLocation,
    name: String,
    #[serde(rename = "nameLocation")]
    name_location: Option<String>,
    anonymous: bool,
    #[serde(rename = "eventSelector")]
    event_selector: Option<String>,
    documentation: Option<StructuredDocumentation>,
    parameters: Option<ParameterList>,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunctionDefinition {
    id: usize,
    src: SourceLocation,
    name: String,
    #[serde(rename = "nameLocation")]
    name_location: Option<String>,
    #[serde(rename = "baseFunctions")]
    base_functions: Option<Vec<usize>>,
    body: Option<Block>,
    #[serde(rename = "documentation")]
    documentation: Option<StructuredDocumentation>,
    #[serde(rename = "functionSelector")]
    function_selector: Option<String>,
    implemented: bool,
    kind: FunctionDefinitionKind,
    modifiers: Vec<ModifierInvocation>,
    overrides: Option<OverrideSpecifier>,
    parameters: ParameterList,
    #[serde(rename = "returnParameters")]
    return_parameters: ParameterList,
    #[serde(rename = "scope")]
    scope: Option<usize>,
    #[serde(rename = "stateMutability")]
    state_mutability: StateMutability,
    #[serde(rename = "virtual")]
    is_virtual: bool,
    visibility: Visibility,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Block {
    id: usize,
    src: SourceLocation,
    documentation: Option<String>,
    statements: Option<Vec<Statement>>,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Break {
    id: usize,
    src: SourceLocation,
    documentation: Option<String>,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Continue {
    id: usize,
    src: SourceLocation,
    documentation: Option<String>,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Body {
    Block(Box<Block>),
    Statement(Box<Statement>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoWhileStatement {
    id: usize,
    src: SourceLocation,
    documentation: Option<String>,
    #[serde(rename = "condition")]
    condition: Expression,
    #[serde(rename = "body")]
    body: Body,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmitStatement {
    id: usize,
    src: SourceLocation,
    documentation: Option<String>,
    #[serde(rename = "eventCall")]
    event_call: FunctionCall,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpressionStatement {
    id: usize,
    src: SourceLocation,
    documentation: Option<String>,
    expression: Expression,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InitializationExpression {
    ExpressionStatement(ExpressionStatement),
    VariableDeclarationStatement(VariableDeclarationStatement)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForStatement {
    id: usize,
    src: SourceLocation,
    documentation: Option<String>,
    body: Body,
    condition: Option<Expression>,
    #[serde(rename = "initializationExpression")]
    initialization_expression: Option<InitializationExpression>,
    #[serde(rename = "loopExpression")]
    loop_expression: Option<ExpressionStatement>,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VariableDeclarationStatement {
    id: usize,
    src: SourceLocation,
    documentation: Option<String>,
    assignments: Vec<Option<usize>>,
    #[serde(rename = "declarations")]
    declarations: Vec<Option<VariableDeclaration>>,
    #[serde(rename = "initialValue")]
    initial_value: Option<Expression>,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IfStatementBody {
    Block(Block),
    Statement(Statement),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IfStatement {
    id: usize,
    src: SourceLocation,
    documentation: Option<String>,
    #[serde(rename = "condition")]
    condition: Expression,
    #[serde(rename = "trueBody")]
    true_body: IfStatementBody,
    #[serde(rename = "falseBody")]
    false_body: Option<IfStatementBody>,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaceholderStatement {
    id: usize,
    src: SourceLocation,
    documentation: Option<String>,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Return {
    id: usize,
    src: SourceLocation,
    documentation: Option<String>,
    expression: Option<Expression>,
    #[serde(rename = "functionReturnParameters")]
    function_return_parameters: Option<usize>,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RevertStatement {
    id: usize,
    src: SourceLocation,
    documentation: Option<String>,
    #[serde(rename = "errorCall")]
    error_call: NodeType,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TryStatement {
    id: usize,
    src: SourceLocation,
    documentation: Option<String>,
    clauses: Vec<TryCatchClause>,
    #[serde(rename = "externalCall")]
    external_call: Expression,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TryCatchClause {
    id: usize,
    src: SourceLocation,
    block: Block,
    #[serde(rename = "errorName")]
    error_name: String,
    parameters: Option<ParameterList>,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UncheckedBlock {
    id: usize,
    src: SourceLocation,
    documentation: Option<String>,
    statements: Vec<Statement>,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WhileStatement {
    id: usize,
    src: SourceLocation,
    documentation: Option<String>,
    body: Statement,
    condition: Expression,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModifierName {
    Identifier(Identifier),
    IdentifierPath(IdentifierPath),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModifierInvocationKind {
    #[serde(rename = "modifierInvocation")]
    ModifierInvocation,
    #[serde(rename = "baseConstructorSpecifier")]
    BaseConstructorSpecifier,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModifierInvocation {
    id: usize,
    src: SourceLocation,
    arguments: Option<Expression>,
    kind: Option<ModifierInvocationKind>,
    #[serde(rename = "modifierName")]
    modifier_name: ModifierName,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModifierDefinition {
    id: usize,
    src: SourceLocation,
    name: String,
    #[serde(rename = "nameLocation")]
    name_location: Option<String>,
    #[serde(rename = "baseModifiers")]
    base_modifiers: Option<Vec<usize>>,
    body: Statement,
    documentation: Option<StructuredDocumentation>,
    overrides: Option<OverrideSpecifier>,
    parameters: ParameterList,
    #[serde(rename = "virtual")]
    is_virtual: bool,
    visibility: Visibility,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructDefinition {
    id: usize,
    src: SourceLocation,
    name: String,
    #[serde(rename = "nameLocation")]
    name_location: Option<String>,
    #[serde(rename = "canonicalName")]
    canonical_name: String,
    members: Vec<VariableDeclaration>,
    scope: usize,
    visibility: Visibility,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserDefinedValueTypeDefinition {
    id: usize,
    src: SourceLocation,
    name: String,
    #[serde(rename = "nameLocation")]
    name_location: Option<String>,
    #[serde(rename = "canonicalName")]
    canonical_name: Option<String>,
    #[serde(rename = "underlyingType")]
    underlying_type: TypeName,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UsingForDirective {
    id: usize,
    src: SourceLocation,
    #[serde(rename = "functionList")]
    function_list: Option<Vec<StructureFunction>>,
    function: Option<IdentifierPath>,
    global: Option<bool>,
    #[serde(rename = "libraryName")]
    library_name: Option<Expression>,
    #[serde(rename = "typeName")]
    type_name: Option<TypeName>,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SymbolAlias {
    foreign: Identifier,
    local: Option<String>,
    name_location: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportDirective {
    id: usize,
    src: SourceLocation,
    #[serde(rename = "absolutePath")]
    absolute_path: String,
    file: String,
    #[serde(rename = "nameLocation")]
    name_location: Option<String>,
    scope: Option<usize>,
    #[serde(rename = "sourceUnit")]
    source_unit: Option<usize>,
    #[serde(rename = "symbolAliases")]
    symbol_aliases: Vec<SymbolAlias>,
    #[serde(rename = "unitAlias")]
    unit_alias: String,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PragmaDirective {
    id: usize,
    src: SourceLocation,
    literals: Vec<String>,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn test_correct_EnumValue_parsing() -> Result<(), String> {
        let ast = fs::read_to_string("../solc-wrapper/tests/files/ast/EnumValue.json").expect("Could not find test data file");
        let res = serde_json::from_str::<EnumValue>(&ast).map_err(|_| "Error deserializing EnumValue".to_string())?;
        assert_eq!(res.name, "item1");
        assert_eq!(res.name_location, Some("72:5:0".to_string()));
        Ok(assert_eq!(res.node_type, NodeType::EnumValue))
    }

    #[test]
    fn test_correct_enum_definition_parsing() -> Result<(), String> {
        let ast = fs::read_to_string("../solc-wrapper/tests/files/ast/EnumDefinition.json").expect("Could not find test data file");
        let res = serde_json::from_str::<EnumDefinition>(&ast).map_err(|_| "Error deserializing EnumDefinition".to_string())?;
        assert_eq!(res.canonical_name, "Test");
        assert_eq!(res.name_location, Some("66:4:0".to_string()));
        Ok(assert_eq!(res.node_type, NodeType::EnumDefinition))
    }

    #[test]
    fn test_correct_event_definition_parsing() -> Result<(), String> {
        let ast = fs::read_to_string("../solc-wrapper/tests/files/ast/EventDefinition.json").expect("Could not find test data file");
        let res = serde_json::from_str::<EventDefinition>(&ast).map_err(|_| "Error deserializing EventDefinition".to_string())?;
        assert_eq!(res.anonymous, false);
        assert_eq!(res.id, 67);
        assert_eq!(res.name, "MintingLocked".to_string());
        assert_eq!(res.src, "1582:45:0".to_string());
        assert_eq!(res.name_location, Some("1588:13:0".to_string()));
        Ok(assert_eq!(res.node_type, NodeType::EventDefinition))
    }

    #[test]
    fn test_correct_return_parsing() -> Result<(), String> {
        let ast = fs::read_to_string("../solc-wrapper/tests/files/ast/Return.json").expect("Could not find test data file");
        let res = serde_json::from_str::<Return>(&ast).map_err(|_| "Error deserializing Return".to_string())?;
        assert_eq!(res.id, 296);
        assert_eq!(res.src, "5761:19:0".to_string());
        Ok(assert_eq!(res.node_type, NodeType::Return))
    }

    #[test]
    fn test_correct_emit_statement_parsing() -> Result<(), String> {
        let ast = fs::read_to_string("../solc-wrapper/tests/files/ast/EmitStatement.json").expect("Could not find test data file");
        let res = serde_json::from_str::<EmitStatement>(&ast).map_err(|_| "Error deserializing EmitStatement".to_string())?;
        assert_eq!(res.id, 287);
        assert_eq!(res.src, "5537:33:0".to_string());
        Ok(assert_eq!(res.node_type, NodeType::EmitStatement))
    }

    #[test]
    fn test_correct_modifier_invocation_parsing() -> Result<(), String> {
        let ast = fs::read_to_string("../solc-wrapper/tests/files/ast/ModifierInvocation.json").expect("Could not find test data file");
        let res = serde_json::from_str::<ModifierInvocation>(&ast).map_err(|_| "Error deserializing ModifierInvocation".to_string())?;

        assert_eq!(res.id, 274);
        assert_eq!(res.src, "5445:13:0".to_string());
        Ok(assert_eq!(res.node_type, NodeType::ModifierInvocation))
    }

    #[test]
    fn test_correct_pragma_directive_parsing() -> Result<(), String> {
        let ast = fs::read_to_string("../solc-wrapper/tests/files/ast/PragmaDirective.json").expect("Could not find test data file");
        let res = serde_json::from_str::<PragmaDirective>(&ast).map_err(|_| "Error deserializing PragmaDirective".to_string())?;
        assert_eq!(res.id, 1);
        assert_eq!(res.src, "33:23:0".to_string());
        assert_eq!(res.literals, vec!["solidity".to_string(),
                                      "0.8".to_string(),
                                      ".16".to_string()] as Vec<String>);
        Ok(assert_eq!(res.node_type, NodeType::PragmaDirective))
    }

    #[test]
    fn test_correct_tuple_expression_parsing() -> Result<(), String> {
        let ast = fs::read_to_string("../solc-wrapper/tests/files/ast/TupleExpression.json").expect("Could not find test data file");
        let res = serde_json::from_str::<PragmaDirective>(&ast).map_err(|_| "Error deserializing TupleExpression".to_string())?;
        assert_eq!(res.id, 16);
        assert_eq!(res.src, "150:12:0".to_string());
        assert_eq!(res.is_inline_array, false);
        Ok(assert_eq!(res.node_type, NodeType::TupleExpression))
    }

    #[test]
    fn test_correct_using_for_directive_parsing() -> Result<(), String> {
        let ast = fs::read_to_string("../solc-wrapper/tests/files/ast/UsingForDirective.json").expect("Could not find test data file");
        let res = serde_json::from_str::<UsingForDirective>(&ast).map_err(|_| "Error deserializing UsingForDirective".to_string())?;
        assert_eq!(res.id, 31);
        assert_eq!(res.global, Some(false));
        assert_eq!(res.src, "1007:36:0".to_string());
        Ok(assert_eq!(res.node_type, NodeType::UsingForDirective))
    }

    #[test]
    fn test_correct_modifier_definition_parsing() -> Result<(), String> {
        let ast = fs::read_to_string("../solc-wrapper/tests/files/ast/ModifierDefinition.json").expect("Could not find test data file");
        let res = serde_json::from_str::<ModifierDefinition>(&ast).map_err(|_| "Error deserializing ModifierDefinition".to_string())?;
        assert_eq!(res.id, 92);
        assert_eq!(res.name, "metadataNotLocked".to_string());
        assert_eq!(res.name_location, Some("1995:17:0".to_string()));
        assert_eq!(res.src, "1986:104:0".to_string());
        assert_eq!(res.is_virtual, false);
        assert_eq!(res.visibility, Visibility::Internal);
        Ok(assert_eq!(res.node_type, NodeType::ModifierDefinition))
    }

    #[test]
    fn test_correct_parameter_list_parsing() -> Result<(), String> {
        let ast = fs::read_to_string("../solc-wrapper/tests/files/ast/ParameterList.json").expect("Could not find test data file");
        let res = serde_json::from_str::<ParameterList>(&ast).map_err(|_| "Error deserializing ParameterList".to_string())?;
        assert_eq!(res.id, 197);
        assert_eq!(res.src, "3904:30:0".to_string());
        Ok(assert_eq!(res.node_type, NodeType::ParameterList))
    }

    #[test]
    fn test_correct_assignment_list_parsing() -> Result<(), String> {
        let ast = fs::read_to_string("../solc-wrapper/tests/files/ast/Assignment.json").expect("Could not find test data file");
        let res = serde_json::from_str::<Assignment>(&ast).map_err(|_| "Error deserializing Assignment".to_string())?;

        assert_eq!(res.id, 141);
        assert_eq!(res.src, "2849:35:0".to_string());
        assert_eq!(res.operator, AssignmentOperator::Equal);
        Ok(assert_eq!(res.node_type, NodeType::Assignment))
    }

    #[test]
    fn test_correct_inheritance_specifier_parsing() -> Result<(), String> {
        let ast = fs::read_to_string("../solc-wrapper/tests/files/ast/InheritanceSpecifier.json").expect("Could not find test data file");
        let res = serde_json::from_str::<InheritanceSpecifier>(&ast).map_err(|_| "Error deserializing InheritanceSpecifier".to_string())?;

        assert_eq!(res.id, 13);
        assert_eq!(res.src, "828:16:0".to_string());
        Ok(assert_eq!(res.node_type, NodeType::InheritanceSpecifier))
    }

    #[test]
    fn test_correct_variable_declaration_parsing() -> Result<(), String> {
        let ast = fs::read_to_string("../solc-wrapper/tests/files/ast/VariableDeclaration.json").expect("Could not find test data file");
        let res = serde_json::from_str::<VariableDeclaration>(&ast).map_err(|_| "Error deserializing VariableDeclaration".to_string())?;
        assert_eq!(res.is_constant, false);
        assert_eq!(res.id, 293);
        assert_eq!(res.mutability, Mutability::Mutable);
        assert_eq!(res.src, "5736:13:0".to_string());
        assert_eq!(res.name_location, Some("-1:-1:-1".to_string()));
        assert_eq!(res.state_variable, false);
        assert_eq!(res.storage_location, StorageLocation::Memory);
        assert_eq!(res.visibility, Visibility::Internal);
        Ok(assert_eq!(res.node_type, NodeType::VariableDeclaration))
    }

    #[test]
    fn test_correct_if_statement_parsing() -> Result<(), String> {
        let ast = fs::read_to_string("../solc-wrapper/tests/files/ast/IfStatement.json").expect("Could not find test data file");
        let res = serde_json::from_str::<IfStatement>(&ast).map_err(|_| "Error deserializing IfStatement".to_string())?;
        assert_eq!(res.id, 19);
        assert_eq!(res.src, "145:103:0".to_string());
        Ok(assert_eq!(res.node_type, NodeType::IfStatement))
    }

    #[test]
    fn test_correct_binary_operation_parsing() -> Result<(), String> {
        let ast = fs::read_to_string("../solc-wrapper/tests/files/ast/BinaryOperation.json").expect("Could not find test data file");
        let res = serde_json::from_str::<BinaryOperation>(&ast).map_err(|_| "Error deserializing TypeDescriptions".to_string())?;

        assert_eq!(res.id, 18);
        assert_eq!(res.src, "203:5:0".to_string());
        assert_eq!(res.argument_types, None);
        assert_eq!(res.is_constant, false);
        assert_eq!(res.is_l_value, false);
        assert_eq!(res.is_pure, false);
        assert_eq!(res.l_value_requested, false);
        assert_eq!(res.operator, BinaryOperator::Ampersand);
        Ok(assert_eq!(res.node_type, NodeType::BinaryOperation))
    }

    #[test]
    fn test_correct_unary_operation_parsing() -> Result<(), String> {
        let ast = fs::read_to_string("../solc-wrapper/tests/files/ast/UnaryOperation.json").expect("Could not find test data file");
        let res = serde_json::from_str::<BinaryOperation>(&ast).map_err(|_| "Error deserializing UnaryOperation".to_string())?;

        assert_eq!(res.id, 7);
        assert_eq!(res.src, "104:6:0".to_string());
        assert_eq!(res.operator, "++".to_string());
        assert_eq!(res.prefix, false);
        Ok(assert_eq!(res.node_type, NodeType::UnaryOperation))
    }

    #[test]
    fn test_correct_unchecked_block_parsing() -> Result<(), String> {
        let ast = fs::read_to_string("../solc-wrapper/tests/files/ast/UncheckedBlock.json").expect("Could not find test data file");
        let res = serde_json::from_str::<BinaryOperation>(&ast).map_err(|_| "Error deserializing UncheckedBlock".to_string())?;

        assert_eq!(res.id, 9);
        assert_eq!(res.src, "104:21:0".to_string());
        Ok(assert_eq!(res.node_type, NodeType::UncheckedBlock))
    }

    #[test]
    fn test_correct_identifier_parsing() -> Result<(), String> {
        let ast = fs::read_to_string("../solc-wrapper/tests/files/ast/Identifier.json").expect("Could not find test data file");
        let res = serde_json::from_str::<Identifier>(&ast).map_err(|_| "Error deserializing TypeDescriptions".to_string())?;

        assert_eq!(res.id, 16);
        assert_eq!(res.src, "203:1:0".to_string());
        assert_eq!(res.name, "a".to_string());
        assert_eq!(res.overloaded_declarations, vec![] as Vec<usize>);
        assert_eq!(res.referenced_declaration, Some(7));
        Ok(assert_eq!(res.node_type, NodeType::Identifier))
    }

    #[test]
    fn test_correct_conditional_parsing() -> Result<(), String> {
        let ast = fs::read_to_string("../solc-wrapper/tests/files/ast/Conditional.json").expect("Could not find test data file");
        let res = serde_json::from_str::<Conditional>(&ast).map_err(|_| "Error deserializing Conditional".to_string())?;

        assert_eq!(res.id, 10);
        assert_eq!(res.src, "158:20:0".to_string());
        assert_eq!(res.is_constant, false);
        assert_eq!(res.is_l_value, false);
        assert_eq!(res.is_pure, true);
        assert_eq!(res.l_value_requested, false);
        assert_eq!(res.node_type, NodeType::Conditional);
        Ok(())
    }

    #[test]
    fn test_correct_type_descriptions_parsing() -> Result<(), String> {
        let ast = fs::read_to_string("../solc-wrapper/tests/files/ast/TypeDescriptions.json").expect("Could not find test data file");
        let res = serde_json::from_str::<TypeDescriptions>(&ast).map_err(|_| "Error deserializing Conditional".to_string())?;

        assert_eq!(res.type_identifier, Some("t_bool".to_string()));
        assert_eq!(res.type_string, Some("bool".to_string()));
        Ok(())
    }

    #[test]
    fn test_correct_elementary_type_name_parsing() -> Result<(), String> {
        let ast = fs::read_to_string("../solc-wrapper/tests/files/ast/ElementaryTypeName.json").expect("Could not find test data file");
        let res = serde_json::from_str::<ElementaryTypeName>(&ast).map_err(|_| "Error deserializing ElementaryTypeName".to_string())?;

        assert_eq!(res.id, 64);
        assert_eq!(res.src, "1602:7:0".to_string());
        assert_eq!(res.name, "address".to_string());
        assert_eq!(res.state_mutability, Some(StateMutability::NonPayable));
        assert_eq!(res.node_type, NodeType::ElementaryTypeName);
        Ok(())
    }

    #[test]
    fn test_correct_function_call_parsing() -> Result<(), String> {
        let ast = fs::read_to_string("../solc-wrapper/tests/files/ast/FunctionCall.json").expect("Could not find test data file");
        let res = serde_json::from_str::<FunctionCall>(&ast).map_err(|_| "Error deserializing FunctionCall".to_string())?;

        assert_eq!(res.id, 78);
        assert_eq!(res.src, "1850:44:0".to_string());
        assert_eq!(res.try_call, false);
        assert_eq!(res.node_type, NodeType::FunctionCall);
        Ok(())
    }

    #[test]
    fn test_correct_literal_parsing() -> Result<(), String> {
        let ast = fs::read_to_string("../solc-wrapper/tests/files/ast/Literal.json").expect("Could not find test data file");
        let res = serde_json::from_str::<Literal>(&ast).map_err(|_| "Error deserializing Literal".to_string())?;

        assert_eq!(res.id, 77);
        assert_eq!(res.src, "1874:19:0".to_string());
        assert_eq!(res.value, Some("Minting is locked".to_string()));
        assert_eq!(res.hex_value, "4d696e74696e67206973206c6f636b6564".to_string());
        assert_eq!(res.kind, LiteralKind::String);
        assert_eq!(res.node_type, NodeType::Literal);
        Ok(())
    }

    #[test]
    fn test_correct_member_access_parsing() -> Result<(), String> {
        let ast = fs::read_to_string("../solc-wrapper/tests/files/ast/MemberAccess.json").expect("Could not find test data file");
        let res = serde_json::from_str::<MemberAccess>(&ast).map_err(|_| "Error deserializing Literal".to_string())?;

        assert_eq!(res.id, 176);
        assert_eq!(res.src, "3535:23:0".to_string());
        assert_eq!(res.member_name, "current".to_string());
        assert_eq!(res.member_location, "3551:7:0".to_string());
        assert_eq!(res.node_type, NodeType::MemberAccess);
        Ok(())
    }

    #[test]
    fn test_correct_block_parsing() -> Result<(), String> {
        let ast = fs::read_to_string("../solc-wrapper/tests/files/ast/Block.json").expect("Could not find test data file");
        let res = serde_json::from_str::<Block>(&ast).map_err(|_| "Error deserializing Block".to_string())?;

        assert_eq!(res.id, 192);
        assert_eq!(res.src, "3511:148:0".to_string());
        assert_eq!(res.statements, Some(vec![] as Vec<Statement>));
        assert_eq!(res.node_type, NodeType::Block);
        Ok(())
    }

    #[test]
    fn test_correct_expression_statement_parsing() -> Result<(), String> {
        let ast = fs::read_to_string("../solc-wrapper/tests/files/ast/ExpressionStatement.json").expect("Could not find test data file");
        let res = serde_json::from_str::<Block>(&ast).map_err(|_| "Error deserializing Expression Statement".to_string())?;

        assert_eq!(res.id, 154);
        assert_eq!(res.src, "2968:35:0".to_string());
        assert_eq!(res.node_type, NodeType::ExpressionStatement);
        Ok(())
    }

    #[test]
    fn test_correct_placeholder_statement_parsing() -> Result<(), String> {
        let ast = fs::read_to_string("../solc-wrapper/tests/files/ast/PlaceholderStatement.json").expect("Could not find test data file");
        let res = serde_json::from_str::<Block>(&ast).map_err(|_| "Error deserializing PlaceholderStatement".to_string())?;

        assert_eq!(res.id, 80);
        assert_eq!(res.src, "1904:1:0".to_string());
        assert_eq!(res.node_type, NodeType::PlaceholderStatement);
        Ok(())
    }

    #[test]
    fn test_correct_contract_definition_parsing() -> Result<(), String> {
        let ast = fs::read_to_string("../solc-wrapper/tests/files/ast/ContractDefinition.json").expect("Could not find test data file");
        let res = serde_json::from_str::<ContractDefinition>(&ast).map_err(|_| "Error deserializing ContractDefinition".to_string())?;

        assert_eq!(res.id, 427);
        assert_eq!(res.src, "783:7774:0".to_string());
        assert_eq!(res.name_location, Some("792:28:0".to_string()));
        assert_eq!(res.name, "StartonERC721MetaTransaction".to_string());
        assert_eq!(res.contract_kind, ContractKind::Contract);
        assert_eq!(res.is_abstract, false);
        assert_eq!(res.node_type, NodeType::ContractDefinition);
        Ok(())
    }

    #[test]
    fn test_correct_function_definition_parsing() -> Result<(), String> {
        let ast = fs::read_to_string("../solc-wrapper/tests/files/ast/FunctionDefinition.json").expect("Could not find test data file");
        let res = serde_json::from_str::<FunctionDefinition>(&ast).map_err(|_| "Error deserializing FunctionDefinition".to_string())?;

        assert_eq!(res.id, 193);
        assert_eq!(res.src, "3392:267:0".to_string());
        assert_eq!(res.name, "mint".to_string());
        assert_eq!(res.name_location, Some("3401:4:0".to_string()));
        assert_eq!(res.implemented, true);
        assert_eq!(res.kind, FunctionDefinitionKind::Function);
        assert_eq!(res.visibility, Visibility::Public);
        assert_eq!(res.is_virtual, false);
        assert_eq!(res.state_mutability, StateMutability::NonPayable);
        assert_eq!(res.node_type, NodeType::FunctionDefinition);
        Ok(())
    }

    #[test]
    fn test_correct_import_directive_parsing() -> Result<(), String> {
        let ast = fs::read_to_string("../solc-wrapper/tests/files/ast/ImportDirective.json").expect("Could not find test data file");
        let res = serde_json::from_str::<ImportDirective>(&ast).map_err(|_| "Error deserializing ImportDirective".to_string())?;

        assert_eq!(res.id, 2);
        assert_eq!(res.src, "58:78:0".to_string());
        assert_eq!(res.name_location, Some("-1:-1:-1".to_string()));
        assert_eq!(res.file, "@openzeppelin/contracts/token/ERC721/extensions/ERC721Enumerable.sol".to_string());
        assert_eq!(res.absolute_path, "@openzeppelin/contracts/token/ERC721/extensions/ERC721Enumerable.sol".to_string());
        assert_eq!(res.unit_alias, "".to_string());
        assert_eq!(res.symbol_aliases, vec![] as Vec<SymbolAlias>);
        assert_eq!(res.node_type, NodeType::ImportDirective);
        Ok(())
    }

    #[test]
    fn test_correct_source_unit_parsing() -> Result<(), String> {
        let ast = fs::read_to_string("../solc-wrapper/tests/files/ast/SourceUnit.json").expect("Could not find test data file");
        let res = serde_json::from_str::<SourceUnit>(&ast).map_err(|_| "Error deserializing SourceUnit".to_string())?;

        assert_eq!(res.id, 428);
        assert_eq!(res.src, "33:8524:0".to_string());
        assert_eq!(res.absolute_path, "wow.sol".to_string());
        assert_eq!(res.license, Some("MIT".to_string()));
        assert_eq!(res.node_type, NodeType::SourceUnit);
        Ok(())
    }

    #[test]
    fn test_correct_structured_documentation_parsing() -> Result<(), String> {
        let ast = fs::read_to_string("../solc-wrapper/tests/files/ast/StructuredDocumentation.json").expect("Could not find test data file");
        let res = serde_json::from_str::<StructuredDocumentation>(&ast).map_err(|_| "Error deserializing StructuredDocumentation".to_string())?;

        assert_eq!(res.id, 63);
        assert_eq!(res.src, "1522:55:0".to_string());
        assert_eq!(res.text, "@notice Event emitted when the minting is locked ".to_string());
        assert_eq!(res.node_type, NodeType::StructuredDocumentation);
        Ok(())
    }

    #[test]
    fn test_correct_break_parsing() -> Result<(), String> {
        let ast = fs::read_to_string("../solc-wrapper/tests/files/ast/Break.json").expect("Could not find test data file");
        let res = serde_json::from_str::<Break>(&ast).map_err(|_| "Error deserializing Break".to_string())?;

        assert_eq!(res.id, 7);
        assert_eq!(res.src, "172:5:0".to_string());
        assert_eq!(res.node_type, NodeType::Break);
        Ok(())
    }

    #[test]
    fn test_correct_continue_parsing() -> Result<(), String> {
        let ast = fs::read_to_string("../solc-wrapper/tests/files/ast/Continue.json").expect("Could not find test data file");
        let res = serde_json::from_str::<Continue>(&ast).map_err(|_| "Error deserializing Continue".to_string())?;

        assert_eq!(res.id, 7);
        assert_eq!(res.src, "172:8:0".to_string());
        assert_eq!(res.node_type, NodeType::Continue);
        Ok(())
    }

    #[test]
    fn test_correct_while_statement_parsing() -> Result<(), String> {
        let ast = fs::read_to_string("../solc-wrapper/tests/files/ast/WhileStatement.json").expect("Could not find test data file");
        let res = serde_json::from_str::<WhileStatement>(&ast).map_err(|_| "Error deserializing WhileStatement".to_string())?;

        assert_eq!(res.id, 9);
        assert_eq!(res.src, "145:46:0".to_string());
        assert_eq!(res.node_type, NodeType::WhileStatement);
        Ok(())
    }

    #[test]
    fn test_correct_do_while_statement_parsing() -> Result<(), String> {
        let ast = fs::read_to_string("../solc-wrapper/tests/files/ast/DoWhileStatement.json").expect("Could not find test data file");
        let res = serde_json::from_str::<DoWhileStatement>(&ast).map_err(|_| "Error deserializing DoWhileStatement".to_string())?;

        assert_eq!(res.id, 9);
        assert_eq!(res.src, "145:50:0".to_string());
        assert_eq!(res.node_type, NodeType::DoWhileStatement);
        Ok(())
    }

    #[test]
    fn test_correct_for_statement_parsing() -> Result<(), String> {
        let ast = fs::read_to_string("../solc-wrapper/tests/files/ast/ForStatement.json").expect("Could not find test data file");
        let res = serde_json::from_str::<ForStatement>(&ast).map_err(|_| "Error deserializing ForStatement".to_string())?;

        assert_eq!(res.id, 18);
        assert_eq!(res.src, "145:60:0".to_string());
        assert_eq!(res.node_type, NodeType::ForStatement);
        Ok(())
    }

    #[test]
    fn test_correct_struct_definition_parsing() -> Result<(), String> {
        let ast = fs::read_to_string("../solc-wrapper/tests/files/ast/StructDefinition.json").expect("Could not find test data file");
        let res = serde_json::from_str::<StructDefinition>(&ast).map_err(|_| "Error deserializing StructDefinition".to_string())?;

        assert_eq!(res.id, 6);
        assert_eq!(res.src, "62:36:0".to_string());
        assert_eq!(res.name_location, Some("69:1:0".to_string()));
        assert_eq!(res.name, "S".to_string());
        assert_eq!(res.scope, 27);
        assert_eq!(res.visibility, Visibility::Public);
        assert_eq!(res.canonical_name, "S".to_string());
        assert_eq!(res.node_type, NodeType::StructDefinition);
        Ok(())
    }

    #[test]
    fn test_correct_try_statement_parsing() -> Result<(), String> {
        let ast = fs::read_to_string("../solc-wrapper/tests/files/ast/TryStatement.json").expect("Could not find test data file");
        let res = serde_json::from_str::<TryStatement>(&ast).map_err(|_| "Error deserializing TryStatement".to_string())?;

        assert_eq!(res.id, 95);
        assert_eq!(res.src, "976:155:0".to_string());
        assert_eq!(res.node_type, NodeType::TryStatement);
        Ok(())
    }
}