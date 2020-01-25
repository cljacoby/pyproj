pub mod cli;
pub mod err;

use err::PyProjErr;

use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

// TODO: It seems like PyProj.path should be type Path and not PathBuf,
// but I can compile-time errors saying PyProf has unknown size at compile time.

// TODO: in PypProj.new, assess using many trait boundaries as opposed to just
// settling on using String

// TODO: Add ability to rename source directory something other than 'src'
// TODO: Assess if default src directory name should be 'src' or package name

// TODO: Intoducing builder pattern for PyProj. How should I handle path? Could
// initialzie it to a default location; however, this could result in directories
// unintentially being created. Setting as Option for now.

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

impl PyProj {
    pub fn new() -> Self {
        Self {
            name: String::from("unnamed_python_project"),
            path: None,
            src_name: String::from("src"),
            with_script: true,
            script_name: String::from("script"),
            tests_name: String::from("tests"),
            with_tests: true,
            with_dockerfile: false,
            // TODO: Don't initialzie dockerfile name unless with_dockerfile is true
            dockerfile_name: String::from("Dockerfile"),
        }
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }

    pub fn create(&self) -> Result<(), PyProjErr> {
        if self.path.is_none() {
            return Err(PyProjErr::app_err(
                "Cannot create project when path is None",
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


        // Create project top level files
        // if self.with_dockerfile {
        //     dirs.push(&self.dockerfile_name);
        // }

        return Ok(());
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_create() {
        // Get UNIX seconds timestamp
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("error: Timestamp calculation failed.")
            .as_secs()
            .to_string();
        // println!("timestamp = {:?}", timestamp);

        // create test directory
        let path = env::temp_dir();
        let name = format!("{}_{}", "test_pyproj", timestamp);
        let pyproj = PyProj::new(name.as_str(), path.clone());
        pyproj.create();

        // Remove after test
        let mut test_dir = path.clone();
        test_dir.push(name);
        fs::remove_dir_all(test_dir.as_path());
    }
}
