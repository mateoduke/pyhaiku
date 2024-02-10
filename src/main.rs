use std::env;
use std::process::Command;

mod semver;

use semver::SemVer;

#[derive(Debug)]
struct PythonVersion {
    sem_ver: SemVer,
    path: String,
}

impl PythonVersion {
    pub fn new(sem_ver: SemVer, path: String) -> Self {
        PythonVersion { sem_ver, path }
    }
}

fn parse_raw_sem_ver(sem_ver: &str) -> Option<SemVer> {
    // semver should be of the form "V:<semver>"
    let split_sem_ver: Vec<&str> = sem_ver.split(':').collect();
    if split_sem_ver.len() < 2 {
        return None;
    }
    let sem = SemVer::from_str(split_sem_ver[1]);
    if sem.is_some() {
        return sem;
    };
    None
}

fn get_version_components(line: &str) -> Option<(SemVer, String)> {
    let mut line_components: Vec<&str> = line.split(' ').collect();
    line_components.retain(|&str_slice| !str_slice.is_empty() && str_slice != "*");

    if line_components.len() < 2 {
        return None;
    }
    // get the first and last element of this vector

    let sem_ver = parse_raw_sem_ver(line_components[0]);
    let path = line_components.last().unwrap().to_string();

    if let Some(ver) = sem_ver {
        return Some((ver, path));
    }
    None
}

fn display_found_python_versions(python_versions: &[PythonVersion]) {
    println!("Installed Python Versions:");
    python_versions.iter().for_each(|version| {
        println!("Version: {} | Path: {}", version.sem_ver, version.path);
    });
}

fn main() {
    if !cfg!(target_os = "windows") {
        println!("This tool is currently only supported for windows systems \n Exiting...");
        return;
    }

    let args: Vec<String> = env::args().collect();
    let desired_python_sem_ver = match SemVer::from_str(args[1].as_str()) {
        Some(version) => version,
        None => {
            println!("Desired python version {} is not a valid semver", &args[1]);
            println!("Valid semver is of pattern <major.minor.patch> or <major.minor>");
            return;
        }
    };

    let output = Command::new("py")
        .args(["-0p"])
        .output()
        .expect("Failed to execute 'py' process");

    let validated_stdout = match String::from_utf8(output.stdout) {
        Ok(version) => version,
        Err(err) => {
            println!("Encountered an error reading version input from 'py' exe");
            println!("{}", err);
            println!("Please make sure you have 'py' installed");
            return;
        }
    };

    let parsed_lines: Vec<&str> = validated_stdout.split("\r\n").collect();

    let mut py_versions: Vec<PythonVersion> = Vec::new();
    parsed_lines.iter().for_each(|line| {
        if let Some(py_version) = get_version_components(line) {
            py_versions.push(PythonVersion::new(py_version.0, py_version.1.to_owned()));
        };
    });

    display_found_python_versions(&py_versions);

    let installed_version: Vec<&PythonVersion> = py_versions
        .iter()
        .filter(|&py_version| py_version.sem_ver == desired_python_sem_ver)
        .collect();

    if installed_version.is_empty() {
        println!(
            "Desired python version: [{}] is not installed via py executable. Please install",
            desired_python_sem_ver
        );
        return;
    }

    let output = Command::new("poetry")
        .args(["env", "use", installed_version[0].path.as_str()])
        .output()
        .expect("Failed to execute 'poetry' process process");

    let output_err = String::from_utf8(output.stderr);
    match output_err {
        Ok(output) => println!("{}", output),
        Err(err) => println!("{}", err),
    }
}
