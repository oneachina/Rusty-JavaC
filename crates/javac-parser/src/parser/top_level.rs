use crate::parser::{Parser, JavaSyntaxKind};

impl Parser {
    pub(crate) fn compilation_unit(&mut self) {
        use JavaSyntaxKind::*;
        self.node(CompilationUnit, |p| {
            if p.at(PackageKw) { p.package_decl(); }
            while p.at(ImportKw) { p.import_decl(); }
            while p.at_any(&[ClassKw, InterfaceKw, EnumKw, RecordKw, At,
                PublicKw, ProtectedKw, PrivateKw, AbstractKw, FinalKw,
                StaticKw, StrictfpKw, SealedKw, NonSealedKw]) {
                p.type_decl();
            }
        });
    }

    pub(crate) fn package_decl(&mut self) {
        use JavaSyntaxKind::*;
        self.node(PackageDecl, |p| {
            p.expect(PackageKw);
            p.qualified_name();
            p.expect(Semi);
        });
    }

    pub(crate) fn import_decl(&mut self) {
        use JavaSyntaxKind::*;
        self.node(ImportDecl, |p| {
            p.expect(ImportKw);
            p.eat(StaticKw);
            p.qualified_name();
            if p.eat(Star) {}
            p.expect(Semi);
        });
    }

    pub(crate) fn modifier_list(&mut self) {
        use JavaSyntaxKind::*;
        self.node(ModifierList, |p| {
            let mods = [PublicKw, ProtectedKw, PrivateKw, AbstractKw, FinalKw,
                StaticKw, StrictfpKw, SealedKw, NonSealedKw, NativeKw,
                SynchronizedKw, TransientKw, VolatileKw, DefaultKw];
            loop {
                if p.at_any(&mods) { p.bump(); }
                else if p.at(At) { p.annotation(); }
                else { break; }
            }
        });
    }

    pub(crate) fn qualified_name(&mut self) {
        use JavaSyntaxKind::*;
        self.node(QualifiedName, |p| {
            p.expect(Ident);
            while p.eat(Dot) { p.expect(Ident); }
        });
    }

    pub(crate) fn annotation(&mut self) {
        use JavaSyntaxKind::*;
        self.node(Annotation, |p| {
            p.expect(At);
            p.qualified_name();
            if p.at(LParen) {
                p.argument_list();
            }
        });
    }
}