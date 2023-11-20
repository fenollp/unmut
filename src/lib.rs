/// `unmut!` drops a value's `mut`. It let's you mark a point in the flow when
/// an identifier mustn't be mutable anymore.
/// See [`drop`](https://doc.rust-lang.org/std/mem/fn.drop.html) to actually drop the value.
///
/// See also [`mute!`]
///
/// ```rust
/// use unmut::unmut;
///
/// let mut x = 42;
/// x *= 2;
/// unmut!(x);
/// // ...
/// ```
/// See `x` is now immutable:
/// ```rust,compile_fail
/// // ...
///# use unmut::unmut;
///# let mut x = 42;
///# x *= 2;
///# unmut!(x);
/// x *= 2;
/// // This fails to compile with:
///# let _ = r#"
/// error[E0384]: cannot assign twice to immutable variable `x`
///  --> src/lib.rs:23:1
///   |
/// 8 | unmut!(x);
///   | ---------
///   | |
///   | first assignment to `x`
///   | help: consider making this binding mutable: `mut x`
/// 9 | x *= 2;
///   | ^^^^^^ cannot assign twice to immutable variable
///
///# "#;
/// ```
#[macro_export]
macro_rules! unmut {
    ($name:ident) => {
        let $name = $name;
    };
}

/// `mute!` drops a value's `mut`.
///
/// See also [`unmut!`]
///
/// Note however that this compiles:
/// ```rust
/// use unmut::mute;
///
/// let x = 42; // not mut
/// mute!(x);   // no complaints
/// ```
#[macro_export]
macro_rules! mute {
    ($name:ident) => {
        let $name = $name;
    };
}
