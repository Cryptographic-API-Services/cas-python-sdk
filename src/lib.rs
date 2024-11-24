use std::result;

mod password_hashers;

use pyo3::{prelude::*, wrap_pymodule};

/// A Python module implemented in Rust.
#[pymodule]
fn cas_python_sdk(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(password_hashers::argon2))?;
    Ok(())
}
