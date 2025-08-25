use crate::c::{CFunctionCall, CInclude, CSrcPatch, CType};
use crate::compilation_error::UnknownInterface;
use crate::core::Of;
use crate::palel::{Expression, Literal, ProcedureCall, Type};
use crate::transpiler_c::transpile_expressions;

pub struct CToolKit {}

impl CToolKit {
    pub fn transpile_interface_call(
        &self,
        input: &ProcedureCall,
    ) -> Of<(CFunctionCall, CSrcPatch)> {
        if input.interface != "debug" {
            return Of::Error(Box::new(UnknownInterface {
                interface: input.interface.clone(),
            }));
        }

        let function_call = CFunctionCall {
            function_name: input.identifier.clone(),
            arguments: transpile_expressions(&input.arguments),
        };

        let patch = CSrcPatch {
            includes: vec![CInclude {
                file: "stdio.h".to_string(),
            }],
        };

        Of::Ok((function_call, patch))
    }

    pub fn infer_type(&self, expr: &Expression) -> Option<CType> {
        match expr {
            Expression::Literal(literal) => match literal {
                Literal::Boolean(_) => Some(int_type()),
                Literal::Number(value) => {
                    if value.contains(".") {
                        Some(double_type())
                    } else {
                        Some(int_type())
                    }
                }
                _ => None,
            },
            _ => None,
        }
    }

    pub fn transpile_builtin_type(&self, input: &Type) -> Option<CType> {
        match input.identifier.as_str() {
            "Int32" => Some(int_type()),
            "Int64" => Some(long_type()),
            "Float32" => Some(float_type()),
            "Float64" => Some(double_type()),
            "Bool" => Some(int_type()),
            _ => None,
        }
    }
}

fn int_type() -> CType {
    CType {
        name: "int".to_string(),
    }
}

fn long_type() -> CType {
    CType {
        name: "long".to_string(),
    }
}

fn float_type() -> CType {
    CType {
        name: "float".to_string(),
    }
}

fn double_type() -> CType {
    CType {
        name: "double".to_string(),
    }
}
