# whatcountry
A one-liner tool for checking ISO-3166 country codes.

Nothing interesting to see here. 

## Installation

```shell
cargo install --git https://github.com/vladvalkov/whatcountry
```

## Usage
```shell
❯ whatcountry --help
Usage: whatcountry [OPTIONS] [COUNTRY]

Arguments:
  [COUNTRY]

Options:
  -e, --exact    Use exact matching on passed string
  -h, --help     Print help
  -V, --version  Print version
```

By default, partially matches countries' Alpha2, Alpha3 codes, and name.
```
❯ whatcountry US
036	AU	AUS	Australia
040	AT	AUT	Austria
096	BN	BRN	Brunei Darussalam
112	BY	BLR	Belarus
196	CY	CYP	Cyprus
480	MU	MUS	Mauritius
535	BQ	BES	Bonaire, Sint Eustatius and Saba
643	RU	RUS	Russian Federation
840	US	USA	United States of America
```

If no argument is passed, prints all entries.

If `--exact` flag is passed, uses exact matching instead.
```shell
❯ whatcountry -e US
840	US	USA	United States of America
❯ whatcountry -e USA
840	US	USA	United States of America
❯ whatcountry -e "United States"
ERROR: Unknown country "United States"
```