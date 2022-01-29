use core::fmt;
use std::{error, str::FromStr};

use byteorder::{BigEndian, ByteOrder};
use chrono::{DateTime, NaiveDateTime, Utc};

pub const KSUID_EPOCH: i64 = 1_400_000_000;

const BASE_62_CHARS: &[u8; 62] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

const TOTAL_BYTES: usize = 20;

#[derive(Debug)]
pub struct Error(String);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

pub trait KsuidLike {
    type Type;
    const TIMESTAMP_BYTES: usize;
    const PAYLOAD_BYTES: usize;

    /// Creates new Ksuid with specified timestamp (DateTime) and optional payload
    ///
    /// # Examples
    /// ```
    /// use svix_ksuid::*;
    ///
    /// let ksuid = Ksuid::new(None, None);
    /// ```
    fn new(timestamp: Option<DateTime<Utc>>, payload: Option<&[u8]>) -> Self::Type;

    /// Creates new Ksuid with specified timestamp (in seconds) and optional payload
    ///
    /// # Examples
    /// ```
    /// use svix_ksuid::*;
    ///
    /// let ksuid = Ksuid::from_seconds(Some(1_621_627_443), None);
    /// ```
    fn from_seconds(timestamp: Option<i64>, payload: Option<&[u8]>) -> Self::Type;

    /// Get the timestamp portion of the ksuid
    ///
    /// # Examples
    /// ```
    /// use svix_ksuid::*;
    ///
    /// let ksuid = Ksuid::new_raw(0, None);
    /// ```
    fn timestamp(&self) -> DateTime<Utc>;

    /// Get the timestamp portion of the ksuid in seconds
    ///
    /// # Examples
    /// ```
    /// use svix_ksuid::*;
    ///
    /// let timestamp = 1_621_627_443;
    /// let ksuid = Ksuid::from_seconds(Some(timestamp), None);
    /// assert_eq!(ksuid.timestamp_seconds(), timestamp);
    /// ```
    fn timestamp_seconds(&self) -> i64 {
        self.timestamp().timestamp()
    }

    /// Get the payload portion of the ksuid
    ///
    /// # Examples
    /// ```
    /// use svix_ksuid::*;
    ///
    /// let buf = b"1234567890ABCDEF";
    /// let ksuid = Ksuid::new(None, Some(&buf[..]));
    /// assert_eq!(ksuid.payload(), &buf[..]);
    /// ```
    fn payload(&self) -> &[u8] {
        &self.bytes()[Self::TIMESTAMP_BYTES..]
    }

    /// Create a new ksuid from bytes
    ///
    /// # Examples
    /// ```
    /// use svix_ksuid::*;
    ///
    /// let bytes = [12u8; 20];
    /// let ksuid = Ksuid::from_bytes(bytes.clone());
    /// assert_eq!(&bytes, ksuid.bytes());
    /// ```
    fn from_bytes(bytes: [u8; TOTAL_BYTES]) -> Self::Type;

    /// Get the ksuid as bytes
    ///
    /// # Examples
    /// ```
    /// use svix_ksuid::*;
    ///
    /// let bytes = [12u8; 20];
    /// let ksuid = Ksuid::from_bytes(bytes.clone());
    /// assert_eq!(&bytes, ksuid.bytes());
    /// ```
    fn bytes(&self) -> &[u8; TOTAL_BYTES];

    /// Convert the Ksuid to base62
    ///
    /// # Examples
    /// ```
    /// use svix_ksuid::*;
    /// use byteorder::{ByteOrder, BigEndian};
    ///
    /// let mut buf = [0u8; 16];
    /// BigEndian::write_u128(&mut buf, 45419194335837378647185401984346151808);
    /// let ksuid = Ksuid::new_raw(1643290698, Some(&buf));
    /// let base62 = ksuid.to_string();
    /// assert_eq!(base62, "DyU8bFOBPZ4LjvsfN0qywt2LjmK");
    ///
    /// ```
    fn to_base62(&self) -> String {
        format!(
            "{:0>27}",
            base_encode::to_string(self.bytes(), 62, BASE_62_CHARS).unwrap()
        )
    }

    /// Load a base62 representation to a Ksuid
    ///
    /// # Examples
    /// ```
    /// use svix_ksuid::*;
    /// use std::str::FromStr;
    ///
    /// let ksuid = Ksuid::from_str("24CtFf3hyVZHdSkQy0nMBa1OjOA").unwrap();
    /// assert_eq!(ksuid.to_string(), "24CtFf3hyVZHdSkQy0nMBa1OjOA");
    ///
    /// ```
    fn from_base62(s: &str) -> Result<Self::Type, Error> {
        if let Some(loaded) = base_encode::from_str(s, 62, BASE_62_CHARS) {
            // Get the last TOTAL_BYTES
            let loaded = if loaded.len() > TOTAL_BYTES {
                &loaded[loaded.len() - TOTAL_BYTES..]
            } else {
                &loaded[..]
            };
            let mut buf = [0u8; TOTAL_BYTES];
            if loaded.len() != TOTAL_BYTES {
                Err(Error(format!(
                    "Got ksuid of unexpected length {}",
                    loaded.len()
                )))
            } else {
                buf.copy_from_slice(loaded);
                Ok(Self::from_bytes(buf))
            }
        } else {
            Err(Error("Failed to decode".to_owned()))
        }
    }
}

/// # Examples
/// ```
/// use svix_ksuid::*;
/// use std::str::FromStr;
///
/// let ksuid = Ksuid::new(None, None);
/// let as_string: String = ksuid.to_string();
/// let ksuid2 = Ksuid::from_str(&as_string).unwrap();
/// assert_eq!(ksuid, ksuid2);
///  ```
/// K-Sortable Unique ID
#[derive(Debug, PartialOrd, Ord, Clone, Copy, PartialEq, Eq)]
pub struct Ksuid([u8; TOTAL_BYTES]);

impl Ksuid {
    /// Creates new Ksuid with specified timestamp (in KSUID Epoch) and optional payload
    ///
    /// # Examples
    /// ```
    /// use svix_ksuid::*;
    ///
    /// let ksuid = Ksuid::new_raw(0, None);
    /// ```
    pub fn new_raw(timestamp: u32, payload: Option<&[u8]>) -> Self {
        let mut buf = [0u8; TOTAL_BYTES];
        BigEndian::write_u32(&mut buf, timestamp);
        if let Some(payload) = payload {
            buf[Self::TIMESTAMP_BYTES..].copy_from_slice(payload);
        } else {
            getrandom::getrandom(&mut buf[Self::TIMESTAMP_BYTES..]).unwrap();
        }
        Self::from_bytes(buf)
    }

    /// Get the raw timestamp value of the ksuid
    /// This is the actual raw value in seconds since `KSUID_EPOCH`
    ///
    /// # Examples
    /// ```
    /// use svix_ksuid::*;
    ///
    /// let ksuid = Ksuid::new(None, None);
    /// let raw = ksuid.timestamp_raw();
    /// ```
    pub fn timestamp_raw(&self) -> u32 {
        BigEndian::read_u32(self.bytes())
    }
}

impl KsuidLike for Ksuid {
    type Type = Ksuid;
    const TIMESTAMP_BYTES: usize = 4;
    const PAYLOAD_BYTES: usize = 16;

    fn new(timestamp: Option<DateTime<Utc>>, payload: Option<&[u8]>) -> Self {
        let timestamp = timestamp.map(|x| x.timestamp());
        Self::from_seconds(timestamp, payload)
    }

    fn from_seconds(timestamp: Option<i64>, payload: Option<&[u8]>) -> Self {
        let timestamp = timestamp.unwrap_or_else(|| Utc::now().timestamp()) - KSUID_EPOCH;
        Self::new_raw(timestamp as u32, payload)
    }

    fn from_bytes(bytes: [u8; TOTAL_BYTES]) -> Self {
        Self(bytes)
    }

    fn bytes(&self) -> &[u8; TOTAL_BYTES] {
        &self.0
    }

    fn timestamp(&self) -> DateTime<Utc> {
        let timestamp = self.timestamp_raw() as i64 + KSUID_EPOCH;
        let naive = NaiveDateTime::from_timestamp(timestamp, 0);
        DateTime::from_utc(naive, Utc)
    }
}

impl ToString for Ksuid {
    fn to_string(&self) -> String {
        self.to_base62()
    }
}

impl FromStr for Ksuid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_base62(s)
    }
}

/// # Examples
/// ```
/// use svix_ksuid::*;
/// use std::str::FromStr;
///
/// let ksuid = KsuidMs::new(None, None);
/// let as_string: String = ksuid.to_string();
/// let ksuid2 = KsuidMs::from_str(&as_string).unwrap();
/// assert_eq!(ksuid, ksuid2);
///  ```
/// K-Sortable Unique ID
///
/// This one has Ms accuracy compared to the normal one that has second accuracy
#[derive(Debug, PartialOrd, Ord, Clone, Copy, PartialEq, Eq)]
pub struct KsuidMs([u8; TOTAL_BYTES]);

impl KsuidMs {
    const U64_BYTES: usize = 8;

    /// Creates new KsuidMs with specified timestamp (in KSUID Epoch) and optional payload
    ///
    /// # Examples
    /// ```
    /// use svix_ksuid::*;
    ///
    /// let ksuid = KsuidMs::new_raw(0, None);
    /// ```
    pub fn new_raw(timestamp: u64, payload: Option<&[u8]>) -> Self {
        let mut buf = [0u8; TOTAL_BYTES];
        let mut timestamp_buf = [0u8; Self::U64_BYTES];
        BigEndian::write_u64(&mut timestamp_buf, timestamp);
        // We only want the TIMESTAMP_BYTES least significant bytes
        buf[..Self::TIMESTAMP_BYTES].copy_from_slice(
            &timestamp_buf[Self::U64_BYTES - Self::TIMESTAMP_BYTES..Self::U64_BYTES],
        );
        if let Some(payload) = payload {
            buf[Self::TIMESTAMP_BYTES..].copy_from_slice(payload);
        } else {
            getrandom::getrandom(&mut buf[Self::TIMESTAMP_BYTES..]).unwrap();
        }
        Self::from_bytes(buf)
    }

    /// Creates new Ksuid with specified timestamp (in milliseconds) and optional payload
    ///
    /// # Examples
    /// ```
    /// use svix_ksuid::*;
    ///
    /// let ksuid = KsuidMs::from_millis(Some(1_621_627_443_000), None);
    /// ```
    pub fn from_millis(timestamp: Option<i64>, payload: Option<&[u8]>) -> Self {
        let timestamp_ms = timestamp.unwrap_or_else(|| Utc::now().timestamp_millis());
        let timestamp_s = (timestamp_ms / 1_000) - KSUID_EPOCH;
        let timestamp_ms = (timestamp_ms % 1_000) >> 2;
        let timestamp = ((timestamp_s << 8) & 0xFFFFFFFF00) | timestamp_ms;
        Self::new_raw(timestamp as u64, payload)
    }

    /// Get the timestamp portion of the ksuid in milliseconds
    ///
    /// # Examples
    /// ```
    /// use svix_ksuid::*;
    ///
    /// let timestamp = 1_621_627_443_000;
    /// let ksuid = KsuidMs::from_millis(Some(timestamp), None);
    /// assert_eq!(ksuid.timestamp_millis(), timestamp);
    /// ```
    pub fn timestamp_millis(&self) -> i64 {
        self.timestamp().timestamp_millis()
    }

    /// Get the raw timestamp value of the ksuid
    /// This is the actual raw value. The four most significant bytes are the seconds since
    /// `KSUID_EPOCH`, and the last byte is the number of 4ms units to add to the epoch.
    ///
    /// # Examples
    /// ```
    /// use svix_ksuid::*;
    ///
    /// let ksuid = Ksuid::new(None, None);
    /// let raw = ksuid.timestamp_raw();
    /// ```
    pub fn timestamp_raw(&self) -> u64 {
        // Remove two bytes from the result (as we are only u48, not u64, and then mask the result)
        BigEndian::read_u64(self.bytes()) >> ((Self::U64_BYTES - Self::TIMESTAMP_BYTES) * 8)
    }
}

impl KsuidLike for KsuidMs {
    type Type = KsuidMs;
    const TIMESTAMP_BYTES: usize = 5;
    const PAYLOAD_BYTES: usize = 15;

    fn new(timestamp: Option<DateTime<Utc>>, payload: Option<&[u8]>) -> Self {
        let timestamp = timestamp.map(|x| x.timestamp_millis());
        Self::from_millis(timestamp, payload)
    }

    fn from_seconds(timestamp: Option<i64>, payload: Option<&[u8]>) -> Self {
        let timestamp = timestamp.map(|x| x * 1_000);
        Self::from_millis(timestamp, payload)
    }

    fn from_bytes(bytes: [u8; TOTAL_BYTES]) -> Self {
        Self(bytes)
    }

    fn bytes(&self) -> &[u8; TOTAL_BYTES] {
        &self.0
    }

    fn timestamp(&self) -> DateTime<Utc> {
        let timestamp = self.timestamp_raw() as i64;
        let seconds = (timestamp >> 8) + KSUID_EPOCH;
        let ns = 1_000_000 * (((timestamp & 0xFF) << 2) % 1_000);

        let naive = NaiveDateTime::from_timestamp(seconds, ns as u32);
        DateTime::from_utc(naive, Utc)
    }
}

impl ToString for KsuidMs {
    fn to_string(&self) -> String {
        self.to_base62()
    }
}

impl FromStr for KsuidMs {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_base62(s)
    }
}