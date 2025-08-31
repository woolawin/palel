use crate::palel::{
    Expression, ExpressionType, Literal, MemoryModifier, SchemaType, Type, TypeFamily, TypePostfix,
    bool_type, charseq_type, float64_type, int32_type,
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
                Some(t) => t.to_type(),
                None => {
                    return None;
                }
            },
        },
        MemoryModifier::Addr => Some(Type::Addr(provided_schema)),
        MemoryModifier::Ref => match provided_schema {
            Some(schema) => Some(Type::Ref(schema)),
            None => match type_of_expression(expr) {
                Some(t) => t.to_type().and_then(as_ref),
                None => {
                    return None;
                }
            },
        },
    }
}

pub fn type_of_expression(expr: &Expression) -> Option<ExpressionType> {
    match expr {
        Expression::Literal(literal) => match literal {
            Literal::Boolean(_) => Some(ExpressionType::Dim(bool_type())),
            Literal::Null => Some(ExpressionType::Null),
            Literal::Number(value) => {
                if value.contains(".") {
                    Some(ExpressionType::Dim(float64_type()))
                } else {
                    Some(ExpressionType::Dim(int32_type()))
                }
            }
            Literal::String(_) => Some(ExpressionType::Dim(charseq_type())),
        },
    }
}

pub fn is_valid_expression_assignment(to: &Type, from: &ExpressionType) -> bool {
    match (to, from) {
        (Type::Addr(to_type), ExpressionType::Addr(from_type)) => match (to_type, from_type) {
            (None, _) => true,
            (Some(_), None) => false,
            (Some(to_addrtype), Some(from_addrtype)) => {
                can_implicitly_convert(to_addrtype, from_addrtype)
            }
        },
        (Type::Addr(to_type), ExpressionType::Dim(from_type) | ExpressionType::Ref(from_type)) => {
            match to_type {
                None => true,
                Some(to_reftype) => can_implicitly_convert(to_reftype, from_type),
            }
        }
        (Type::Addr(_), ExpressionType::Null) => true,
        (Type::Ref(to_reftype), ExpressionType::Ref(from_reftype)) => {
            can_implicitly_convert(to_reftype, from_reftype)
        }
        (Type::Ref(to_reftype), ExpressionType::Dim(from_dimtype)) => {
            can_implicitly_convert(to_reftype, from_dimtype)
        }
        (Type::Ref(_), ExpressionType::Addr(_)) => false,
        (Type::Ref(schema), ExpressionType::Null) => schema.postfix == TypePostfix::Opt,
        (Type::Dim(to_dimtype), ExpressionType::Dim(from_dimtype)) => {
            can_implicitly_convert(to_dimtype, from_dimtype)
        }
        (Type::Dim(_), ExpressionType::Addr(_) | ExpressionType::Ref(_)) => false,
        (Type::Dim(schema), ExpressionType::Null) => schema.postfix == TypePostfix::Opt,
    }
}

pub fn can_implicitly_convert(to: &SchemaType, from: &SchemaType) -> bool {
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

    #[test]
    fn test_same_types() {
        assert!(can_implicitly_convert(&int32_type(), &int32_type()));
        assert!(can_implicitly_convert(&int64_type(), &int64_type()));
        assert!(can_implicitly_convert(&float32_type(), &float32_type()));
        assert!(can_implicitly_convert(&float64_type(), &float64_type()));
        assert!(can_implicitly_convert(&bool_type(), &bool_type()));
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
