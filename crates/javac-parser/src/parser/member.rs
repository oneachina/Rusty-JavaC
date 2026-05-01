use crate::parser::{Parser, JavaSyntaxKind};

impl Parser {
    pub(crate) fn record_component_list(&mut self) {
        use JavaSyntaxKind::*;
        self.node(RecordComponentList, |p| {
            p.expect(LParen);
            while !p.at(RParen) && p.kind() != Error {
                p.node(RecordComponent, |p| {
                    p.modifier_list();
                    p.type_();
                    p.expect(Ident);
                });
                if !p.eat(Comma) { break; }
            }
            p.expect(RParen);
        });
    }

    pub(crate) fn class_member(&mut self) {
        use JavaSyntaxKind::*;
        if self.at(LBrace) {
            self.node(InstanceInit, |p| { p.block(); });
            return;
        }
        if self.at(StaticKw) && self.look(1) == LBrace {
            self.node(StaticInit, |p| { p.eat(StaticKw); p.block(); });
            return;
        }

        self.modifier_list();

        if self.at_any(&[ClassKw, InterfaceKw, EnumKw, RecordKw]) ||
           (self.at(At) && self.look(1) == InterfaceKw)
        {
            self.type_decl();
            return;
        }

        if self.is_constructor() {
            self.constructor_decl();
        } else if self.is_method_decl() {
            self.method_decl();
        } else {
            self.field_decl();
        }
    }

    pub(crate) fn is_constructor(&self) -> bool {
        let i = self.pos;
        i < self.tokens.len()
            && self.tokens[i].kind == JavaSyntaxKind::Ident
            && i + 1 < self.tokens.len()
            && (self.tokens[i + 1].kind == JavaSyntaxKind::LParen
                || self.tokens[i + 1].kind == JavaSyntaxKind::LBrace)
    }

    pub(crate) fn constructor_decl(&mut self) {
        use JavaSyntaxKind::*;
        self.node(ConstructorDecl, |p| {
            p.expect(Ident);
            if p.at(LParen) { p.formal_param_list(); }
            if p.at(ThrowsKw) { p.throws_clause(); }
            p.node(MethodBody, |p| { p.block(); });
        });
    }

    pub(crate) fn is_method_decl(&mut self) -> bool {
        let mut i = self.pos;
        while i < self.tokens.len() && self.tokens[i].kind == JavaSyntaxKind::At {
            i += 1;
            if i < self.tokens.len() && self.tokens[i].kind == JavaSyntaxKind::Ident { i += 1; }
            if i < self.tokens.len() && self.tokens[i].kind == JavaSyntaxKind::LParen {
                let mut depth = 0;
                while i < self.tokens.len() {
                    match self.tokens[i].kind {
                        JavaSyntaxKind::LParen => depth += 1,
                        JavaSyntaxKind::RParen => { depth -= 1; if depth == 0 { i += 1; break; } }
                        _ => {}
                    }
                    i += 1;
                }
            }
        }
        let primitives = [JavaSyntaxKind::IntKw, JavaSyntaxKind::LongKw, JavaSyntaxKind::ShortKw,
            JavaSyntaxKind::ByteKw, JavaSyntaxKind::CharKw, JavaSyntaxKind::FloatKw,
            JavaSyntaxKind::DoubleKw, JavaSyntaxKind::BooleanKw, JavaSyntaxKind::VoidKw];
        if i < self.tokens.len() && primitives.contains(&self.tokens[i].kind) {
            i += 1;
        } else {
            while i < self.tokens.len() && self.tokens[i].kind == JavaSyntaxKind::Ident {
                i += 1;
                if i < self.tokens.len() && self.tokens[i].kind == JavaSyntaxKind::Lt {
                    let mut depth = 0;
                    while i < self.tokens.len() {
                        match self.tokens[i].kind {
                            JavaSyntaxKind::Lt => depth += 1,
                            JavaSyntaxKind::Gt => { depth -= 1; if depth == 0 { i += 1; break; } }
                            _ => {}
                        }
                        i += 1;
                    }
                }
                if i < self.tokens.len() && self.tokens[i].kind == JavaSyntaxKind::Dot { i += 1; } else { break; }
            }
        }
        while i + 1 < self.tokens.len() && self.tokens[i].kind == JavaSyntaxKind::LBrack && self.tokens[i + 1].kind == JavaSyntaxKind::RBrack {
            i += 2;
        }
        if i < self.tokens.len() && self.tokens[i].kind == JavaSyntaxKind::Ident {
            i += 1;
            while i + 1 < self.tokens.len() && self.tokens[i].kind == JavaSyntaxKind::LBrack && self.tokens[i + 1].kind == JavaSyntaxKind::RBrack {
                i += 2;
            }
        }
        i < self.tokens.len() && self.tokens[i].kind == JavaSyntaxKind::LParen
    }

    pub(crate) fn method_decl(&mut self) {
        use JavaSyntaxKind::*;
        self.node(MethodDecl, |p| {
            p.type_();
            p.expect(Ident);
            p.formal_param_list();
            if p.at(ThrowsKw) { p.throws_clause(); }
            if p.eat(Semi) {
            } else {
                p.node(MethodBody, |p| { p.block(); });
            }
        });
    }

    pub(crate) fn field_decl(&mut self) {
        use JavaSyntaxKind::*;
        self.node(FieldDecl, |p| {
            p.type_();
            p.node(VarDeclaratorList, |p| {
                loop {
                    p.node(VarDeclarator, |p| {
                        p.expect(Ident);
                        while p.eat(LBrack) { p.expect(RBrack); }
                        if p.eat(Eq) { p.expr(); }
                    });
                    if !p.eat(Comma) { break; }
                }
            });
            p.expect(Semi);
        });
    }

    pub(crate) fn formal_param_list(&mut self) {
        use JavaSyntaxKind::*;
        self.node(FormalParamList, |p| {
            p.expect(LParen);
            while !p.at(RParen) && p.kind() != Error {
                if p.at(ThisKw) {
                    p.node(ReceiverParam, |p| { p.bump(); });
                    break;
                }
                p.node(FormalParam, |p| {
                    p.modifier_list();
                    p.type_();
                    p.eat(Ellipsis);
                    p.expect(Ident);
                    while p.eat(LBrack) { p.expect(RBrack); }
                });
                if !p.eat(Comma) { break; }
            }
            p.expect(RParen);
        });
    }

    pub(crate) fn throws_clause(&mut self) {
        use JavaSyntaxKind::*;
        self.node(ThrowsClause, |p| {
            p.expect(ThrowsKw);
            p.node(ExceptionTypeList, |p| { p.type_list(); });
        });
    }
}