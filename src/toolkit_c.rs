use crate::c::{
    CExpression, CFunctionCall, CInclude, CLiteral, CSrcPatch, CType, double_type, float_type,
    int_type, long_type, void_type,
};
use crate::compilation_error::{TypeNotNullable, UnknownInterface};
use crate::palel::{ProcedureCall, Type};
use crate::transpiler_c::{CTranspile, transpile_expressions};
use crate::transpiler_c_patch::merge_patch;

use CTranspile::*;

pub struct CToolKit {}

impl CToolKit {
    pub fn transpile_interface_call(&self, input: &ProcedureCall) -> CTranspile<CFunctionCall> {
        if input.interface != "debug" {
            return Error(Box::new(UnknownInterface {
                interface: input.interface.clone(),
            }));
        }
        let mut patch = CSrcPatch {
            includes: vec![CInclude {
                file: "stdio.h".to_string(),
            }],
        };
        let expressions = match transpile_expressions(&input.arguments, self) {
            Ok(exprs, in_patch) => {
                merge_patch(&mut patch, &in_patch);
                exprs
            }
            Error(e) => {
                return Error(e);
            }
        };

        let function_call = CFunctionCall {
            function_name: input.identifier.clone(),
            arguments: expressions,
        };

        Ok(function_call, patch)
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

    pub fn transpile_null(&self, typ: &Type) -> CTranspile<CExpression> {
        let limits = CSrcPatch {
            includes: vec![CInclude {
                file: "limits.h".to_string(),
            }],
        };
        let float = CSrcPatch {
            includes: vec![CInclude {
                file: "float.h".to_string(),
            }],
        };
        match typ {
            Type::Addr(_) => Ok(zero_literal().to_expression(), CSrcPatch::default()),
            Type::Ref(_) => Ok(zero_literal().to_expression(), CSrcPatch::default()),
            Type::Dim(dimtype) => match dimtype.identifier.as_str() {
                "Int32" => Ok(int_min_variable(), limits),
                "Int64" => Ok(long_min_variable(), limits),
                "Float64" => Ok(CExpression::Variable("-DBL_MAX".to_string()), float),
                _ => Error(Box::new(TypeNotNullable {
                    received_type: typ.clone(),
                })),
            },
        }
    }
}

fn zero_literal() -> CLiteral {
    return CLiteral::Number("0".to_string());
}

fn int_min_variable() -> CExpression {
    return CExpression::Variable("INT_MIN".to_string());
}

fn long_min_variable() -> CExpression {
    return CExpression::Variable("LONG_MIN".to_string());
}
