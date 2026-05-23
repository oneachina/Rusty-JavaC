use crate::MethodRef;
use javac_ty::Ty;
use rust_asm::opcodes;

const STRING_CLASS: &str = "java/lang/String";
const OBJECT_CLASS: &str = "java/lang/Object";
const THROWABLE_CLASS: &str = "java/lang/Throwable";

pub(super) fn resolve_instance_method(receiver: &Ty, name: &str, args: &[Ty]) -> Option<MethodRef> {
    match (receiver.erasure(), name, args) {
        (Ty::Class(owner), "hashCode", []) if owner.as_str() == OBJECT_CLASS => Some(MethodRef {
            owner: OBJECT_CLASS.to_string(),
            name: "hashCode".to_string(),
            descriptor: "()I".to_string(),
            return_ty: Ty::Int,
            params: Vec::new(),
            opcode: opcodes::INVOKEVIRTUAL,
            is_interface: false,
            access_flags: 0,
        }),
        (Ty::Class(_), "printStackTrace", []) => Some(MethodRef {
            owner: THROWABLE_CLASS.to_string(),
            name: "printStackTrace".to_string(),
            descriptor: "()V".to_string(),
            return_ty: Ty::Void,
            params: Vec::new(),
            opcode: opcodes::INVOKEVIRTUAL,
            is_interface: false,
            access_flags: 0,
        }),
        (Ty::Class(owner), "length", []) if owner.as_str() == STRING_CLASS => Some(MethodRef {
            owner: STRING_CLASS.to_string(),
            name: "length".to_string(),
            descriptor: "()I".to_string(),
            return_ty: Ty::Int,
            params: Vec::new(),
            opcode: opcodes::INVOKEVIRTUAL,
            is_interface: false,
            access_flags: 0,
        }),
        (Ty::Class(owner), "charAt", [Ty::Int]) if owner.as_str() == STRING_CLASS => {
            Some(MethodRef {
                owner: STRING_CLASS.to_string(),
                name: "charAt".to_string(),
                descriptor: "(I)C".to_string(),
                return_ty: Ty::Char,
                params: vec![Ty::Int],
                opcode: opcodes::INVOKEVIRTUAL,
                is_interface: false,
                access_flags: 0,
            })
        }
        _ => None,
    }
}
