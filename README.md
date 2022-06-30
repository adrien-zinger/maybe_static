# Maybe static

Initialize in a lazy way a static variable with parameters.

```rust
use maybe_static::maybe_static;

pub fn get_ip2(opt: Option<(&str, &str)>) -> Result<&'static String, &'static str> {
    maybe_static!(opt, String, |(addr, port)| format!("{addr}:{port}"))
}

fn main() {
    println!("{}", get_ip2(Some(("hello", "world"))).unwrap());
    println!("{}", get_ip2(None).unwrap());
}
```

<small> Initially developed around the article <a href="https://www.maybeuninit.com/2022/04/07/lazy-or-not-lazy.html">in the maybeuninit blog</a></small>
