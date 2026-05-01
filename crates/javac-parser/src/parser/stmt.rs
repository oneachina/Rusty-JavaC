use crate::parser::{Parser, JavaSyntaxKind};

impl Parser {
    pub(crate) fn block(&mut self) {
        use JavaSyntaxKind::*;
        self.node(Block, |p| {
            p.expect(LBrace);
            while !p.at(RBrace) && p.kind() != Error { p.stmt(); }
            p.expect(RBrace);
        });
    }

    pub(crate) fn stmt(&mut self) {
        use JavaSyntaxKind::*;
        match self.kind() {
            LBrace => self.block(),
            IfKw => self.if_stmt(),
            ForKw => self.for_stmt(),
            WhileKw => self.while_stmt(),
            DoKw => self.do_stmt(),
            SwitchKw => self.switch_expr(),
            TryKw => self.try_stmt(),
            ReturnKw => self.return_stmt(),
            ThrowKw => self.throw_stmt(),
            BreakKw => self.break_stmt(),
            ContinueKw => self.continue_stmt(),
            SynchronizedKw => self.synchronized_stmt(),
            AssertKw => self.assert_stmt(),
            YieldKw => self.yield_stmt(),
            Semi => { self.node(EmptyStmt, |p| { p.bump(); }); }
            _ => self.expr_or_local_decl(),
        }
    }

    pub(crate) fn if_stmt(&mut self) {
        use JavaSyntaxKind::*;
        self.node(IfStmt, |p| {
            p.expect(IfKw);
            p.expect(LParen); p.expr(); p.expect(RParen);
            p.stmt();
            if p.eat(ElseKw) { p.stmt(); }
        });
    }

    pub(crate) fn for_stmt(&mut self) {
        use JavaSyntaxKind::*;
        self.node(ForStmt, |p| {
            p.expect(ForKw);
            p.expect(LParen);
            if p.is_foreach() {
                p.for_each();
            } else {
                p.for_init();
                p.expect(Semi);
                if !p.at(Semi) { p.expr(); }
                p.expect(Semi);
                if !p.at(RParen) { p.expr_list(); }
                p.expect(RParen);
                p.stmt();
            }
        });
    }

    pub(crate) fn is_foreach(&mut self) -> bool {
        let mut i = self.pos;
        let mut depth = 1i32;
        while i < self.tokens.len() {
            match self.tokens[i].kind {
                JavaSyntaxKind::LParen => depth += 1,
                JavaSyntaxKind::RParen => { depth -= 1; if depth == 0 { return false; } }
                JavaSyntaxKind::Colon if depth == 1 => return true,
                _ => {}
            }
            i += 1;
        }
        false
    }

    pub(crate) fn for_each(&mut self) {
        use JavaSyntaxKind::*;
        self.node(ForEach, |p| {
            p.modifier_list();
            p.type_();
            p.expect(Ident);
            p.expect(Colon);
            p.expr();
            p.expect(RParen);
            p.stmt();
        });
    }

    pub(crate) fn for_init(&mut self) {
        use JavaSyntaxKind::*;
        self.node(ForInit, |p| {
            if p.at(Semi) { return; }
            if p.is_local_var_decl() {
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
            } else {
                p.expr();
                if p.eat(Comma) { p.expr_list(); }
            }
        });
    }

    pub(crate) fn while_stmt(&mut self) {
        use JavaSyntaxKind::*;
        self.node(WhileStmt, |p| {
            p.expect(WhileKw);
            p.expect(LParen); p.expr(); p.expect(RParen);
            p.stmt();
        });
    }

    pub(crate) fn do_stmt(&mut self) {
        use JavaSyntaxKind::*;
        self.node(DoStmt, |p| {
            p.expect(DoKw);
            p.stmt();
            p.expect(WhileKw);
            p.expect(LParen); p.expr(); p.expect(RParen);
            p.expect(Semi);
        });
    }

    pub(crate) fn switch_expr(&mut self) {
        use JavaSyntaxKind::*;
        self.node(SwitchStmt, |p| {
            p.expect(SwitchKw);
            p.expect(LParen); p.expr(); p.expect(RParen);
            p.node(SwitchBlock, |p| {
                p.expect(LBrace);
                while !p.at(RBrace) && p.kind() != Error {
                    p.switch_label();
                }
                p.expect(RBrace);
            });
        });
    }

    pub(crate) fn switch_label(&mut self) {
        use JavaSyntaxKind::*;
        self.node(SwitchLabel, |p| {
            if p.eat(CaseKw) {
                p.expr();
                if p.eat(Colon) {
                    while !p.at_any(&[CaseKw, DefaultKw, RBrace]) && p.kind() != Error {
                        p.stmt();
                    }
                } else {
                    p.expect(Arrow);
                    p.node(SwitchRule, |p| {
                        if p.at(ThrowKw) { p.stmt(); }
                        else if p.at(LBrace) { p.block(); }
                        else { p.expr(); p.eat(Semi); }
                    });
                }
            } else {
                p.expect(DefaultKw);
                if p.eat(Colon) {
                    while !p.at_any(&[CaseKw, RBrace]) && p.kind() != Error {
                        p.stmt();
                    }
                } else {
                    p.expect(Arrow);
                    p.node(SwitchRule, |p| {
                        if p.at(ThrowKw) { p.stmt(); }
                        else if p.at(LBrace) { p.block(); }
                        else { p.expr(); p.eat(Semi); }
                    });
                }
            }
        });
    }

    pub(crate) fn try_stmt(&mut self) {
        use JavaSyntaxKind::*;
        self.node(TryStmt, |p| {
            p.expect(TryKw);
            if p.at(LParen) {
                p.node(TryWithResources, |p| {
                    p.expect(LParen);
                    while !p.at(RParen) && p.kind() != Error {
                        p.node(Resource, |p| {
                            p.modifier_list();
                            p.type_();
                            p.expect(Ident);
                            if p.eat(Eq) { p.expr(); }
                        });
                        if !p.eat(Semi) { break; }
                    }
                    p.expect(RParen);
                });
            }
            p.block();
            while p.at(CatchKw) { p.catch_clause(); }
            if p.at(FinallyKw) {
                p.node(FinallyClause, |p| { p.expect(FinallyKw); p.block(); });
            }
        });
    }

    pub(crate) fn catch_clause(&mut self) {
        use JavaSyntaxKind::*;
        self.node(CatchClause, |p| {
            p.expect(CatchKw);
            p.expect(LParen);
            p.modifier_list();
            p.type_();
            p.expect(Ident);
            p.expect(RParen);
            p.block();
        });
    }

    pub(crate) fn return_stmt(&mut self) {
        use JavaSyntaxKind::*;
        self.node(ReturnStmt, |p| {
            p.expect(ReturnKw);
            if !p.at(Semi) { p.expr(); }
            p.expect(Semi);
        });
    }

    pub(crate) fn throw_stmt(&mut self) {
        use JavaSyntaxKind::*;
        self.node(ThrowStmt, |p| {
            p.expect(ThrowKw);
            p.expr();
            p.expect(Semi);
        });
    }

    pub(crate) fn break_stmt(&mut self) {
        use JavaSyntaxKind::*;
        self.node(BreakStmt, |p| {
            p.expect(BreakKw);
            if p.at(Ident) { p.bump(); }
            p.expect(Semi);
        });
    }

    pub(crate) fn continue_stmt(&mut self) {
        use JavaSyntaxKind::*;
        self.node(ContinueStmt, |p| {
            p.expect(ContinueKw);
            if p.at(Ident) { p.bump(); }
            p.expect(Semi);
        });
    }

    pub(crate) fn synchronized_stmt(&mut self) {
        use JavaSyntaxKind::*;
        self.node(SynchronizedStmt, |p| {
            p.expect(SynchronizedKw);
            p.expect(LParen); p.expr(); p.expect(RParen);
            p.block();
        });
    }

    pub(crate) fn assert_stmt(&mut self) {
        use JavaSyntaxKind::*;
        self.node(AssertStmt, |p| {
            p.expect(AssertKw);
            p.expr();
            if p.eat(Colon) { p.expr(); }
            p.expect(Semi);
        });
    }

    pub(crate) fn yield_stmt(&mut self) {
        use JavaSyntaxKind::*;
        self.node(YieldStmt, |p| {
            p.expect(YieldKw);
            p.expr();
            p.expect(Semi);
        });
    }

    pub(crate) fn is_local_var_decl(&self) -> bool {
        use JavaSyntaxKind::*;
        let primitives = [IntKw, LongKw, ShortKw, ByteKw, CharKw,
            FloatKw, DoubleKw, BooleanKw, VoidKw, VarKw];
        if self.at_any(&primitives) { return true; }
        if !self.at(Ident) { return false; }
        let mut i = self.pos;
        while i < self.tokens.len() && self.tokens[i].kind == Ident {
            i += 1;
            if i < self.tokens.len() && self.tokens[i].kind == Lt {
                let mut depth = 0;
                while i < self.tokens.len() {
                    match self.tokens[i].kind {
                        Lt => depth += 1,
                        Gt => { depth -= 1; if depth == 0 { i += 1; break; } }
                        _ => {}
                    }
                    i += 1;
                }
            }
            if i < self.tokens.len() && self.tokens[i].kind == Dot { i += 1; } else { break; }
        }
        while i + 1 < self.tokens.len() && self.tokens[i].kind == LBrack && self.tokens[i + 1].kind == RBrack {
            i += 2;
        }
        i < self.tokens.len() && self.tokens[i].kind == Ident
    }

    pub(crate) fn local_var_decl(&mut self) {
        use JavaSyntaxKind::*;
        self.node(LocalVarDecl, |p| {
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

    pub(crate) fn expr_or_local_decl(&mut self) {
        use JavaSyntaxKind::*;
        if self.is_local_var_decl() {
            self.local_var_decl();
        } else {
            self.node(ExprStmt, |p| {
                p.expr();
                p.expect(Semi);
            });
        }
    }
}