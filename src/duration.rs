use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::ops::Deref;
use std::time::Duration;

#[derive(Debug, Clone, Default)]
pub struct DurationConf(Duration);

impl DurationConf {
    pub fn new(duration: Duration) -> Self {
        Self(duration)
    }
}

impl<'de> Deserialize<'de> for DurationConf {
    fn deserialize<D>(deserializer: D) -> Result<DurationConf, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let duration = parse_duration(&s).map_err(serde::de::Error::custom)?;
        Ok(DurationConf(duration))
    }
}

impl Serialize for DurationConf {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let duration_str = format_duration(self.0);
        serializer.serialize_str(&duration_str)
    }
}

impl Deref for DurationConf {
    type Target = Duration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<Duration> for DurationConf {
    fn into(self) -> Duration {
        self.0
    }
}

fn parse_duration(s: &str) -> Result<Duration, String> {
    let re = Regex::new(r"^(\d+)\s*(ns|us|ms|s|m|h)$").map_err(|_| "Invalid regex")?;
    let caps = re.captures(s).ok_or_else(|| format!("Invalid duration: {}", s))?;
    let value: u64 = caps[1].parse().map_err(|_| "Invalid number")?;
    match &caps[2] {
        "ns" => Ok(Duration::from_nanos(value)),
        "us" => Ok(Duration::from_micros(value)),
        "ms" => Ok(Duration::from_millis(value)),
        "s" => Ok(Duration::from_secs(value)),
        "m" => Ok(Duration::from_secs(value * 60)),
        "h" => Ok(Duration::from_secs(value * 3600)),
        _ => Err("Invalid unit".to_string()),
    }
}

fn format_duration(duration: Duration) -> String {
    if duration.as_nanos() % 1_000 != 0 {
        format!("{}ns", duration.as_nanos())
    } else if duration.as_micros() % 1_000 != 0 {
        format!("{}us", duration.as_micros())
    } else if duration.as_millis() % 1_000 != 0 {
        format!("{}ms", duration.as_millis())
    } else if duration.as_secs() % 60 != 0 {
        format!("{}s", duration.as_secs())
    } else if duration.as_secs() % 3600 != 0 {
        format!("{}m", duration.as_secs() / 60)
    } else {
        format!("{}h", duration.as_secs() / 3600)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_duration() {
        assert_eq!(parse_duration("1ns").unwrap(), Duration::from_nanos(1));
        assert_eq!(parse_duration("1us").unwrap(), Duration::from_micros(1));
        assert_eq!(parse_duration("1ms").unwrap(), Duration::from_millis(1));
        assert_eq!(parse_duration("1s").unwrap(), Duration::from_secs(1));
        assert_eq!(parse_duration("1m").unwrap(), Duration::from_secs(60));
        assert_eq!(parse_duration("1h").unwrap(), Duration::from_secs(3600));
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(Duration::from_nanos(1)), "1ns");
        assert_eq!(format_duration(Duration::from_micros(1)), "1us");
        assert_eq!(format_duration(Duration::from_millis(1)), "1ms");
        assert_eq!(format_duration(Duration::from_secs(1)), "1s");
        assert_eq!(format_duration(Duration::from_secs(60)), "1m");
        assert_eq!(format_duration(Duration::from_secs(3600)), "1h");
    }
}
