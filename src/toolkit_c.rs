use crate::c::{CFunctionCall, CInclude, CSrc, CSrcPatch};
use crate::palel::ProcedureCall;
use crate::transpiler_c::transpile_arguments;

pub struct CToolKit {}

impl CToolKit {
    pub fn transpile_interface_call(&self, input: &ProcedureCall) -> (CFunctionCall, CSrcPatch) {
        let function_call = CFunctionCall {
            function_name: input.identifier.clone(),
            arguments: transpile_arguments(&input.argument_list),
        };

        let patch = CSrcPatch {
            includes: vec![CInclude {
                file: "stdio.h".to_string(),
            }],
        };

        (function_call, patch)
    }
}
