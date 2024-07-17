use std::fmt::{Display, Formatter};

use iso_country::data::CountryCode;

pub fn all_countries() -> String {
    let countries = iso_country::data::all()
        .iter()
        .map(|x| DisplayCountryCode(x).to_string())
        .collect::<Vec<_>>()
        .join("\r\n");
    return countries
}
pub fn find_country(country: &String, exact: bool) -> Option<String> {
    let s = country.to_lowercase();

    let s_ref = s.as_str();

    if exact {

    }
    let v: Vec<_> = iso_country::data::all()
        .iter()
        .filter(|c| {
            if !exact {
                return c.num.ends_with(s_ref)
                    || c.alpha2.to_lowercase().contains(s_ref)
                    || c.alpha3.to_lowercase().contains(s_ref)
                    || c.name.to_lowercase().contains(s_ref)
            } else {
                return c.num.trim_start_matches("0") == s_ref.trim_start_matches("0")
                    || c.alpha2.to_lowercase() == s_ref
                    || c.alpha3.to_lowercase() == s_ref
                    || c.name.to_lowercase() == s_ref
            }
        })
        .map(|x| DisplayCountryCode(x).to_string()).collect();

    match v.len() {
        0 => None,
        _ => Some(v.join("\r\n"))
    }
}

struct DisplayCountryCode<'a>(&'a CountryCode<'a>);

impl Display for DisplayCountryCode<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\t{}\t{}\t{}",
               self.0.num, self.0.alpha2, self.0.alpha3, self.0.name)
    }
}