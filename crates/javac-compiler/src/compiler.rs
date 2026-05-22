use crate::config::CompilerConfig;
use javac_ast::JavaSyntaxNode;
use std::path::{Path, PathBuf};

pub struct Compiler {
    config: CompilerConfig,
}

impl Compiler {
    pub fn new(config: CompilerConfig) -> Self {
        Self { config }
    }

    pub fn compile(self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        for source_file in &self.config.source_files {
            match std::fs::read_to_string(source_file) {
                Ok(source) => {
                    if let Err(e) = self.compile_source(&source, source_file) {
                        errors.extend(e);
                    }
                }
                Err(e) => {
                    errors.push(format!("failed to read {}: {}", source_file, e));
                }
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn compile_source(&self, source: &str, filename: &str) -> Result<(), Vec<String>> {
        let parse = javac_parser::Parser::parse(source);
        if !parse.errors.is_empty() {
            return Err(parse
                .errors
                .iter()
                .map(|e| format!("{}: {}", filename, e.message))
                .collect());
        }

        let root = JavaSyntaxNode::new_root(parse.green_node);
        let unit = javac_hir::lowering::lower(&root).ok_or_else(|| {
            vec![format!(
                "{}: unsupported source shape; expected one empty top-level class",
                filename
            )]
        })?;
        let class_name = unit
            .type_decls
            .first()
            .map(|decl| decl.name)
            .ok_or_else(|| vec![format!("{}: no class declaration found", filename)])?;
        let bytes = javac_bytecode::class_gen::gen_class(&unit)
            .map_err(|e| vec![format!("{}: {}", filename, e)])?;

        let class_path = class_file_path(&self.config.output_dir, class_name.as_str());
        if let Some(parent) = class_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| vec![format!("failed to create {}: {}", parent.display(), e)])?;
        }
        std::fs::write(&class_path, bytes)
            .map_err(|e| vec![format!("failed to write {}: {}", class_path.display(), e)])?;
        Ok(())
    }
}

fn class_file_path(output_dir: &str, class_name: &str) -> PathBuf {
    let mut path = Path::new(output_dir).to_path_buf();
    for segment in class_name.split('/') {
        path.push(segment);
    }
    path.set_extension("class");
    path
}
