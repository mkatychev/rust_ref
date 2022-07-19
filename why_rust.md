---
marp: true
---

# Why Rust

---

* Compiled (Rust) vs. Interpreted languages (Python)
* Strong Typing (Rust) vs. Weak Typing (Python)
* [The borrow checker (how does Rust enforce AXM)](https://blog.logrocket.com/introducing-the-rust-borrow-checker/)
* [`null`: The Billion Dollar Mistake](https://www.infoq.com/presentations/Null-References-The-Billion-Dollar-Mistake-Tony-Hoare/)
* Object inheritance (Python) vs. Compositional inheritance (Rust)

---

## Strengths

- Compiled code [about same performance](https://benchmarksgame-team.pages.debian.net/benchmarksgame/box-plot-summary-charts.html) as C / C++, and excellent [memory and energy efficiency](https://dl.acm.org/doi/10.1145/3136014.3136031).
- Can [avoid 70% of all safety issues](https://www.chromium.org/Home/chromium-security/memory-safety) present in C / C++, and most memory issues.
- Strong type system prevents [data races](https://doc.rust-lang.org/nomicon/races.html), brings ['fearless concurrency'](https://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html) (amongst others).
- Seamless C interop, and [dozens of supported platforms](https://doc.rust-lang.org/rustc/platform-support.html) (based on LLVM).
- ["Most loved language"](https://survey.stackoverflow.co/2022/#section-most-loved-dreaded-and-wanted-programming-scripting-and-markup-languages
  ) for ~~4~~ ~~5~~ ~~6~~ 7 years in a row. 🤷‍♀️
- Modern tooling: `cargo` (builds _just work_), `clippy` (550+ code quality lints), `rustup` (easy toolchain mgmt).


---

## Weaknesses

- Steep learning curve; compiler enforcing (esp. memory) rules that would be "best practices" elsewhere.
- Missing Rust-native libs in some domains, target platforms (esp. embedded), IDE features.
- Longer compile times than "similar" code in other languages.
- No formal language specification, can prevent legal use in some domains (aviation, medical, …).
- Careless (use of `unsafe` in) libraries can secretly break safety guarantees.

---

## XOR

[**eXclusive OR**] also known as _exclusive disjunction_ is a logical operation that is `true`
if and _only_ if its arguments differ (one is true, the other is false).

![width:200px](https://upload.wikimedia.org/wikipedia/commons/thumb/4/46/Venn0110.svg/300px-Venn0110.svg.png)

---

[aliasing XOR mutability](http://plv.mpi-sws.org/rustbelt/ghostcell/paper.pdf)

>  Rust’s type system enforces the discipline of aliasing _XOR_ mutability
>  (AXM, for short): a value of type `T` may either have *multiple aliases*
>  (called shared references), of type `&T`, or it may be mutated via a
>  *unique, mutable reference*, of type `&mut T`, but it *may not be both
>  aliased and mutable at the same time*

---

## Null

![](https://quotefancy.com/media/wallpaper/3840x2160/2234305-Edsger-W-Dijkstra-Quote-The-computing-scientist-s-main-challenge.jpg)

---

## Null contd.

In computing, a null pointer or null reference is a value saved for indicating
that the pointer or reference does not refer to a valid object. Programs
routinely use null pointers to represent conditions such as the end of a list
of unknown length or the failure to perform some action; this use of null
pointers can be compared to nullable types and to the Nothing value in an
option type.

---

![](https://static.javatpoint.com/tutorial/rust/images/rust-generics.png)

[`Option`](https://doc.rust-lang.org/std/option/enum.Option.html)

```rust
pub enum Option<T> {
    None,
    Some(T),
}
```

---
