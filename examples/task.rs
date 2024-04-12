use enum_handler::enum_handler;
use std::{
    sync::{
        mpsc::{self, SyncSender},
        Arc,
    },
    thread,
};

struct Executor;

struct Print {
    message: String,
}
struct Exit;

struct Context {
    shutdown_signal: SyncSender<()>,
}

enum_handler! {
    TaskMessage,
    (Executor, Arc<Context>),
    Print,
    Exit,
}

trait Handler<M> {
    fn execute(&mut self, msg: M, context: Arc<Context>);
}

impl Handler<Print> for Executor {
    fn execute(&mut self, msg: Print, _: Arc<Context>) {
        println!("{}", msg.message);
    }
}

impl Handler<Exit> for Executor {
    fn execute(&mut self, _: Exit, context: Arc<Context>) {
        println!("exit message received");
        context.shutdown_signal.send(()).unwrap();
    }
}

fn main() {
    let (tx, rx) = mpsc::channel::<TaskMessage>();
    let mut executor = Executor;

    let (shutdown_signal, shutdown_rx) = mpsc::sync_channel(1);

    let context = Arc::new(Context { shutdown_signal });

    thread::spawn(move || loop {
        let task = rx.recv().unwrap();
        task.execute(&mut executor, Arc::clone(&context));
    });

    tx.send(TaskMessage::from(Print {
        message: String::from("Hello, world!"),
    }))
    .unwrap();

    tx.send(TaskMessage::from(Exit)).unwrap();

    shutdown_rx.recv().unwrap();
    println!("shutdown");
}
