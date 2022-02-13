use super::Plane;
use std::f64::consts::{FRAC_PI_2, PI};

/// A utility function which mirrors the first quadrant of planes in a vector into the second quadrant.
/// This means that we start with 0 -> \pi / 2 filled, and we return 0 -> \pi filled.
pub fn mirror_first_quadrant(planes: &Vec<Plane>) -> Vec<Plane> {
    let mut ret_planes = planes.clone();
    ret_planes.extend(planes.iter().rev().skip(1).map(|pl| {
        let mut newpl = pl.clone();
        *newpl.mut_angle() = (FRAC_PI_2 - pl.angle()) + FRAC_PI_2;
        newpl
    }));

    ret_planes
}

/// A utility function which mirrors the first quadrant of planes in a vector into the second quadrant.
/// This means that we start with 0 -> \pi filled, and we return 0 -> 2 \pi filled.
pub fn mirror_first_hemisphere(planes: &Vec<Plane>) -> Vec<Plane> {
    let mut ret_planes = planes.clone();
    let take_planes = planes.iter().count() - 2;

    ret_planes.extend(
        planes
            .into_iter()
            .rev()
            .skip(1)
            .take(take_planes)
            .map(|pl| {
                let mut newpl = pl.clone();
                *newpl.mut_angle() = (PI - pl.angle()) + PI;
                newpl
            }),
    );

    ret_planes
}

/// A utility function which takes the hemisphere occupying the second and third quadrants (90 degree - 270 degree)
/// and mirrors this onto the opposing (270 degree -> 90 degree) hemisphere. 
pub fn mirror_second_and_third_quadrants(planes: &Vec<Plane>) -> Vec<Plane> {
    let half = planes.iter().count() / 2;

    // Assemble the first quadrant from the data we have. 
    let mut ret_planes: Vec<Plane> = planes
    .iter()
    .skip(1)
    .take(half)
    .rev()
    .map(|pl| {
        let mut new_plane = pl.clone();
        *new_plane.mut_angle() = PI - pl.angle();
        new_plane
    })
    .collect();

    // Now copy the 90 - 270 degree planes. 
    ret_planes.extend(planes.iter().map(|pl| pl.clone()));

    let tmp_planes = planes
        .iter()
        .skip(half + 1)
        .take(half - 1)
        .rev()
        .map(|pl| {
            let mut new_plane = pl.clone();
            *new_plane.mut_angle() = pl.angle() + 2.0 * (3.0 * FRAC_PI_2 - pl.angle());
            new_plane
        });
    ret_planes.extend(tmp_planes.into_iter());

    ret_planes

}