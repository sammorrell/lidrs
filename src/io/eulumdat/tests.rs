use super::{EulumdatFile, EulumdatSymmetry};
use crate::{photweb::PhotometricWeb, util::geom::degrees_to_radians};
use approx::assert_relative_eq;
use std::path::Path;

/// Example file provided by Paul Bourne's documentation:
/// http://paulbourke.net/dataformats/ldt/
const EXAMPLE_LDT_FILE: &str = include_str!("example.ldt");

#[test]
fn test_parse_ldt() {
    let mut ldt = EulumdatFile::new();
    match ldt.parse(&EXAMPLE_LDT_FILE.to_owned()) {
        Ok(_) => {
            // Check that the arrays are the correct length.
            assert_eq!(ldt.c_angles().iter().count(), ldt.n_cplanes());
            assert_eq!(
                ldt.g_angles().iter().count(),
                ldt.n_luminous_intensities_per_cplane()
            );
            assert_eq!(
                ldt.intensities().iter().count(),
                (ldt.mc2() - ldt.mc1() + 1) * ldt.n_luminous_intensities_per_cplane()
            );
        }
        Err(e) => assert!(false, "LDT parse error: {}", e),
    }
}

#[test]
fn test_parse_ldt_file() {
    match EulumdatFile::parse_file(Path::new("./src/io/eulumdat/example.ldt")) {
        Ok(ldt) => {
            // Check that the arrays are the correct length.
            assert_eq!(ldt.c_angles().iter().count(), ldt.n_cplanes());
            assert_eq!(
                ldt.g_angles().iter().count(),
                ldt.n_luminous_intensities_per_cplane()
            );
            assert_eq!(
                ldt.intensities().iter().count(),
                (ldt.mc2() - ldt.mc1() + 1) * ldt.n_luminous_intensities_per_cplane()
            );
        }
        Err(e) => assert!(false, "LDT file parse error: {}", e),
    }
}

#[test]
fn test_ldt_into_photweb() {
    let mut ldt = EulumdatFile::new();
    match ldt.parse(&EXAMPLE_LDT_FILE.to_owned()) {
        Ok(_) => {
            // Now attempt to convert to a photometric web.
            let photweb: PhotometricWeb = ldt.clone().into();

            // Test that the parameters have made it across.
            assert_eq!(photweb.planes().iter().count() as usize, ldt.n_cplanes());
        }
        Err(e) => assert!(false, "LDT parse error: {}", e),
    }
}

/// In this test I will be testing that the reconcilliation of symmetry in the photometric web
/// is correct and behaves as we expect for symmetry around the C0-180 C-planes.
#[test]
fn test_get_planes_c0c180_symmetry() {
    let mut ldt = EulumdatFile::new();
    let mut photweb = PhotometricWeb::new();
    ldt.set_n_cplanes(18 as usize);
    ldt.set_c_angles(
        (0..190)
            .step_by(10)
            .map(|ang| ang as f64)
            .collect::<Vec<f64>>(),
    );
    ldt.set_g_angles(vec![0.0]);
    ldt.set_n_luminous_intensities_per_cplane(1 as usize);
    ldt.set_intensities(
        (0..190)
            .step_by(10)
            .map(|ang| ang as f64)
            .collect::<Vec<f64>>(),
    );
    ldt.set_symmetry(EulumdatSymmetry::C0C180Plane);

    // Perform the conversion.
    photweb = ldt.clone().into();

    // Check that we have the correct number of angles.
    assert_eq!(photweb.planes().iter().count(), 36);

    // Check that all of the planes are filled with the correct angle.
    let _ = vec![
        0.0, 10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 100.0, 110.0, 120.0, 130.0,
        140.0, 150.0, 160.0, 170.0, 180.0, 190.0, 200.0, 210.0, 220.0, 230.0, 240.0, 250.0, 260.0,
        270.0, 280.0, 290.0, 300.0, 310.0, 320.0, 330.0, 340.0, 350.0,
    ]
    .iter()
    .zip(photweb.planes())
    .map(|(test, pl)| assert_relative_eq!(pl.angle_deg(), test, epsilon = 1E-6))
    .collect::<Vec<_>>();

    // Check that the angles have ended up where we expect them to.
    assert_eq!(
        photweb
            .planes()
            .iter()
            .map(|pl| pl.intensities()[0])
            .collect::<Vec<f64>>(),
        vec![
            0.0, 10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 100.0, 110.0, 120.0, 130.0,
            140.0, 150.0, 160.0, 170.0, 180.0, 170.0, 160.0, 150.0, 140.0, 130.0, 120.0, 110.0,
            100.0, 90.0, 80.0, 70.0, 60.0, 50.0, 40.0, 30.0, 20.0, 10.0
        ]
    );
}

/// Tests the conversation to photometric web for the C90 - C270 plane case.
#[test]
fn test_get_planes_c90c270_symmetry() {
    let mut ldt = EulumdatFile::new();
    let mut photweb = PhotometricWeb::new();
    ldt.set_n_cplanes(18 as usize);
    ldt.set_c_angles(
        (90..280)
            .step_by(10)
            .map(|ang| ang as f64)
            .collect::<Vec<f64>>(),
    );
    ldt.set_g_angles(vec![0.0]);
    ldt.set_n_luminous_intensities_per_cplane(1 as usize);
    ldt.set_intensities(
        (90..280)
            .step_by(10)
            .map(|ang| ang as f64)
            .collect::<Vec<f64>>(),
    );
    ldt.set_symmetry(EulumdatSymmetry::C90C270Plane);
    // Perform the conversation.
    photweb = ldt.clone().into();

    // Check that we have the correct number of angles.
    assert_eq!(photweb.planes().iter().count(), 36);
    // Check that all of the planes are filled with the correct angle.
    let _ = vec![
        0.0, 10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 100.0, 110.0, 120.0, 130.0,
        140.0, 150.0, 160.0, 170.0, 180.0, 190.0, 200.0, 210.0, 220.0, 230.0, 240.0, 250.0, 260.0,
        270.0, 280.0, 290.0, 300.0, 310.0, 320.0, 330.0, 340.0, 350.0,
    ]
    .iter()
    .zip(photweb.planes())
    .map(|(test, pl)| assert_relative_eq!(pl.angle_deg(), test, epsilon = 1E-6))
    .collect::<Vec<_>>();

    // Check that the angles have ended up where we expect them to.
    assert_eq!(
        photweb
            .planes()
            .iter()
            .map(|pl| pl.intensities()[0])
            .collect::<Vec<f64>>(),
        vec![
            180.0, 170.0, 160.0, 150.0, 140.0, 130.0, 120.0, 110.0, 100.0, 90.0, 100.0, 110.0,
            120.0, 130.0, 140.0, 150.0, 160.0, 170.0, 180.0, 190.0, 200.0, 210.0, 220.0, 230.0,
            240.0, 250.0, 260.0, 270.0, 260.0, 250.0, 240.0, 230.0, 220.0, 210.0, 200.0, 190.0,
        ]
    );
}

/// Tests the conversation to photometric web for the C90 - C270 plane case.
#[test]
fn test_get_planes_c0c180c90c270_symmetry() {
    let mut ldt = EulumdatFile::new();
    let mut photweb = PhotometricWeb::new();
    ldt.set_n_cplanes(9 as usize);
    ldt.set_c_angles(
        (0..100)
            .step_by(10)
            .map(|ang| ang as f64)
            .collect::<Vec<f64>>(),
    );
    ldt.set_g_angles(vec![0.0]);
    ldt.set_n_luminous_intensities_per_cplane(1 as usize);
    ldt.set_intensities(
        (0..100)
            .step_by(10)
            .map(|ang| ang as f64)
            .collect::<Vec<f64>>(),
    );
    ldt.set_symmetry(EulumdatSymmetry::C0C180C90C270Plane);

    // Perform the conversion.
    photweb = ldt.clone().into();

    // Check that we have the correct number of angles.
    assert_eq!(photweb.planes().iter().count(), 36);

    let _ = vec![
        0.0, 10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 100.0, 110.0, 120.0, 130.0,
        140.0, 150.0, 160.0, 170.0, 180.0, 190.0, 200.0, 210.0, 220.0, 230.0, 240.0, 250.0, 260.0,
        270.0, 280.0, 290.0, 300.0, 310.0, 320.0, 330.0, 340.0, 350.0,
    ]
    .iter()
    .zip(photweb.planes())
    .map(|(test, pl)| assert_relative_eq!(pl.angle_deg(), test, epsilon = 1E-6))
    .collect::<Vec<_>>();

    // Check that the angles have ended up where we expect them to.
    assert_eq!(
        photweb
            .planes()
            .iter()
            .map(|pl| pl.intensities()[0])
            .collect::<Vec<f64>>(),
        vec![
            0.0, 10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 80.0, 70.0, 60.0, 50.0,
            40.0, 30.0, 20.0, 10.0, 0.0, 10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0,
            80.0, 70.0, 60.0, 50.0, 40.0, 30.0, 20.0, 10.0
        ]
    );
}

/// Check that in the case of spherical symmetry, we only end up with a single plane.
#[test]
fn test_get_planes_spherical_symmetry() {
    let mut ldt = EulumdatFile::new();
    let mut photweb = PhotometricWeb::new();
    ldt.set_n_cplanes(1 as usize);
    ldt.set_c_angles(vec![0.0]);
    ldt.set_g_angles(vec![0.0]);
    ldt.set_n_luminous_intensities_per_cplane(1 as usize);
    ldt.set_intensities(vec![1.0]);
    ldt.set_symmetry(EulumdatSymmetry::AboutVerticalAxis);

    // Perform the conversion.
    photweb = ldt.clone().into();

    // Check that we have the correct number of angles.
    assert_eq!(photweb.planes().iter().count(), 1);
    assert_eq!(
        photweb
            .planes()
            .iter()
            .map(|pl| pl.angle())
            .collect::<Vec<f64>>(),
        vec![0.0]
    );
    // Check that the angles have ended up where we expect them to.
    assert_eq!(
        photweb
            .planes()
            .iter()
            .map(|pl| pl.intensities()[0])
            .collect::<Vec<f64>>(),
        vec![1.0]
    );
}
