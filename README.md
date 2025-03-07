# Config types for rust

This is a collection of config types for Rust.
It is intended to be used with 
- [config](https://crates.io/crates/config)
- [figment](https://crates.io/crates/figment)
- or any other configuration library that supports `serde`.

## Usage
See [example](/example) for usage.

### DurationConf
`DurationConf` is a wrapper around `std::time::Duration` that allows you to specify duration in human readable format in your config.

For example, you can specify duration in your config file like this:
```toml
duration = "5m"
```

Supported suffixes(case insensitive):
- `ns` - nanoseconds
- `us` - microseconds
- `ms` - milliseconds
- `s` - seconds
- `m` - minutes
- `h` - hours

### ByteSizeConf
`ByteSizeConf` allows you to specify byte size in your config.

For example, if you want to specify max file size in your config file, you can do it like this:
```toml
max_file_size = "10Mi"
```

Supported suffixes(case insensitive):
- `B` - bytes
- `Ki` - kibibytes
- `Kb` - kilobytes
- `Mi` - mebibytes
- `Mb` - megabytes
- `Gi` - gibibytes
- `Gb` - gigabytes
- `Ti` - tebibytes
- `Tb` - terabytes