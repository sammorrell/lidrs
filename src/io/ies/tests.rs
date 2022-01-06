use super::{IESFile, LuminousOpeningUnits};

const IES_FILE: &str = "IESNA91
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
    match IESFile::parse(&IES_FILE.to_owned()) {
        Err(e) => assert!(false, "Parse error: {}", e),
        Ok(ies) => {
            // Now check that all of the values have made it in from the file.
            assert_eq!(ies.test_number(), "Simple demo intensity distribution");
            assert_eq!(ies.manufacturer(), "Lightscape Technologies, Inc.");
            assert_eq!(ies.rated_lumens(), None);
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
        }
    }
}
