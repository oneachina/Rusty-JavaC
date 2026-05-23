use crate::MethodRef;
use javac_ty::Ty;
use rust_asm::opcodes;

const FILE_INPUT_STREAM: &str = "java/io/FileInputStream";

pub(super) fn resolve_instance_method(receiver: &Ty, name: &str, args: &[Ty]) -> Option<MethodRef> {
    match (receiver.erasure(), name, args) {
        (Ty::Class(owner), "read", []) if owner.as_str() == FILE_INPUT_STREAM => Some(MethodRef {
            owner: FILE_INPUT_STREAM.to_string(),
            name: "read".to_string(),
            descriptor: "()I".to_string(),
            return_ty: Ty::Int,
            params: Vec::new(),
            opcode: opcodes::INVOKEVIRTUAL,
            is_interface: false,
            access_flags: 0,
        }),
        (Ty::Class(owner), "close", []) if owner.as_str() == FILE_INPUT_STREAM => Some(MethodRef {
            owner: FILE_INPUT_STREAM.to_string(),
            name: "close".to_string(),
            descriptor: "()V".to_string(),
            return_ty: Ty::Void,
            params: Vec::new(),
            opcode: opcodes::INVOKEVIRTUAL,
            is_interface: false,
            access_flags: 0,
        }),
        _ => None,
    }
}
