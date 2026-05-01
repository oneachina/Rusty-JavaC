use crate::parser::{Parser, JavaSyntaxKind};

impl Parser {
    pub(crate) fn expr(&mut self) { self.assignment_expr(); }

    pub(crate) fn assignment_expr(&mut self) {
        use JavaSyntaxKind::*;
        self.ternary_expr();
        if self.at_any(&[Eq, PlusEq, MinusEq, StarEq, SlashEq, AmpEq, PipeEq, CaretEq, PercentEq, LtLtEq, GtGtEq, GtGtGtEq]) {
            self.bump();
            self.assignment_expr();
        }
    }

    pub(crate) fn ternary_expr(&mut self) {
        use JavaSyntaxKind::*;
        self.binary_expr(0);
        if self.eat(Question) {
            self.expr();
            self.expect(Colon);
            self.ternary_expr();
        }
    }

    pub(crate) fn binary_expr(&mut self, min_prec: usize) {
        self.unary_expr();
        loop {
            let prec = self.binop_prec();
            if prec == 0 || prec < min_prec { break; }
            self.bump();
            self.binary_expr(prec + 1);
        }
    }

    pub(crate) fn binop_prec(&self) -> usize {
        use JavaSyntaxKind::*;
        match self.kind() {
            PipePipe => 1,
            AmpAmp => 2,
            Pipe => 3,
            Caret => 4,
            Amp => 5,
            EqEq | Neq => 6,
            Lt | Gt | Le | Ge | InstanceofKw => 7,
            LtLt | GtGt | GtGtGt => 8,
            Plus | Minus => 9,
            Star | Slash | Percent => 10,
            _ => 0,
        }
    }

    pub(crate) fn unary_expr(&mut self) {
        use JavaSyntaxKind::*;
        match self.kind() {
            Plus | Minus => { self.bump(); self.unary_expr(); }
            Inc | Dec => { self.bump(); self.unary_expr(); }
            Tilde | Bang => { self.bump(); self.unary_expr(); }
            _ => { self.cast_or_postfix_expr(); }
        }
    }

    pub(crate) fn cast_or_postfix_expr(&mut self) {
        use JavaSyntaxKind::*;
        if self.at(LParen) {
            if self.is_cast() {
                self.node(CastExpr, |p| {
                    p.expect(LParen);
                    p.type_();
                    p.expect(RParen);
                    p.unary_expr();
                });
                self.postfix_suffix();
                return;
            }
        }
        self.primary_expr();
        self.postfix_suffix();
    }

    pub(crate) fn is_cast(&mut self) -> bool {
        use JavaSyntaxKind::*;
        if !self.at(LParen) { return false; }
        let mut i = self.pos + 1;
        let primitives = [IntKw, LongKw, ShortKw, ByteKw, CharKw, FloatKw, DoubleKw, BooleanKw];
        if i < self.tokens.len() && primitives.contains(&self.tokens[i].kind) {
            i += 1;
            while i + 1 < self.tokens.len() && self.tokens[i].kind == LBrack && self.tokens[i + 1].kind == RBrack {
                i += 2;
            }
            return i < self.tokens.len() && self.tokens[i].kind == RParen;
        }
        if i < self.tokens.len() && self.tokens[i].kind == Ident {
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
            if i < self.tokens.len() && self.tokens[i].kind == RParen {
                return true;
            }
        }
        false
    }

    pub(crate) fn postfix_suffix(&mut self) {
        use JavaSyntaxKind::*;
        loop {
            match self.kind() {
                Dot => {
                    self.bump();
                    if self.at(NewKw) {
                        self.new_expr();
                    } else {
                        self.expect(Ident);
                    }
                }
                LBrack => {
                    self.bump();
                    self.expr();
                    self.expect(RBrack);
                }
                LParen => { self.argument_list(); }
                Inc | Dec => { self.bump(); break; }
                _ => break,
            }
        }
    }

    pub(crate) fn primary_expr(&mut self) {
        use JavaSyntaxKind::*;
        match self.kind() {
            IntLiteral | LongLiteral | FloatLiteral | DoubleLiteral
            | CharLiteral | StringLiteral | TextBlockLiteral
            | TrueKw | FalseKw | NullKw => {
                self.node(Literal, |p| { p.bump(); });
            }
            ThisKw => { self.node(ThisExpr, |p| { p.bump(); }); }
            SuperKw => { self.node(SuperExpr, |p| { p.bump(); }); }
            NewKw => { self.new_expr(); }
            SwitchKw => { self.switch_expr(); }
            LParen => {
                self.node(ParenExpr, |p| {
                    p.bump();
                    p.expr();
                    p.expect(RParen);
                });
            }
            Ident => { self.name_expr(); }
            _ => { self.err_and_bump(format!("unexpected token in expression: {:?}", self.kind())); }
        }
    }

    pub(crate) fn name_expr(&mut self) {
        use JavaSyntaxKind::*;
        self.node(Name, |p| {
            p.expect(Ident);
            while p.eat(Dot) {
                p.node(MemberSelect, |p| { p.expect(Ident); });
            }
        });
    }

    pub(crate) fn new_expr(&mut self) {
        use JavaSyntaxKind::*;
        self.node(NewExpr, |p| {
            p.expect(NewKw);
            p.type_no_array();
            if p.at(LBrack) {
                p.bump();
                if !p.at(RBrack) { p.expr(); }
                p.expect(RBrack);
                while p.eat(LBrack) {
                    if !p.at(RBrack) { p.expr(); }
                    p.expect(RBrack);
                }
                if p.at(LBrace) { p.array_init(); }
            } else {
                p.argument_list();
                if p.at(LBrace) { p.class_body(); }
            }
        });
    }

    pub(crate) fn array_init(&mut self) {
        use JavaSyntaxKind::*;
        self.node(ArrayInit, |p| {
            p.expect(LBrace);
            while !p.at(RBrace) && p.kind() != Error {
                p.expr();
                if !p.eat(Comma) { break; }
            }
            p.eat(Comma);
            p.expect(RBrace);
        });
    }

    pub(crate) fn argument_list(&mut self) {
        use JavaSyntaxKind::*;
        self.expect(LParen);
        if !self.at(RParen) {
            self.expr();
            while self.eat(Comma) { self.expr(); }
        }
        self.expect(RParen);
    }

    pub(crate) fn expr_list(&mut self) {
        self.expr();
        while self.eat(JavaSyntaxKind::Comma) { self.expr(); }
    }
}