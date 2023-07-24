# Description

Add macro `Serde` to [bincode](https://github.com/bincode-org/bincode) to make it easier to use.

# Install

```
cargo add bincode_macro
```

Or add dependencies.
```
[dependencies]
bincode = { version = "=2.0.0-rc.3" }
bincode_macro = "0.1"
```

# Usage

```
use bincode::{error, Decode, Encode};
use bincode_macro::Serde;

#[derive(Serde, Encode, Decode, PartialEq, Debug)]
pub struct Entity {
    pub x: u16,
    pub y: u32,
}

fn main() {
    let mut entity = Entity { x: 1, y: 4 };

    let encoded: Vec<u8> = entity.pack().unwrap();
    println!("{:?} {}", encoded, encoded.len());

    let (decoded, len): (Entity, usize) = entity.unpack(&encoded).unwrap();
    println!("{:?}, {}\n", decoded, len);
}
```
