use crate::c::*;

pub fn render(src: &CSrc) -> String {
    let mut output = String::new();
    for include in &src.includes {
        output.push_str(&render_include(include));
    }
    for function in &src.functions {
        output.push_str(&render_function(function));
    }
    output
}

pub fn render_include(include: &CInclude) -> String {
    format!("#include <{}>\n", include.file)
}

fn render_function(function: &CFunction) -> String {
    let mut output = String::new();
    output.push_str(render_type(&function.return_type));
    output.push_str(" ");
    output.push_str(&function.name);
    output.push_str("()\n");
    output.push_str(&render_block(&function.block));
    output
}

fn render_block(block: &CBlock) -> String {
    let mut output = String::new();
    output.push_str("{\n");
    for statement in &block.statements {
        output.push_str(&render_statement(statement));
    }
    output.push_str("}\n");
    output
}

fn render_statement(statement: &CStatement) -> String {
    let mut output = match statement {
        CStatement::FunctionCall(function_call) => render_function_call(function_call),
        CStatement::Return(ret) => render_return(ret),
        _ => panic!("TODO"),
    };
    output.push_str(";\n");
    output
}

fn render_return(ret: &CReturn) -> String {
    let mut output = String::new();
    output.push_str("return");
    if let Some(expr) = &ret.value {
        output.push_str(" ");
        output.push_str(render_expression(&expr).as_str());
    }
    output
}

fn render_function_call(function_call: &CFunctionCall) -> String {
    let mut output = String::new();
    output.push_str(&function_call.function_name);
    output.push_str("(");
    for (idx, argument) in function_call.arguments.iter().enumerate() {
        let is_last = idx == function_call.arguments.len() - 1;
        output.push_str(&render_expression(argument));
        if !is_last {
            output.push_str(",");
        }
    }
    output.push_str(")");
    output
}

fn render_expression(argument: &CExpression) -> String {
    match argument {
        CExpression::Literal(literal) => render_literal(literal),
    }
}
fn render_literal(literal: &CLiteral) -> String {
    match literal {
        CLiteral::Number(value) => value.clone(),
        CLiteral::String(value) => format!("\"{}\"", value),
    }
}

fn render_type(typ: &CType) -> &str {
    &typ.name
}

#[cfg(test)]
mod tests {
    use super::*;

    fn norm(input: &str) -> String {
        input
            .lines()
            .map(|line| line.trim_start())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn test_hello_word() {
        let src = CSrc {
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

        let expected = r#"
        #include <stdio.h>
        int main()
        {
        printf("Hello World");
        return 0;
        }
        "#;
        let actual = render(&src);

        assert_eq!(norm(&actual), norm(&expected))
    }
}
