# Runtime Extension Wrapper | Rust

This library is a runtime extension wrapper for Rust extensions, built for the Blockless Network

[Blockless](https://blockless.network) module starter repository.

## Usage

```rust
use runtime_extension_rust::CGIExtension;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let mut ext = CGIExtension::new(
        String::from("example"),
        String::from("ex"),
        String::from("An example CGI extension"),
    );

    fn sample_method(params: Vec<String>) -> Result<String, String> {
        Ok(format!("Sample method called with args: {:?}", params))
    }

    ext.export(String::from("sample_method"), async_method);

    ext.execute().await;

    Ok(())
}
```
