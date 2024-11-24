use cas_lib::password_hashers::{bcrypt::CASBCrypt, cas_password_hasher::CASPasswordHasher};
use pyo3::prelude::*;

#[pyfunction]
fn hash(password_to_hash: String) -> String {
    let result = CASBCrypt::hash_password(password_to_hash);
    result
}

#[pyfunction]
fn hash_threadpool(password_to_hash: String) -> String {
    let result = CASBCrypt::hash__password_threadpool(password_to_hash);
    result
}

#[pyfunction]
fn verify_password(hashed_password: String, password_to_verify: String,) -> bool {
    let result = CASBCrypt::verify_password(hashed_password, password_to_verify);
    result
}

#[pyfunction]
fn verify_password_threadpool(hashed_password: String, password_to_verify: String,) -> bool {
    let result = CASBCrypt::verify_password_threadpool(hashed_password, password_to_verify);
    result
}

#[pymodule]
pub fn bcrypt(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hash, m)?)?;
    m.add_function(wrap_pyfunction!(hash_threadpool, m)?)?;
    m.add_function(wrap_pyfunction!(verify_password, m)?)?;
    m.add_function(wrap_pyfunction!(verify_password_threadpool, m)?)?;
    Ok(())
}