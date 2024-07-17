use std::fmt::{Display, Formatter};
use iso_country::data::CountryCode;

const LOWERCASE_COUNTRIES: Vec<CountryCode> = iso_country::data::all()
    .into_iter()
    .map(|country| {
        CountryCode{
            alpha2: country.alpha2.to_lowercase().as_str(),
            alpha3: country.alpha3.to_lowercase().as_str(),
            name: country.name.to_lowercase().as_str(),
            num: country.num,
        }
    })
    .collect();


fn find_country(country: String) -> String {
    let s = country.to_lowercase().as_str();

    let v: Vec<_> = LOWERCASE_COUNTRIES
        .iter()
        .filter(|c| {
            c.num == s
                || c.alpha2.contains(s)
                || c.alpha3.contains(s)
                || c.name.contains(s)
        })
        .map(|x| DisplayCountryCode(x).to_string()).collect();

    if v.is_empty() {
        return String::from("Unknown country!")
    }

    v.join("\n")
}

struct DisplayCountryCode<'a>(&'a CountryCode<'a>);

impl Display for DisplayCountryCode<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\t{}\t{}\t{}",
            self.0.num, self.0.alpha2, self.0.alpha3, self.0.name
        )
    }
}
