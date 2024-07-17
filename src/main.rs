
extern crate clap;
extern crate iso_country;

use clap::Parser;
use iso_country::data::all;
use find_country::{all_countries, find_country};

mod find_country;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    country: Option<String>,

    #[arg(short, long, help = "Use exact matching")]
    exact: bool
}




fn main() {
    let cli = Cli::parse();

    match cli.country {
        Some(country) => {
            let country = find_country(&country, cli.exact)
                .unwrap_or_else(|| format!("ERROR: Unknown country \"{}\"", &country));

            println!("{}", country)
        }
        None => {
            let all_countries = all_countries();
            println!("{}", all_countries)
        }
    }
}



