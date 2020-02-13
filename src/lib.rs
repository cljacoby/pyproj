pub mod cli;
pub mod err;
pub mod files;

use err::PyProjErr;

use clap::{Error, ErrorKind};
// use clap::output::fmt::{Colorizer};

use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::time::{Instant, SystemTime, UNIX_EPOCH};


// TODO: Evaluate optimizing any excessive String allocations when handling
// string argument data between PyProj, PyProjBuilder, and PyProj.create()
#[derive(Debug)]
pub struct PyProj {
    name: String,
    path: Option<PathBuf>,
    src_name: String,
    with_script: bool,
    with_tests: bool,
    with_dockerfile: bool,
    script_name: String,
    tests_name: String,
    dockerfile_name: String,
}

// TODO: Reconsider use of unwrap on current_dir()
// TODO: Don't initialzie dockerfile name unless with_dockerfile is true
// TODO: Finish setter methods for properties
// TOOD: Should there be .exists() and .remove() methods? These would be easier with config file.
impl PyProj {
    pub fn new() -> Self {
        Self {
            name: String::from("pyproj"),
            path: Some(env::current_dir().unwrap()),
            src_name: String::from("src"),
            with_script: true,
            script_name: String::from("script"),
            tests_name: String::from("tests"),
            with_tests: true,
            with_dockerfile: false,
            dockerfile_name: String::from("Dockerfile"),
        }
    }

    pub fn name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn path(&mut self, path: &Path) {
        self.path = Some(path.to_path_buf());
    }

    pub fn create(&self) -> Result<(), PyProjErr> {
        if self.path.is_none() {
            return Err(PyProjErr::app_err(
                "PyProj path is None, cannot create project.",
            ));
        }

        // Create project root directory
        let mut proj_path = self.path.as_ref().unwrap().clone();
        proj_path.push(&self.name);
        if proj_path.exists() {
            return Err(PyProjErr::app_err("Path alerady exists"));
        }
        fs::create_dir(proj_path.as_path()).map_err(PyProjErr::Io)?;

        // Create project top level directories
        let mut dirs: Vec<&String> = Vec::new();
        dirs.push(&self.src_name);
        if self.with_script {
            dirs.push(&self.script_name);
        }
        if self.with_tests {
            dirs.push(&self.tests_name);
        }

        for path in dirs.iter() {
            proj_path.push(path);
            fs::create_dir(proj_path.as_path()).map_err(PyProjErr::Io)?;
            proj_path.pop();
        }

        // Create top level files
        // Create project top level files
        // if self.with_dockerfile {
        //     dirs.push(&self.dockerfile_name);
        // }

        return Ok(());
    }

}


// *****************************************************************************
// *****************************************************************************
// *****************************************************************************


#[cfg(test)]
mod tests {

    use super::*;

    fn timestamp() -> Result<String, PyProjErr> {
        Ok(SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(PyProjErr::SystemTime)?
            .as_secs()
            .to_string())
    }

    // TODO: This works in so far as creating the project, but doesn't do any actual
    // validation. Add verification of the actual directroy structure on disk.
    #[test]
    fn test_create() -> Result<(), PyProjErr> {

        // create test project in $TMP directory
        let timestamp = timestamp().expect("Failed to create timestamp");
        let name = format!("pyproj_{}", &timestamp);
        let path = env::temp_dir();
        let mut pyproj = PyProj::new();
        pyproj.name(&name);
        pyproj.path(&path);
        println!("{:?}", pyproj);
        pyproj.create();

        // Remove after test
        let mut test_dir = path.clone();
        test_dir.push(&pyproj.name);
        fs::remove_dir_all(test_dir.as_path());

        Ok(())
    }

    #[test]
    fn test_error() {
        let err = Error {
            message: "This is the message of my test error".to_string(),
            kind: ErrorKind::ValueValidation,
            info: None,
        };
        // err.exit();

    }

    #[test]
    fn test_fs() {        
        test_walk();
    }
}
