use countries::{create_images_dir, get_json, write_to_file_and_download};

fn main() {
    match create_images_dir() {
        Ok(_) => (),
        Err(_) => panic!("Couldn't create folder"),
    };
    let countries = match get_json() {
        Ok(json) => json,
        Err(err) => panic!("Couldn't retrieve countries json: {}", err),
    };
    match write_to_file_and_download(countries) {
        Ok(_) => println!("Success"),
        Err(_) => panic!("Couldn't write to file"),
    };
}
