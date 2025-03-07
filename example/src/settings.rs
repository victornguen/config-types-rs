use config::{Config, ConfigBuilder};
use serde::{Deserialize, Serialize};
use config_types::{ByteSizeConf, DurationConf, SecretConf};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct MySettings {
    pub duration: DurationConf,
    pub size_of_data: ByteSizeConf,
    pub secret: SecretConf,
}

impl MySettings {
    pub fn new(location: &str) -> anyhow::Result<Self> {
        let mut builder = Config::builder()
            .add_source(config::File::with_name(location))
            .build()?;
        let settings = builder.try_deserialize()?;
        Ok(settings)
    }
}