use std::ops::Deref;
use crate::ast::ast::*;

pub enum Nodes {
    SourceUnit(SourceUnit),
    ContractDefinition(Box<ContractDefinition>),
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
    TryCatchClause(Box<TryCatchClause>),
    UncheckedBlock(Box<UncheckedBlock>),
    VariableDeclarationStatement(Box<VariableDeclarationStatement>),
    WhileStatement(Box<WhileStatement>),
    FunctionDefinition(Box<FunctionDefinition>),
    ModifierDefinition(Box<ModifierDefinition>),
    StructDefinition(Box<StructDefinition>),
    UserDefinedValueTypeDefinition(Box<UserDefinedValueTypeDefinition>),
    VariableDeclaration(Box<VariableDeclaration>),
    EnumDefinition(Box<EnumDefinition>),
    EnumValue(Box<EnumValue>),
    ErrorDefinition(Box<ErrorDefinition>),
    EventDefinition(Box<EventDefinition>),
    UsingForDirective(Box<UsingForDirective>),
    UserDefinedTypeName(Box<UserDefinedTypeName>),
    ImportDirective(Box<ImportDirective>),
    PragmaDirective(Box<PragmaDirective>),
    FunctionTypeName(Box<FunctionTypeName>),
    Mapping(Box<Mapping>),
    ElementaryTypeName(Box<ElementaryTypeName>),
    ParameterList(Box<ParameterList>),
    OverrideSpecifier(Box<OverrideSpecifier>),
    InheritanceSpecifier(Box<InheritanceSpecifier>),
    ModifierInvocation(Box<ModifierInvocation>)
}

fn check_statement_node(node: Statement, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    match node {
        Statement::Block(block) => {
            check_block_node(block, nodes, node_type);
        }
        Statement::Break(break_statement) => {
            check_break_node(break_statement, nodes, node_type);
        }
        Statement::Continue(continue_statement) => {
            check_continue_node(continue_statement, nodes, node_type);
        }
        Statement::DoWhileStatement(do_while_statement) => {
            check_do_while_statement_node(do_while_statement, nodes, node_type);
        }
        Statement::EmitStatement(emit_statement) => {
            check_emit_statement_node(emit_statement, nodes, node_type);
        }
        Statement::ExpressionStatement(expression_statement) => {
            check_expression_statement_node(expression_statement, nodes, node_type);
        }
        Statement::ForStatement(for_statement) => {
            check_for_statement_node(for_statement, nodes, node_type);
        }
        Statement::IfStatement(if_statement) => {
            check_if_statement_node(if_statement, nodes, node_type);
        }
        Statement::PlaceholderStatement(placeholder_statement) => {
            check_placeholder_statement_node(placeholder_statement, nodes, node_type);
        }
        Statement::Return(return_statement) => {
            check_return_node(return_statement, nodes, node_type);
        }
        Statement::RevertStatement(revert_statement) => {
            check_revert_statement_node(revert_statement, nodes, node_type);
        }
        Statement::TryStatement(try_statement) => {
            check_try_statement_node(try_statement, nodes, node_type);
        }
        Statement::UncheckedBlock(unchecked_block) => {
            check_unchecked_block_node(unchecked_block, nodes, node_type);
        }
        Statement::VariableDeclarationStatement(variable_declaration_statement) => {
            check_variable_declaration_statement_node(variable_declaration_statement, nodes, node_type);
        }
        Statement::WhileStatement(while_statement) => {
            check_while_statement_node(while_statement, nodes, node_type);
        }
    }
}

fn check_body_node(body: Body, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    match body {
        Body::Block(block) => {
            check_block_node(block, nodes, node_type);
        }
        Body::Statement(statement) => {
            check_statement_node(*statement, nodes, node_type);
        }
    }
}

fn check_function_type_name_node(node: Box<FunctionTypeName>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::FunctionTypeName {
        nodes.push(Nodes::FunctionTypeName(node.clone()));
    }
    check_parameter_list_node(Box::new(node.parameter_types), nodes, node_type.clone());
    check_parameter_list_node(Box::new(node.return_parameter_types), nodes, node_type);
}

fn check_typename_node(node: TypeName, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    match node {
        TypeName::ArrayTypeName(node) => {
            check_typename_node(node.base_type, nodes, node_type);
        },
        TypeName::ElementaryTypeName(node) => {
            check_elementary_type_name_node(node, nodes, node_type);
        },
        TypeName::FunctionTypeName(node) => {
            check_function_type_name_node(node, nodes, node_type);
        },
        TypeName::Mapping(node) => {
            check_mapping_node(node, nodes, node_type);
        },
        TypeName::UserDefinedTypeName(node) => {
            nodes.push(Nodes::UserDefinedTypeName(node));
        },
    }
}

fn check_mapping_node(node: Box<Mapping>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::Mapping {
        nodes.push(Nodes::Mapping(node.clone()));
    }
    check_typename_node(node.key_type, nodes, node_type.clone());
    check_typename_node(node.value_type, nodes, node_type.clone());
}

fn check_expression_node(node: Expression, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    match node {
        Expression::Assignment(node) => {
            check_assignment_node(node, nodes, node_type);
        }
        Expression::BinaryOperation(node) => {
            check_binary_operation_node(node, nodes, node_type);
        }
        Expression::Conditional(node) => {
            check_conditional_node(node, nodes, node_type);
        }
        Expression::ElementaryTypeNameExpression(node) => {
            check_elementary_type_name_expression_node(node, nodes, node_type);
        }
        Expression::FunctionCall(node) => {
            check_function_call_node(node, nodes, node_type);
        }
        Expression::FunctionCallOptions(node) => {
            check_function_call_options_node(node, nodes, node_type);
        }
        Expression::Identifier(node) => {
            check_identifier_node(node, nodes, node_type);
        }
        Expression::IdentifierPath(node) => {
            check_identifier_path_node(node, nodes, node_type);
        }
        Expression::IndexAccess(node) => {
            check_index_access_node(node, nodes, node_type);
        }
        Expression::IndexRangeAccess(node) => {
            check_index_range_access_node(node, nodes, node_type);
        }
        Expression::Literal(node) => {
            check_literal_node(node, nodes, node_type);
        }
        Expression::MemberAccess(node) => {
            check_member_access_node(node, nodes, node_type);
        }
        Expression::NewExpression(node) => {
            check_new_expression_node(node, nodes, node_type);
        }
        Expression::TupleExpression(node) => {
            check_tuple_expression_node(node, nodes, node_type);
        }
        Expression::UnaryOperation(node) => {
            check_unary_operation_node(node, nodes, node_type);
        }
    }
}

fn check_parameter_list_node(node: Box<ParameterList>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::ParameterList {
        nodes.push(Nodes::ParameterList(node.clone()));
    }
    for parameter in node.parameters {
        check_variable_declaration_node(Box::new(parameter), nodes, node_type.clone());
    }
}

fn check_assignment_node(node: Box<Assignment>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::Assignment {
        nodes.push(Nodes::Assignment(node.clone()));
    }
    check_expression_node(node.left_hand_side, nodes, node_type.clone());
    check_expression_node(node.right_hand_side, nodes, node_type);
}

fn check_binary_operation_node(node: Box<BinaryOperation>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::BinaryOperation {
        nodes.push(Nodes::BinaryOperation(node.clone()));
    }
    check_expression_node(node.left_expression, nodes, node_type.clone());
    check_expression_node(node.right_expression, nodes, node_type.clone());
}

fn check_conditional_node(node: Box<Conditional>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::Conditional {
        nodes.push(Nodes::Conditional(node.clone()));
    }
    check_expression_node(node.condition, nodes, node_type.clone());
    check_expression_node(node.true_expression, nodes, node_type.clone());
    check_expression_node(node.false_expression, nodes, node_type);
}

fn check_elementary_type_name_expression_node(node: Box<ElementaryTypeNameExpression>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::ElementaryTypeNameExpression {
        nodes.push(Nodes::ElementaryTypeNameExpression(node.clone()));
    }
    check_elementary_type_name_node(Box::new(node.type_name), nodes, node_type);
}

fn check_elementary_type_name_node(node: Box<ElementaryTypeName>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::ElementaryTypeName {
        nodes.push(Nodes::ElementaryTypeName(node.clone()));
    }
}

fn check_function_call_node(node: Box<FunctionCall>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::FunctionCall {
        nodes.push(Nodes::FunctionCall(node.clone()));
    }
    check_expression_node(node.expression, nodes, node_type.clone());
    for argument in node.arguments {
        check_expression_node(argument, nodes, node_type.clone());
    }
}

fn check_function_call_options_node(node: Box<FunctionCallOptions>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::FunctionCallOptions {
        nodes.push(Nodes::FunctionCallOptions(node.clone()));
    }
    check_expression_node(node.expression, nodes, node_type.clone());
    for option in node.options {
        check_expression_node(option, nodes, node_type.clone());
    }
}

fn check_identifier_node(node: Box<Identifier>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::Identifier {
        nodes.push(Nodes::Identifier(node.clone()));
    }
}

fn check_identifier_path_node(node: Box<IdentifierPath>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::IdentifierPath {
        nodes.push(Nodes::IdentifierPath(node.clone()));
    }
}

fn check_index_access_node(node: Box<IndexAccess>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::IndexAccess {
        nodes.push(Nodes::IndexAccess(node.clone()));
    }
    check_expression_node(node.base_expression, nodes, node_type.clone());
    if node.index_expression.is_some() {
        check_expression_node(node.index_expression.unwrap(), nodes, node_type);
    }
}

fn check_index_range_access_node(node: Box<IndexRangeAccess>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::IndexRangeAccess {
        nodes.push(Nodes::IndexRangeAccess(node.clone()));
    }
    check_expression_node(node.base_expression, nodes, node_type.clone());
    if node.start_expression.is_some() {
        check_expression_node(node.start_expression.unwrap(), nodes, node_type.clone());
    }
    if node.end_expression.is_some() {
        check_expression_node(node.end_expression.unwrap(), nodes, node_type);
    }
}

fn check_literal_node(node: Box<Literal>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::Literal {
        nodes.push(Nodes::Literal(node.clone()));
    }
}

fn check_member_access_node(node: Box<MemberAccess>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::MemberAccess {
        nodes.push(Nodes::MemberAccess(node.clone()));
    }
    check_expression_node(node.expression, nodes, node_type);
}

fn check_new_expression_node(node: Box<NewExpression>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::NewExpression {
        nodes.push(Nodes::NewExpression(node.clone()));
    }
}

fn check_tuple_expression_node(node: Box<TupleExpression>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::TupleExpression {
        nodes.push(Nodes::TupleExpression(node.clone()));
    }
    for expression in node.components {
        check_expression_node(expression, nodes, node_type.clone());
    }
}

fn check_unary_operation_node(node: Box<UnaryOperation>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::UnaryOperation {
        nodes.push(Nodes::UnaryOperation(node.clone()));
    }
    check_expression_node(node.sub_expression, nodes, node_type);
}

fn check_block_node(node: Box<Block>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::Block {
        nodes.push(Nodes::Block(node.clone()));
    }
    if node.statements.is_some() {
        for statement in node.statements.unwrap() {
            check_statement_node(statement, nodes, node_type.clone());
        }
    }
}

fn check_break_node(node: Box<Break>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::Break {
        nodes.push(Nodes::Break(node.clone()));
    }
}

fn check_continue_node(node: Box<Continue>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::Continue {
        nodes.push(Nodes::Continue(node.clone()));
    }
}

fn check_do_while_statement_node(node: Box<DoWhileStatement>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::DoWhileStatement {
        nodes.push(Nodes::DoWhileStatement(node.clone()));
    }
    check_body_node(node.body, nodes, node_type.clone());
    check_expression_node(node.condition, nodes, node_type);
}

fn check_emit_statement_node(node: Box<EmitStatement>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::EmitStatement {
        nodes.push(Nodes::EmitStatement(node.clone()));
    }
    check_function_call_node(Box::new(node.event_call), nodes, node_type);
}

fn check_expression_statement_node(node: Box<ExpressionStatement>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::ExpressionStatement {
        nodes.push(Nodes::ExpressionStatement(node.clone()));
    }
    check_expression_node(node.expression, nodes, node_type);
}

fn check_for_statement_node(node: Box<ForStatement>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::ForStatement {
        nodes.push(Nodes::ForStatement(node.clone()));
    }
    check_body_node(node.body, nodes, node_type.clone());
    if node.condition.is_some() {
        check_expression_node(node.condition.unwrap(), nodes, node_type.clone());
    }
    if node.loop_expression.is_some() {
        check_expression_statement_node(Box::new(node.loop_expression.unwrap()), nodes, node_type.clone());
    }
    if node.initialization_expression.is_some() {
        let initialization_expression = node.initialization_expression.unwrap();
        match initialization_expression {
            InitializationExpression::VariableDeclarationStatement(variable_declaration_statement) => {
                check_variable_declaration_statement_node(Box::new(variable_declaration_statement), nodes, node_type.clone());
            },
            InitializationExpression::ExpressionStatement(expression_statement) => {
                check_expression_statement_node(Box::new(expression_statement), nodes, node_type.clone());
            },
        }
    }
}

fn check_if_statement_node(node: Box<IfStatement>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::IfStatement {
        nodes.push(Nodes::IfStatement(node.clone()));
    }
    check_expression_node(node.condition, nodes, node_type.clone());
    check_body_node(node.true_body, nodes, node_type.clone());
    if node.false_body.is_some() {
        check_body_node(node.false_body.unwrap(), nodes, node_type);
    }
}

fn check_placeholder_statement_node(node: Box<PlaceholderStatement>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::PlaceholderStatement {
        nodes.push(Nodes::PlaceholderStatement(node.clone()));
    }
}

fn check_return_node(node: Box<Return>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::Return {
        nodes.push(Nodes::Return(node.clone()));
    }
    if node.expression.is_some() {
        check_expression_node(node.expression.unwrap(), nodes, node_type);
    }
}

fn check_revert_statement_node(node: Box<RevertStatement>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::RevertStatement {
        nodes.push(Nodes::RevertStatement(node.clone()));
    }
    check_statement_node(node.error_call, nodes, node_type);
}

fn check_try_statement_node(node: Box<TryStatement>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::TryStatement {
        nodes.push(Nodes::TryStatement(node.clone()));
    }
    check_expression_node(node.external_call, nodes, node_type.clone());
    for catch_clause in node.clauses {
        check_try_catch_clause_node(Box::new(catch_clause), nodes, node_type.clone());
    }
}

fn check_try_catch_clause_node(node: Box<TryCatchClause>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::TryCatchClause {
        nodes.push(Nodes::TryCatchClause(node.clone()));
    }
    check_block_node(Box::new(node.block), nodes, node_type);
}

fn check_unchecked_block_node(node: Box<UncheckedBlock>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::UncheckedBlock {
        nodes.push(Nodes::UncheckedBlock(node.clone()));
    }
    for statement in node.statements {
        check_statement_node(statement, nodes, node_type.clone());
    }
}

fn check_variable_declaration_statement_node(node: Box<VariableDeclarationStatement>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::VariableDeclarationStatement {
        nodes.push(Nodes::VariableDeclarationStatement(node.clone()));
    }
    for variable_declaration in node.declarations {
        if variable_declaration.is_some() {
            check_variable_declaration_node(Box::new(variable_declaration.unwrap()), nodes, node_type.clone());
        }
    }
}

fn check_while_statement_node(node: Box<WhileStatement>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::WhileStatement {
        nodes.push(Nodes::WhileStatement(node.clone()));
    }
    check_statement_node(node.body, nodes, node_type.clone());
    check_expression_node(node.condition, nodes, node_type);
}

fn check_modifier_invocation_node(node: Box<ModifierInvocation>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::ModifierInvocation {
        nodes.push(Nodes::ModifierInvocation(node.clone()));
    }
    if node.arguments.is_some() {
        check_expression_node(node.arguments.unwrap(), nodes, node_type.clone());
    }
    match node.modifier_name {
        ModifierName::Identifier(identifier) => {
            check_identifier_node(Box::new(identifier), nodes, node_type);
        },
        ModifierName::IdentifierPath(identifier_path) => {
            check_identifier_path_node(Box::new(identifier_path), nodes, node_type);
        }
    };
}

fn check_function_definition_node(node: Box<FunctionDefinition>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::FunctionDefinition {
        nodes.push(Nodes::FunctionDefinition(node.clone()));
    }
    if node.body.is_some() {
        check_block_node(Box::new(node.body.unwrap()), nodes, node_type.clone());
    }
    for modifier in node.modifiers {
        check_modifier_invocation_node(Box::new(modifier), nodes, node_type.clone());
    }
    if node.overrides.is_some() {
        check_override_specifier_node(Box::new(node.overrides.unwrap()), nodes, node_type.clone());
    }
    check_parameter_list_node(Box::new(node.parameters), nodes, node_type.clone());
    check_parameter_list_node(Box::new(node.return_parameters), nodes, node_type);
}

fn check_override_specifier_node(node: Box<OverrideSpecifier>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::OverrideSpecifier {
        nodes.push(Nodes::OverrideSpecifier(node.clone()));
    }
    match node.overrides {
        OverridesEnum::Identifier(identifier_list) => {
            for identifier in identifier_list {
                check_identifier_path_node(Box::new(identifier), nodes, node_type.clone());
            }
        },
        OverridesEnum::UserDefinedTypeName(user_defined_type_name) => {
            for user_defined_type in user_defined_type_name {
                check_user_defined_type_name_node(Box::new(user_defined_type), nodes, node_type.clone());
            }
        },
    }
}

fn check_modifier_definition_node(node: Box<ModifierDefinition>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::ModifierDefinition {
        nodes.push(Nodes::ModifierDefinition(node.clone()));
    }
    check_statement_node(node.body, nodes, node_type.clone());
    if node.overrides.is_some() {
        check_override_specifier_node(Box::new(node.overrides.unwrap()), nodes, node_type.clone());
    }
    check_parameter_list_node(Box::new(node.parameters), nodes, node_type);
}

fn check_struct_definition_node(node: Box<StructDefinition>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::StructDefinition {
        nodes.push(Nodes::StructDefinition(node.clone()));
    }
    for member in node.members {
        check_variable_declaration_node(Box::new(member), nodes, node_type.clone());
    }
}

fn check_user_defined_value_type_definition_node(node: Box<UserDefinedValueTypeDefinition>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::UserDefinedValueTypeDefinition {
        nodes.push(Nodes::UserDefinedValueTypeDefinition(node.clone()));
    }
    check_typename_node(node.underlying_type, nodes, node_type);
}

fn check_variable_declaration_node(node: Box<VariableDeclaration>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::VariableDeclaration {
        nodes.push(Nodes::VariableDeclaration(node.clone()));
    }
    if node.value.is_some() {
        check_expression_node(node.value.unwrap(), nodes, node_type.clone());
    }
    if node.type_name.is_some() {
        check_typename_node(node.type_name.unwrap(), nodes, node_type);
    }
}

fn check_enum_definition_node(node: Box<EnumDefinition>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::EnumDefinition {
        nodes.push(Nodes::EnumDefinition(node.clone()));
    }
    for member in node.members {
        check_enum_value_node(Box::new(member), nodes, node_type.clone());
    }
}

fn check_enum_value_node(node: Box<EnumValue>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::EnumValue {
        nodes.push(Nodes::EnumValue(node.clone()));
    }
}

fn check_error_definition_node(node: Box<ErrorDefinition>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::ErrorDefinition {
        nodes.push(Nodes::ErrorDefinition(node.clone()));
    }
    if node.parameters.is_some() {
        check_parameter_list_node(Box::new(node.parameters.unwrap()), nodes, node_type);
    }
}

fn check_event_definition_node(node: Box<EventDefinition>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::EventDefinition {
        nodes.push(Nodes::EventDefinition(node.clone()));
    }
    if node.parameters.is_some() {
        check_parameter_list_node(Box::new(node.parameters.unwrap()), nodes, node_type);
    }
}

fn check_using_for_directive_node(node: Box<UsingForDirective>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::UsingForDirective {
        nodes.push(Nodes::UsingForDirective(node.clone()));
    }
    if node.function_list.is_some() {
        let function_list = node.function_list.unwrap();
        for function in function_list {
            check_identifier_path_node(Box::new(function.function), nodes, node_type.clone());
        }
    }
    if node.function.is_some() {
        check_identifier_path_node(Box::new(node.function.unwrap()), nodes, node_type.clone());
    }
    if node.library_name.is_some() {
        check_expression_node(node.library_name.unwrap(), nodes, node_type.clone());
    }
    if node.type_name.is_some() {
        check_typename_node(node.type_name.unwrap(), nodes, node_type);
    }
}

fn check_inheritance_specifier_node(node: Box<InheritanceSpecifier>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::InheritanceSpecifier {
        nodes.push(Nodes::InheritanceSpecifier(node.clone()));
    }
    if node.arguments.is_some() {
        for argument in node.arguments.unwrap() {
            check_expression_node(argument, nodes, node_type.clone());
        }
    }
}

fn check_contract_definition_node(node: Box<ContractDefinition>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::ContractDefinition {
        nodes.push(Nodes::ContractDefinition(node.clone()));
    }
    for base in node.base_contracts {
        check_inheritance_specifier_node(Box::new(base), nodes, node_type.clone());
    }
    for node in node.nodes {
        check_contract_definition_child_node(Box::new(node), nodes, node_type.clone());
    }
}

fn check_contract_definition_child_node(node: Box<ContractDefinitionChildNodes>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    match *node {
        ContractDefinitionChildNodes::UsingForDirective(node) => check_using_for_directive_node(node, nodes, node_type),
        ContractDefinitionChildNodes::StructDefinition(node) => check_struct_definition_node(node, nodes, node_type),
        ContractDefinitionChildNodes::EnumDefinition(node) => check_enum_definition_node(node, nodes, node_type),
        ContractDefinitionChildNodes::EventDefinition(node) => check_event_definition_node(node, nodes, node_type),
        ContractDefinitionChildNodes::FunctionDefinition(node) => check_function_definition_node(node, nodes, node_type),
        ContractDefinitionChildNodes::ModifierDefinition(node) => check_modifier_definition_node(node, nodes, node_type),
        ContractDefinitionChildNodes::ErrorDefinition(node) => check_error_definition_node(node, nodes, node_type),
        ContractDefinitionChildNodes::UserDefinedValueTypeDefinition(node) => check_user_defined_value_type_definition_node(node, nodes, node_type),
        ContractDefinitionChildNodes::VariableDeclaration(node) => check_variable_declaration_node(node, nodes, node_type),
    }
}


fn check_user_defined_type_name_node(node: Box<UserDefinedTypeName>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::UserDefinedTypeName {
        nodes.push(Nodes::UserDefinedTypeName(node.clone()));
    }
    if node.path_node.is_some() {
        check_identifier_path_node(Box::new(node.path_node.unwrap()), nodes, node_type);
    }
}

fn check_import_directive_node(node: Box<ImportDirective>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::ImportDirective {
        nodes.push(Nodes::ImportDirective(node.clone()));
    }
}

fn check_pragma_directive_node(node: Box<PragmaDirective>, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    if node_type == NodeType::PragmaDirective {
        nodes.push(Nodes::PragmaDirective(node.clone()));
    }
}

fn check_source_unit_child_node(ast: SourceUnitChildNodes, nodes: &mut Vec<Nodes>, node_type: NodeType) {
    match ast {
        SourceUnitChildNodes::ContractDefinition(node) => {
            check_contract_definition_node(node, nodes, node_type);
        }
        SourceUnitChildNodes::EnumDefinition(node) => {
            check_enum_definition_node(node, nodes, node_type);
        }
        SourceUnitChildNodes::ErrorDefinition(node) => {
            check_error_definition_node(node, nodes, node_type);
        }
        SourceUnitChildNodes::ImportDirective(node) => {
            check_import_directive_node(node, nodes, node_type);
        }
        SourceUnitChildNodes::PragmaDirective(node) => {
            check_pragma_directive_node(node, nodes, node_type);
        }
        SourceUnitChildNodes::StructDefinition(node) => {
            check_struct_definition_node(node, nodes, node_type);
        }
        SourceUnitChildNodes::UsingForDirective(node) => {
            check_using_for_directive_node(node, nodes, node_type);
        }
        _ => {}
    }
}

pub fn get_all_nodes_by_type(ast: SourceUnit, node_type: NodeType) -> Vec<Nodes> {
    let mut nodes = Vec::new();
    if ast.node_type == node_type {
        nodes.push(Nodes::SourceUnit(ast));
        return nodes;
    }
    for node in ast.nodes {
        check_source_unit_child_node(node, &mut nodes, node_type.clone());
    }
    nodes
}