use crate::parser::{Parser, JavaSyntaxKind};

impl Parser {
    pub(crate) fn type_(&mut self) {
        use JavaSyntaxKind::*;
        self.node(Type, |p| {
            if p.at(VarKw) {
                p.bump();
            } else if p.at_any(&[IntKw, LongKw, ShortKw, ByteKw, CharKw,
                FloatKw, DoubleKw, BooleanKw, VoidKw]) {
                p.node(PrimitiveType, |p| { p.bump(); });
            } else {
                p.class_type();
            }
            while p.at(LBrack) {
                p.bump();
                p.expect(RBrack);
            }
        });
    }

    pub(crate) fn type_no_array(&mut self) {
        use JavaSyntaxKind::*;
        self.node(Type, |p| {
            if p.at(VarKw) {
                p.bump();
            } else if p.at_any(&[IntKw, LongKw, ShortKw, ByteKw, CharKw,
                FloatKw, DoubleKw, BooleanKw, VoidKw]) {
                p.node(PrimitiveType, |p| { p.bump(); });
            } else {
                p.class_type();
            }
        });
    }

    pub(crate) fn class_type(&mut self) {
        use JavaSyntaxKind::*;
        self.node(ClassType, |p| {
            p.expect(Ident);
            if p.at(Lt) { p.type_arg_list(); }
            while p.eat(Dot) {
                p.node(ClassTypeSegment, |p| {
                    p.expect(Ident);
                    if p.at(Lt) { p.type_arg_list(); }
                });
            }
        });
    }

    pub(crate) fn type_arg_list(&mut self) {
        use JavaSyntaxKind::*;
        self.node(TypeArgList, |p| {
            p.expect(Lt);
            while !p.at(Gt) && p.kind() != Error {
                if p.at(Question) {
                    p.node(WildcardType, |p| {
                        p.bump();
                        if p.eat(ExtendsKw) || p.eat(SuperKw) { p.type_(); }
                    });
                } else {
                    p.type_();
                }
                if !p.eat(Comma) { break; }
            }
            p.expect(Gt);
        });
    }

    pub(crate) fn type_param_list(&mut self) {
        use JavaSyntaxKind::*;
        self.node(TypeParamList, |p| {
            p.expect(Lt);
            while !p.at(Gt) && p.kind() != Error {
                p.node(TypeParam, |p| {
                    p.expect(Ident);
                    if p.eat(ExtendsKw) {
                        p.node(TypeBound, |p| { p.type_list(); });
                    }
                });
                if !p.eat(Comma) { break; }
            }
            p.expect(Gt);
        });
    }

    pub(crate) fn type_list(&mut self) {
        self.type_();
        while self.eat(JavaSyntaxKind::Comma) { self.type_(); }
    }
}