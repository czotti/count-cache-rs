# Count Cache
 [![Latest Version](http://meritbadge.herokuapp.com/count_cache)](https://crates.io/crates/count_cache) [![Build Status](https://travis-ci.org/czotti/count-cache-rs.svg?branch=master)](https://travis-ci.org/czotti/count-cache-rs) [![dependency status](https://deps.rs/repo/github/czotti/count-cache-rs/status.svg)](https://deps.rs/repo/github/czotti/count-cache-rs)

This create provide a simple way to use a counted cache system.
When the count drop to zero, the cache delete the associated key, and the elements cannot be accessed anymore.


Here is an exemple:
```rust
use count_cache::CountCache;

fn main() {
    let ccache = CountCache::new();

    ccache.insert("test", 10.256, 2);
    assert_eq!(ccache.get(&"test").expect("Err"), 10.256);
    assert_eq!(ccache.get(&"test").expect("Err"), 10.256);
    assert!(ccache.get(&"test").is_err());
    assert!(ccache.get(&"test").is_err());
}
```
