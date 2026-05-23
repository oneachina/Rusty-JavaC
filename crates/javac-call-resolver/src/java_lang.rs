use crate::{FieldRef, MethodRef};
use javac_ty::Ty;
use rust_asm::opcodes;

const STRING_CLASS: &str = "java/lang/String";
const OBJECT_CLASS: &str = "java/lang/Object";
const INTEGER_CLASS: &str = "java/lang/Integer";
const THROWABLE_CLASS: &str = "java/lang/Throwable";
const ILLEGAL_ARGUMENT_EXCEPTION_CLASS: &str = "java/lang/IllegalArgumentException";

pub fn class_name(simple_name: &str) -> Option<&'static str> {
    match simple_name {
        "String" => Some(STRING_CLASS),
        "Object" => Some(OBJECT_CLASS),
        "Integer" => Some(INTEGER_CLASS),
        "Throwable" => Some(THROWABLE_CLASS),
        "IllegalArgumentException" => Some(ILLEGAL_ARGUMENT_EXCEPTION_CLASS),
        _ => None,
    }
}

pub fn internal_class_name(internal_name: &str) -> Option<&'static str> {
    match internal_name {
        STRING_CLASS => Some(STRING_CLASS),
        OBJECT_CLASS => Some(OBJECT_CLASS),
        INTEGER_CLASS => Some(INTEGER_CLASS),
        THROWABLE_CLASS => Some(THROWABLE_CLASS),
        ILLEGAL_ARGUMENT_EXCEPTION_CLASS => Some(ILLEGAL_ARGUMENT_EXCEPTION_CLASS),
        _ => None,
    }
}

pub fn package_name(package: &str) -> bool {
    package == "java/lang"
}

pub fn resolve_static_field(_owner: &str, _name: &str) -> Option<FieldRef> {
    None
}

pub fn resolve_instance_method(receiver: &Ty, name: &str, args: &[Ty]) -> Option<MethodRef> {
    match (receiver.erasure(), name, args) {
        (Ty::Class(owner), "hashCode", []) if owner.as_str() == OBJECT_CLASS => Some(MethodRef {
            owner: OBJECT_CLASS,
            name: "hashCode",
            descriptor: "()I".to_string(),
            return_ty: Ty::Int,
            opcode: opcodes::INVOKEVIRTUAL,
            is_interface: false,
        }),
        (Ty::Class(_), "printStackTrace", []) => Some(MethodRef {
            owner: THROWABLE_CLASS,
            name: "printStackTrace",
            descriptor: "()V".to_string(),
            return_ty: Ty::Void,
            opcode: opcodes::INVOKEVIRTUAL,
            is_interface: false,
        }),
        (Ty::Class(owner), "length", []) if owner.as_str() == STRING_CLASS => Some(MethodRef {
            owner: STRING_CLASS,
            name: "length",
            descriptor: "()I".to_string(),
            return_ty: Ty::Int,
            opcode: opcodes::INVOKEVIRTUAL,
            is_interface: false,
        }),
        (Ty::Class(owner), "charAt", [Ty::Int]) if owner.as_str() == STRING_CLASS => {
            Some(MethodRef {
                owner: STRING_CLASS,
                name: "charAt",
                descriptor: "(I)C".to_string(),
                return_ty: Ty::Char,
                opcode: opcodes::INVOKEVIRTUAL,
                is_interface: false,
            })
        }
        _ => None,
    }
}
