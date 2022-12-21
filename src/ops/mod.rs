pub mod err;

use crate::{
    ops::err::Error,
    photweb::{PhotometricWeb, Plane}
};

/// This function will average the provided photmetric webs, making some assumptions about their structure:
/// - First, it assumes that they have the same number of planes, and that those planes are at the same angles.
/// - It also assumes that the intensities in the planes are the same, and that they are at the same angles.
/// 
/// This is to say that the structure of the photometric webs should all be the same. This function will not interpolate,
/// or anything like that, it will only average the intensity values in identically strcutured webs. 
/// Before performing the operation, it will check that it is possible, and return an appropriate Error is not. 
pub fn average_photmetric_web_intensities(input_webs: Vec<&PhotometricWeb>) -> Result<PhotometricWeb, Error> {
    // Check that all have the same number of planes.
    let n_planes_vec: Vec<usize> = input_webs
        .iter()
        .map(|web| web.n_planes())
        .collect();

    // Check that we have more than zero planes, else we are done. 
    if n_planes_vec.len() == 0 { return Err(Error::NoPlanes) };
    
    if n_planes_vec.iter().min() != n_planes_vec.iter().max() {
        let idx = n_planes_vec.windows(2).position(|vals| vals[0] != vals[1]).unwrap();
        return Err(Error::InconsistentNumberOfPlanes(n_planes_vec[0], n_planes_vec[idx + 1], idx + 1)) 
    };

    let n_planes = *n_planes_vec.first().unwrap();

    // Assemble angles and Check that all have the same angles in their planes. 
    let plane_angles: Vec<f64> = input_webs
        .first()
        .unwrap()
        .planes()
        .iter()
        .map(|pl| pl.angle())
        .collect();
    
    // Check that angles are consistent between
    let angles = Vec::from(input_webs.first().unwrap().planes()[0].angles());
    for web in input_webs.iter() {
        for pl in web.planes() {
            if pl.angles() != angles {
                return Err(Error::InconsistentPlaneAngles)
            }
        }
    };
    
    let n_samples = input_webs.len();
    // Checking out of the way. Now construct the planes, and the photometric web object.
    let planes = (0..n_planes).map(|i_plane| {
        let average_intensities: Vec<f64> = angles.iter().enumerate().map(|(idx, _)| {
            input_webs
            .iter()
            .map(|web| web.planes()[i_plane]
            .intensities()[idx])
            .sum::<f64>() / n_samples as f64
        }).collect();

        // Assemble the Plane.
        let mut plane = Plane::new();
        plane.set_angle(plane_angles[i_plane]);
        plane.set_angles(angles.clone());
        plane.set_intensities(average_intensities);
        plane
    }).collect();

    let mut photweb = PhotometricWeb::new();
    photweb.set_planes(planes);
    Ok(photweb)
}