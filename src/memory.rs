use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default)]
pub struct ByteSizeConf(u64);

impl ByteSizeConf {
    pub fn as_bytes(&self) -> u64 {
        self.0
    }

    pub fn as_kilobytes(&self) -> u64 {
        self.0 / 1000
    }

    pub fn as_kibibytes(&self) -> u64 {
        self.0 / 1024
    }

    pub fn as_megabytes(&self) -> u64 {
        self.0 / 10u64.pow(6)
    }

    pub fn as_mebibytes(&self) -> u64 {
        self.0 / 2u64.pow(20)
    }

    pub fn as_gigabytes(&self) -> u64 {
        self.0 / 10u64.pow(9)
    }

    pub fn as_gibibytes(&self) -> u64 {
        self.0 / 2u64.pow(30)
    }

    fn to_string(&self) -> String {
        let size = self.0;
        if size < 1024 {
            format!("{} bytes", size)
        } else if size < 2u64.pow(20) {
            format!("{} KiB", size / 1024)
        } else if size < 2u64.pow(30) {
            format!("{} MiB", size / 2u64.pow(20))
        } else {
            format!("{} GiB", size / 2u64.pow(30))
        }
    }
}

impl<'de> Deserialize<'de> for ByteSizeConf {
    fn deserialize<D>(deserializer: D) -> Result<ByteSizeConf, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let size = parse_byte_size(&s).map_err(serde::de::Error::custom)?;
        Ok(ByteSizeConf(size))
    }
}

impl Serialize for ByteSizeConf {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl std::fmt::Display for ByteSizeConf {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

fn parse_byte_size(s: &str) -> Result<u64, String> {
    let re = Regex::new(r"^(\d+)\s*(b|kb|ki|mb|mi|gb|gi|tb|ti)$").map_err(|_| "Invalid regex")?;
    let s = s.to_lowercase();
    let caps = re
        .captures(s.as_str())
        .ok_or_else(|| format!("Invalid byte size: {}", s))?;
    let size = caps
        .get(1)
        .ok_or_else(|| "Invalid byte size".to_string())?
        .as_str()
        .parse()
        .map_err(|_| "Invalid byte size".to_string())?;
    let unit = caps
        .get(2)
        .ok_or_else(|| "Invalid byte size".to_string())?
        .as_str()
        .to_lowercase();
    let unit = unit.as_str();

    match unit {
        "b" => Ok(size),
        "kb" => Ok(size * 1000),
        "ki" => Ok(size * 2u64.pow(10)),
        "mb" => Ok(size * 10u64.pow(6)),
        "mi" => Ok(size * 2u64.pow(20)),
        "gb" => Ok(size * 10u64.pow(9)),
        "gi" => Ok(size * 2u64.pow(30)),
        "tb" => Ok(size * 10u64.pow(12)),
        "ti" => Ok(size * 2u64.pow(40)),
        _ => Err("Invalid byte size".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_byte_size() {
        assert_eq!(parse_byte_size("1b").unwrap(), 1);
        assert_eq!(parse_byte_size("1kb").unwrap(), 1000);
        assert_eq!(parse_byte_size("1ki").unwrap(), 1024);
        assert_eq!(parse_byte_size("1mb").unwrap(), 1000 * 1000);
        assert_eq!(parse_byte_size("1mi").unwrap(), 1024 * 1024);
        assert_eq!(parse_byte_size("1gb").unwrap(), 1000 * 1000 * 1000);
        assert_eq!(parse_byte_size("1gi").unwrap(), 1024 * 1024 * 1024);
        assert_eq!(parse_byte_size("1tb").unwrap(), 1000 * 1000 * 1000 * 1000);
        assert_eq!(parse_byte_size("1ti").unwrap(), 1024 * 1024 * 1024 * 1024);
    }

    #[test]
    fn test_byte_size_display() {
        assert_eq!(ByteSizeConf(1).to_string(), "1 bytes");
        assert_eq!(ByteSizeConf(1024).to_string(), "1 KiB");
        assert_eq!(ByteSizeConf(1024 * 1024).to_string(), "1 MiB");
        assert_eq!(ByteSizeConf(1024 * 1024 * 1024).to_string(), "1 GiB");
    }
}
