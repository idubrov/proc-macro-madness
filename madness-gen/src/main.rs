use std::io::Write;

const TYPES: usize = 300;
const FIELDS: usize = 200;

pub fn main() {
    let out = std::fs::File::create("madness-types/src/gen.rs").unwrap();
    let mut out = std::io::BufWriter::new(out);
    for t in 0..TYPES {
        writeln!(out, "#[madness]").unwrap();
        writeln!(out, "pub struct Type{} {{", t).unwrap();
        for f in 0..FIELDS {
            writeln!(out, "    #[madness_tag]").unwrap();
            writeln!(out, "    pub field{}: usize,", f).unwrap();
        }
        writeln!(out, "}}").unwrap();
    }
}
