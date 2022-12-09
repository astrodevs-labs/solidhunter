use std::collections::HashMap;
use semver::Op;
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
pub enum NodeType {
    // Expressions
    Assignment,
    BinaryOperation,
    Conditional,
    ElementaryTypeNameExpression,
    FunctionCall,
    FunctionCallOptions,
    Identifier,
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

    // Yul statements
    YulAssignment,
    YulBlock,
    YulBreak,
    YulContinue,
    YulExpressionStatement,
    YulLeave,
    YulForLoop,
    YulFunctionDefinition,
    YulIf,
    YulSwitch,
    YulVariableDeclaration,
    YulCase,

    // Yul expressions
    YulFunctionCall,
    YulIdentifier,
    YulLiteral,

    // Yul literals
    YulLiteralValue,
    YulHexValue,

    // Definitions
    ContractDefinition,
    FunctionDefinition,
    EventDefinition,
    ErrorDefinition,
    ModifierDefinition,
    StructDefinition,
    EnumDefinition,
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
    node_type: NodeType
}

pub struct StructureFunction {
    function: IdentifierPath
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceUnitChildNodes {
    ContractDefinition,
    StructDefinition,
    EnumDefinition,
    ErrorDefinition,
    UsingForDirective,
    PragmaDirective,
    ImportDirective,
    Other(String)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceUnit {
    id: usize,
    src: SourceLocation,
    #[serde(rename = "absolutePath")]
    absolute_path: String,
    #[serde(rename = "exportedSymbols")]
    exported_symbols: HashMap<String, Vec<String>>,
    #[serde(rename = "license", skip_serializing_if = "Option::is_none")]
    license: Option<String>,
    #[serde(rename = "nodes")]
    nodes: Vec<SourceUnitChildNodes>,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractDefinitionChildNodes {
    EnumDefinition,
    ErrorDefinition,
    EventDefinition,
    FunctionDefinition,
    ModifierDefinition,
    StructDefinition,
    UserDefinedValueTypeDefinition,
    UsingForDirective,
    VariableDeclaration
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
    is_fully_implemented: bool,
    #[serde(rename = "linearizedBaseContracts")]
    linearized_base_contracts: Vec<usize>,
    #[serde(rename = "nodes")]
    nodes: Vec<ContractDefinitionChildNodes>,
    #[serde(rename = "scope")]
    scope: usize,
    #[serde(rename = "usedErrors")]
    used_errors: Vec<usize>,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

pub struct Assigment {
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
    is_constant: bool,
    #[serde(rename = "isLValue")]
    is_l_value: bool,
    #[serde(rename = "isPure")]
    is_pure: bool,
    #[serde(rename = "lValueRequested")]
    l_value_requested: bool,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
    #[serde(rename = "arguments")]
    arguments: Vec<Expression>,
    #[serde(rename = "expression")]
    expression: Expression,
    kind: FunctionCallKind,
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
    is_constant: bool,
    #[serde(rename = "isLValue")]
    is_l_value: bool,
    #[serde(rename = "isPure")]
    is_pure: bool,
    #[serde(rename = "lValueRequested")]
    l_value_requested: bool,
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
    is_constant: bool,
    #[serde(rename = "isLValue")]
    is_l_value: bool,
    #[serde(rename = "isPure")]
    is_pure: bool,
    #[serde(rename = "lValueRequested")]
    l_value_requested: bool,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
    #[serde(rename = "expression")]
    expression: Expression,
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
    parameter_types: ParametersList,
    #[serde(rename = "returnParameterTypes")]
    return_parameter_types: ParametersList,
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
    overrides: Option<OverridesSpecifier>,
    scope: usize,
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
    referenced_declaration: usize,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentifierPath {
    id: usize,
    src: SourceLocation,
    name: String,
    #[serde(rename = "referencedDeclaration")]
    referenced_declaration: usize,
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
    kind: FunctionKind,
    modifiers: Vec<ModifierInvocation>,
    overrides: Option<OverrideSpecifier>,
    parameters: ParameterList,
    #[serde(rename = "returnParameters")]
    return_parameters: ParameterList,
    #[serde(rename = "scope")]
    scope: usize,
    #[serde(rename = "stateMutability")]
    state_mutability: StateMutability,
    #[serde(rename = "virtual")]
    id_virtual: bool,
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

pub enum Body {
    Block(Block),
    Statement(Statement),
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

//export interface PlaceholderStatement {
//id: number;
//src: SourceLocation;
//documentation?: string;
//nodeType: "PlaceholderStatement";
//}

pub struct PlaceholderStatement {
    id: usize,
    src: SourceLocation,
    documentation: Option<String>,
    #[serde(rename = "nodeType")]
    node_type: NodeType::PlaceholderStatement
}

//export interface Return {
//id: number;
//src: SourceLocation;
//documentation?: string;
//expression?: Expression | null;
//functionReturnParameters: number;
//nodeType: "Return";
//}

pub struct Return {
    id: usize,
    src: SourceLocation,
    documentation: Option<String>,
    expression: Option<Expression>,                 //TODO: Faire expression + multiples types
    #[serde(rename = "functionReturnParameters")]
    function_return_parameters: usize,
    #[serde(rename = "nodeType")]
    node_type: NodeType::Return
}

//export interface RevertStatement {
//id: number;
//src: SourceLocation;
//documentation?: string;
//errorCall: FunctionCall;
//nodeType: "RevertStatement";
//}

pub struct RevertStatement {
    id: usize,
    src: SourceLocation,
    documentation: Option<String>,
    #[serde(rename = "errorCall")]
    error_call: NodeType::FunctionCall,
    #[serde(rename = "nodeType")]
    node_type: NodeType::RevertStatement
}

//export interface TryStatement {
//id: number;
//src: SourceLocation;
//documentation?: string;
//clauses: TryCatchClause[];
//externalCall: FunctionCall;
//nodeType: "TryStatement";
//}

pub struct TryStatement {
    id: usize,
    src: SourceLocation,
    documentation: Option<String>,
    clauses: Vec<TryCatchClause>,
    #[serde(rename = "externalCall")]
    external_call: NodeType::FunctionCall,
    #[serde(rename = "nodeType")]
    node_type: NodeType::TryStatement
}

//export interface TryCatchClause {
//id: number;
//src: SourceLocation;
//block: Block;
//errorName: string;
//parameters?: ParameterList | null;
//nodeType: "TryCatchClause";
//}

pub struct TryCatchClause {
    id: usize,
    src: SourceLocation,
    block: NodeType::Block,
    #[serde(rename = "errorName")]
    error_name: String,
    parameters: Option<NodeType::ParameterList>,         //TODO: Multiple types
    #[serde(rename = "nodeType")]
    node_type: NodeType::TryCatchClause
}

//export interface UncheckedBlock {
//id: number;
//src: SourceLocation;
//documentation?: string;
//statements: Statement[];
//nodeType: "UncheckedBlock";
//}

pub struct UncheckedBlock {
    id: usize,
    src: SourceLocation,
    documentation: Option<String>,
    statements: Vec<Statement>,                 //TODO: Faire Statement
    #[serde(rename = "nodeType")]
    node_type: NodeType::UncheckedBlock
}

//export interface WhileStatement {
//id: number;
//src: SourceLocation;
//documentation?: string;
//body: Block | Statement;
//condition: Expression;
//nodeType: "WhileStatement";
//}

pub struct WhileStatement {
    id: usize,
    src: SourceLocation,
    documentation: Option<String>,
    body: NodeType::Block,                          //TODO: Multiple types
    condition: Expresion,                           //TODO: Faire expression
    #[serde(rename = "nodeType")]
    node_type: NodeType::WhileStatement
}

//export interface ModifierInvocation {
//id: number;
//src: SourceLocation;
//arguments?: Expression[] | null;
//kind?: "modifierInvocation" | "baseConstructorSpecifier";
//modifierName: Identifier | IdentifierPath;
//nodeType: "ModifierInvocation";
//}

pub struct ModifierInvocation {
    id: usize,
    src: SourceLocation,
    arguments: Option<Expresion>,                   //TODO: Faire expression + multiple types
    kind: Option<NodeType::ModifierInvocation>,     //TODO: Multiple types
    #[serde(rename = "modifierName")]
    modifier_name: Identifier,                       //TODO: Multiple types
    #[serde(rename = "nodeType")]
    node_type: NodeType::ModifierInvocation
}

//export interface ModifierDefinition {
//id: number;
//src: SourceLocation;
//name: string;
//nameLocation?: string;
//baseModifiers?: number[] | null;
//body: Block;
//documentation?: StructuredDocumentation | null;
//overrides?: OverrideSpecifier | null;
//parameters: ParameterList;
//virtual: boolean;
//visibility: Visibility;
//nodeType: "ModifierDefinition";
//}

pub struct ModifierDefinition {
    id: usize,
    src: SourceLocation,
    name: String,
    #[serde(rename = "nameLocation")]
    name_location: Option<String>,
    #[serde(rename = "baseModifiers")]
    base_modifiers: Option<Vec<usize>>,                 //TODO: Multiple types
    body: NodeType::Block,
    documentation: Option<StructuredDocumentation>,     //TODO: Multiple types
    overrides: Option<OverrideSpecifier>,               //TODO: Multiple types
    parameters: NodeType::ParameterList,
    #[serde(rename = "isVirtual")]
    is_virtual: bool,                                    //TODO: J'ai rename en isVirtual car virtual ne marchait pas
    visibility: Visibility,
    #[serde(rename = "nodeType")]
    node_type: NodeType::ModifierDefinition
}

//export interface StructDefinition {
//id: number;
//src: SourceLocation;
//name: string;
//nameLocation?: string;
//canonicalName: string;
//members: VariableDeclaration[];
//scope: number;
//visibility: Visibility;
//nodeType: "StructDefinition";
//}

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
    node_type: NodeType::StructDefinition
}

//export interface UserDefinedValueTypeDefinition {
//id: number;
//src: SourceLocation;
//name: string;
//nameLocation?: string;
//canonicalName?: string;
//underlyingType: TypeName;
//nodeType: "UserDefinedValueTypeDefinition";
//}

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
    node_type: NodeType::UserDefinedValueTypeDefinition
}


//export interface UsingForDirective {
//id: number;
//src: SourceLocation;
//functionList?: {
//function: IdentifierPath;
//}[];
//global?: boolean;
//libraryName?: UserDefinedTypeName | IdentifierPath;
//typeName?: TypeName | null;
//nodeType: "UsingForDirective";
//}

pub struct UsingForDirective {
    id: usize,
    src: SourceLocation,
    #[serde(rename = "functionList")]
    function_list: Option<Vec<StructureFunction>>,
    function: IdentifierPath,
    global: Option<bool>,
    //#[serde(rename = "libraryName")]
    // library_name: Option<UserDefinedTypeName>  //TODO: Multiple types
    //#[serde(rename = "typeName")]
    // type_name: Option<TypeName>  //TODO: Multiple types
    #[serde(rename = "nodeType")]
    node_type: NodeType::UsingForDirective
}


//export interface ImportDirective {
//id: number;
//src: SourceLocation;
//absolutePath: string;
//file: string;
//nameLocation?: string;
//scope: number;
//sourceUnit: number;
//symbolAliases: {
//foreign: Identifier;
//local?: string | null;
//nameLocation?: string;
//}[];
//unitAlias: string;
//nodeType: "ImportDirective";
//}
//

pub struct ImportDirective {
    id: usize,
    src: SourceLocation,
    #[serde(rename = "absolutePath")]
    absolute_path: String,
    file: String,
    #[serde(rename = "nameLocation")]
    name_location: Option<String>,
    scope: usize,
    #[serde(rename = "sourceUnit")]
    source_unit: usize,
    #[serde(rename = "symbolAliases")]
    symbol_aliases: SymbolAliases,
    #[serde(rename = "unitAlias")]
    unit_alias: String,
    #[serde(rename = "nodeType")]
    node_type: NodeType::ImportDirective,
}

//export interface PragmaDirective {
//id: number;
//src: SourceLocation;
//literals: string[];
//nodeType: "PragmaDirective";
//}

pub struct PragmaDirective {
    id: usize,
    src: SourceLocation,
    literals: Vec<String>,
    #[serde(rename = "nodeType")]
    node_type: NodeType::PragmaDirective,
}

