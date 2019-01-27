# diffable-rs
A new Diffable trait that helps compare to instances of the same struct.

## Usage

Add the package as a dependency in your `Cargo.toml`
```toml
[dependencies]
diffable = "0.1.0"
```

Find a struct that you want to make `Diffable` and derive it:
```rust
use diffable::Diffable;
#[derive(Diffable)]
struct Test {
    an_int: i32,
    a_bool: bool,
}
```

An then diff it against another instance:
```rust
fn main() {
    let test1 = Test {
        an_int: 24,
        a_bool: false,
    };
    let test2 = Test {
        an_int: 23,
        a_bool: true,
    };
    let diff = test1.diff(&test2);
    println!("{:?}", diff);
    // Output:
    // TestDiff { an_int: Some((24, 23)), a_bool: Some((false, true)) }
}
```
