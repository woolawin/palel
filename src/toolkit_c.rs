use crate::c::{CFunctionCall, CInclude, CSrcPatch, CType};
use crate::compilation_error::UnknownInterface;
use crate::core::Of;
use crate::palel::{ProcedureCall, Type};
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

    pub fn transpile_builtin_type(&self, input: &Type) -> Option<CType> {
        match input.identifier.as_str() {
            "Int32" => Some(CType {
                name: "int".to_string(),
            }),
            "Int64" => Some(CType {
                name: "long".to_string(),
            }),
            "Float32" => Some(CType {
                name: "float".to_string(),
            }),
            "Float64" => Some(CType {
                name: "double".to_string(),
            }),
            "Bool" => Some(CType {
                name: "int".to_string(),
            }),
            _ => None,
        }
    }
}
