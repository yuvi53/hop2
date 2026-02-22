use std::error::Error;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::PathBuf;

use crate::{Data, set_defaults};

pub fn load(database_path: PathBuf) -> Vec<Data> {
    let file = match fs::read_to_string(&database_path) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                if !database_path.parent().unwrap().exists() {
                    fs::create_dir_all(database_path.parent().unwrap())
                        .expect(&format!("couldn't create directory: {}", &database_path.display()));
                    return Vec::new();
                }
                File::create(&database_path)
                    .expect(&format!("couldn't create file: {}", &database_path.display()));
                return Vec::new();
            },
            _ => panic!("problem opening the file: {error:?}"),
        }
    };
    let results: Vec<Data> = file
        .lines()
        .map(|line| {
            let v: Vec<&str> = line.split("\t").collect();
            let weight = v[0]
                .parse::<f64>()
                .expect("couldn't convert &str to f64(while parsing)");
            let path = v[1].trim();
            Data {
                weight,
                path: PathBuf::from(path),
            }
        })
    .collect();
    results
}

pub fn save(data: Vec<Data>) -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    for Data { weight, path } in data {
        let path = path.to_str().unwrap();
        buffer.push_str(&format!("{}\t{}\n", weight, path));
    }
    let database_path = set_defaults();
    if !database_path.parent().unwrap().exists() {
        fs::create_dir_all(&database_path.parent().unwrap())
            .expect(&format!("couldn't create directory: {}", &database_path.display()));
    }
    let mut file = File::create(&database_path)?;
    write!(file, "{}", buffer)?;
    Ok(())
    }

pub fn exist_in_database(queried_path: &PathBuf) -> bool {
    let database = load(set_defaults());
    let mut exist = false;
    for Data { weight: _, ref path } in database {
        if path == queried_path {
            exist = true;
        }
    }
    exist
}
