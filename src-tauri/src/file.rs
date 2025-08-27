use std::{fs::File, io::{Read, Write}};

// Composition over inheritance!
// https://medium.com/comsystoreply/28-days-of-rust-part-2-composition-over-inheritance-cab1b106534a
// https://tyfkda.github.io/blog/2020/09/27/composition-over-inheritance.html
fn read(path: &str) -> std::io::Result<()> {
    let mut f = File::open(path)?;
    let mut data = vec![];
    f.read_to_end(&mut data)?;
    Ok(())

}

fn create(data: &str, filename: &str, path: &str) -> std::io::Result<()> {
    let mut f = File::create_new(format!("{}/{}", path, filename))?;
    f.write_all(data.as_bytes())?;
    Ok(())
}

fn write(data: &str, filename: &str) {
}
