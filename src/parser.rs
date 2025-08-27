use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;

use crate::build_task::SrcFile;
use crate::compilation_error::{CompilationError, FailedToParseSrcFile};
use crate::palel::*;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct PalelParser;

pub fn parse(src: &mut Src, file: &SrcFile) -> Option<Box<dyn CompilationError>> {
    let mut parse = match PalelParser::parse(Rule::src, &file.content) {
        Ok(p) => p,
        Err(e) => {
            println!("{}", e);
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
            Rule::statement => {
                if let Some(statement) = parse_statement(inner) {
                    do_block.statements.push(statement);
                }
            }
            _ => {}
        }
    }
    do_block
}

fn parse_statement(rule: Pair<'_, Rule>) -> Option<Statement> {
    for inner in rule.into_inner() {
        match inner.as_rule() {
            Rule::procedure_call => {
                return Some(parse_procedure_call(inner).to_statement());
            }
            Rule::return_stmt => {
                return Some(parse_return_statement(inner).to_statement());
            }
            Rule::variable_statement => {
                return parse_variable_declaration(inner).map(|vd| vd.to_statement());
            }
            _ => {}
        }
    }
    None
}

fn parse_variable_declaration(rule: Pair<'_, Rule>) -> Option<VariableDeclaration> {
    let mut var = VariableDeclaration {
        memory: MemoryModifier::Var,
        identifier: "".to_string(),
        value_type: None,
        value: Expression::Literal(Literal::Null),
    };
    for inner in rule.into_inner() {
        match inner.as_rule() {
            Rule::memory_modifier => {
                var.memory = get_memory_modifier(inner);
            }
            Rule::variable_identifier => {
                var.identifier = get_identifier(inner);
            }
            Rule::type_spec => {
                var.value_type = Some(parse_type_spec(inner));
            }
            Rule::expression => {
                match parse_expression(inner) {
                    Some(expr) => {
                        var.value = expr;
                    }
                    None => {
                        return None;
                    }
                };
            }
            _ => {}
        }
    }

    Some(var)
}

fn parse_return_statement(rule: Pair<'_, Rule>) -> Return {
    let mut return_stmt = Return { value: None };
    for inner in rule.into_inner() {
        match inner.as_rule() {
            Rule::expression => {
                return_stmt.value = parse_expression(inner);
            }
            _ => {}
        }
    }
    return_stmt
}

fn parse_procedure_call(rule: Pair<'_, Rule>) -> ProcedureCall {
    let mut procedure_call = ProcedureCall {
        interface: "".to_string(),
        identifier: "".to_string(),
        arguments: Vec::new(),
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
                procedure_call.arguments = parse_argument_list(inner);
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

fn parse_type_spec(rule: Pair<'_, Rule>) -> Type {
    let mut typ = null_type();
    for inner in rule.into_inner() {
        match inner.as_rule() {
            Rule::type_name => {
                typ.set_identifier(inner.as_str().to_string());
            }
            Rule::type_postfix => {
                match inner.as_str() {
                    "?" => {
                        typ.postfix = TypePostfix::Opt;
                    }
                    "!" => {
                        typ.postfix = TypePostfix::Err;
                    }
                    _ => {}
                };
            }
            _ => {}
        }
    }
    return typ;
}

fn get_memory_modifier(rule: Pair<'_, Rule>) -> MemoryModifier {
    match rule.as_str() {
        "dim" => MemoryModifier::Dim,
        "ref" => MemoryModifier::Ref,
        "addr" => MemoryModifier::Addr,
        _ => MemoryModifier::Var,
    }
}

fn parse_argument_list(rule: Pair<'_, Rule>) -> Vec<Expression> {
    let mut expressions: Vec<Expression> = Vec::new();
    for inner in rule.into_inner() {
        match inner.as_rule() {
            Rule::expression => {
                if let Some(expression) = parse_expression(inner) {
                    expressions.push(expression);
                }
            }
            _ => {}
        }
    }
    expressions
}

fn parse_expression(rule: Pair<'_, Rule>) -> Option<Expression> {
    for inner in rule.into_inner() {
        match inner.as_rule() {
            Rule::string => {
                return Some(Literal::String(get_string(inner)).to_expression());
            }
            Rule::number => {
                return Some(Literal::Number(get_value(inner)).to_expression());
            }
            Rule::boolean => {
                return Some(Literal::Boolean(get_bool_value(inner)).to_expression());
            }
            Rule::null => {
                return Some(Literal::Null.to_expression());
            }
            _ => {}
        }
    }
    None
}

fn get_string(rule: Pair<'_, Rule>) -> String {
    let val = rule.as_str();
    val[1..val.len() - 1].to_string()
}

fn get_value(rule: Pair<'_, Rule>) -> String {
    rule.as_str().to_string()
}

fn get_bool_value(rule: Pair<'_, Rule>) -> bool {
    rule.as_str() == "true"
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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
                            arguments: Vec::new(),
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
                            arguments: vec![
                                Literal::String("Hello World".to_string()).to_expression(),
                            ],
                        }
                        .to_statement(),
                    ],
                },
            }],
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_variable_declarations() {
        let input = r#"
        program do
           dim a = 1
           ref b = 2
           var c = 3
           addr d = 4
           dim e Int32 = -5
           dim f Float64 = 6.2
           dim g Bool = true

           insert_space_here
           insert_space_here

           dim my_z_var Int64 = null

           dim maybe_num Int32? = null
       end
        "#
        .replace("insert_space_here", "   ");

        let actual = run(&input);
        let expected = Src {
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
                                family: TypeFamily::Int,
                                size: Some(32),
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
                                family: TypeFamily::Float,
                                size: Some(64),
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
                                family: TypeFamily::None,
                                size: None,
                            }),
                            value: Expression::Literal(Literal::Boolean(true)),
                        }
                        .to_statement(),
                        VariableDeclaration {
                            memory: MemoryModifier::Dim,
                            identifier: "my_z_var".to_string(),
                            value_type: Some(Type {
                                identifier: "Int64".to_string(),
                                postfix: TypePostfix::None,
                                family: TypeFamily::Int,
                                size: Some(64),
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
                                family: TypeFamily::Int,
                                size: Some(32),
                            }),
                            value: Expression::Literal(Literal::Null),
                        }
                        .to_statement(),
                    ],
                },
            }],
        };
        assert_eq!(actual, expected)
    }
}
