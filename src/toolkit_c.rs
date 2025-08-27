use crate::c::{
    CFunctionCall, CInclude, CSrcPatch, CType, double_type, float_type, int_type, long_type,
    void_type,
};
use crate::compilation_error::UnknownInterface;
use crate::core::Of;
use crate::palel::{ProcedureCall, Type};
use crate::transpiler_c::{CTranspile, transpile_expressions};

pub struct CToolKit {}

impl CToolKit {
    pub fn transpile_interface_call(&self, input: &ProcedureCall) -> CTranspile<CFunctionCall> {
        if input.interface != "debug" {
            return CTranspile::Error(Box::new(UnknownInterface {
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

        CTranspile::Ok(function_call, patch)
    }

    pub fn transpile_type(&self, typ: &Type) -> Option<CType> {
        fn map_type(type_name: &String) -> Option<CType> {
            match type_name.as_str() {
                "Int32" => Some(int_type()),
                "Int64" => Some(long_type()),
                "Float32" => Some(float_type()),
                "Float64" => Some(double_type()),
                "Bool" => Some(int_type()),
                _ => None,
            }
        }

        fn as_pointer(typ: CType) -> CType {
            CType {
                name: typ.name,
                is_pointer: true,
            }
        }

        match typ {
            Type::Addr(_) => Some(void_type(true)),
            Type::Ref(reftyp) => map_type(&reftyp.identifier).map(as_pointer),
            Type::Dim(dimtype) => map_type(&dimtype.identifier),
        }
    }
}
