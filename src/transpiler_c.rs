use crate::c::*;
use crate::core::Of;
use crate::palel::*;
use crate::toolkit_c::CToolKit;
use crate::transpiler_c_patch::{merge_patch, patch_src};

pub fn transpile(input: &Src, toolkit: &CToolKit) -> Of<CSrc> {
    let mut src = CSrc {
        includes: vec![],
        functions: vec![],
    };
    if let Some(program) = input.programs.get(0) {
        match transpile_program(program, toolkit) {
            Of::Error(err) => return Of::Error(err),
            Of::Ok((program, patch)) => {
                src.functions.push(program);
                patch_src(&mut src, &patch);
            }
        }
    }
    Of::Ok(src)
}

fn transpile_program(input: &Program, toolkit: &CToolKit) -> Of<(CFunction, CSrcPatch)> {
    let mut patch = CSrcPatch::default();
    let mut block = match transpile_block(&input.do_block, toolkit) {
        Of::Error(err) => return Of::Error(err),
        Of::Ok((block, in_patch)) => {
            merge_patch(&mut patch, &in_patch);
            block
        }
    };
    let ret_stmt = Return {
        value: Some(Literal::Number("0".to_string()).to_expression()),
    };
    match transpile_return(&ret_stmt) {
        Of::Error(err) => return Of::Error(err),
        Of::Ok((ret, in_patch)) => {
            merge_patch(&mut patch, &in_patch);
            block.statements.push(ret.to_statement());
        }
    };
    let function = CFunction {
        name: "main".to_string(),
        return_type: int_type(),
        block: block,
    };
    Of::Ok((function, patch))
}

fn transpile_block(input: &DoBlock, toolkit: &CToolKit) -> Of<(CBlock, CSrcPatch)> {
    let mut statements: Vec<CStatement> = vec![];
    let mut patch = CSrcPatch::default();
    for statement in &input.statements {
        match transpile_statement(statement, toolkit) {
            Of::Error(err) => return Of::Error(err),
            Of::Ok((statement, in_patch)) => {
                merge_patch(&mut patch, &in_patch);
                statements.push(statement);
            }
        };
    }
    let block = CBlock {
        statements: statements,
    };
    return Of::Ok((block, patch));
}

fn transpile_statement(input: &Statement, toolkit: &CToolKit) -> Of<(CStatement, CSrcPatch)> {
    match input {
        Statement::ProcedureCall(procedure_call) => {
            match transpile_procedure_call(procedure_call, toolkit) {
                Of::Error(err) => Of::Error(err),
                Of::Ok((function_call, in_patch)) => {
                    Of::Ok((function_call.to_statement(), in_patch))
                }
            }
        }
        Statement::Return(ret) => match transpile_return(ret) {
            Of::Error(err) => Of::Error(err),
            Of::Ok((ret, patch)) => Of::Ok((ret.to_statement(), patch)),
        },
        Statement::Variable(variable_declaration) => {
            match transpile_variable_declaration(variable_declaration, toolkit) {
                Of::Error(err) => Of::Error(err),
                Of::Ok((var, patch)) => Of::Ok((var.to_statement(), patch)),
            }
        }
    }
}

fn transpile_variable_declaration(
    input: &VariableDeclaration,
    toolkit: &CToolKit,
) -> Of<(CVariableDeclaration, CSrcPatch)> {
    let mut var = CVariableDeclaration {
        name: input.identifier.clone(),
        var_type: void_type(),
        is_pointer: false,
    };

    if input.memory == MemoryModifier::Ref || input.memory == MemoryModifier::Addr {
        var.is_pointer = true;
    }

    if let Some(typ) = &input.value_type {
        if let Some(builtin) = toolkit.transpile_builtin_type(typ) {
            var.var_type = builtin;
        } else {
            var.var_type = CType {
                name: typ.identifier.clone(),
            };
        }
    }

    Of::Ok((var, CSrcPatch::default()))
}

fn transpile_return(input: &Return) -> Of<(CReturn, CSrcPatch)> {
    let ret = CReturn {
        value: input.value.as_ref().map(|expr| transpile_expression(&expr)),
    };

    Of::Ok((ret, CSrcPatch::default()))
}

fn transpile_procedure_call(
    input: &ProcedureCall,
    toolkit: &CToolKit,
) -> Of<(CFunctionCall, CSrcPatch)> {
    if !input.interface.is_empty() {
        return toolkit.transpile_interface_call(input);
    }
    let function_call = CFunctionCall {
        function_name: input.identifier.clone(),
        arguments: transpile_expressions(&input.arguments),
    };

    Of::Ok((function_call, CSrcPatch::default()))
}

pub fn transpile_expressions(input: &Vec<Expression>) -> Vec<CExpression> {
    let mut expressions: Vec<CExpression> = vec![];
    for argument in input {
        expressions.push(transpile_expression(argument));
    }
    expressions
}

fn transpile_expression(input: &Expression) -> CExpression {
    match input {
        Expression::Literal(literal) => transpile_literal(&literal).to_expression(),
    }
}

fn transpile_literal(input: &Literal) -> CLiteral {
    match input {
        Literal::String(str) => CLiteral::String(str.clone()),
        Literal::Number(num) => CLiteral::Number(num.clone()),
        Literal::Boolean(value) => {
            if value == "true" {
                true_literal()
            } else {
                false_literal()
            }
        }
        Literal::Null => CLiteral::Number("0".to_string()),
    }
}

fn void_type() -> CType {
    CType {
        name: "void".to_string(),
    }
}

fn int_type() -> CType {
    CType {
        name: "int".to_string(),
    }
}

fn true_literal() -> CLiteral {
    return CLiteral::Number("0".to_string());
}

fn false_literal() -> CLiteral {
    return CLiteral::Number("1".to_string());
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
                            value_type: None,
                            value: Expression::Literal(Literal::Number("1".to_string())),
                        }
                        .to_statement(),
                        VariableDeclaration {
                            memory: MemoryModifier::Ref,
                            identifier: "b".to_string(),
                            value_type: None,
                            value: Expression::Literal(Literal::Number("2".to_string())),
                        }
                        .to_statement(),
                        VariableDeclaration {
                            memory: MemoryModifier::Var,
                            identifier: "c".to_string(),
                            value_type: None,
                            value: Expression::Literal(Literal::Number("3".to_string())),
                        }
                        .to_statement(),
                        VariableDeclaration {
                            memory: MemoryModifier::Addr,
                            identifier: "d".to_string(),
                            value_type: None,
                            value: Expression::Literal(Literal::Number("4".to_string())),
                        }
                        .to_statement(),
                        VariableDeclaration {
                            memory: MemoryModifier::Dim,
                            identifier: "e".to_string(),
                            value_type: Some(Type {
                                identifier: "Int32".to_string(),
                                postfix: TypePostfix::None,
                            }),
                            value: Expression::Literal(Literal::Number("-5".to_string())),
                        }
                        .to_statement(),
                        VariableDeclaration {
                            memory: MemoryModifier::Dim,
                            identifier: "f".to_string(),
                            value_type: Some(Type {
                                identifier: "Float64".to_string(),
                                postfix: TypePostfix::None,
                            }),
                            value: Expression::Literal(Literal::Number("6.2".to_string())),
                        }
                        .to_statement(),
                        VariableDeclaration {
                            memory: MemoryModifier::Dim,
                            identifier: "g".to_string(),
                            value_type: Some(Type {
                                identifier: "Bool".to_string(),
                                postfix: TypePostfix::None,
                            }),
                            value: Expression::Literal(Literal::Boolean("true".to_string())),
                        }
                        .to_statement(),
                        VariableDeclaration {
                            memory: MemoryModifier::Dim,
                            identifier: "my_z_var".to_string(),
                            value_type: Some(Type {
                                identifier: "Int64".to_string(),
                                postfix: TypePostfix::None,
                            }),
                            value: Expression::Literal(Literal::Null),
                        }
                        .to_statement(),
                        VariableDeclaration {
                            memory: MemoryModifier::Dim,
                            identifier: "maybe_num".to_string(),
                            value_type: Some(Type {
                                identifier: "Int32".to_string(),
                                postfix: TypePostfix::Opt,
                            }),
                            value: Expression::Literal(Literal::Null),
                        }
                        .to_statement(),
                    ],
                },
            }],
        };
        let actual = run(&src);
        let expected = CSrc {
            includes: vec![],
            functions: vec![CFunction {
                name: "main".to_string(),
                return_type: CType {
                    name: "int".to_string(),
                },
                block: CBlock {
                    statements: vec![
                        CVariableDeclaration {
                            name: "a".to_string(),
                            is_pointer: false,
                            var_type: CType {
                                name: "void".to_string(),
                            },
                        }
                        .to_statement(),
                        CVariableDeclaration {
                            name: "b".to_string(),
                            is_pointer: true,
                            var_type: CType {
                                name: "void".to_string(),
                            },
                        }
                        .to_statement(),
                        CVariableDeclaration {
                            name: "c".to_string(),
                            is_pointer: false,
                            var_type: CType {
                                name: "void".to_string(),
                            },
                        }
                        .to_statement(),
                        CVariableDeclaration {
                            name: "d".to_string(),
                            is_pointer: true,
                            var_type: CType {
                                name: "void".to_string(),
                            },
                        }
                        .to_statement(),
                        CVariableDeclaration {
                            name: "e".to_string(),
                            is_pointer: false,
                            var_type: CType {
                                name: "int".to_string(),
                            },
                        }
                        .to_statement(),
                        CVariableDeclaration {
                            name: "f".to_string(),
                            is_pointer: false,
                            var_type: CType {
                                name: "double".to_string(),
                            },
                        }
                        .to_statement(),
                        CVariableDeclaration {
                            name: "g".to_string(),
                            is_pointer: false,
                            var_type: CType {
                                name: "int".to_string(),
                            },
                        }
                        .to_statement(),
                        CVariableDeclaration {
                            name: "my_z_var".to_string(),
                            is_pointer: false,
                            var_type: CType {
                                name: "long".to_string(),
                            },
                        }
                        .to_statement(),
                        CVariableDeclaration {
                            name: "maybe_num".to_string(),
                            is_pointer: false,
                            var_type: CType {
                                name: "int".to_string(),
                            },
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
