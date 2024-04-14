# enum_handler

Inspired by [enum_dispatch](https://gitlab.com/antonok/enum_dispatch)

```rust
enum_handler! {
    // #[boxed]
    // #[reference]
    enum CounterMessage {
        Variant1,
        Variant2,
        Variant3,
        // VariantN,
    },
    // (Type of executor, [Type of context])
    (Executor)
}
```

## Attributes

### `boxed`

The variant struct will be wrapped in a `Box`.

### `reference`

The `handle` method will be called with `message.as_ref()`

## Examples

- [simple](./examples/simple.rs)
- [task](./examples/task.rs)