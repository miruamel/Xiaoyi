//! @module knowledge::graph::repo::scanner
//! @brief Repository scanner that builds an AST graph from source files
//! @group Knowledge
//! @since 0.1.0
//! @author Miruamel
//! @see crate::knowledge::graph::repo

use std::path::{Path, PathBuf};
use std::fs::{read_dir, read_to_string};

use regex::Regex;

use crate::xiaoyi::core::error::Result;

use crate::xiaoyi::knowledge::graph::ast_graph::{AstGraph, AstNode, AstNodeKind, AstEdge, AstEdgeKind};

/// Scans a repository and builds an abstract syntax tree graph.
#[derive(Debug, Clone)]
pub struct RepoScanner {
    /// Root directory to scan.
    root: PathBuf,
    /// File extensions to include (e.g., ["rs", "py"]).
    include_ext: Vec<String>,
}

impl RepoScanner {
    /// Create a new repository scanner.
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            include_ext: vec!["rs".to_string()],
        }
    }

    /// Add a file extension to include in scanning.
    pub fn with_ext(mut self, ext: &str) -> Self {
        self.include_ext.push(ext.to_string());
        self
    }

    /// Scan the repository and return an AST graph.
    pub fn scan(&self) -> Result<AstGraph> {
        let mut graph = AstGraph::new();
        let mut id_counter = 1u64;

        // Compile regex patterns once.
        let fn_pattern = Regex::new(r"\b(?:pub\s+|async\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)\")?;
        let struct_pattern = Regex::new(r"\b(?:pub\s+)?struct\s+([A-Za-z_][A-Za-z0-9_]*)\")?;
        let enum_pattern = Regex::new(r"\b(?:pub\s+)?enum\s+([A-Za-z_][A-Za-z0-9_]*)\")?;
        let trait_pattern = Regex::new(r"\b(?:pub\s+)?trait\s+([A-Za-z_][A-Za-z0-9_]*)\")?;
        let impl_pattern = Regex::new(r"\b(?:pub\s+)?impl\s+([A-Za-z_][A-Za-z0-9_]*)\")?;
        let const_pattern = Regex::new(r"\b(?:pub\s+)?const\s+([A-Za-z_][A-Za-z0-9_]*)\")?;
        let use_pattern = Regex::new(r"\buse\s+([A-Za-z_][A-Za-z0-9_]*(?:\.[A-Za-z_][A-Za-z0-9_]*)*)\")?;

        // Walk the directory recursively.
        self.walk_dir(&self.root, &mut graph, &mut id_counter, 
                      Some(&fn_pattern), Some(&struct_pattern),
                      Some(&enum_pattern), Some(&trait_pattern),
                      Some(&impl_pattern), Some(&const_pattern),
                      Some(&use_pattern))?;

        Ok(graph)
    }

    /// Recursively walk the directory and build the AST graph.
    fn walk_dir(
        &self,
        dir: &Path,
        graph: &mut AstGraph,
        id_counter: &mut u64,
        fn_pattern: Option<&Regex>,
        struct_pattern: Option<&Regex>,
        enum_pattern: Option<&Regex>,
        trait_pattern: Option<&Regex>,
        impl_pattern: Option<&Regex>,
        const_pattern: Option<&Regex>,
        use_pattern: Option<&Regex>,
    ) -> Result<()> {
        let entries = read_dir(dir)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            let file_name = entry.file_name();

            // Skip hidden files/directories, target, and .git.
            if file_name.to_string_lossy().starts_with('.') ||
               file_name == "target" ||
               file_name == ".git" {
                continue;
            }

            if path.is_dir() {
                // Recurse into subdirectory.
                self.walk_dir(
                    &path,
                    graph,
                    id_counter,
                    fn_pattern,
                    struct_pattern,
                    enum_pattern,
                    trait_pattern,
                    impl_pattern,
                    const_pattern,
                    use_pattern,
                )?;
            } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                // Check if extension is included.
                if self.include_ext.iter().any(|inc| inc == ext) {
                    // Process the source file.
                    self.process_file(
                        &path,
                        graph,
                        id_counter,
                        fn_pattern,
                        struct_pattern,
                        enum_pattern,
                        trait_pattern,
                        impl_pattern,
                        const_pattern,
                        use_pattern,
                    )?;
                }
            }
        }
        Ok(())
    }

    /// Process a single source file and add its nodes to the graph.
    fn process_file(
        &self,
        path: &Path,
        graph: &mut AstGraph,
        id_counter: &mut u64,
        fn_pattern: Option<&Regex>,
        struct_pattern: Option<&Regex>,
        enum_pattern: Option<&Regex>,
        trait_pattern: Option<&Regex>,
        impl_pattern: Option<&Regex>,
        const_pattern: Option<&Regex>,
        use_pattern: Option<&Regex>,
    ) -> Result<()> {
        // Get relative path for label.
        let rel_path = path.strip_prefix(&self.root)
            .unwrap_or(path)
            .to_string_lossy()
            .to_string();

        // Create Module node.
        let module_id = *id_counter;
        *id_counter += 1;
        let module_node = AstNode::new(
            module_id,
            AstNodeKind::Module,
            rel_path.clone(),
            None,
        );
        graph.add_node(module_node);

        // Read file content.
        let content = match read_to_string(path) {
            Ok(c) => c,
            Err(_) => {
                // Skip file on read error.
                return Ok(());
            }
        };

        // Process each line for detected constructs.
        for (line_num, line) in content.lines().enumerate() {
            // Detect Function.
            if let Some(caps) = fn_pattern.and_then(|pat| pat.captures(line)) {
                if let Some(name) = caps.get(1) {
                    let func_id = *id_counter;
                    *id_counter += 1;
                    let func_node = AstNode::new(
                        func_id,
                        AstNodeKind::Function,
                        name.as_str().to_string(),
                        Some((line_num + 1, 0)),
                    );
                    graph.add_node(func_node);
                    // Add Contains edge from Module to Function.
                    let contains_edge = AstEdge::new(module_id, func_id, AstEdgeKind::Contains);
                    graph.add_edge(contains_edge)?;
                }
            }

            // Detect Struct.
            if let Some(caps) = struct_pattern.and_then(|pat| pat.captures(line)) {
                if let Some(name) = caps.get(1) {
                    let struct_id = *id_counter;
                    *id_counter += 1;
                    let struct_node = AstNode::new(
                        struct_id,
                        AstNodeKind::Struct,
                        name.as_str().to_string(),
                        Some((line_num + 1, 0)),
                    );
                    graph.add_node(struct_node);
                    let contains_edge = AstEdge::new(module_id, struct_id, AstEdgeKind::Contains);
                    graph.add_edge(contains_edge)?;
                }
            }

            // Detect Enum.
            if let Some(caps) = enum_pattern.and_then(|pat| pat.captures(line)) {
                if let Some(name) = caps.get(1) {
                    let enum_id = *id_counter;
                    *id_counter += 1;
                    let enum_node = AstNode::new(
                        enum_id,
                        AstNodeKind::Enum,
                        name.as_str().to_string(),
                        Some((line_num + 1, 0)),
                    );
                    graph.add_node(enum_node);
                    let contains_edge = AstEdge::new(module_id, enum_id, AstEdgeKind::Contains);
                    graph.add_edge(contains_edge)?;
                }
            }

            // Detect Trait.
            if let Some(caps) = trait_pattern.and_then(|pat| pat.captures(line)) {
                if let Some(name) = caps.get(1) {
                    let trait_id = *id_counter;
                    *id_counter += 1;
                    let trait_node = AstNode::new(
                        trait_id,
                        AstNodeKind::Trait,
                        name.as_str().to_string(),
                        Some((line_num + 1, 0)),
                    );
                    graph.add_node(trait_node);
                    let contains_edge = AstEdge::new(module_id, trait_id, AstEdgeKind::Contains);
                    graph.add_edge(contains_edge)?;
                }
            }

            // Detect Impl.
            if let Some(caps) = impl_pattern.and_then(|pat| pat.captures(line)) {
                if let Some(name) = caps.get(1) {
                    let impl_id = *id_counter;
                    *id_counter += 1;
                    let impl_node = AstNode::new(
                        impl_id,
                        AstNodeKind::Impl,
                        name.as_str().to_string(),
                        Some((line_num + 1, 0)),
                    );
                    graph.add_node(impl_node);
                    let contains_edge = AstEdge::new(module_id, impl_id, AstEdgeKind::Contains);
                    graph.add_edge(contains_edge)?;
                }
            }

            // Detect Const.
            if let Some(caps) = const_pattern.and_then(|pat| pat.captures(line)) {
                if let Some(name) = caps.get(1) {
                    let const_id = *id_counter;
                    *id_counter += 1;
                    let const_node = AstNode::new(
                        const_id,
                        AstNodeKind::Const,
                        name.as_str().to_string(),
                        Some((line_num + 1, 0)),
                    );
                    graph.add_node(const_node);
                    let contains_edge = AstEdge::new(module_id, const_id, AstEdgeKind::Contains);
                    graph.add_edge(contains_edge)?;
                }
            }

            // Detect Use (Import).
            if let Some(caps) = use_pattern.and_then(|pat| pat.captures(line)) {
                if let Some(used) = caps.get(1) {
                    let import_id = *id_counter;
                    *id_counter += 1;
                    let import_node = AstNode::new(
                        import_id,
                        AstNodeKind::Import,
                        used.as_str().to_string(),
                        Some((line_num + 1, 0)),
                    );
                    graph.add_node(import_node);
                    let contains_edge = AstEdge::new(module_id, import_id, AstEdgeKind::Contains);
                    graph.add_edge(contains_edge)?;
                }
            }
        }

        Ok(())
    }
}