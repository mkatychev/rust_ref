# Lifetimes

[Borrowing](https://doc.rust-lang.org/stable/book/ch10-03-lifetime-syntax.html)

```rust
    {
        let r;                // ---------+-- 'a
                              //          |
        {                     //          |
            let x = 5;        // -+-- 'b  |
            r = &x;           //  |       |
        }                     // -+       |
                              //          |
        println!("r: {}", r); //          |
    }                         // ---------+

```

[Elision](https://doc.rust-lang.org/rust-by-example/scope/lifetime/elision.html)

```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

[Mutability](https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html)

```rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

[Exclusive Mutability](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references)

```rust
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
```

#### Listing 10-18: Annotations of the lifetimes of r and x, named 'a and 'b, respectively

# Resources
[Lifetimes](https://doc.rust-lang.org/stable/book/ch10-03-lifetime-syntax.html)
[Cargo Expand](https://github.com/dtolnay/cargo-expand)
