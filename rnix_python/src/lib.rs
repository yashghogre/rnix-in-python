use pyo3::prelude::*;
use pyo3::types::PyDict;
use rnix::{Root, SyntaxNode};

#[pyfunction]
fn parse_nix(py: Python<'_>, input: &str) -> PyResult<Py<PyDict>> {
    let ast = Root::parse(input);
    let dict = PyDict::new(py);

    // Example: Extract root node and error count
    dict.set_item("root_kind", ast.syntax().to_string())?;

    Ok(dict.into())
}

#[pymodule]
fn rnix_python(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Use the Bound<PyModule> API for PyO3 0.20
    m.add_function(wrap_pyfunction!(parse_nix, m)?)?;
    Ok(())
}
