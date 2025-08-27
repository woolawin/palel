use crate::palel::{
    Expression, Literal, MemoryModifier, Type, TypeFamily, TypePostfix, VariableType, bool_type,
    float64_type, int32_type, null_type,
};

pub fn determine_variable_type(
    memory: MemoryModifier,
    spec: Option<Type>,
    expr: &Expression,
) -> Option<VariableType> {
    match memory {
        MemoryModifier::Addr => Some(VariableType::Addr(spec)),
        MemoryModifier::Ref => spec
            .or(type_of_expression(&expr))
            .map(|t| VariableType::Ref(t)),
        MemoryModifier::Dim | MemoryModifier::Var => spec
            .or(type_of_expression(&expr))
            .map(|t| VariableType::Dim(t)),
    }
}

pub fn type_of_expression(expr: &Expression) -> Option<Type> {
    match expr {
        Expression::Literal(literal) => match literal {
            Literal::Boolean(_) => Some(bool_type()),
            Literal::Null => Some(null_type()),
            Literal::Number(value) => {
                if value.contains(".") {
                    Some(float64_type())
                } else {
                    Some(int32_type())
                }
            }
            Literal::String(_) => None,
        },
    }
}

pub fn is_valid_expression_assignment(to: VariableType, from: Type) -> bool {
    match to {
        VariableType::Addr(typ) => match typ {
            None => true,
            Some(addrtyp) => can_implicitly_convert(addrtyp, from),
        },
        VariableType::Dim(dimtype) => can_implicitly_convert(dimtype, from),
        VariableType::Ref(reftype) => can_implicitly_convert(reftype, from),
    }
}

pub fn can_implicitly_convert(to: Type, from: Type) -> bool {
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

    fn with_postfix(typ: Type, postfix: TypePostfix) -> Type {
        Type {
            identifier: typ.identifier,
            postfix: postfix,
            family: typ.family,
            size: typ.size,
        }
    }

    #[test]
    fn test_same_types() {
        assert!(can_implicitly_convert(int32_type(), int32_type()));
        assert!(can_implicitly_convert(int64_type(), int64_type()));
        assert!(can_implicitly_convert(float32_type(), float32_type()));
        assert!(can_implicitly_convert(float64_type(), float64_type()));
        assert!(can_implicitly_convert(bool_type(), bool_type()));
    }

    #[test]
    fn test_null() {
        assert!(!can_implicitly_convert(int32_type(), null_type()));
        assert!(!can_implicitly_convert(int64_type(), null_type()));
        assert!(!can_implicitly_convert(float32_type(), null_type()));
        assert!(!can_implicitly_convert(float64_type(), null_type()));

        assert!(can_implicitly_convert(
            with_postfix(int32_type(), TypePostfix::Opt),
            null_type()
        ));
        assert!(can_implicitly_convert(
            with_postfix(int64_type(), TypePostfix::Opt),
            null_type()
        ));
        assert!(can_implicitly_convert(
            with_postfix(float32_type(), TypePostfix::Opt),
            null_type()
        ));
        assert!(can_implicitly_convert(
            with_postfix(float64_type(), TypePostfix::Opt),
            null_type()
        ));
    }

    #[test]
    fn test_widening() {
        assert!(can_implicitly_convert(int64_type(), int32_type()));
        assert!(can_implicitly_convert(float64_type(), float32_type()));

        assert!(!can_implicitly_convert(int32_type(), int64_type()));
        assert!(!can_implicitly_convert(float32_type(), float64_type()));
    }

    #[test]
    fn test_incompatible() {
        assert!(!can_implicitly_convert(int64_type(), float64_type()));
        assert!(!can_implicitly_convert(float64_type(), int64_type()));
        assert!(!can_implicitly_convert(bool_type(), int64_type()));
        assert!(!can_implicitly_convert(float64_type(), bool_type()));
    }
}
