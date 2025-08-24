use crate::c::{CFunctionCall, CInclude, CSrcPatch};
use crate::compilation_error::UnknownInterface;
use crate::core::Of;
use crate::palel::ProcedureCall;
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
}
