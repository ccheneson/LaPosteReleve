use anyhow;
use std::{
    path::{Path, PathBuf},
    vec,
};

pub fn list_files<P: AsRef<Path>>(
    file_path: P,
    extension: Option<&str>,
) -> anyhow::Result<Vec<PathBuf>> {
    let mut csv: Vec<PathBuf> = vec![];
    for file in std::fs::read_dir(file_path)? {
        let f_path = file?.path();
        if extension.is_some() {
            match f_path.extension() {
                Some(ex) if ex == extension.unwrap() => csv.push(f_path),
                _ => continue,
            }
        } else {
            csv.push(f_path)
        }
    }
    Ok(csv)
}

#[cfg(test)]
mod tests {

    use std::path::PathBuf;
    use crate::csv::dir::list_files;

    #[test]
    fn test_existing_folder() -> anyhow::Result<()> {
        // Tests run at the project level
        let result: Vec<PathBuf> = list_files("./", Some("toml"))?;

        assert_eq!(
            result.contains(&PathBuf::from("./Cargo.toml")), // ./ because it is PathBuf
            true,
            "Cargo.toml not found!"
        );

        Ok(())
    }

    #[test]
    fn test_non_existing_folder() {
        // Tests run at the project level
        //The below path is probably invalid
        match list_files("/hello", Some("toml")) {
            Ok(_) => assert!(false, "Test failed: it looks like there is an existing /hello"),
            Err(_) => {
                assert!(true, "handle correctly non existing source of files");
            }            
        }
    }
}
