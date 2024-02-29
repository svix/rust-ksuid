use serde::Deserialize;
#[cfg(feature = "serde")]
use serde::Serialize;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::{
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
    str::FromStr,
};
use svix_ksuid::*;

const BASE_16_CHARS: &[u8; 16] = b"0123456789ABCDEF";

#[derive(Clone, Debug, PartialEq, Deserialize)]
struct TestDataLine {
    timestamp: u64,
    payload: String,
    ksuid: String,
}

fn read_lines(file: PathBuf) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(file)?;
    Ok(io::BufReader::new(file).lines())
}

/// Test compatibility with the segment reference implementation
#[test]
fn test_reference_compat() -> Result<(), String> {
    let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file.push("tests/test_kuids.txt");

    for line in read_lines(file).unwrap() {
        let line = line.unwrap();
        let data_line: TestDataLine = serde_json::from_str(&line).unwrap();
        let payload = base_encode::from_str(&data_line.payload, 16, BASE_16_CHARS).unwrap();
        let payload = &payload[payload.len() - Ksuid::PAYLOAD_BYTES..];
        let ksuid = Ksuid::from_str(&data_line.ksuid).unwrap();
        let constructed_ksuid = Ksuid::new_raw(
            (data_line.timestamp - KSUID_EPOCH as u64) as u32,
            Some(payload),
        );

        assert_eq!(data_line.ksuid, constructed_ksuid.to_string());
        assert_eq!(ksuid, constructed_ksuid);
        assert_eq!(ksuid.payload(), payload);
        assert_eq!(
            ksuid.timestamp_raw(),
            (data_line.timestamp - KSUID_EPOCH as u64) as u32
        );
    }
    Ok(())
}

/// Test compatibility with the segment reference implementation
#[test]
fn test_ksuidms_reference_compat() -> Result<(), String> {
    let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file.push("tests/test_kuids.txt");

    for line in read_lines(file).unwrap() {
        let line = line.unwrap();
        let data_line: TestDataLine = serde_json::from_str(&line).unwrap();
        let ksuid = Ksuid::from_str(&data_line.ksuid).unwrap();
        let ksuidms = KsuidMs::new(
            Some(ksuid.timestamp()),
            Some(&ksuid.payload()[..KsuidMs::PAYLOAD_BYTES]),
        );
        assert_eq!(ksuid.timestamp(), ksuidms.timestamp());

        let ksuidms_from = KsuidMs::new(Some(ksuidms.timestamp()), Some(ksuidms.payload()));
        assert_eq!(ksuidms_from.payload(), ksuidms.payload());
        assert_eq!(ksuidms_from.timestamp(), ksuidms.timestamp());

        let ksuidms = KsuidMs::from_str(&data_line.ksuid).unwrap();
        let timediff = ksuidms.timestamp() - ksuid.timestamp();
        assert!(timediff.whole_milliseconds().abs() <= 1_000);
        assert_eq!(ksuidms.to_base62(), data_line.ksuid);
    }
    Ok(())
}

#[test]
fn test_ksuidms_corner_cases() -> Result<(), String> {
    // Trying an invalid value for the last KsuidMs byte.
    let buf = [0xFFu8; 16];
    let ksuid = Ksuid::from_seconds(Some(40_000), Some(&buf));
    let ksuidms = KsuidMs::from_bytes(*ksuid.bytes());
    let timediff = ksuidms.timestamp() - ksuid.timestamp();
    assert!(timediff.whole_milliseconds().abs() <= 1_000);
    Ok(())
}

#[test]
fn test_ordering() -> Result<(), String> {
    let ksuid1 = Ksuid::from_seconds(Some(1_555_555_555), None);
    let ksuid2 = Ksuid::from_seconds(Some(1_777_777_777), None);

    assert!(ksuid1 < ksuid2);
    assert!(ksuid1 <= ksuid2);
    assert!(ksuid1 == ksuid1);
    assert!(ksuid2 > ksuid1);
    assert!(ksuid2 >= ksuid1);
    Ok(())
}

#[test]
fn test_hash() {
    // given
    let mut set = HashSet::new();
    let ksuid1 = Ksuid::new(None, None);
    let ksuid2 = Ksuid::new(None, None);
    // when
    set.insert(ksuid1.clone());
    set.insert(ksuid2.clone());
    // then
    assert_eq!(set.len(), 2);
    assert!(set.contains(&ksuid1));
    assert!(set.contains(&ksuid2));
}

#[test]
fn test_hash_ms() {
    // given
    let mut set = HashSet::new();
    let ksuidms1 = KsuidMs::new(None, None);
    let ksuidms2 = KsuidMs::new(None, None);
    // when
    set.insert(ksuidms1.clone());
    set.insert(ksuidms2.clone());
    // then
    assert_eq!(set.len(), 2);
    assert!(set.contains(&ksuidms1));
    assert!(set.contains(&ksuidms2));
}

#[test]
fn test_hash_are_deterministic() {
    let cases: Vec<(&str, u64)> = vec![
        ("000000pryYUMiBILyxOCoroLz6w", 7055508431330265589),
        ("02GY99XXwBHbeBundUPJoqYpvet", 3000553476947541532),
        ("04X6IIDacKaayM4WWom0flpf3mY", 15766199744355542564),
    ];

    for case in cases {
        // given
        let id_as_str = case.0;
        let expected_hash = case.1;
        // when
        let ksuid = Ksuid::from_str(id_as_str).expect("valid id");
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        ksuid.hash(&mut hasher);
        let hashed = hasher.finish();
        // then
        assert_eq!(hashed, expected_hash);
    }
}

#[cfg(feature = "serde")]
#[derive(Serialize, Deserialize)]
struct TestKsuid {
    id: Ksuid,
}

#[cfg(feature = "serde")]
#[derive(Serialize, Deserialize)]
struct TestKsuidMs {
    id: KsuidMs,
}

#[cfg(feature = "serde")]
#[test]
fn test_serialize_to_base62() {
    let b62 = "1srOrx2ZWZBpBUvZwXKQmoEYga2";
    let json = r#"{"id":"1srOrx2ZWZBpBUvZwXKQmoEYga2"}"#;
    let ksuid_obj = TestKsuid {
        id: Ksuid::from_base62(b62).unwrap(),
    };
    let ksuidms_obj = TestKsuidMs {
        id: KsuidMs::from_base62(b62).unwrap(),
    };

    let test_cases = vec![
        serde_json::to_string(&ksuid_obj),
        serde_json::to_string(&ksuidms_obj),
    ];
    for serialized in test_cases {
        assert!(serialized.is_ok());
        let serialized = serialized.unwrap();
        assert_eq!(serialized, json);
    }
}

#[cfg(feature = "serde")]
#[test]
fn test_deserialize_from_base62() {
    let b62 = "1srOrx2ZWZBpBUvZwXKQmoEYga2";
    let json = r#"{"id":"1srOrx2ZWZBpBUvZwXKQmoEYga2"}"#;

    let ksuid_obj: TestKsuid = serde_json::from_str(json).unwrap();
    let ksuidms_obj: TestKsuidMs = serde_json::from_str(json).unwrap();
    assert_eq!(ksuid_obj.id.to_string(), b62);
    assert_eq!(ksuidms_obj.id.to_string(), b62);
}

#[test]
fn test_deserialize_bad_base62_length() {
    let short_b62 = "ZBpBUvZwXKQmoEYga2";
    let long_b62 = "1srOrx2ZWZBpBUvZwXKQmoEYga21srOrx2ZWZBpBUvZwXKQmoEYga2";

    let result = Ksuid::from_base62(short_b62);
    assert!(result.is_err(), "Short base62 strings should fail to parse");

    let result = Ksuid::from_base62(long_b62);
    assert!(result.is_err(), "Long base62 strings should fail to parse");
}
