use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::io::Write;
use std::{fs, io};

const IMAGE_DIR: &'static str = "images";

#[derive(Serialize, Deserialize)]
pub struct Country {
    code: String,
    name: String,
    capital: Option<String>,
    population: usize,
}

pub fn create_images_dir() -> std::io::Result<()> {
    fs::create_dir(IMAGE_DIR)?;
    Ok(())
}

pub fn get_json() -> Result<Vec<Country>, Box<dyn Error>> {
    let resp = reqwest::blocking::get("https://restcountries.com/v3.1/all")?;
    let parsed_json: Value = serde_json::from_str(&resp.text().unwrap())?;
    let query = r#"[.[] | {"name": .["name"] | .["common"], "code": .["cca2"], "capital": .["capital"] | .[0], "population": .["population"]}]"#;
    let output = jq_rs::run(query, &parsed_json.to_string())?;
    let parsed_json: Vec<Country> = serde_json::from_str(&output)?;
    Ok(parsed_json)
}

fn download_image(code: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://flagsapi.com/{}/flat/64.png", code);
    let mut resp = reqwest::blocking::get(url)?;
    let mut file = fs::File::create(format!("{}/{}.png", IMAGE_DIR, code))?;
    resp.copy_to(&mut file)?;
    Ok(())
}

pub fn write_to_file_and_download(countries: Vec<Country>) -> io::Result<()> {
    let mut file = fs::File::create("countries.txt")?;
    let file_headers = "Flag\tName\tCapital\tPopulation\n";
    let _ = file.write_all(file_headers.as_bytes());
    for country in countries {
        let capital = match country.capital {
            Some(cap) => cap,
            None => String::from("none"),
        };
        let line = format!(
            "{}\t{}\t{}\t{}\n",
            country.code, country.name, capital, country.population
        );
        let _ = download_image(&country.code);
        let _ = file.write_all(line.as_bytes());
    }
    Ok(())
}
