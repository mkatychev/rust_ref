# Errors

`Err` vs `Error`
`Result` vs `Option`

[`Result`](https://doc.rust-lang.org/std/result/):
* `Result<T, E>` is the type used for returning and propagating errors:
    ```rust
    pub enum Result<T, E> {
        Ok(T),
        Err(E),
    }
* It is an enum with the variants:
    - `Ok(T)`, representing success and containing a value
    - `Err(E)`, representing error and containing an error value

[`Option`](https://doc.rust-lang.org/std/option/):
* Type `Option` represents an optional value.
    ```rust
    pub enum Option<T> {
        None,
        Some(T),
    }
* `Option` is either `Some(T)` or `None`
* They have a number of uses
    
    - Initial values
    - Return values for functions that are not defined over their entire input range (partial functions)
    - Return value for otherwise reporting simple errors, where None is returned on error
    - Optional struct fields
    - Struct fields that can be loaned or "taken"
    - Optional function arguments
    - Nullable pointers
    - Swapping things out of difficult situations


[`trait`](https://doc.rust-lang.org/std/keyword.trait.html):
* A trait is like an interface that data types can implement. When a type implements a trait it can be treated abstractly as that trait using generics or trait objects.
* Traits can serve as markers or carry other logical semantics that aren't expressed through their items.

[`dyn`](https://doc.rust-lang.org/std/keyword.dyn.html):
1. Unlike generic parameters or `impl Trait`, the compiler does not know the concrete type that is being passed. That is, the type has been erased.
1. As such, a `dyn Trait` reference contains two pointers.
1. One pointer goes to the data (e.g., an instance of a struct).
1. Another pointer goes to a map of method call names to function pointers (known as a virtual method table or vtable).


[`dyn SomeTrait` vs. `trait SomeTrait`](https://doc.rust-lang.org/book/ch17-02-trait-objects.html)

https://doc.rust-lang.org/std/prelude/index.html
https://rosettacode.org/wiki/Enumerations#Rust
