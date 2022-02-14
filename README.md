# `lidrs` - Luminous Intensity Distributions in Rust
A rust crate for reading and representing light intensity distributions via photometric webs. 
This crate can read and write:
- Illuminating Engineering Society (LM-63) standard (`.ies`) files
  - 1986
  - IESNA91
  - IESNA:LM-63-1995
  - IESNA:LM-63-2002
- EULUMDAT (`.ldt` / `.eul`) files

## Caveats
Note that currently, although all standards should be readable using the `lidrs::io::ies` module, I have only implemented conversions to the `PhotometricWeb` struct for **type C** photometry. If you would like to implement this, please submit a PR. 
