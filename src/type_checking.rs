use crate::palel::{MemoryModifier, Type, TypeFamily};

pub fn is_valid_variable_declaration(memory: MemoryModifier, variable: Type, value: Type) -> bool {
    match memory {
        MemoryModifier::Dim | MemoryModifier::Var => can_implicitly_convert(variable, value),
        MemoryModifier::Addr | MemoryModifier::Ref => return false,
    }
}

pub fn can_implicitly_convert(to: Type, from: Type) -> bool {
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
        assert!(can_implicitly_convert(int32_type(), int32_type()));
        assert!(can_implicitly_convert(int64_type(), int64_type()));
        assert!(can_implicitly_convert(float32_type(), float32_type()));
        assert!(can_implicitly_convert(float64_type(), float64_type()));
        assert!(can_implicitly_convert(bool_type(), bool_type()));
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
