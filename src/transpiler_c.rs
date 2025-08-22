use crate::c::*;
use crate::palel::*;

pub fn transpile(input: &Src) -> CSrc {
    let mut src = CSrc {
        includes: vec![],
        functions: vec![],
    };
    if let Some(program) = input.programs.get(0) {
        src.functions.push(transpile_program(program));
    }
    src
}

fn transpile_program(input: &Program) -> CFunction {
    CFunction {
        name: "main".to_string(),
        return_type: void_type(),
        block: transpile_block(&input.do_block),
    }
}

fn transpile_block(input: &DoBlock) -> CBlock {
    let mut statements: Vec<CStatement> = vec![];
    for statement in &input.statements {
        match statement {
            Statement::ProcedureCall(procedure_call) => {
                statements.push(transpile_procedure_call(procedure_call).as_statement());
            }
        }
    }
    CBlock {
        statements: statements,
    }
}

fn transpile_procedure_call(input: &ProcedureCall) -> CFunctionCall {
    let mut arguments: Vec<CArgument> = vec![];
    for argument in &input.argument_list {
        arguments.push(transpile_argument(argument));
    }
    CFunctionCall {
        function_name: input.identifier.clone(),
        arguments: arguments,
    }
}

fn transpile_argument(input: &Argument) -> CArgument {
    match input {
        Argument::Literal(literal) => transpile_literal(&literal).as_argument(),
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
    }
}

fn void_type() -> CType {
    CType {
        name: "void".to_string(),
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

    #[test]
    fn test_transpile_hello_world() {
        let src = Src {
            programs: vec![Program {
                do_block: DoBlock {
                    statements: vec![
                        ProcedureCall {
                            interface: "debug".to_string(),
                            identifier: "printf".to_string(),
                            argument_list: vec![
                                Literal::String("Hello World".to_string()).as_argument(),
                            ],
                        }
                        .as_statement(),
                    ],
                },
            }],
        };

        let actual = transpile(&src);
        let expected = CSrc {
            includes: vec![CInclude {
                file: "stdio.h".to_string(),
            }],
            functions: vec![CFunction {
                name: "main".to_string(),
                return_type: CType {
                    name: "void".to_string(),
                },
                block: CBlock {
                    statements: vec![
                        CFunctionCall {
                            function_name: "printf".to_string(),
                            arguments: vec![
                                CLiteral::String("Hello World".to_string()).as_argument(),
                            ],
                        }
                        .as_statement(),
                    ],
                },
            }],
        };

        assert_eq!(actual, expected)
    }
}
