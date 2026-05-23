use crate::MethodRef;
use javac_ty::Ty;
use rust_asm::opcodes;

const FILE_INPUT_STREAM: &str = "java/io/FileInputStream";
const IO_EXCEPTION: &str = "java/io/IOException";

pub fn class_name(simple_name: &str) -> Option<&'static str> {
    match simple_name {
        "FileInputStream" => Some(FILE_INPUT_STREAM),
        "IOException" => Some(IO_EXCEPTION),
        _ => None,
    }
}

pub fn internal_class_name(internal_name: &str) -> Option<&'static str> {
    match internal_name {
        FILE_INPUT_STREAM => Some(FILE_INPUT_STREAM),
        IO_EXCEPTION => Some(IO_EXCEPTION),
        _ => None,
    }
}

pub fn package_name(package: &str) -> bool {
    package == "java/io"
}

pub fn resolve_instance_method(receiver: &Ty, name: &str, args: &[Ty]) -> Option<MethodRef> {
    match (receiver.erasure(), name, args) {
        (Ty::Class(owner), "read", []) if owner.as_str() == FILE_INPUT_STREAM => Some(MethodRef {
            owner: FILE_INPUT_STREAM,
            name: "read",
            descriptor: "()I".to_string(),
            return_ty: Ty::Int,
            opcode: opcodes::INVOKEVIRTUAL,
            is_interface: false,
        }),
        (Ty::Class(owner), "close", []) if owner.as_str() == FILE_INPUT_STREAM => Some(MethodRef {
            owner: FILE_INPUT_STREAM,
            name: "close",
            descriptor: "()V".to_string(),
            return_ty: Ty::Void,
            opcode: opcodes::INVOKEVIRTUAL,
            is_interface: false,
        }),
        _ => None,
    }
}
