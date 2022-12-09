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
pub enum LitteralKind {
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

pub struct EnumValue {
    id: usize,
    src: SourceLocation,
    name: String,
    #[serde(rename = "nameLocation", skip_serializing_if = "Option::is_none")]
    name_location: Option<String>,
    #[serde(rename = "nodeType")]
    node_type: NodeType,
}

pub struct TypeDescriptions {
    #[serde(rename = "typeIdentifier", skip_serializing_if = "Option::is_none")]
    type_identifier: Option<String>,
    #[serde(rename = "typeString", skip_serializing_if = "Option::is_none")]
    type_string: Option<String>
}

pub struct StructuredDocumentation {
    id: usize,
    src: SourceLocation,
    text: String,
    #[serde(rename = "nodeType")]
    node_type: NodeType
}

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
    #[serde(rename = "symbolAliases")]
    symbol_aliases: Vec<SymbolAlias>,
    #[serde(rename = "symbolDependencies")]
    symbol_dependencies: Vec<SymbolDependency>,
    #[serde(rename = "symbolMapping")]
    symbol_mapping: HashMap<String, Vec<String>>,
    #[serde(rename = "usedSymbols")]
    used_symbols: HashMap<String, Vec<String>>
}









export interface SourceUnit {
id: number;
src: SourceLocation;
absolutePath: string;
exportedSymbols: {
[k: string]: number[] | undefined;
};
license?: string | null;
nodes: (
| ContractDefinition
| EnumDefinition
| ErrorDefinition
| FunctionDefinition
| ImportDirective
| PragmaDirective
| StructDefinition
| UserDefinedValueTypeDefinition
| UsingForDirective
| VariableDeclaration
)[];
nodeType: "SourceUnit";
}
export interface ContractDefinition {
id: number;
src: SourceLocation;
name: string;
nameLocation?: string;
abstract: boolean;
baseContracts: InheritanceSpecifier[];
canonicalName?: string;
contractDependencies: number[];
contractKind: "contract" | "interface" | "library";
documentation?: StructuredDocumentation | null;
fullyImplemented: boolean;
linearizedBaseContracts: number[];
nodes: (
| EnumDefinition
| ErrorDefinition
| EventDefinition
| FunctionDefinition
| ModifierDefinition
| StructDefinition
| UserDefinedValueTypeDefinition
| UsingForDirective
| VariableDeclaration
)[];
scope: number;
usedErrors?: number[];
nodeType: "ContractDefinition";
}
export interface InheritanceSpecifier {
id: number;
src: SourceLocation;
arguments?: Expression[] | null;
baseName: UserDefinedTypeName | IdentifierPath;
nodeType: "InheritanceSpecifier";
}
export interface Assignment {
id: number;
src: SourceLocation;
argumentTypes?: TypeDescriptions[] | null;
isConstant: boolean;
isLValue: boolean;
isPure: boolean;
lValueRequested: boolean;
typeDescriptions: TypeDescriptions;
leftHandSide: Expression;
operator: "=" | "+=" | "-=" | "*=" | "/=" | "%=" | "|=" | "&=" | "^=" | ">>=" | "<<=";
rightHandSide: Expression;
nodeType: "Assignment";
}
export interface BinaryOperation {
id: number;
src: SourceLocation;
argumentTypes?: TypeDescriptions[] | null;
isConstant: boolean;
isLValue: boolean;
isPure: boolean;
lValueRequested: boolean;
typeDescriptions: TypeDescriptions;
commonType: TypeDescriptions;
leftExpression: Expression;
operator:
| "+"
| "-"
| "*"
| "/"
| "%"
| "**"
| "&&"
| "||"
| "!="
| "=="
| "<"
| "<="
| ">"
| ">="
| "^"
| "&"
| "|"
| "<<"
| ">>";
rightExpression: Expression;
nodeType: "BinaryOperation";
}
export interface Conditional {
id: number;
src: SourceLocation;
argumentTypes?: TypeDescriptions[] | null;
isConstant: boolean;
isLValue: boolean;
isPure: boolean;
lValueRequested: boolean;
typeDescriptions: TypeDescriptions;
condition: Expression;
falseExpression: Expression;
trueExpression: Expression;
nodeType: "Conditional";
}
export interface ElementaryTypeNameExpression {
id: number;
src: SourceLocation;
argumentTypes?: TypeDescriptions[] | null;
isConstant: boolean;
isLValue: boolean;
isPure: boolean;
lValueRequested: boolean;
typeDescriptions: TypeDescriptions;
typeName: ElementaryTypeName;
nodeType: "ElementaryTypeNameExpression";
}
export interface ElementaryTypeName {
id: number;
src: SourceLocation;
typeDescriptions: TypeDescriptions;
name: string;
stateMutability?: StateMutability;
nodeType: "ElementaryTypeName";
}
export interface FunctionCall {
id: number;
src: SourceLocation;
argumentTypes?: TypeDescriptions[] | null;
isConstant: boolean;
isLValue: boolean;
isPure: boolean;
lValueRequested: boolean;
typeDescriptions: TypeDescriptions;
arguments: Expression[];
expression: Expression;
kind: "functionCall" | "typeConversion" | "structConstructorCall";
names: string[];
tryCall: boolean;
nodeType: "FunctionCall";
}
export interface FunctionCallOptions {
id: number;
src: SourceLocation;
argumentTypes?: TypeDescriptions[] | null;
isConstant: boolean;
isLValue?: boolean;
isPure: boolean;
lValueRequested: boolean;
typeDescriptions: TypeDescriptions;
expression: Expression;
names: string[];
options: Expression[];
nodeType: "FunctionCallOptions";
}
export interface Identifier {
id: number;
src: SourceLocation;
argumentTypes?: TypeDescriptions[] | null;
name: string;
overloadedDeclarations: number[];
referencedDeclaration?: number | null;
typeDescriptions: TypeDescriptions;
nodeType: "Identifier";
}
export interface IndexAccess {
id: number;
src: SourceLocation;
argumentTypes?: TypeDescriptions[] | null;
isConstant: boolean;
isLValue: boolean;
isPure: boolean;
lValueRequested: boolean;
typeDescriptions: TypeDescriptions;
baseExpression: Expression;
indexExpression?: Expression | null;
nodeType: "IndexAccess";
}
export interface IndexRangeAccess {
id: number;
src: SourceLocation;
argumentTypes?: TypeDescriptions[] | null;
isConstant: boolean;
isLValue: boolean;
isPure: boolean;
lValueRequested: boolean;
typeDescriptions: TypeDescriptions;
baseExpression: Expression;
endExpression?: Expression | null;
startExpression?: Expression | null;
nodeType: "IndexRangeAccess";
}
export interface Literal {
id: number;
src: SourceLocation;
argumentTypes?: TypeDescriptions[] | null;
isConstant: boolean;
isLValue: boolean;
isPure: boolean;
lValueRequested: boolean;
typeDescriptions: TypeDescriptions;
hexValue: string;
kind: "bool" | "number" | "string" | "hexString" | "unicodeString";
subdenomination?: null;
value?: string | null;
nodeType: "Literal";
}
export interface MemberAccess {
id: number;
src: SourceLocation;
argumentTypes?: TypeDescriptions[] | null;
isConstant: boolean;
isLValue: boolean;
isPure: boolean;
lValueRequested: boolean;
typeDescriptions: TypeDescriptions;
expression: Expression;
memberName: string;
referencedDeclaration?: number | null;
nodeType: "MemberAccess";
}
export interface NewExpression {
id: number;
src: SourceLocation;
argumentTypes?: TypeDescriptions[] | null;
isConstant: boolean;
isLValue?: boolean;
isPure: boolean;
lValueRequested: boolean;
typeDescriptions: TypeDescriptions;
typeName: TypeName;
nodeType: "NewExpression";
}
export interface ArrayTypeName {
id: number;
src: SourceLocation;
typeDescriptions: TypeDescriptions;
baseType: TypeName;
length?: Expression | null;
nodeType: "ArrayTypeName";
}
export interface FunctionTypeName {
id: number;
src: SourceLocation;
typeDescriptions: TypeDescriptions;
parameterTypes: ParameterList;
returnParameterTypes: ParameterList;
stateMutability: StateMutability;
visibility: Visibility;
nodeType: "FunctionTypeName";
}
export interface ParameterList {
id: number;
src: SourceLocation;
parameters: VariableDeclaration[];
nodeType: "ParameterList";
}
export interface VariableDeclaration {
id: number;
src: SourceLocation;
name: string;
nameLocation?: string;
baseFunctions?: number[] | null;
constant: boolean;
documentation?: StructuredDocumentation | null;
functionSelector?: string;
indexed?: boolean;
mutability: Mutability;
overrides?: OverrideSpecifier | null;
scope: number;
stateVariable: boolean;
storageLocation: StorageLocation;
typeDescriptions: TypeDescriptions;
typeName?: TypeName | null;
value?: Expression | null;
visibility: Visibility;
nodeType: "VariableDeclaration";
}
export interface OverrideSpecifier {
id: number;
src: SourceLocation;
overrides: UserDefinedTypeName[] | IdentifierPath[];
nodeType: "OverrideSpecifier";
}
export interface UserDefinedTypeName {
id: number;
src: SourceLocation;
typeDescriptions: TypeDescriptions;
contractScope?: null;
name?: string;
pathNode?: IdentifierPath;
referencedDeclaration: number;
nodeType: "UserDefinedTypeName";
}
export interface IdentifierPath {
id: number;
src: SourceLocation;
name: string;
referencedDeclaration: number;
nodeType: "IdentifierPath";
}
export interface Mapping {
id: number;
src: SourceLocation;
typeDescriptions: TypeDescriptions;
keyType: TypeName;
valueType: TypeName;
nodeType: "Mapping";
}
export interface TupleExpression {
id: number;
src: SourceLocation;
argumentTypes?: TypeDescriptions[] | null;
isConstant: boolean;
isLValue: boolean;
isPure: boolean;
lValueRequested: boolean;
typeDescriptions: TypeDescriptions;
components: Expression[];
isInlineArray: boolean;
nodeType: "TupleExpression";
}
export interface UnaryOperation {
id: number;
src: SourceLocation;
argumentTypes?: TypeDescriptions[] | null;
isConstant: boolean;
isLValue: boolean;
isPure: boolean;
lValueRequested: boolean;
typeDescriptions: TypeDescriptions;
operator: "++" | "--" | "-" | "!" | "delete";
prefix: boolean;
subExpression: Expression;
nodeType: "UnaryOperation";
}
export interface ErrorDefinition {
id: number;
src: SourceLocation;
name: string;
nameLocation: string;
documentation?: StructuredDocumentation | null;
errorSelector?: string;
parameters: ParameterList;
nodeType: "ErrorDefinition";
}
export interface EventDefinition {
id: number;
src: SourceLocation;
name: string;
nameLocation?: string;
anonymous: boolean;
eventSelector?: string;
documentation?: StructuredDocumentation | null;
parameters: ParameterList;
nodeType: "EventDefinition";
}
export interface FunctionDefinition {
id: number;
src: SourceLocation;
name: string;
nameLocation?: string;
baseFunctions?: number[];
body?: Block | null;
documentation?: StructuredDocumentation | null;
functionSelector?: string;
implemented: boolean;
kind: "function" | "receive" | "constructor" | "fallback" | "freeFunction";
modifiers: ModifierInvocation[];
overrides?: OverrideSpecifier | null;
parameters: ParameterList;
returnParameters: ParameterList;
scope: number;
stateMutability: StateMutability;
virtual: boolean;
visibility: Visibility;
nodeType: "FunctionDefinition";
}
export interface Block {
id: number;
src: SourceLocation;
documentation?: string;
statements?: Statement[] | null;
nodeType: "Block";
}
export interface Break {
id: number;
src: SourceLocation;
documentation?: string;
nodeType: "Break";
}
export interface Continue {
id: number;
src: SourceLocation;
documentation?: string;
nodeType: "Continue";
}
export interface DoWhileStatement {
id: number;
src: SourceLocation;
documentation?: string;
body: Block | Statement;
condition: Expression;
nodeType: "DoWhileStatement";
}
export interface EmitStatement {
id: number;
src: SourceLocation;
documentation?: string;
eventCall: FunctionCall;
nodeType: "EmitStatement";
}
export interface ExpressionStatement {
id: number;
src: SourceLocation;
documentation?: string;
expression: Expression;
nodeType: "ExpressionStatement";
}
export interface ForStatement {
id: number;
src: SourceLocation;
documentation?: string;
body: Block | Statement;
condition?: Expression | null;
initializationExpression?: (ExpressionStatement | VariableDeclarationStatement) | null;
loopExpression?: ExpressionStatement | null;
nodeType: "ForStatement";
}
export interface VariableDeclarationStatement {
id: number;
src: SourceLocation;
documentation?: string;
assignments: (number | null)[];
declarations: (VariableDeclaration | null)[];
initialValue?: Expression | null;
nodeType: "VariableDeclarationStatement";
}
export interface IfStatement {
id: number;
src: SourceLocation;
documentation?: string;
condition: Expression;
falseBody?: (Statement | Block) | null;
trueBody: Statement | Block;
nodeType: "IfStatement";
}
export interface InlineAssembly {
id: number;
src: SourceLocation;
documentation?: string;
AST: YulBlock;
evmVersion:
| "homestead"
| "tangerineWhistle"
| "spuriousDragon"
| "byzantium"
| "constantinople"
| "petersburg"
| "istanbul"
| "berlin"
| "london";
externalReferences: {
declaration: number;
isOffset: boolean;
isSlot: boolean;
src: SourceLocation;
valueSize: number;
suffix?: "slot" | "offset";
}[];
flags?: "memory-safe"[];
nodeType: "InlineAssembly";
}
export interface YulBlock {
src: SourceLocation;
statements: YulStatement[];
nodeType: "YulBlock";
}
export interface YulAssignment {
src: SourceLocation;
value: YulExpression;
variableNames: YulIdentifier[];
nodeType: "YulAssignment";
}
export interface YulFunctionCall {
src: SourceLocation;
arguments: YulExpression[];
functionName: YulIdentifier;
nodeType: "YulFunctionCall";
}
export interface YulIdentifier {
src: SourceLocation;
name: string;
nodeType: "YulIdentifier";
}
export interface YulLiteralValue {
src: SourceLocation;
value: string;
kind: "number" | "string" | "bool";
type: string;
nodeType: "YulLiteral";
}
export interface YulLiteralHexValue {
src: SourceLocation;
hexValue: string;
kind: "number" | "string" | "bool";
type: string;
value?: string;
nodeType: "YulLiteral";
}
export interface YulBreak {
src: SourceLocation;
nodeType: "YulBreak";
}
export interface YulContinue {
src: SourceLocation;
nodeType: "YulContinue";
}
export interface YulExpressionStatement {
src: SourceLocation;
expression: YulExpression;
nodeType: "YulExpressionStatement";
}
export interface YulLeave {
src: SourceLocation;
nodeType: "YulLeave";
}
export interface YulForLoop {
src: SourceLocation;
body: YulBlock;
condition: YulExpression;
post: YulBlock;
pre: YulBlock;
nodeType: "YulForLoop";
}
export interface YulFunctionDefinition {
src: SourceLocation;
body: YulBlock;
name: string;
parameters: YulTypedName[];
returnVariables: YulTypedName[];
nodeType: "YulFunctionDefinition";
}
export interface YulTypedName {
src: SourceLocation;
name: string;
type: string;
nodeType: "YulTypedName";
}
export interface YulIf {
src: SourceLocation;
body: YulBlock;
condition: YulExpression;
nodeType: "YulIf";
}
export interface YulSwitch {
src: SourceLocation;
cases: YulCase[];
expression: YulExpression;
nodeType: "YulSwitch";
}
export interface YulCase {
src: SourceLocation;
body: YulBlock;
value: "default" | YulLiteral;
nodeType: "YulCase";
}
export interface YulVariableDeclaration {
src: SourceLocation;
value?: YulExpression | null;
variables: YulTypedName[];
nodeType: "YulVariableDeclaration";
}
export interface PlaceholderStatement {
id: number;
src: SourceLocation;
documentation?: string;
nodeType: "PlaceholderStatement";
}
export interface Return {
id: number;
src: SourceLocation;
documentation?: string;
expression?: Expression | null;
functionReturnParameters: number;
nodeType: "Return";
}
export interface RevertStatement {
id: number;
src: SourceLocation;
documentation?: string;
errorCall: FunctionCall;
nodeType: "RevertStatement";
}
export interface TryStatement {
id: number;
src: SourceLocation;
documentation?: string;
clauses: TryCatchClause[];
externalCall: FunctionCall;
nodeType: "TryStatement";
}
export interface TryCatchClause {
id: number;
src: SourceLocation;
block: Block;
errorName: string;
parameters?: ParameterList | null;
nodeType: "TryCatchClause";
}
export interface UncheckedBlock {
id: number;
src: SourceLocation;
documentation?: string;
statements: Statement[];
nodeType: "UncheckedBlock";
}
export interface WhileStatement {
id: number;
src: SourceLocation;
documentation?: string;
body: Block | Statement;
condition: Expression;
nodeType: "WhileStatement";
}
export interface ModifierInvocation {
id: number;
src: SourceLocation;
arguments?: Expression[] | null;
kind?: "modifierInvocation" | "baseConstructorSpecifier";
modifierName: Identifier | IdentifierPath;
nodeType: "ModifierInvocation";
}
export interface ModifierDefinition {
id: number;
src: SourceLocation;
name: string;
nameLocation?: string;
baseModifiers?: number[] | null;
body: Block;
documentation?: StructuredDocumentation | null;
overrides?: OverrideSpecifier | null;
parameters: ParameterList;
virtual: boolean;
visibility: Visibility;
nodeType: "ModifierDefinition";
}
export interface StructDefinition {
id: number;
src: SourceLocation;
name: string;
nameLocation?: string;
canonicalName: string;
members: VariableDeclaration[];
scope: number;
visibility: Visibility;
nodeType: "StructDefinition";
}
export interface UserDefinedValueTypeDefinition {
id: number;
src: SourceLocation;
name: string;
nameLocation?: string;
canonicalName?: string;
underlyingType: TypeName;
nodeType: "UserDefinedValueTypeDefinition";
}
export interface UsingForDirective {
id: number;
src: SourceLocation;
functionList?: {
function: IdentifierPath;
}[];
global?: boolean;
libraryName?: UserDefinedTypeName | IdentifierPath;
typeName?: TypeName | null;
nodeType: "UsingForDirective";
}
export interface ImportDirective {
id: number;
src: SourceLocation;
absolutePath: string;
file: string;
nameLocation?: string;
scope: number;
sourceUnit: number;
symbolAliases: {
foreign: Identifier;
local?: string | null;
nameLocation?: string;
}[];
unitAlias: string;
nodeType: "ImportDirective";
}
export interface PragmaDirective {
id: number;
src: SourceLocation;
literals: string[];
nodeType: "PragmaDirective";
}
