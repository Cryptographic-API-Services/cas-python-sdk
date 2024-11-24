use cas_lib::password_hashers::{argon2::CASArgon, cas_password_hasher::CASPasswordHasher};
use pyo3::prelude::*;

#[pyfunction]
fn hash(password_to_hash: String) -> String {
    let result = CASArgon::hash_password(password_to_hash);
    result
}

#[pymodule]
pub fn argon2(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hash, m)?)?;
    Ok(())
}