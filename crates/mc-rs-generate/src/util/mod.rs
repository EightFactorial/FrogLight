#![allow(dead_code)]

use std::{fs::File, io::Write, path::Path};

/// Create the file if it doesn't exist, otherwise return None
pub fn create_file(path: &Path) -> Result<Option<File>, std::io::Error> {
    match path.exists() {
        true => Ok(None),
        false => Ok(Some(File::create(path)?)),
    }
}

/// Create the file with the given imports if it doesn't exist
pub fn create_file_with(imports: &[String], path: &Path) -> Result<Option<File>, std::io::Error> {
    let file = create_file(path)?;

    if let Some(mut file) = file {
        for import in imports {
            writeln!(file, "use {};", import)?;
        }
        writeln!(file)?;

        Ok(Some(file))
    } else {
        Ok(file)
    }
}

/// Create a module file at the given path if it doesn't exist
pub fn create_module(path: &Path) -> Result<Option<File>, std::io::Error> {
    let mod_path = path.join("mod.rs");
    let file = create_file(&mod_path)?;

    if let Some(mut file) = file {
        add_submodules(&mut file, path)?;

        Ok(Some(file))
    } else {
        Ok(file)
    }
}

/// Create a module with the given imports if it doesn't exist
pub fn create_module_with(imports: &[String], path: &Path) -> Result<Option<File>, std::io::Error> {
    let file = create_module(path)?;

    if let Some(mut file) = file {
        for import in imports {
            writeln!(file, "use {};", import)?;
        }
        writeln!(file)?;

        Ok(Some(file))
    } else {
        Ok(file)
    }
}

/// Add all submodules in the given path to the file
pub fn add_submodules(file: &mut File, path: &Path) -> Result<(), std::io::Error> {
    for entry in path.read_dir()? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let name = path.file_name().unwrap().to_str().unwrap();
            writeln!(file, "pub mod {};", name)?;
        } else if path.is_file()
            && path.display().to_string().ends_with(".rs")
            && !path.display().to_string().contains("mod.rs")
        {
            let name = path.file_stem().unwrap().to_str().unwrap();
            writeln!(file, "pub mod {};", name)?;
        }
    }
    writeln!(file)?;

    Ok(())
}
