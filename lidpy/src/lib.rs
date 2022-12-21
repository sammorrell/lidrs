use pyo3::prelude::*;
use std::path::Path;

pub mod photweb;
pub mod plane;
pub mod ops;
pub use self::{photweb::*, plane::*, ops::*};

#[pyfunction]
fn read_lid_file(path_str: String) -> PyResult<PhotometricWeb> {
    let path = Path::new(&path_str);
    let lid = lidrs::photweb::PhotometricWebBuilder::from_file(path).build().unwrap();
    Ok(PhotometricWeb{
        pw: lid,
    })
}

#[pymodule]
pub fn lidpy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<photweb::PhotometricWeb>()?;
    m.add_class::<plane::Plane>()?;

    m.add_wrapped(wrap_pyfunction!(read_lid_file))?;

    // Register submodules.
    ops::register_ops_module(_py, m)?;
    
    Ok(())
}