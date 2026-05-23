use crate::platform;
use crate::{FieldRef, MethodRef, calls};
use javac_ty::{MethodSig, Ty};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct ClassCatalog {
    classes: HashSet<String>,
    packages: HashSet<String>,
    simple_names: HashMap<String, SimpleName>,
    fields: HashMap<(String, String), FieldRef>,
    methods: HashMap<(String, String), Vec<MethodRef>>,
    interfaces: HashSet<String>,
}

#[derive(Debug, Clone)]
enum SimpleName {
    Unique(String),
    Ambiguous,
}

impl ClassCatalog {
    pub fn platform() -> Self {
        let mut catalog = Self::default();
        for class_name in platform::classes() {
            catalog.insert_internal_class(class_name);
        }
        catalog
    }

    pub fn insert_internal_class(&mut self, internal_name: impl AsRef<str>) {
        let Some(internal_name) = normalize_internal_name(internal_name.as_ref()) else {
            return;
        };
        if !self.classes.insert(internal_name.clone()) {
            return;
        }

        if let Some((package, simple_name)) = internal_name.rsplit_once('/') {
            self.packages.insert(package.to_string());
            self.insert_simple_name(simple_name, &internal_name);
        } else {
            self.insert_simple_name(&internal_name, &internal_name);
        }
    }

    pub fn mark_interface(&mut self, internal_name: impl AsRef<str>) {
        if let Some(internal_name) = normalize_internal_name(internal_name.as_ref()) {
            self.interfaces.insert(internal_name);
        }
    }

    pub fn insert_field(
        &mut self,
        owner: impl AsRef<str>,
        name: impl AsRef<str>,
        descriptor: impl AsRef<str>,
        ty: Ty,
        access_flags: u16,
    ) {
        let owner = owner.as_ref().to_string();
        let name = name.as_ref().to_string();
        let descriptor = descriptor.as_ref().to_string();
        self.fields.insert(
            (owner.clone(), name.clone()),
            FieldRef {
                owner,
                name,
                descriptor,
                ty,
                access_flags,
            },
        );
    }

    pub fn insert_method(
        &mut self,
        owner: impl AsRef<str>,
        sig: MethodSig,
        access_flags: u16,
        is_interface: bool,
    ) {
        let owner = owner.as_ref().to_string();
        let descriptor = sig.descriptor();
        let opcode = if is_interface {
            rust_asm::opcodes::INVOKEINTERFACE
        } else {
            rust_asm::opcodes::INVOKEVIRTUAL
        };
        let method = MethodRef {
            owner: owner.clone(),
            name: sig.name.to_string(),
            descriptor,
            return_ty: sig.return_type,
            params: sig.params,
            opcode,
            is_interface,
            access_flags,
        };
        self.methods
            .entry((owner, method.name.clone()))
            .or_default()
            .push(method);
    }

    pub fn contains_internal_class(&self, internal_name: &str) -> bool {
        self.classes.contains(internal_name)
    }

    pub fn contains_package(&self, package: &str) -> bool {
        self.packages.contains(package)
    }

    pub fn resolve_import(&self, path: &str, is_wildcard: bool) -> bool {
        let internal_name = path.replace('.', "/");
        if is_wildcard {
            self.contains_package(&internal_name)
        } else {
            self.contains_internal_class(&internal_name)
        }
    }

    pub fn resolve_qualified_name(&self, name: &str) -> Option<&str> {
        let internal_name = name.replace('.', "/");
        self.classes.get(internal_name.as_str()).map(String::as_str)
    }

    pub fn resolve_java_lang(&self, simple_name: &str) -> Option<&str> {
        let internal_name = format!("java/lang/{simple_name}");
        self.classes.get(internal_name.as_str()).map(String::as_str)
    }

    pub fn resolve_simple_name(&self, simple_name: &str) -> Option<&str> {
        match self.simple_names.get(simple_name)? {
            SimpleName::Unique(internal_name) => Some(internal_name.as_str()),
            SimpleName::Ambiguous => None,
        }
    }

    pub fn resolve_static_field(&self, owner: &str, name: &str) -> Option<FieldRef> {
        calls::resolve_static_field(owner, name).or_else(|| {
            self.fields
                .get(&(owner.to_string(), name.to_string()))
                .cloned()
        })
    }

    pub fn resolve_instance_method(
        &self,
        receiver: &Ty,
        name: &str,
        args: &[Ty],
    ) -> Option<MethodRef> {
        if let Some(method) = calls::resolve_instance_method(receiver, name, args) {
            return Some(method);
        }

        let owner = match receiver.erasure() {
            Ty::Class(owner) => owner.to_string(),
            Ty::Array(_) => Ty::object().internal_name(),
            _ => return None,
        };
        let methods = self.methods.get(&(owner, name.to_string()))?;
        methods
            .iter()
            .find(|method| params_match(&method.params, args))
            .cloned()
    }

    fn insert_simple_name(&mut self, simple_name: &str, internal_name: &str) {
        match self.simple_names.get(simple_name) {
            Some(SimpleName::Unique(existing)) if existing == internal_name => {}
            Some(_) => {
                self.simple_names
                    .insert(simple_name.to_string(), SimpleName::Ambiguous);
            }
            None => {
                self.simple_names.insert(
                    simple_name.to_string(),
                    SimpleName::Unique(internal_name.to_string()),
                );
            }
        }
    }
}

impl Default for ClassCatalog {
    fn default() -> Self {
        Self {
            classes: HashSet::new(),
            packages: HashSet::new(),
            simple_names: HashMap::new(),
            fields: HashMap::new(),
            methods: HashMap::new(),
            interfaces: HashSet::new(),
        }
    }
}

fn params_match(expected: &[Ty], actual: &[Ty]) -> bool {
    expected.len() == actual.len()
        && expected
            .iter()
            .zip(actual)
            .all(|(expected, actual)| expected.erasure() == actual.erasure())
}

fn normalize_internal_name(name: &str) -> Option<String> {
    let name = name.trim().trim_end_matches(".class");
    if name.is_empty() || name.starts_with('[') || name == "module-info" {
        return None;
    }
    Some(name.replace('.', "/"))
}
