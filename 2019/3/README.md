# What I learned
It was quite hard assignment for a person who doesn't know Rust =)

* It's a first assignment where I started using `struct` and `tuple`
* Implemented `std::fmt::Display` for better debugging
* Learned how to deal with `None` value, e.g. for intersection method
    * We can use `Option<T>` and then unwrap it with `match` `Some(inner)` and `None`
    * We can use `Result<T, Err>` and return either `Ok(T)` or `Err(some_err)`
    * `?` is used for unwrapping successful values, but it throws an error (`panic`) if the value is `None` or `Err`
* Math problem wasn't too hard but a bit frustrating. In this assignment all intersections are created by perpendicular segments,
so intersection is `Point` with `x` value of vertical segment and `y` of horizontal one.