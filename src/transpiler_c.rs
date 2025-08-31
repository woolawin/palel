use crate::c::*;
use crate::compilation_error::{
    CompilationError, CouldNotTranspileType, IncompatibleTypes, VariableTypeAmbiguous,
};
use crate::core::Of;
use crate::palel::*;
use crate::toolkit_c::CToolKit;
use crate::transpiler_c_patch::{merge_patch, patch_src};
use crate::type_checking::{
    determine_variable_type, is_valid_expression_assignment, type_of_expression,
};

pub enum CTranspile<T> {
    Ok(T, CSrcPatch),
    Error(Box<dyn CompilationError>),
}

use CTranspile::*;

pub fn transpile(input: &Src, toolkit: &CToolKit) -> Of<CSrc> {
    let mut src = CSrc {
        includes: vec![],
        functions: vec![],
    };
    if let Some(program) = input.programs.get(0) {
        match transpile_program(program, toolkit) {
            Error(err) => return Of::Error(err),
            Ok(program, patch) => {
                src.functions.push(program);
                patch_src(&mut src, &patch);
            }
        }
    }
    Of::Ok(src)
}

fn transpile_program(input: &Program, toolkit: &CToolKit) -> CTranspile<CFunction> {
    let mut patch = CSrcPatch::default();
    let mut block = match transpile_block(&input.do_block, toolkit) {
        Error(err) => return Error(err),
        Ok(block, in_patch) => {
            merge_patch(&mut patch, &in_patch);
            block
        }
    };
    let ret_stmt = Return {
        value: Some(Literal::Number("0".to_string()).to_expression()),
    };
    match transpile_return(&ret_stmt, toolkit) {
        Error(err) => return Error(err),
        Ok(ret, in_patch) => {
            merge_patch(&mut patch, &in_patch);
            block.statements.push(ret.to_statement());
        }
    };
    let function = CFunction {
        name: "main".to_string(),
        return_type: int_type(),
        block: block,
    };
    Ok(function, patch)
}

fn transpile_block(input: &DoBlock, toolkit: &CToolKit) -> CTranspile<CBlock> {
    let mut statements: Vec<CStatement> = vec![];
    let mut patch = CSrcPatch::default();
    for statement in &input.statements {
        match transpile_statement(statement, toolkit) {
            Error(err) => return Error(err),
            Ok(statement, in_patch) => {
                merge_patch(&mut patch, &in_patch);
                statements.push(statement);
            }
        };
    }
    let block = CBlock {
        statements: statements,
    };
    return Ok(block, patch);
}

fn transpile_statement(input: &Statement, toolkit: &CToolKit) -> CTranspile<CStatement> {
    match input {
        Statement::ProcedureCall(procedure_call) => {
            match transpile_procedure_call(procedure_call, toolkit) {
                Error(err) => Error(err),
                Ok(function_call, in_patch) => Ok(function_call.to_statement(), in_patch),
            }
        }
        Statement::Return(ret) => match transpile_return(ret, toolkit) {
            Error(err) => Error(err),
            Ok(ret, patch) => Ok(ret.to_statement(), patch),
        },
        Statement::Variable(variable_declaration) => {
            match transpile_variable_declaration(variable_declaration, toolkit) {
                Error(err) => Error(err),
                Ok(var, patch) => Ok(var.to_statement(), patch),
            }
        }
    }
}

fn transpile_variable_declaration(
    input: &VariableDeclaration,
    toolkit: &CToolKit,
) -> CTranspile<CVariableDeclaration> {
    let variable_type: Type = match determine_variable_type(
        input.memory.clone(),
        input.schema_type.clone(),
        &input.expression,
    ) {
        Some(t) => t.clone(),
        None => return Error(Box::new(VariableTypeAmbiguous {})),
    };

    let expression_type = match type_of_expression(&input.expression) {
        Some(t) => t,
        None => return Error(Box::new(VariableTypeAmbiguous {})),
    };

    if !is_valid_expression_assignment(&variable_type, &expression_type) {
        return Error(Box::new(IncompatibleTypes {
            expected: variable_type,
            actual: expression_type,
        }));
    }

    let mut patch = CSrcPatch::default();

    let expression = match transpile_expression(&input.expression, &variable_type, toolkit) {
        Ok(expr, in_patch) => {
            merge_patch(&mut patch, &in_patch);
            expr
        }
        Error(err) => {
            return Error(err);
        }
    };

    let var = CVariableDeclaration {
        name: input.identifier.clone(),
        var_type: match toolkit.transpile_type(&variable_type) {
            Ok(typ, in_patch) => match typ {
                Some(t) => {
                    merge_patch(&mut patch, &in_patch);
                    t
                }
                None => return Error(Box::new(CouldNotTranspileType {})),
            },
            Error(err) => {
                return Error(err);
            }
        },
        value: expression,
    };
    Ok(var, patch)
}

fn transpile_return(input: &Return, toolkit: &CToolKit) -> CTranspile<CReturn> {
    match &input.value {
        Some(value) => match transpile_expression_unknown_type(&value, toolkit) {
            Ok(expr, in_patch) => Ok(CReturn { value: Some(expr) }, in_patch),
            Error(e) => {
                return Error(e);
            }
        },
        None => Ok(CReturn { value: None }, CSrcPatch::default()),
    }
}

fn transpile_procedure_call(
    input: &ProcedureCall,
    toolkit: &CToolKit,
) -> CTranspile<CFunctionCall> {
    if !input.interface.is_empty() {
        return toolkit.transpile_interface_call(input);
    }

    let mut patch = CSrcPatch::default();
    let expressions = match transpile_expressions(&input.arguments, toolkit) {
        Ok(exprs, in_patch) => {
            merge_patch(&mut patch, &in_patch);
            exprs
        }
        Error(e) => {
            return Error(e);
        }
    };

    let function_call = CFunctionCall {
        function_name: input.identifier.clone(),
        arguments: expressions,
    };

    Ok(function_call, patch)
}

pub fn transpile_expressions(
    input: &Vec<Expression>,
    toolkit: &CToolKit,
) -> CTranspile<Vec<CExpression>> {
    let mut patch = CSrcPatch::default();
    let mut expressions: Vec<CExpression> = vec![];
    for argument in input {
        let expr = match transpile_expression_unknown_type(argument, toolkit) {
            Ok(expr, in_patch) => {
                merge_patch(&mut patch, &in_patch);
                expr
            }
            Error(e) => {
                return Error(e);
            }
        };
        expressions.push(expr);
    }
    Ok(expressions, patch)
}

fn transpile_expression_unknown_type(
    input: &Expression,
    toolkit: &CToolKit,
) -> CTranspile<CExpression> {
    let typ = match type_of_expression(input).and_then(ExpressionType::to_type) {
        Some(t) => t,
        None => {
            return Error(Box::new(VariableTypeAmbiguous {}));
        }
    };
    match input {
        Expression::Literal(literal) => transpile_literal(&literal, &typ, toolkit),
    }
}

fn transpile_expression(
    input: &Expression,
    typ: &Type,
    toolkit: &CToolKit,
) -> CTranspile<CExpression> {
    match input {
        Expression::Literal(literal) => transpile_literal(&literal, typ, toolkit),
    }
}

fn transpile_literal(input: &Literal, typ: &Type, toolkit: &CToolKit) -> CTranspile<CExpression> {
    match input {
        Literal::String(str) => Ok(
            CLiteral::String(str.clone()).to_expression(),
            CSrcPatch::default(),
        ),
        Literal::Number(num) => Ok(
            CLiteral::Number(num.clone()).to_expression(),
            CSrcPatch::default(),
        ),
        Literal::Boolean(value) => {
            if *value {
                Ok(true_literal().to_expression(), CSrcPatch::default())
            } else {
                Ok(false_literal().to_expression(), CSrcPatch::default())
            }
        }
        Literal::Null => toolkit.transpile_null(typ),
    }
}

fn true_literal() -> CLiteral {
    return CLiteral::Number("1".to_string());
}

fn false_literal() -> CLiteral {
    return CLiteral::Number("0".to_string());
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const TOOLKIT: CToolKit = CToolKit {};

    fn run(input: &Src) -> CSrc {
        match transpile(&input, &TOOLKIT) {
            Of::Ok(value) => value,
            Of::Error(err) => {
                panic!("{}", err.message())
            }
        }
    }

    #[test]
    fn test_transpile_hello_world() {
        let src = Src {
            programs: vec![Program {
                do_block: DoBlock {
                    statements: vec![
                        ProcedureCall {
                            interface: "debug".to_string(),
                            identifier: "printf".to_string(),
                            arguments: vec![
                                Literal::String("Hello World".to_string()).to_expression(),
                            ],
                        }
                        .to_statement(),
                    ],
                },
            }],
        };

        let actual = run(&src);
        let expected = CSrc {
            includes: vec![CInclude {
                file: "stdio.h".to_string(),
            }],
            functions: vec![CFunction {
                name: "main".to_string(),
                return_type: CType {
                    name: "int".to_string(),
                    is_pointer: false,
                },
                block: CBlock {
                    statements: vec![
                        CFunctionCall {
                            function_name: "printf".to_string(),
                            arguments: vec![
                                CLiteral::String("Hello World".to_string()).to_expression(),
                            ],
                        }
                        .to_statement(),
                        CReturn {
                            value: Some(CLiteral::Number("0".to_string()).to_expression()),
                        }
                        .to_statement(),
                    ],
                },
            }],
        };

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_transpile_variable_delcarations() {
        let src = Src {
            programs: vec![Program {
                do_block: DoBlock {
                    statements: vec![
                        VariableDeclaration {
                            memory: MemoryModifier::Dim,
                            identifier: "a".to_string(),
                            schema_type: None,
                            expression: Expression::Literal(Literal::Number("1".to_string())),
                        }
                        .to_statement(),
                        VariableDeclaration {
                            memory: MemoryModifier::Ref,
                            identifier: "b".to_string(),
                            schema_type: None,
                            expression: Expression::Literal(Literal::Number("2".to_string())),
                        }
                        .to_statement(),
                        VariableDeclaration {
                            memory: MemoryModifier::Var,
                            identifier: "c".to_string(),
                            schema_type: None,
                            expression: Expression::Literal(Literal::Number("3".to_string())),
                        }
                        .to_statement(),
                        VariableDeclaration {
                            memory: MemoryModifier::Addr,
                            identifier: "d".to_string(),
                            schema_type: None,
                            expression: Expression::Literal(Literal::Number("4".to_string())),
                        }
                        .to_statement(),
                        VariableDeclaration {
                            memory: MemoryModifier::Dim,
                            identifier: "e".to_string(),
                            schema_type: Some(SchemaType {
                                identifier: SchemaIdentifier::Int32,
                                postfix: TypePostfix::None,
                                family: TypeFamily::Int,
                                width: Some(32),
                            }),
                            expression: Expression::Literal(Literal::Number("-5".to_string())),
                        }
                        .to_statement(),
                        VariableDeclaration {
                            memory: MemoryModifier::Dim,
                            identifier: "f".to_string(),
                            schema_type: Some(SchemaType {
                                identifier: SchemaIdentifier::Float64,
                                postfix: TypePostfix::None,
                                family: TypeFamily::Float,
                                width: Some(64),
                            }),
                            expression: Expression::Literal(Literal::Number("6.2".to_string())),
                        }
                        .to_statement(),
                        VariableDeclaration {
                            memory: MemoryModifier::Dim,
                            identifier: "g".to_string(),
                            schema_type: Some(SchemaType {
                                identifier: SchemaIdentifier::Bool,
                                postfix: TypePostfix::None,
                                family: TypeFamily::None,
                                width: None,
                            }),
                            expression: Expression::Literal(Literal::Boolean(true)),
                        }
                        .to_statement(),
                        VariableDeclaration {
                            memory: MemoryModifier::Dim,
                            identifier: "h".to_string(),
                            schema_type: None,
                            expression: Expression::Literal(Literal::Number("3.14".to_string())),
                        }
                        .to_statement(),
                        VariableDeclaration {
                            memory: MemoryModifier::Dim,
                            identifier: "my_z_var".to_string(),
                            schema_type: Some(SchemaType {
                                identifier: SchemaIdentifier::Int64,
                                postfix: TypePostfix::None,
                                family: TypeFamily::Int,
                                width: Some(64),
                            }),
                            expression: Expression::Literal(Literal::Number("0".to_string())),
                        }
                        .to_statement(),
                        VariableDeclaration {
                            memory: MemoryModifier::Dim,
                            identifier: "maybe_num".to_string(),
                            schema_type: Some(SchemaType {
                                identifier: SchemaIdentifier::Int32,
                                postfix: TypePostfix::Opt,
                                family: TypeFamily::Int,
                                width: Some(32),
                            }),
                            expression: Expression::Literal(Literal::Null),
                        }
                        .to_statement(),
                    ],
                },
            }],
        };
        let actual = run(&src);
        let expected = CSrc {
            includes: vec![CInclude {
                file: "stdint.h".to_string(),
            }],
            functions: vec![CFunction {
                name: "main".to_string(),
                return_type: CType {
                    name: "int".to_string(),
                    is_pointer: false,
                },
                block: CBlock {
                    statements: vec![
                        CVariableDeclaration {
                            name: "a".to_string(),
                            var_type: CType {
                                name: "int32_t".to_string(),
                                is_pointer: false,
                            },
                            value: CLiteral::Number("1".to_string()).to_expression(),
                        }
                        .to_statement(),
                        CVariableDeclaration {
                            name: "b".to_string(),
                            var_type: CType {
                                name: "int32_t".to_string(),
                                is_pointer: true,
                            },
                            value: CLiteral::Number("2".to_string()).to_expression(),
                        }
                        .to_statement(),
                        CVariableDeclaration {
                            name: "c".to_string(),
                            var_type: CType {
                                name: "int32_t".to_string(),
                                is_pointer: false,
                            },
                            value: CLiteral::Number("3".to_string()).to_expression(),
                        }
                        .to_statement(),
                        CVariableDeclaration {
                            name: "d".to_string(),
                            var_type: CType {
                                name: "void".to_string(),
                                is_pointer: true,
                            },
                            value: CLiteral::Number("4".to_string()).to_expression(),
                        }
                        .to_statement(),
                        CVariableDeclaration {
                            name: "e".to_string(),
                            var_type: CType {
                                name: "int32_t".to_string(),
                                is_pointer: false,
                            },
                            value: CLiteral::Number("-5".to_string()).to_expression(),
                        }
                        .to_statement(),
                        CVariableDeclaration {
                            name: "f".to_string(),
                            var_type: CType {
                                name: "double".to_string(),
                                is_pointer: false,
                            },
                            value: CLiteral::Number("6.2".to_string()).to_expression(),
                        }
                        .to_statement(),
                        CVariableDeclaration {
                            name: "g".to_string(),
                            var_type: CType {
                                name: "int".to_string(),
                                is_pointer: false,
                            },
                            value: CLiteral::Number("1".to_string()).to_expression(),
                        }
                        .to_statement(),
                        CVariableDeclaration {
                            name: "h".to_string(),
                            var_type: CType {
                                name: "double".to_string(),
                                is_pointer: false,
                            },
                            value: CLiteral::Number("3.14".to_string()).to_expression(),
                        }
                        .to_statement(),
                        CVariableDeclaration {
                            name: "my_z_var".to_string(),
                            var_type: CType {
                                name: "int64_t".to_string(),
                                is_pointer: false,
                            },
                            value: CLiteral::Number("0".to_string()).to_expression(),
                        }
                        .to_statement(),
                        CVariableDeclaration {
                            name: "maybe_num".to_string(),
                            var_type: CType {
                                name: "int32_t".to_string(),
                                is_pointer: false,
                            },
                            value: CExpression::Variable("INT32_MIN".to_string()),
                        }
                        .to_statement(),
                        CReturn {
                            value: Some(CLiteral::Number("0".to_string()).to_expression()),
                        }
                        .to_statement(),
                    ],
                },
            }],
        };

        assert_eq!(actual, expected)
    }
}
