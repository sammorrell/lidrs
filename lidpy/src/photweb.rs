use pyo3::prelude::*;
use pyo3::exceptions::PyRuntimeError;
use lidrs::{
    photweb,
};
use crate::Plane;
use std::path::Path;

#[pyclass]
pub struct PhotometricWeb {
    pub pw: photweb::PhotometricWeb,
}

#[pymethods]
impl PhotometricWeb {
    #[new]
    fn new() -> Self {
        Self {
            pw: photweb::PhotometricWeb::new(),
        }
    }

    pub fn n_planes(&self) -> usize {
        self.pw.n_planes()
    }

    pub fn is_spherically_symmetric(&self) -> bool {
        self.pw.is_spherically_symmetric()
    }

    pub fn total_intensity(&self) -> f64 {
        self.pw.total_intensity()
    }

    pub fn planes(&self) -> Vec<Plane> {
        self.pw.planes().into_iter().map(|plane| Plane::from_lidrs_plane(plane)).collect()
    }

    pub fn get_cplane_pair(&self, angle_lower_deg: f64, angle_upper_deg: f64) -> Option<(Vec<f64>, Vec<f64>)> {
        self.pw.get_cplane_pair(angle_lower_deg, angle_upper_deg)
    }

    pub fn to_ies_file(&self, filepath: String) -> PyResult<()> {
        match <lidrs::io::ies::IesFile as photweb::PhotometricWebWriter>::write(&self.pw, &Path::new(&filepath)) {
            Ok(()) => Ok(()),
            Err(e) => Err(PyRuntimeError::new_err( format!("{}", e) )),
        }
    }

    pub fn to_eulumdat_file(&self, filepath: String) -> PyResult<()> {
        match <lidrs::io::eulumdat::EulumdatFile as photweb::PhotometricWebWriter>::write(&self.pw, &Path::new(&filepath)) {
            Ok(()) => Ok(()),
            Err(e) => Err(PyRuntimeError::new_err( format!("{}", e) )),
        }
    }
}