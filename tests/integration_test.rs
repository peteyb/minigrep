use minigrep;
use minigrep::Config;

#[test]
fn run_with_file() -> Result<(), String> {
    let config = Config::new(&[
        "minigrep".to_string(),
        "nobody".to_string(),
        "tests/test.txt".to_string(),
    ])
    .unwrap();

    match minigrep::run(config) {
        Ok(()) => Ok(()),
        Err(_e) => Err(String::from("No file")),
    }
}

#[test]
fn run_without_file() {
    let config = Config::new(&[
        "minigrep".to_string(),
        "nobody".to_string(),
        "tests/fail.txt".to_string(),
    ])
    .unwrap();

    let error = minigrep::run(config).unwrap_err();
    let error = error.downcast_ref::<std::io::Error>().unwrap();
    assert!(error.kind() == std::io::ErrorKind::NotFound);
}
