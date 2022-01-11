use super::{IesFile, LuminousOpeningUnits};

const IESNA_1991_FILE: &str = "IESNA91
[TEST] Simple demo intensity distribution 
[MANUFAC] Lightscape Technologies, Inc.
TILT=NONE
1
-1
1
8
1
1
2
0.0 0.0 0.0
1.0 1.0 0.0
0.0 5.0 10.0 20.0 30.0 45.0 65.0 90.0
0.0
1000.0 1100.0 1300.0 1150.0 930.0 650.0 350.0 0.0
";

#[test]
fn basic_parse_test() {
    let mut ies = IesFile::new();
    match ies.parse(&IESNA_1991_FILE.to_owned()) {
        Err(e) => assert!(false, "Parse error: {}", e),
        Ok(_) => {
            // Now check that all of the values have made it in from the file.
            assert_eq!(ies.candela_multiplying_factor(), 1.0);
            assert_eq!(ies.n_horizontal_angles(), 1);
            assert_eq!(ies.n_vertical_angles(), 8);
            assert_eq!(*ies.luminous_opening_units(), LuminousOpeningUnits::Meters);
            assert_eq!(ies.luminous_opening_width(), 0.0);
            assert_eq!(ies.luminous_opening_length(), 0.0);
            assert_eq!(ies.luminous_opening_height(), 0.0);

            assert_eq!(
                ies.vertical_angles(),
                vec![0.0, 5.0, 10.0, 20.0, 30.0, 45.0, 65.0, 90.0]
            );
            assert_eq!(ies.horizontal_angles(), vec![0.0]);
            assert_eq!(
                ies.candela_values(),
                vec![1000.0, 1100.0, 1300.0, 1150.0, 930.0, 650.0, 350.0, 0.0]
            );

            // Check that the arrays have been correctly read.
            assert_eq!(
                ies.vertical_angles().iter().count(),
                ies.n_vertical_angles()
            );
            assert_eq!(
                ies.horizontal_angles().iter().count(),
                ies.n_horizontal_angles()
            );

            // Check that angles are valid.
            assert!(IesFile::vertical_angles_valid(
                &ies.vertical_angles().to_vec()
            ));
            assert!(IesFile::horizontal_angles_valid(
                &ies.horizontal_angles().to_vec()
            ));

        }
    }
}

const KEYWORDS_TEST: &str = "[TEST] ABC1234
[TESTLAB] ABC Laboratories 
[ISSUEDATE] 18-FEB-2001
[MANUFAC] Aardvark lighting Inc. 
[LUMCAT] SKYVIEW 123-XYZ-abs-400 
[LUMINAIRE] Wide beam flood to be used without tilt
[LAMPCAT] MH-400-CLEAR
[LAMP] 400 Watt Metal Halide
[BALLASTCAT] Global 16G6031-17R
[BALLAST] 400W 277V MH Magnetic
[MAINTCAT] 4
[OTHER] This luminaire is useful as an indirect flood
[MORE] and to reduce light pollution in down light applications. 
[LAMPPOSITION] 0,0
[SEARCH] POLLUTION SPORTS INDIRECT
[_NEMATYPE] 4h x 6v
[_PRICE] Make us an offer
TILT=";

#[test]
fn parse_keywords_test() {
    let mut ies = IesFile::new();
    match ies.parse_keywords(&KEYWORDS_TEST.to_owned()) {
        Ok(_) => {
            assert_eq!(ies.keywords().len(), 16);

            assert_eq!(ies.keywords().get("TEST"), Some(&String::from("ABC1234")));

            // Test more implementation.
            assert_eq!(ies.keywords().get("OTHER"), Some(&String::from("This luminaire is useful as an indirect flood and to reduce light pollution in down light applications. ")));
            assert_eq!(ies.keywords().get("MAINTCAT"), Some(&String::from("4")));

            // Test getting optional fields.
            assert_eq!(
                ies.keywords().get("_NEMATYPE"),
                Some(&String::from("4h x 6v"))
            );
            assert_eq!(
                ies.keywords().get("_PRICE"),
                Some(&String::from("Make us an offer"))
            );
        }
        Err(e) => assert!(false, "Keyword parse error: {}", e),
    }
}

const TILT_TEST: &str = "TILT=INCLUDE
1
7
0 15 30 45 60 75 90
1.0 0.95 0.94 0.90 0.88 0.87 0.94";

#[test]
fn parse_tilt_include_test() {
    let mut ies = IesFile::new();

    match ies.parse_tilt(&TILT_TEST.to_owned()) {
        Ok(_) => {}
        Err(e) => assert!(false, "Tilt parse error: {}", e),
    }
}

#[test]
fn parse_tilt_none_test() {
    const TILT_NONE: &str = "TILT=NONE\n";

    let mut ies = IesFile::new();
    match ies.parse_tilt(&TILT_NONE.to_owned()) {
        Ok(_) => {
            assert!(ies.tilt().is_none());
        }
        Err(e) => assert!(false, "Tilt parse error: {}", e),
    }
}

const VALUES_TEST: &str = "TILT=INCLUDE
1
13
0 15 30 45 60 75 90 105 120 135 150 165 180
1.0 .95 .94 .90 .88 .87 .98 .87 .88 .90 .94 .95 1.0
1 50000 1 5 3 1 1 .5 .6 0
1.0 1.0 495
0 22.5 45 67.5 90
0 45 90
100000 50000 25000 10000 5000
100000 35000 16000 8000 3000
100000 20000 10000 5000 1000";

#[test]
fn parse_properties_test() {
    let mut ies = IesFile::new();
    match ies.parse_properties(&VALUES_TEST.to_owned()) {
        Ok(_) => {
            // Check that the arrays have been correctly read.
            assert_eq!(
                ies.vertical_angles().iter().count(),
                ies.n_vertical_angles()
            );
            assert_eq!(
                ies.horizontal_angles().iter().count(),
                ies.n_horizontal_angles()
            );

            // Check that angles are valid.
            assert!(IesFile::vertical_angles_valid(
                &ies.vertical_angles().to_vec()
            ));
            assert!(IesFile::horizontal_angles_valid(
                &ies.horizontal_angles().to_vec()
            ));
        }
        Err(e) => assert!(false, "Properties parse error: {}", e),
    }
}
