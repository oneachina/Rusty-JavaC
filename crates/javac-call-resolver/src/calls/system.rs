use crate::{FieldRef, MethodRef};
use javac_ty::Ty;
use rust_asm::opcodes;
use ustr::Ustr;

const SYSTEM_CLASS: &str = "java/lang/System";
const PRINT_STREAM_CLASS: &str = "java/io/PrintStream";

pub(super) fn resolve_static_field(owner: &str, name: &str) -> Option<FieldRef> {
    match (owner, name) {
        (SYSTEM_CLASS, "out") => Some(FieldRef {
            owner: SYSTEM_CLASS.to_string(),
            name: "out".to_string(),
            descriptor: "Ljava/io/PrintStream;".to_string(),
            ty: Ty::Class(Ustr::from(PRINT_STREAM_CLASS)),
            access_flags: 0x0008,
        }),
        _ => None,
    }
}

pub(super) fn resolve_instance_method(receiver: &Ty, name: &str, args: &[Ty]) -> Option<MethodRef> {
    match (receiver.erasure(), name) {
        (Ty::Class(owner), "println") if owner.as_str() == PRINT_STREAM_CLASS => Some(MethodRef {
            owner: PRINT_STREAM_CLASS.to_string(),
            name: "println".to_string(),
            descriptor: void_method_descriptor(args),
            return_ty: Ty::Void,
            params: args.to_vec(),
            opcode: opcodes::INVOKEVIRTUAL,
            is_interface: false,
            access_flags: 0,
        }),
        _ => None,
    }
}

fn void_method_descriptor(args: &[Ty]) -> String {
    let mut descriptor = String::from("(");
    for arg in args {
        descriptor.push_str(&arg.erasure().descriptor());
    }
    descriptor.push_str(")V");
    descriptor
}
