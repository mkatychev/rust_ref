# Attribute Tags
```rust
struct RustStruct {
    #[json="premium_id,omitempty"]
    premium_id String,
}
```

```go
type GoStruct struct {
	PremiumID     string `json:"premium_id,omitempty"`
}
```



# Macros
[Standard Macros/Macros By Example](https://doc.rust-lang.org/reference/macros-by-example.html)
> allows users to define syntax extension in a declarative way. We call such extensions "macros by example" or simply "macros".
[Procedural Macros](https://doc.rust-lang.org/reference/procedural-macros.html#procedural-macros):
> Procedural macros allow you to run code at compile time that operates over Rust syntax, both consuming and producing Rust syntax. You can sort of think of procedural macros as functions from an AST to another AST.
* Function-like macros - `custom!(...)`
* Derive macros - `#[derive(CustomDerive)]`
* Attribute macros - `#[CustomAttribute]`

# More

[Little Book of Rust Macros](https://veykril.github.io/tlborm/)
[Cargo Expand](https://github.com/dtolnay/cargo-expand)
