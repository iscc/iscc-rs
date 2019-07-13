# iscc-rs
Rust implementation of the [ISCC specification](https://iscc.codes)

### Documentation
* https://iscc.codes
* https://docs.rs/iscc-rs

### Usage
Add this to your `Cargo.toml`:

```toml
[dependencies]
iscc-rs = "0.1"
```

### Example
This example shows how to create an ISCC Code.
```rust
use std::error::Error;
use std::process;

use iscc_rs::{content_id_text, data_id, instance_id, meta_id};

fn example() -> Result<(), Box<dyn Error>> {
    // Generate ISCC Component Codes
    let (mid, _title, _extra) = meta_id("Title of Content", "");
    let cid = content_id_text("some text", false);
    let did = data_id("tests/test_data/mediafile.html")?;
    let (iid, _tophash) = instance_id("tests/test_data/mediafile.html")?;

    // Join ISCC Components to fully qualified ISCC Code
    let iscc_code = [mid, cid, did, iid].join("-");
    println!("ISCC: {}", iscc_code);

    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("Error running example: {}", err);
        process::exit(1);
    }
}
```
