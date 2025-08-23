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
    let block = match transpile_block(&input.do_block, toolkit) {
        Of::Error(err) => return Of::Error(err),
        Of::Ok((block, in_patch)) => {
            merge_patch(&mut patch, &in_patch);
            block
        }
    };
    let function = CFunction {
        name: "main".to_string(),
        return_type: void_type(),
        block: block,
    };
    Of::Ok((function, patch))
}

fn transpile_block(input: &DoBlock, toolkit: &CToolKit) -> Of<(CBlock, CSrcPatch)> {
    let mut statements: Vec<CStatement> = vec![];
    let mut patch = CSrcPatch::default();
    for statement in &input.statements {
        match statement {
            Statement::ProcedureCall(procedure_call) => {
                match transpile_procedure_call(procedure_call, toolkit) {
                    Of::Error(err) => return Of::Error(err),
                    Of::Ok((statement, in_patch)) => {
                        merge_patch(&mut patch, &in_patch);
                        statements.push(statement.to_statement());
                    }
                }
            }
        }
    }
    let block = CBlock {
        statements: statements,
    };
    return Of::Ok((block, patch));
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
        arguments: transpile_arguments(&input.argument_list),
    };

    Of::Ok((function_call, CSrcPatch::default()))
}

pub fn transpile_arguments(input: &Vec<Argument>) -> Vec<CArgument> {
    let mut arguments: Vec<CArgument> = vec![];
    for argument in input {
        arguments.push(transpile_argument(argument));
    }
    arguments
}

fn transpile_argument(input: &Argument) -> CArgument {
    match input {
        Argument::Literal(literal) => transpile_literal(&literal).to_argument(),
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

    const TOOLKIT: CToolKit = CToolKit {};

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
                                Literal::String("Hello World".to_string()).to_argument(),
                            ],
                        }
                        .to_statement(),
                    ],
                },
            }],
        };

        let actual = transpile(&src, &TOOLKIT).unwrap();
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
                                CLiteral::String("Hello World".to_string()).to_argument(),
                            ],
                        }
                        .to_statement(),
                    ],
                },
            }],
        };

        assert_eq!(actual, expected)
    }
}
