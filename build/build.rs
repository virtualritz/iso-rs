use std::env;
use std::error::Error;

use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

mod codegen;
mod countries;
mod macros;
mod time;

fn main() -> Result<(), Box<dyn Error>> {
    if std::env::var("DOCS_RS").is_ok() {
        return Ok(());
    }
    let path = Path::new(&env::var_os("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    file.write(
        countries::get_countries(time::get_time()?)?
            .to_string()
            .as_bytes(),
    )?;

    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
