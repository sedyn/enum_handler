use enum_handler::enum_handler;

#[derive(Debug)]
struct Variant1 {
    value: i32,
}

#[derive(Debug)]
struct Variant2 {}

#[derive(Debug)]
struct Variant3 {}

struct Executor;

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

fn main() {
    enum_handler! {
        CounterMessage,
        (Executor),
        Variant1,
        Variant2,
        Variant3,
    }

    let message = CounterMessage::from(Variant1 { value: 100 });
    let mut executor = Executor;
    message.execute(&mut executor);
}
