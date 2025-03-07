mod settings;

use serde::Deserialize;
use serde_json;


fn main() {
    let settings = settings::MySettings::new("settings.toml").unwrap();
    println!("Settings:\n{}", serde_json::to_string_pretty(&settings).unwrap());
    println!("Duration: {:?}", settings.duration);
    println!("ByteSize: {:?} ki", settings.size_of_data.as_kibibytes());
}
