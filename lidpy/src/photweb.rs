use pyo3::prelude::*;
use lidrs::photweb;

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
}