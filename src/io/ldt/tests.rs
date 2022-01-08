use super::LdtFile;
use std::path::Path;

/// Example file provided by Paul Bourne's documentation:
/// http://paulbourke.net/dataformats/ldt/
const EXAMPLE_LDT_FILE: &str = include_str!("example.ldt");

#[test]
fn test_parse_ldt() {
    let mut ldt = LdtFile::new();
    match ldt.parse(&EXAMPLE_LDT_FILE.to_owned()) {
        Ok(_) => {

            // Check that the arrays are the correct length. 
            assert_eq!(ldt.c_angles().iter().count(), ldt.n_cplanes());
            assert_eq!(ldt.g_angles().iter().count(), ldt.n_luminous_intensities_per_cplane());
            assert_eq!(ldt.intensities().iter().count(), (ldt.mc2() - ldt.mc1() + 1) * ldt.n_luminous_intensities_per_cplane())
        },
        Err(e) => assert!(false, "LDT parse error: {}", e),
    }
}

#[test]
fn test_parse_ldt_file() {
    match LdtFile::parse_file(Path::new("./src/io/ldt/example.ldt")) {
        Ok(ldt) => {

            // Check that the arrays are the correct length. 
            assert_eq!(ldt.c_angles().iter().count(), ldt.n_cplanes());
            assert_eq!(ldt.g_angles().iter().count(), ldt.n_luminous_intensities_per_cplane());
            assert_eq!(ldt.intensities().iter().count(), (ldt.mc2() - ldt.mc1() + 1) * ldt.n_luminous_intensities_per_cplane())
        },
        Err(e) => assert!(false, "LDT file parse error: {}", e),
    }
}