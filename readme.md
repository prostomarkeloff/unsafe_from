Unsafe versions of standard library `From<T>` and `Into<T>`.

# Install

```toml
[dependencies]
unsafe_from = "1.0"
```

# Use 

```rust
use unsafe_from::UnsafeFrom;

struct MyUnsafeType(i32);

unsafe impl UnsafeFrom<i32> for MyUnsafeType {
    unsafe fn unsafe_from(t: i32) -> MyUnsafeType {
        MyUnsafeType(t)
    }
}
```