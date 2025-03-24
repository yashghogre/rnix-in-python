use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rnix::Root;
use rnix::ast::{self, AttrpathValue, HasEntry};
use rowan::ast::AstNode; // Needed to use `cast()`
use rowan::TextRange;

/// Searches the parsed Nix AST for an attribute whose key matches `attr_name`
/// and replaces its value with `new_value`.
fn replace_attr_value(code: &str, attr_name: &str, new_value: &str) -> String {
    let root = Root::parse(code);
    let syntax = root.syntax();

    // We'll collect changes as (global_start, global_end, replacement_text)
    let mut changes: Vec<(usize, usize, String)> = Vec::new();

    // Traverse the AST for attribute sets.
    for node in syntax.descendants() {
        if let Some(attr_set) = ast::AttrSet::cast(node.clone()) {
            // Iterate over attribute entries in the set.
            for entry in attr_set.entries() {
                // Convert the entry into an AttrpathValue.
                if let Some(apv) = AttrpathValue::cast(entry.syntax().clone()) {
                    // av.attrpath() returns an Option; unwrap it if present.
                    if let Some(attrpath) = apv.attrpath() {
                        let key_str = attrpath.to_string().trim().to_string();
                        if key_str == attr_name {
                            // av.value() returns an Option for the value expression.
                            if let Some(expr) = apv.value() {
                                // Get the byte range of the value using its syntax node.
                                let range: TextRange = expr.syntax().text_range();
                                let start = usize::from(range.start());
                                let end = usize::from(range.end());
                                changes.push((start, end, new_value.to_string()));
                            }
                        }
                    }
                }
            }
        }
    }

    // Sort changes in reverse order so that earlier byte offsets remain valid.
    changes.sort_by(|a, b| b.0.cmp(&a.0));

    // Apply each change to a mutable copy of the original code.
    let mut modified_code = code.to_string();
    for (start, end, replacement_text) in changes {
        if modified_code.is_char_boundary(start) && modified_code.is_char_boundary(end) {
            modified_code.replace_range(start..end, &replacement_text);
        } else {
            eprintln!("Skipping replacement for invalid boundaries: {}..{}", start, end);
        }
    }
    modified_code
}

#[pyfunction]
fn replace_value(input: &str, attr_name: &str, new_value: &str) -> PyResult<String> {
    Ok(replace_attr_value(input, attr_name, new_value))
}

/// Module initializer for the Python extension.
#[pymodule]
fn rnix_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(replace_value, m)?)?;
    Ok(())
}

