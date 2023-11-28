

# OCSF Rust Crawler

This Rust application fetches JSON data from a specified URL ( The url is taken from the OCSF Schema Server docker image. ) and saves it to a file every second.

## Usage

1. Clone the repository
```
git clone https://github.com/mranv/ocsf-rust-crawler
```

2. Build the application 
```
cargo build
```

3. Run the application
```
cargo run  
```

## Configuration

The following parameters can be configured in `main.rs`:

- `url` - URL to fetch JSON data from 
- `duration` - Total runtime of the application
- `interval` - Interval (in seconds) between fetches

## Output

- Fetched JSON data is saved to a file named `ocsf_schema_sample_data<timestamp>.json` 
- Status and errors are printed to stdout

## Dependencies

- reqwest
- serde
- serde_json 
- chrono

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) file for details.

