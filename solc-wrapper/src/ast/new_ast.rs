use std::collections::{BTreeMap, HashMap};
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
pub struct NodeReference {
    pub id: usize,
    #[serde(rename = "nodeType")]
    pub node_type: NodeType,

    #[serde(flatten)]
    pub other: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SymbolAliases {
    foreign: NodeReference,
    local: Option<String>,
    #[serde(rename = "nameLocation")]
    pub name_location: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractKind {
    #[serde(rename = "contract")]
    Contract,
    #[serde(rename = "interface")]
    Interface,
    #[serde(rename = "library")]
    Library
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeDescriptions {
    #[serde(rename = "typeIdentifier", skip_serializing_if = "Option::is_none")]
    type_identifier: Option<String>,
    #[serde(rename = "typeString", skip_serializing_if = "Option::is_none")]
    type_string: Option<String>
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Node {
    pub id: usize,
    pub src: SourceLocation,
    #[serde(rename = "nodeType")]
    pub node_type: NodeType,
    pub literals: Option<Vec<String>>,
    pub name: Option<String>,
    #[serde(rename = "absolutePath")]
    pub absolute_path: Option<String>,
    #[serde(rename = "nameLocation")]
    pub name_location: Option<String>,
    pub file: Option<String>,
    #[serde(rename = "canonicalName")]
    pub canonical_name: Option<String>,
    pub members: Option<Vec<Node>>,
    pub scope: Option<usize>,
    #[serde(rename = "typeIdentifier")]
    pub type_identifier: Option<String>,
    #[serde(rename = "typeString")]
    pub type_string: Option<String>,
    pub text: Option<String>,
    #[serde(rename = "exportedSymbols")]
    pub exported_symbols: Option<HashMap<String, Vec<String>>>,
    #[serde(rename = "license")]
    pub license: Option<String>,
    #[serde(rename = "nodes")]
    pub nodes: Option<Vec<Node>>,
    #[serde(rename = "sourceUnit")]
    pub source_unit: Option<usize>,
    #[serde(rename = "unitAlias")]
    pub unit_alias: Option<String>,
    #[serde(rename = "abstract")]
    pub is_abstract: Option<bool>,
    #[serde(rename = "baseContracts")]
    pub base_contracts: Option<Vec<Node>>,
    #[serde(rename = "functionList")]
    pub function_list: Option<Vec<Node>>,
    pub function: Option<Box<Node>>,
    #[serde(rename = "contractDependencies")]
    pub contract_dependencies: Option<Vec<usize>>,
    #[serde(rename = "contractKind")]
    pub contract_kind: Option<ContractKind>,
    pub global: Option<bool>,
    #[serde(rename = "documentation")]
    pub documentation: Option<Box<Node>>,  // ou string
    #[serde(rename = "underlyingType")]
    pub underlying_type: Option<String>,
    #[serde(rename = "fullyImplemented")]
    pub is_fully_implemented: Option<bool>,
    #[serde(rename = "linearizedBaseContracts")]
    pub linearized_base_contracts: Option<Vec<usize>>,
    #[serde(rename = "usedErrors")]
    pub used_errors: Option<Vec<usize>>,
    #[serde(rename = "arguments")]
    pub arguments: Option<Vec<Node>>,  // ou juste NodeReference
    pub visibility: Option<Visibility>,
    #[serde(rename = "baseName")]
    pub base_name: Option<Box<Node>>,
    #[serde(rename = "baseModifiers")]
    pub base_modifiers: Option<Vec<usize>>,
    pub overrides: Option<Box<Node>>,
    pub body : Option<Box<Node>>,
    #[serde(rename = "referencedDeclaration")]
    pub referenced_declaration: Option<usize>,
    #[serde(rename = "pathNode")]
    pub path_node: Option<Box<Node>>,
    #[serde(rename = "isVirtual")]
    pub is_virtual: Option<bool>,
    pub parameters: Option<Box<Node>>,
    pub kind: Option<Box<Node>>,
    #[serde(rename = "modifierName")]
    pub modifier_name: Option<Box<Node>>,
    pub condition: Option<Box<Node>>,
    pub statements: Option<Vec<Node>>,
    pub block: Option<Box<Node>>,
    #[serde(rename = "errorName")]
    pub error_name: Option<String>,
    pub error_call: Option<Box<Node>>,
    pub external_call: Option<Box<Node>>,
    pub clauses: Option<Vec<Node>>,
    pub expression: Option<Box<Node>>,
    #[serde(rename = "falseBody")]
    pub false_body: Option<Box<Node>>,
    #[serde(rename = "trueBody")]
    pub true_body: Option<Box<Node>>,
    pub function_return_parameters: Option<usize>,
    pub assignments: Option<Vec<usize>>,
    pub declarations: Option<Vec<Option<Box<Node>>>>,
    #[serde(rename = "initialValue")]
    pub initial_value: Option<Box<Node>>,
    #[serde(rename = "initializationExpression")]
    pub initialization_expression: Option<Box<Node>>,
    #[serde(rename = "loopExpression")]
    pub loop_expression: Option<Box<Node>>,
    #[serde(rename = "eventCall")]
    pub event_call: Option<Box<Node>>,
    #[serde(rename = "virtual")]
    pub id_virtual: Option<bool>,
    #[serde(rename = "stateMutability")]
    pub state_mutability: Option<StateMutability>,
    #[serde(rename = "returnParameters")]
    pub return_parameters: Option<Box<Node>>,
    #[serde(rename = "baseFunctions")]
    pub base_functions: Option<Vec<usize>>,
    #[serde(rename = "functionSelector")]
    pub function_selector: Option<String>,
    pub implemented: Option<bool>,
    pub modifiers: Vec<NodeReference>,
    pub anonymous: Option<bool>,
    #[serde(rename = "eventSelector")]
    pub event_selector: Option<String>,
    #[serde(rename = "errorSelector")]
    pub error_selector: Option<String>,
    #[serde(rename = "argumentTypes")]
    pub argument_types: Option<Vec<TypeDescriptions>>,
    #[serde(rename = "isConstant")]
    pub is_constant: Option<bool>,
    #[serde(rename = "isLValue")]
    pub is_l_value: Option<bool>,
    #[serde(rename = "isPure")]
    pub is_pure: Option<bool>,
    #[serde(rename = "lValueRequested")]
    pub l_value_requested: Option<bool>,
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: Option<TypeDescriptions>,
    #[serde(rename = "operator")]
    pub operator: Option<UnaryOperator>,
    pub prefix: Option<bool>,
    #[serde(rename = "subExpression")]
    pub sub_expression: Option<Box<Node>>,
    pub components: Option<Vec<Node>>,
    #[serde(rename = "isInlineArray")]
    pub is_inline_array: Option<bool>,
    #[serde(rename = "keyType")]
    pub key_type: Option<Box<Node>>,
    #[serde(rename = "valueType")]
    pub value_type: Option<Box<Node>>,
    pub indexed: Option<bool>,
    pub mutability: Option<Mutability>,
    #[serde(rename = "stateVariable")]
    pub state_variable: Option<bool>,
    #[serde(rename = "storageLocation")]
    pub storage_location: Option<StorageLocation>,
    #[serde(rename = "typeName")]
    pub type_name: Option<Box<Node>>,
    #[serde(rename = "value")]
    pub value: Option<Box<Node>>,
    #[serde(rename = "parameterTypes")]
    pub parameter_types: Option<Box<Node>>,
    #[serde(rename = "returnParameterTypes")]
    pub return_parameter_types: Option<Box<Node>>,
    #[serde(rename = "baseType")]
    pub base_type: Option<Box<Node>>,
    pub length: Option<Box<Node>>,
    #[serde(rename = "memberName")]
    pub member_name: Option<String>,
    #[serde(rename = "hexValue")]
    pub hex_value: Option<String>,
    #[serde(rename = "baseExpression")]
    pub base_expression: Option<Box<Node>>,
    #[serde(rename = "endExpression", skip_serializing_if = "Option::is_none")]
    pub end_expression: Option<Box<Node>>,
    #[serde(rename = "startExpression", skip_serializing_if = "Option::is_none")]
    pub start_expression: Option<Box<Node>>,
    #[serde(rename = "indexExpression", skip_serializing_if = "Option::is_none")]
    pub index_expression: Option<Box<Node>>,
    #[serde(rename = "overloadedDeclarations")]
    pub overloaded_declarations: Option<Vec<usize>>,
    pub names: Option<Vec<String>>,
    pub options: Option<Vec<Node>>,
    #[serde(rename = "tryCall")]
    pub try_call: Option<bool>,
    #[serde(rename = "falseExpression")]
    pub false_expression: Option<Box<Node>>,
    #[serde(rename = "trueExpression")]
    pub true_expression: Option<Box<Node>>,
    #[serde(rename = "commonType")]
    pub common_type: Option<TypeDescriptions>,
    #[serde(rename = "leftExpression")]
    pub left_expression: Option<Box<Node>>,
    #[serde(rename = "rightExpression")]
    pub right_expression: Option<Box<Node>>,
    #[serde(rename = "leftHandSide")]
    pub left_hand_side: Option<Box<Node>>,
    #[serde(rename = "rightHandSide")]
    pub right_hand_side: Option<Box<Node>>,
}