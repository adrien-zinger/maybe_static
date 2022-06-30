# Maybe static

Initialize in a lazy way a static variable with parameters.

Example of a simple usage:

```rust
use maybe_static::maybe_static;

pub fn get_lazy(opt: Option<(&str, &str)>) -> Result<&'static String, &'static str> {
    maybe_static!(opt, String, |(a, b)| format!("{a}:{b}"))
}

fn main() {
    println!("{}", get_lazy(Some(("hello", "world"))).unwrap());
    println!("{}", get_lazy(None).unwrap());
}
```

The *macro* will create a local static variable that is initialized once, ONLY ONCE. It's
also totally thread safe.

```rust
fn main() {
    println!("{}", get_lazy(Some(("hello", "world"))).unwrap());
    // print hello:world (initialize)
    println!("{}", get_lazy(None).unwrap());
    // print hello:world
    println!("{}", get_lazy(Some(("foo", "bar"))).unwrap());
    // still print hello:world
}
```

Require a `Some` for the first initialization.

```rust
fn main() {
    println!("{}", get_lazy(None).unwrap()); // Error!
}
```

Create a new unique value by scope.

```rust
fn main() {
    let a = maybe_static!(Some(("hello", "world")), String, |(a, b)| format!(
        "{a}:{b}"
    ))
    .unwrap();
    let b = maybe_static!(Some(("foo", "bar")), String, |(a, b)| format!(
        "{a}:{b}"
    ))
    .unwrap();
    println!("{a}\n{b}")
    // hello:world
    // foo:bar


    for i in 0..3 {
        print!("{}", maybe_static!(Some(i), usize, |i| i).unwrap());
    }
    // 000
}
```

<small> Initially developed around the article <a href="https://www.maybeuninit.com/2022/04/07/lazy-or-not-lazy.html">in the maybeuninit blog</a></small>
