use std::sync::mpsc::{Receiver, SendError, Sender, TryIter, TryRecvError};

pub struct Waiter<O, D> {
    order: Sender<O>,
    serve: Receiver<D>,
}

impl<O, D> Waiter<O, D> {
    pub fn new(order: Sender<O>, serve: Receiver<D>) -> Self {
        Waiter { order, serve }
    }
    pub fn order(&self, order: O) {
        self.try_order(order).unwrap();
    }
    pub fn try_order(&self, order: O) -> Result<(), SendError<O>> {
        self.order.send(order)
    }
    pub fn serve(&self) -> D {
        self.try_serve().unwrap()
    }
    pub fn try_serve(&self) -> Result<D, TryRecvError> {
        self.serve.try_recv()
    }
    pub fn try_iter(&self) -> TryIter<'_, D> {
        self.serve.try_iter()
    }
}
#[macro_export]
macro_rules! cook {
    ( { $($init:stmt ;)* }, | $name:ident | $($recipe:tt)* ) => {
        {
            use std::sync::mpsc;
            use std::thread;
            use crate::cook::Waiter;
            let (order_sender, order_receiver) = mpsc::channel();
            let (dish_sender, dish_receiver) = mpsc::channel();
            thread::spawn(move || {
                $($init)*
                loop {
                    if let Ok($name) = order_receiver.try_recv() {
                        let _ = dish_sender.send($($recipe)*);
                    };
                }
            });
            Waiter::new(order_sender, dish_receiver)
        }
    };

    ( | $name:ident | $($recipe:tt)* ) => {
        {
            use std::sync::mpsc;
            use std::thread;
            use crate::cook::Waiter;
            let (order_sender, order_receiver) = mpsc::channel();
            let (dish_sender, dish_receiver) = mpsc::channel();
            thread::spawn(move || {
                loop {
                    if let Ok($name) = order_receiver.try_recv() {
                        let _ = dish_sender.send($($recipe)*);
                    };
                }
            });
            Waiter::new(order_sender, dish_receiver)
        }
    };
}
