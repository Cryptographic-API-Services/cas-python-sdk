use cas_lib::password_hashers::{argon2::CASArgon, cas_password_hasher::CASPasswordHasher};
use pyo3::prelude::*;

#[pyfunction]
fn hash(password_to_hash: String) -> String {
    let result = CASArgon::hash_password(password_to_hash);
    result
}

#[pyfunction]
fn hash_threadpool(password_to_hash: String) -> String {
    let result = CASArgon::hash__password_threadpool(password_to_hash);
    result
}

#[pyfunction]
fn verify_password(hashed_password: String, password_to_verify: String,) -> bool {
    let result = CASArgon::verify_password(hashed_password, password_to_verify);
    result
}

#[pyfunction]
fn verify_password_threadpool(hashed_password: String, password_to_verify: String,) -> bool {
    let result = CASArgon::verify_password_threadpool(hashed_password, password_to_verify);
    result
}

#[pymodule]
pub fn argon2(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hash, m)?)?;
    m.add_function(wrap_pyfunction!(hash_threadpool, m)?)?;
    m.add_function(wrap_pyfunction!(verify_password, m)?)?;
    m.add_function(wrap_pyfunction!(verify_password_threadpool, m)?)?;
    Ok(())
}