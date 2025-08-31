use crate::c::{
    CExpression, CFunctionCall, CInclude, CLiteral, CSrcPatch, CType, double_type, float_type,
    int_type, long_type, void_type,
};
use crate::compilation_error::{TypeNotNullable, UnknownInterface};
use crate::palel::{ProcedureCall, SchemaIdentifier, Type};
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

    pub fn transpile_type(&self, typ: &Type) -> CTranspile<Option<CType>> {
        fn patch(stdint: bool) -> CSrcPatch {
            let mut p = CSrcPatch { includes: vec![] };
            if stdint {
                p.includes.push(CInclude {
                    file: "stdint.h".to_string(),
                });
            }
            p
        };
        fn map_type(type_name: &SchemaIdentifier, pointer: bool) -> CTranspile<Option<CType>> {
            match type_name {
                SchemaIdentifier::Int32 => Ok(
                    Some(CType {
                        name: "int32_t".to_string(),
                        is_pointer: pointer,
                    }),
                    patch(true),
                ),
                SchemaIdentifier::Int64 => Ok(
                    Some(CType {
                        name: "int64_t".to_string(),
                        is_pointer: pointer,
                    }),
                    patch(true),
                ),
                SchemaIdentifier::Float32 => Ok(
                    Some(CType {
                        name: "float".to_string(),
                        is_pointer: pointer,
                    }),
                    patch(false),
                ),
                SchemaIdentifier::Float64 => Ok(
                    Some(CType {
                        name: "double".to_string(),
                        is_pointer: pointer,
                    }),
                    patch(false),
                ),
                SchemaIdentifier::Bool => Ok(
                    Some(CType {
                        name: "int".to_string(),
                        is_pointer: pointer,
                    }),
                    patch(false),
                ),
                _ => Ok(None, CSrcPatch::default()),
            }
        }

        match typ {
            Type::Addr(_) => Ok(
                Some(CType {
                    name: "void".to_string(),
                    is_pointer: true,
                }),
                CSrcPatch::default(),
            ),
            Type::Ref(reftyp) => map_type(&reftyp.identifier, true),
            Type::Dim(dimtype) => map_type(&dimtype.identifier, false),
        }
    }

    pub fn transpile_null(&self, typ: &Type) -> CTranspile<CExpression> {
        let stdint = CSrcPatch {
            includes: vec![CInclude {
                file: "stdint.h".to_string(),
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
            Type::Dim(dimtype) => match dimtype.identifier {
                SchemaIdentifier::Int32 => Ok(int32_min_variable(), stdint),
                SchemaIdentifier::Int64 => Ok(int64_min_variable(), stdint),
                SchemaIdentifier::Float64 => {
                    Ok(CExpression::Variable("-DBL_MAX".to_string()), float)
                }
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

fn int32_min_variable() -> CExpression {
    return CExpression::Variable("INT32_MIN".to_string());
}

fn int64_min_variable() -> CExpression {
    return CExpression::Variable("INT64_MIN".to_string());
}
