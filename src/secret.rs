use serde::Deserialize;

#[derive(Clone, Default)]
pub struct SecretConf(String);

impl SecretConf {
    pub fn new(s: &str) -> Self {
        SecretConf(s.to_string())
    }

    fn to_string(&self) -> String {
        format!("{}", "*".repeat(self.0.len()))
    }
}

impl<'de> Deserialize<'de> for SecretConf {
    fn deserialize<D>(deserializer: D) -> Result<SecretConf, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(SecretConf(s))
    }
}

impl std::fmt::Display for SecretConf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl std::fmt::Debug for SecretConf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret() {
        let s = SecretConf::new("secret");
        assert_eq!(s.to_string(), "*".repeat(6));
    }
}