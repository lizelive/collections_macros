some macros for dicts

creates `hashmap`, `hashset`, `btreeset`, `btreemap`

map has two sytaxts



```rust
let a = 1;
let b = 2;
hashmap![
    a: a+b,
    b: b
],
```
is equal to
```rust
std::collections::HashMap::from([(a, a), (b, a + b)])
```

but if you dont include commas it acts like yml and gets names from ident
```rust
hashmap![
    a: a+b
    b: b
],
```
is equal to
```rust
std::collections::HashMap::from([("a", a + b), ("b", b)])
```
