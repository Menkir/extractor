# extractor [![Build Status](https://travis-ci.org/Menkir/bsys.svg?branch=master)](https://travis-ci.org/Menkir/bsys)
Experimental API to extract the general error title.

## Installation
Apply changes on your `Cargo.toml`
```` toml
[dependencies]
extractor = "0.1.0"
````

## Example
```
error[E0382]: use of moved value: 'v'
--> examples/fail.rs:4:29
|
3 |     let v2 = v;
|         -- value moved here
4 |     println!("v[0] is: {}\", v[0]);
  |                             ^ value used here after move
|
= note: move occurs because `v` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait"
```

Based on this compiler error the general error title would be e.g `use of moved value`.
The purpose of this crate is to make the use of [resa](https://github.com/Menkir/resa) more easier by outsourcing different semantic code.

Because a search for `error[E0382]: use of moved value: 'v'` would lead to empty results, we need to extract the general issue of this.