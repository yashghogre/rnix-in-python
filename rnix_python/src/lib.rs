/*
use pyo3::prelude::*;
use pyo3::types::PyDict;
use rnix::{Root, SyntaxNode};

#[pyfunction]
fn parse_nix(py: Python<'_>, input: &str) -> PyResult<Py<PyDict>> {
    let ast = Root::parse(input);
    let dict = PyDict::new(py);

    // Example: Extract root node and error count
    dict.set_item("root_kind", ast.tree().to_string())?;

    Ok(dict.into())
}

#[pymodule]
fn rnix_python(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Use the Bound<PyModule> API for PyO3 0.20
    m.add_function(wrap_pyfunction!(parse_nix, m)?)?;
    Ok(())
}

*/

use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::{Bound, wrap_pyfunction};
use rnix::Root;
use rowan::TextRange;

/// Your replacement function: takes a Nix code string,
/// a search substring, and a replacement substring.
/// It returns a modified string with the replacements applied.
fn replace_nix_value(code: &str, search: &str, replace: &str) -> String {
    let root = Root::parse(code);
    let syntax = root.syntax();

    // Collect all changes as (global_start, global_end, new_text)
    let mut changes: Vec<(usize, usize, String)> = Vec::new();

    // Iterate over each descendant node in the AST.
    for node in syntax.descendants() {
        let node_range: TextRange = node.text_range();
        let node_start = usize::from(node_range.start());
        let node_text = node.text().to_string(); // text for this node

        // Find all occurrences of `search` within this node.
        let mut pos = 0;
        while let Some(idx) = node_text[pos..].find(search) {
            let found_index = pos + idx;
            let global_start = node_start + found_index;
            let global_end = global_start + search.len();
            // Ensure we are at valid UTF-8 boundaries in the original string.
            if code.is_char_boundary(global_start) && code.is_char_boundary(global_end) {
                changes.push((global_start, global_end, replace.to_string()));
            } else {
                eprintln!("Skipping occurrence with invalid boundaries at {}..{}", global_start, global_end);
            }
            pos = found_index + search.len();
        }
    }

    // Sort changes in reverse order (from end of the file to the beginning).
    // This ensures that earlier replacements don't affect later ones.
    changes.sort_by(|a, b| b.0.cmp(&a.0));

    // Create a mutable copy of the original code and apply the changes.
    let mut modified_code = code.to_string();
    for (start, end, new_text) in changes {
        modified_code.replace_range(start..end, &new_text);
    }
    modified_code
}


#[pyfunction]
fn replace_value(input: &str, search: &str, replacement: &str) -> PyResult<String> {
    Ok(replace_nix_value(input, search, replacement))
}

#[pymodule]
fn rnix_python(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(replace_value, m)?)?;
    Ok(())
}

