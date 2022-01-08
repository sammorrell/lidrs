use property::Property;

#[derive(Default, Debug, Property)]
pub struct CPlane {
    npts: usize,
    verticle_angles: Vec<f32>,
    values: Vec<f32>
}