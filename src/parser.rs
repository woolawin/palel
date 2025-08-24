use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;

use crate::compilation_error::{CompilationError, FailedToParseSrcFile};
use crate::palel::*;
use crate::build_task::SrcFile;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct PalelParser;

pub fn parse(src: &mut Src, file: &SrcFile) -> Option<Box<dyn CompilationError>> {
    let mut parse = match PalelParser::parse(Rule::src, &file.content) {
        Ok(p) => p,
        Err(_) => {
            return Some(Box::new(FailedToParseSrcFile {
                file: file.file.clone(),
            }));
        }
    };
    if let Some(root) = parse.next() {
        parse_root(src, root);
        None
    } else {
        Some(Box::new(FailedToParseSrcFile {
            file: file.file.clone(),
        }))
    }
}

fn parse_root(src: &mut Src, root: Pair<'_, Rule>) {
    for pair in root.into_inner() {
        match pair.as_rule() {
            Rule::program => src.programs.push(parse_program(pair)),
            _ => {}
        }
    }
}

fn parse_program(rule: Pair<'_, Rule>) -> Program {
    let mut program = Program {
        do_block: DoBlock {
            statements: Vec::new(),
        },
    };
    for inner in rule.into_inner() {
        match inner.as_rule() {
            Rule::do_block => program.do_block = parse_do_block(inner),
            _ => {}
        }
    }
    program
}

fn parse_do_block(rule: Pair<'_, Rule>) -> DoBlock {
    let mut do_block = DoBlock {
        statements: Vec::new(),
    };
    for inner in rule.into_inner() {
        match inner.as_rule() {
            Rule::procedure_call => {
                do_block
                    .statements
                    .push(parse_procedure_call(inner).to_statement());
            }
            _ => {}
        }
    }
    do_block
}

fn parse_procedure_call(rule: Pair<'_, Rule>) -> ProcedureCall {
    let mut procedure_call = ProcedureCall {
        interface: "".to_string(),
        identifier: "".to_string(),
        argument_list: Vec::new(),
    };
    for inner in rule.into_inner() {
        match inner.as_rule() {
            Rule::interface_identifier => {
                procedure_call.interface = get_identifier(inner);
            }
            Rule::procedure_identifier => {
                procedure_call.identifier = get_identifier(inner);
            }
            Rule::argument_list => {
                procedure_call.argument_list = parse_argument_list(inner);
            }
            _ => {}
        }
    }
    procedure_call
}

fn get_identifier(rule: Pair<'_, Rule>) -> String {
    for inner in rule.into_inner() {
        match inner.as_rule() {
            Rule::identifier => {
                return inner.as_str().to_string();
            }
            _ => {}
        }
    }
    return "".to_string();
}

fn parse_argument_list(rule: Pair<'_, Rule>) -> Vec<Argument> {
    let mut arguments: Vec<Argument> = Vec::new();
    for inner in rule.into_inner() {
        match inner.as_rule() {
            Rule::string => {
                arguments.push(Literal::String(get_string(inner)).to_argument());
            }
            _ => {}
        }
    }
    arguments
}

fn get_string(rule: Pair<'_, Rule>) -> String {
    let val = rule.as_str();
    val[1..val.len() - 1].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(input: &str) -> Src {
        let file = SrcFile {
            file: "./code.palel".to_string(),
            content: input.to_string(),
        };
        let mut actual = Src::default();
        match parse(&mut actual, &file) {
            Some(err) => {
                panic!("{}", err.message())
            }
            None => {}
        }
        actual
    }

    #[test]
    fn test_simple_debug() {
        let input = r#"
        program do
            debug:print()
        end
        "#;
        let actual = run(input);
        let expected = Src {
            programs: vec![Program {
                do_block: DoBlock {
                    statements: vec![
                        ProcedureCall {
                            interface: "debug".to_string(),
                            identifier: "print".to_string(),
                            argument_list: Vec::new(),
                        }
                        .to_statement(),
                    ],
                },
            }],
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_simple_debug_hello_world() {
        let input = r#"
        program do
            debug:print("Hello World")
        end
        "#;
        let actual = run(&input);
        let expected = Src {
            programs: vec![Program {
                do_block: DoBlock {
                    statements: vec![
                        ProcedureCall {
                            interface: "debug".to_string(),
                            identifier: "print".to_string(),
                            argument_list: vec![
                                Literal::String("Hello World".to_string()).to_argument(),
                            ],
                        }
                        .to_statement(),
                    ],
                },
            }],
        };
        assert_eq!(actual, expected);
    }
}
