use pyo3::prelude::*;
use pyo3::exceptions::PyRuntimeError;
use lidrs::ops;

pub fn register_ops_module(_py: Python<'_>, parent_module: &PyModule) -> PyResult<()> {
    let ops_module = PyModule::new(_py, "ops")?;
    ops_module.add_function(wrap_pyfunction!(average_photmetric_web_intensities, ops_module)?)?;
    parent_module.add_submodule(ops_module)?;
    Ok(())
}

#[pyfunction]
pub fn average_photmetric_web_intensities(input_webs: Vec<PyRef<crate::photweb::PhotometricWeb>>) -> PyResult<crate::photweb::PhotometricWeb> {
    let webs: Vec<&lidrs::photweb::PhotometricWeb> = input_webs.iter().map(|pyweb| &pyweb.pw ).collect();
    match ops::average_photmetric_web_intensities(webs) {
        Ok(pw) => Ok(crate::PhotometricWeb { pw }),
        Err(e) => Err(PyRuntimeError::new_err( format!("{}", e) )),
    }
}