use std::{error::Error, fs, path::{self, PathBuf}};

use regex::Regex;
use rfd::FileDialog;
use serde::{Deserialize, Serialize};

pub fn read_txt_to_json(file_path: &std::path::PathBuf) -> Result<PathBuf, Box<dyn Error>> {
    let txt = fs::read_to_string(file_path)?;

    let mut disc_no = 0u32;
    let disc_regex = Regex::new(r"^(\d+)\.$")?;
    let movie_regex = Regex::new(r"^(\d{4})(.*?)((儿童))?$")?;
    let mut movies = Vec::new();

    for line in txt.lines().map(str::trim).filter(|l| ! l.is_empty()) {
        if let Some(no) = disc_number(line, &disc_regex) {
            disc_no = no;
        } else {
            if let Some(movie) = parse_movie(disc_no, line, &movie_regex) {
                movies.push(movie);
            } 
        }
    }
    
    save_to_json(movies)
}

fn save_to_json(movies: Vec<Movie>) -> Result<PathBuf, Box<dyn Error>> {
    let json_str = serde_json::to_string_pretty(&movies)?;
    let path = FileDialog::new().add_filter("Json", &["json"])
    .set_title("Save data to Json file")
    .set_directory(r"/")
    .save_file()
    .ok_or_else(|| "No save location selected".to_string())?;

    fs::write(&path, json_str)?;

    Ok(path)
}

fn parse_movie(disc_no: u32, line: &str, re: &Regex) -> Option<Movie> {
    re.captures(line).map(|caps| {
        Movie { 
            disc: disc_no, 
            year: caps.get(1).unwrap().as_str().trim().to_string(), 
            title: caps.get(2).unwrap().as_str().trim().to_string(), 
            remark: caps.get(3).map(|m|m.as_str().trim().to_string()), 
        }
    })
}

fn disc_number(line: &str, re: &Regex) -> Option<u32> {
    re.captures(line)
    .map(|caps| caps.get(1).unwrap().as_str().parse::<u32>().unwrap())
}

#[derive(Debug, Serialize, Deserialize)]
struct Movie {
    disc: u32,
    year: String,
    title: String,
    remark: Option<String>
}