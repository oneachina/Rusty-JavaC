use crate::MethodRef;
use javac_ty::Ty;

const LIST_CLASS: &str = "java/util/List";
const ARRAY_LIST_CLASS: &str = "java/util/ArrayList";
const FUNCTION_CLASS: &str = "java/util/function/Function";

pub fn class_name(simple_name: &str) -> Option<&'static str> {
    match simple_name {
        "List" => Some(LIST_CLASS),
        "ArrayList" => Some(ARRAY_LIST_CLASS),
        "Function" => Some(FUNCTION_CLASS),
        _ => None,
    }
}

pub fn internal_class_name(internal_name: &str) -> Option<&'static str> {
    match internal_name {
        LIST_CLASS => Some(LIST_CLASS),
        ARRAY_LIST_CLASS => Some(ARRAY_LIST_CLASS),
        FUNCTION_CLASS => Some(FUNCTION_CLASS),
        _ => None,
    }
}

pub fn package_name(package: &str) -> bool {
    matches!(package, "java/util" | "java/util/function")
}

pub fn resolve_instance_method(_receiver: &Ty, _name: &str, _args: &[Ty]) -> Option<MethodRef> {
    None
}
