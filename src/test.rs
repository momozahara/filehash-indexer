use std::{fs::File, io::BufReader, path::Path};

use data_encoding::HEXUPPER;

use crate::sha256_digest;

#[test]
fn test_hash() {
    let input = File::open("test/test.txt").unwrap();
    let reader = BufReader::new(input);
    let digest = sha256_digest(reader).unwrap();
    let hash = HEXUPPER.encode(digest.as_ref()).to_lowercase();

    assert_eq!(
        hash,
        "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f"
    )
}

#[test]
fn visit_dirs() {
    crate::visit_dirs(Path::new("test"), &|entry| {
        let path = entry.path();
        assert_eq!(path.as_os_str().to_str().unwrap(), "test\\test.txt")
    })
    .unwrap();
}
