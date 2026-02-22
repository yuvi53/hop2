use dotenvy::dotenv;
use std::env;
use std::path::PathBuf;

#[cfg(test)]
mod tests;

pub mod data;
pub mod search;

#[derive(PartialEq, Debug, Clone)]
pub struct Data {
    pub weight: f64,
    pub path: PathBuf,
}

pub const FUZZY_MATCH_THRESHOLD: f64 = 0.6;

pub fn set_defaults() -> PathBuf {
    dotenv().ok();
    let mut data_home: PathBuf = match env::var("XDG_DATA_HOME") {
        Ok(path) => PathBuf::from(&path),
        Err(_) => [&env::var("HOME").unwrap(), ".local", "share"]
            .iter()
            .collect(),
    };
    data_home.push("hop2/hop2.txt");
    data_home
}

pub fn add_path(path: PathBuf, data: &mut Vec<Data>, weight: Option<f64>) {
    let weight = match weight {
        Some(num) => num,
        None => 10.0,
    };

    if path == PathBuf::from(env::var("HOME").unwrap()) {
        return;
    }
    match data::exist_in_database(&path) {
        false => {
            data.push(Data { weight, path });
        }
        true => {
            for Data {
                weight: lweight,
                path: lpath,
            } in data.iter_mut()
            {
                if path == *lpath {
                    *lweight = ((*lweight * *lweight) + (weight * weight)).sqrt();
                }
            }
        }
    }
}

pub fn find_matches(needle: &str, mut entries: Vec<Data>) -> PathBuf {
    let is_cwd = |entry: &Data| {
        let pwd = std::env::current_dir().expect("couldn't get the working directory");
        pwd == entry.path
    };
    let meets_threshold = |entry: &Data| {
        let entry = entry
            .path
            .file_name()
            .expect("couldn't get the dir name")
            .to_str()
            .expect("couldn't convert OsStr into &str");
        search::match_percent(&entry, &needle) >= FUZZY_MATCH_THRESHOLD
    };
    let match_anywhere = |entry: &Data| {
        let mut exist = false;
        for component in entry.path.iter() {
            if needle == component.to_str().unwrap() {
                exist = true;
            }
        }
        exist
    };
    entries.sort_by(|a, b| b.weight.total_cmp(&a.weight));
    let entries: Vec<Data> = entries
        .into_iter()
        .filter(|entry| !is_cwd(entry) && entry.path.exists())
        .collect();

    let entries: Vec<Data> = entries
        .clone()
        .into_iter()
        .filter(|entry| entry.path.ends_with(&needle))
        .chain(entries.clone().into_iter().filter(meets_threshold))
        .chain(entries.clone().into_iter().filter(match_anywhere))
        .collect();
    let mut matches: Vec<Data> = Vec::new();
    for entry in entries.into_iter() {
        if !matches.contains(&entry) {
            matches.push(entry);
        }
    }
    matches[0].path.clone()
}
