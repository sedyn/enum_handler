# enum_handler

Inspired by [enum_dispatch](https://gitlab.com/antonok/enum_dispatch)

```rust
enum_handler! {
    CounterMessage,
    // type of handler, [type of context] 
    (Executor),
    Variant1,
    Variant2,
    Variant3,
    // VariantN,
}
```

will be expanded as

```rust
enum CounterMessage {
    Variant1(Variant1),
    Variant2(Variant2),
    Variant3(Variant3),
}
impl CounterMessage {
    pub fn execute(self, executor: &mut Executor) {
        match self {
            CounterMessage::Variant1(message) => executor.execute(message),
            CounterMessage::Variant2(message) => executor.execute(message),
            CounterMessage::Variant3(message) => executor.execute(message),
        }
    }
}
impl From<Variant1> for CounterMessage {
    fn from(value: Variant1) -> Self {
        Self::Variant1(value)
    }
}
impl From<Variant2> for CounterMessage {
    fn from(value: Variant2) -> Self {
        Self::Variant2(value)
    }
}
impl From<Variant3> for CounterMessage {
    fn from(value: Variant3) -> Self {
        Self::Variant3(value)
    }
}
```

The execute method of CounterMessage requires Handler trait.

```rust
trait Handler<M>: Sized {
    fn execute(&mut self, msg: M);
}

impl Handler<Variant1> for Executor {
    fn execute(&mut self, msg: Variant1) {
        println!("{msg:?}")
    }
}

impl Handler<Variant2> for Executor {
    fn execute(&mut self, msg: Variant2) {
        println!("{msg:?}")
    }
}

impl Handler<Variant3> for Executor {
    fn execute(&mut self, msg: Variant3) {
        println!("{msg:?}")
    }
}
```

## Examples

- [simple](./examples/simple.rs)
- [task](./examples/task.rs)