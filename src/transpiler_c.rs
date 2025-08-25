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
        _ => panic!("TODO"),
    }
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
}
