use std::{path::{Path, PathBuf}, vec};
use anyhow;


pub fn list_files<P: AsRef<Path>>(file_path: P, extension: Option<&str>) -> anyhow::Result<Vec<PathBuf>>  {
    let mut csv : Vec<PathBuf> = vec!();
    for file in std::fs::read_dir(file_path)? {
        let f_path = file?.path();
        if extension.is_some() {
            match f_path.extension() {
                Some(ex) if ex == extension.unwrap() => csv.push(f_path),
                _ => continue
            }
        } else {
            csv.push(f_path)
        }
    }
    Ok(csv)
}


#[test]
fn test() -> anyhow::Result<()> {

    // Tests run at the project level
    let result: Vec<PathBuf> = list_files("./", Some("toml"))?;
    
    assert_eq!(
        result.contains(&PathBuf::from("./Cargo.toml")), // ./ because it is PathBuf
        true,
        "Cargo.toml not found!"
    );

    Ok(())

}