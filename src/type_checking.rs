use crate::palel::{
    Expression, Literal, MemoryModifier, SchemaType, Type, TypeFamily, TypePostfix, bool_type,
    charseq_type, float64_type, int32_type, null_type,
};

pub fn determine_variable_type(
    memory: MemoryModifier,
    provided_schema: Option<SchemaType>,
    expr: &Expression,
) -> Option<Type> {
    fn as_ref(typ: Type) -> Option<Type> {
        match typ {
            Type::Addr(schema) => match schema {
                Some(addrtyp) => Some(Type::Ref(addrtyp)),
                None => None,
            },
            Type::Dim(schema) => Some(Type::Ref(schema)),
            rf @ Type::Ref(_) => Some(rf),
        }
    }
    match memory {
        MemoryModifier::Dim | MemoryModifier::Var => match provided_schema {
            Some(schema) => Some(Type::Dim(schema)),
            None => match type_of_expression(expr) {
                Some(t) => {
                    if t.is_null() {
                        None
                    } else {
                        Some(t)
                    }
                }
                None => {
                    return None;
                }
            },
        },
        MemoryModifier::Addr => Some(Type::Addr(provided_schema)),
        MemoryModifier::Ref => match provided_schema {
            Some(schema) => Some(Type::Ref(schema)),
            None => match type_of_expression(expr) {
                Some(t) => as_ref(t),
                None => {
                    return None;
                }
            },
        },
    }
}

pub fn type_of_expression(expr: &Expression) -> Option<Type> {
    match expr {
        Expression::Literal(literal) => match literal {
            Literal::Boolean(_) => Some(Type::Dim(bool_type())),
            Literal::Null => Some(Type::Dim(null_type())),
            Literal::Number(value) => {
                if value.contains(".") {
                    Some(Type::Dim(float64_type()))
                } else {
                    Some(Type::Dim(int32_type()))
                }
            }
            Literal::String(_) => Some(Type::Dim(charseq_type())),
        },
    }
}

pub fn is_valid_expression_assignment(to: &Type, from: &Type) -> bool {
    match (to, from) {
        (Type::Addr(to_type), Type::Addr(from_type)) => match (to_type, from_type) {
            (None, _) => true,
            (Some(_), None) => false,
            (Some(to_addrtype), Some(from_addrtype)) => {
                can_implicitly_convert(to_addrtype, from_addrtype)
            }
        },
        (Type::Addr(to_type), Type::Dim(from_type) | Type::Ref(from_type)) => match to_type {
            None => true,
            Some(to_reftype) => can_implicitly_convert(to_reftype, from_type),
        },
        (Type::Ref(to_reftype), Type::Ref(from_reftype)) => {
            can_implicitly_convert(to_reftype, from_reftype)
        }
        (Type::Ref(to_reftype), Type::Dim(from_dimtype)) => {
            can_implicitly_convert(to_reftype, from_dimtype)
        }
        (Type::Ref(_), Type::Addr(_)) => false,
        (Type::Dim(to_dimtype), Type::Dim(from_dimtype)) => {
            can_implicitly_convert(to_dimtype, from_dimtype)
        }
        (Type::Dim(_), Type::Addr(_) | Type::Ref(_)) => false,
    }
}

pub fn can_implicitly_convert(to: &SchemaType, from: &SchemaType) -> bool {
    if to.postfix == TypePostfix::Opt && from.is_null() {
        return true;
    }
    if to.family == TypeFamily::None || from.family == TypeFamily::None {
        return to == from;
    }

    if to.family != from.family {
        return false;
    }

    let to_size = to.size.unwrap_or(0);
    let from_size = from.size.unwrap_or(0);

    return to_size >= from_size;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::palel::{bool_type, float32_type, float64_type, int32_type, int64_type};

    fn with_postfix(typ: SchemaType, postfix: TypePostfix) -> SchemaType {
        SchemaType {
            identifier: typ.identifier,
            postfix: postfix,
            family: typ.family,
            size: typ.size,
        }
    }

    #[test]
    fn test_same_types() {
        assert!(can_implicitly_convert(&int32_type(), &int32_type()));
        assert!(can_implicitly_convert(&int64_type(), &int64_type()));
        assert!(can_implicitly_convert(&float32_type(), &float32_type()));
        assert!(can_implicitly_convert(&float64_type(), &float64_type()));
        assert!(can_implicitly_convert(&bool_type(), &bool_type()));
    }

    #[test]
    fn test_null() {
        assert!(!can_implicitly_convert(&int32_type(), &null_type()));
        assert!(!can_implicitly_convert(&int64_type(), &null_type()));
        assert!(!can_implicitly_convert(&float32_type(), &null_type()));
        assert!(!can_implicitly_convert(&float64_type(), &null_type()));

        assert!(can_implicitly_convert(
            &with_postfix(int32_type(), TypePostfix::Opt),
            &null_type()
        ));
        assert!(can_implicitly_convert(
            &with_postfix(int64_type(), TypePostfix::Opt),
            &null_type()
        ));
        assert!(can_implicitly_convert(
            &with_postfix(float32_type(), TypePostfix::Opt),
            &null_type()
        ));
        assert!(can_implicitly_convert(
            &with_postfix(float64_type(), TypePostfix::Opt),
            &null_type()
        ));
    }

    #[test]
    fn test_widening() {
        assert!(can_implicitly_convert(&int64_type(), &int32_type()));
        assert!(can_implicitly_convert(&float64_type(), &float32_type()));

        assert!(!can_implicitly_convert(&int32_type(), &int64_type()));
        assert!(!can_implicitly_convert(&float32_type(), &float64_type()));
    }

    #[test]
    fn test_incompatible() {
        assert!(!can_implicitly_convert(&int64_type(), &float64_type()));
        assert!(!can_implicitly_convert(&float64_type(), &int64_type()));
        assert!(!can_implicitly_convert(&bool_type(), &int64_type()));
        assert!(!can_implicitly_convert(&float64_type(), &bool_type()));
    }
}
