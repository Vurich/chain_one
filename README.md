# `chain_one`

Annoyed that you `[T; N]` doesn't implement `IntoIterator<Item = T>`? Annoyed that `Chain` doesn't implement `ExactSizeIterator`? Do I have a project for you!

```rust
assert_eq!(iter![1, 2, 3, 4, 5].collect::<Vec<_>>(), vec![1, 2, 3, 4, 5]);
assert_eq!(iter![1, 2, 3, 4, 5].rev().collect::<Vec<_>>(), vec![5, 4, 3, 2, 1]);
assert_eq!((0..5).chain_one(0).collect::<Vec<_>>(), vec![0, 1, 2, 3, 4, 0]);
```
