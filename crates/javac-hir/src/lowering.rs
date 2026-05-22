use crate::hir::*;
use javac_ast::ast::{AstNode, ClassDecl, CompilationUnit as AstCompilationUnit};
use javac_ast::{JavaSyntaxKind, JavaSyntaxNode};
use ustr::Ustr;

pub fn lower(node: &JavaSyntaxNode) -> Option<CompilationUnit> {
    let unit = AstCompilationUnit::cast(node.clone())?;
    if unit.package().is_some() || unit.imports().next().is_some() {
        return None;
    }

    let mut pending_flags = 0;
    let mut type_decls = Vec::new();
    for child in node.children() {
        match child.kind() {
            JavaSyntaxKind::ModifierList => pending_flags = access_flags(&child),
            JavaSyntaxKind::ClassDecl => {
                let class = ClassDecl::cast(child)?;
                type_decls.push(lower_class_decl(class, pending_flags)?);
                pending_flags = 0;
            }
            JavaSyntaxKind::InterfaceDecl
            | JavaSyntaxKind::EnumDecl
            | JavaSyntaxKind::RecordDecl
            | JavaSyntaxKind::AnnotationDecl => return None,
            _ => {}
        }
    }

    if type_decls.len() != 1 {
        return None;
    }

    Some(CompilationUnit {
        package: None,
        imports: Vec::new(),
        type_decls: vec![type_decls.remove(0)],
    })
}

fn lower_class_decl(class: ClassDecl, access_flags: u16) -> Option<TypeDecl> {
    if class
        .body()
        .is_some_and(|body| body.members().next().is_some())
    {
        return None;
    }

    let name = class.name()?;
    Some(TypeDecl {
        id: HirId(0),
        name: Ustr::from(name.text()),
        kind: TypeDeclKind::Class,
        access_flags,
        super_class: None,
        interfaces: Vec::new(),
        type_params: Vec::new(),
        fields: Vec::new(),
        methods: Vec::new(),
        inner_types: Vec::new(),
    })
}

fn access_flags(node: &JavaSyntaxNode) -> u16 {
    node.descendants_with_tokens()
        .filter_map(|element| element.into_token())
        .fold(0, |flags, token| match token.kind() {
            JavaSyntaxKind::PublicKw => flags | 0x0001,
            JavaSyntaxKind::FinalKw => flags | 0x0010,
            JavaSyntaxKind::AbstractKw => flags | 0x0400,
            _ => flags,
        })
}
