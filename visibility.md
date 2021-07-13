# VISIBILITY

By default, everything in Rust is *private*, with two exceptions: Associated
items in a `pub` Trait are public by default; Enum variants
in a `pub` enum are also public by default. When an item is declared as `pub`,
it can be thought of as being accessible to the outside world. For example:

```rust
# fn main() {}
// Declare a private struct
struct Foo;

// Declare a public struct with a private field
pub struct Bar {
    field: i32,
}

// Declare a public enum with two public variants
pub enum State {
    PubliclyAccessibleState,
    PubliclyAccessibleState2,
}
```

With the notion of an item being either public or private, Rust allows item
accesses in two cases:

1. If an item is public, then it can be accessed externally from some module
   `m` if you can access all the item's ancestor modules from `m`. You can
   also potentially be able to name the item through re-exports. See below.
2. If an item is private, it may be accessed by the current module and its
   descendants.

## `pub(in path)`, `pub(crate)`, `pub(super)`, and `pub(self)`

In addition to public and private, Rust allows users to declare an item as
visible only within a given scope. The rules for `pub` restrictions are as
follows:
- `pub(in path)` makes an item visible within the provided `path`.
   `pub(in path)` must start with `crate`, `self`, or `super`.
   `pub(in path)` usually follows these patterns:
   * `pub(in crate::other_mod)`
   * `pub(in super::other_mod)`
- `pub(crate)` makes an item visible within the current crate. This is equivalent
  to `pub(in crate)`.
- `pub(super)` makes an item visible to the parent module. This is equivalent
  to `pub(in super)`.
- `pub(self)` makes an item visible to the current module. This is equivalent
to `pub(in self)` or not using `pub` at all.


Here's an example:

```rust
pub mod outer_mod {
    pub mod inner_mod {
        // This function is visible within `outer_mod`
        pub(in crate::outer_mod) fn outer_mod_visible_fn() {}
        // Same as above, this is only valid in the 2015 edition.
        pub(in outer_mod) fn outer_mod_visible_fn_2015() {}

        // This function is visible to the entire crate
        pub(crate) fn crate_visible_fn() {}

        // This function is visible within `outer_mod`
        pub(super) fn super_mod_visible_fn() {
            // This function is visible since we're in the same `mod`
            inner_mod_visible_fn();
        }

        // This function is visible only within `inner_mod`,
        // which is the same as leaving it private.
        pub(self) fn inner_mod_visible_fn() {}
    }
    pub fn foo() {
        inner_mod::outer_mod_visible_fn();
        inner_mod::crate_visible_fn();
        inner_mod::super_mod_visible_fn();

        // This function is no longer visible since we're outside of `inner_mod`
        // Error! `inner_mod_visible_fn` is private
        //inner_mod::inner_mod_visible_fn();
    }
}

fn bar() {
    // This function is still visible since we're in the same crate
    outer_mod::inner_mod::crate_visible_fn();

    // This function is no longer visible since we're outside of `outer_mod`
    // Error! `super_mod_visible_fn` is private
    //outer_mod::inner_mod::super_mod_visible_fn();

    // This function is no longer visible since we're outside of `outer_mod`
    // Error! `outer_mod_visible_fn` is private
    //outer_mod::inner_mod::outer_mod_visible_fn();

    outer_mod::foo();
}

fn main() { bar() }
```

https://doc.rust-lang.org/reference/visibility-and-privacy.html
