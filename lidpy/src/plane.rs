use pyo3::prelude::*;
use lidrs::photweb;

#[pyclass]
pub struct Plane {
    pl: photweb::Plane,
}

#[pymethods]
impl Plane {

    #[new]
    fn new() -> Self {
        Self {
            pl: photweb::Plane::new(),
        }
    }

    pub fn set_angle_degrees(&mut self, ang_deg: f64) {
        self.pl.set_angle_degrees(ang_deg)
    }

    pub fn set_angles_degrees(&mut self, ang_deg: Vec<f64>) {
        self.pl.set_angles_degrees(&ang_deg)
    }

    pub fn n_samples(&self) -> usize {
        self.pl.n_samples()
    }

    pub fn angle_deg(&self) -> f64 {
        self.pl.angle_deg()
    }

    pub fn angles_deg(&self) -> Vec<f64> {
        self.pl.angles_deg()
    }

    pub fn delta_angle(&self, i: usize) -> f64 {
        self.pl.delta_angle(i)
    }

    pub fn integrate_intensity(&self) -> f64 {
        self.pl.integrate_intensity()
    }
}