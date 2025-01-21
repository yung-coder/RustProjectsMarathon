use std::error::Error;

use csv;

fn read_from_file(path: &str) -> Result<(), Box<csv::Error>> {
    let mut reader = csv::Reader::from_path(path)?;

    for result in reader.records() {
        let record = result?;

        println!("{:?}", record);
    }

    Ok(())
}

fn main() {
    if let Err(e) = read_from_file("./new.csv") {
        eprintln!("{}", e);
    }
}
