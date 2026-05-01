use crate::parser::{Parser, JavaSyntaxKind};

impl Parser {
    pub(crate) fn type_decl(&mut self) {
        use JavaSyntaxKind::*;
        self.modifier_list();
        match self.kind() {
            ClassKw => self.class_decl(),
            InterfaceKw => self.interface_decl(),
            EnumKw => self.enum_decl(),
            RecordKw => self.record_decl(),
            At if self.look(1) == InterfaceKw => self.annotation_decl(),
            _ => self.err_and_bump("expected type declaration"),
        }
    }

    pub(crate) fn class_decl(&mut self) {
        use JavaSyntaxKind::*;
        self.node(ClassDecl, |p| {
            p.expect(ClassKw);
            p.expect(Ident);
            if p.at(Lt) { p.type_param_list(); }
            if p.eat(ExtendsKw) { p.type_(); }
            if p.eat(ImplementsKw) { p.type_list(); }
            if p.at(PermitsKw) { p.permits_clause(); }
            p.class_body();
        });
    }

    pub(crate) fn interface_decl(&mut self) {
        use JavaSyntaxKind::*;
        self.node(InterfaceDecl, |p| {
            p.expect(InterfaceKw);
            p.expect(Ident);
            if p.at(Lt) { p.type_param_list(); }
            if p.eat(ExtendsKw) { p.type_list(); }
            if p.at(PermitsKw) { p.permits_clause(); }
            p.class_body();
        });
    }

    pub(crate) fn enum_decl(&mut self) {
        use JavaSyntaxKind::*;
        self.node(EnumDecl, |p| {
            p.expect(EnumKw);
            p.expect(Ident);
            if p.at(Lt) { p.type_param_list(); }
            if p.eat(ImplementsKw) { p.type_list(); }
            p.enum_body();
        });
    }

    pub(crate) fn record_decl(&mut self) {
        use JavaSyntaxKind::*;
        self.node(RecordDecl, |p| {
            p.expect(RecordKw);
            p.expect(Ident);
            if p.at(Lt) { p.type_param_list(); }
            p.record_component_list();
            if p.eat(ImplementsKw) { p.type_list(); }
            p.class_body();
        });
    }

    pub(crate) fn annotation_decl(&mut self) {
        use JavaSyntaxKind::*;
        self.node(AnnotationDecl, |p| {
            p.expect(At);
            p.expect(InterfaceKw);
            p.expect(Ident);
            p.class_body();
        });
    }

    pub(crate) fn permits_clause(&mut self) {
        use JavaSyntaxKind::*;
        self.node(PermitsClause, |p| {
            p.expect(PermitsKw);
            p.type_list();
        });
    }

    pub(crate) fn class_body(&mut self) {
        use JavaSyntaxKind::*;
        self.node(ClassBody, |p| {
            p.expect(LBrace);
            while !p.at(RBrace) && p.kind() != Error { p.class_member(); }
            p.expect(RBrace);
        });
    }

    pub(crate) fn enum_body(&mut self) {
        use JavaSyntaxKind::*;
        self.node(EnumBody, |p| {
            p.expect(LBrace);
            if !p.at(RBrace) { p.enum_constant_list(); }
            if p.eat(Semi) {
                while !p.at(RBrace) && p.kind() != Error { p.class_member(); }
            }
            p.expect(RBrace);
        });
    }

    pub(crate) fn enum_constant_list(&mut self) {
        use JavaSyntaxKind::*;
        self.node(EnumConstantList, |p| {
            loop {
                p.enum_constant();
                if !p.eat(Comma) { break; }
            }
        });
    }

    pub(crate) fn enum_constant(&mut self) {
        use JavaSyntaxKind::*;
        self.node(EnumConstant, |p| {
            p.modifier_list();
            p.expect(Ident);
            if p.at(LParen) { p.argument_list(); }
            if p.at(LBrace) { p.class_body(); }
        });
    }
}