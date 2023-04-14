# LetHimCook-rs

![hollup... Let him cook](https://i.kym-cdn.com/entries/icons/original/000/041/943/1aa1blank.png)

## Intro

LetHimCook-rs is a simple library for spawning asynchronous looping threads that accept requests and return feedback. It is made for the convenience for my other projects. Also, this is probably a named design pattern but I don't know about it so Imma call it cooking.

## Concepts

### Order

Command for requesting a thread

### Serve / Dish

Feedback from a request

### Cook

Thread that handle requests and create feedback

### Waiter

Handle for sending request and returning feedback
## Syntax
```
cook!( { <init> } , | <order name> | <recipe> ); // Correct
cook!( | <order name> | <serve expression> );    // Correct
cook!( <function with single arguement> );       // Wrong
```
- init : initiate the `Cook`
- order name : the name of the passed in order, used to reference in <recipe>
- recipe : how the `Dish` is created

The `cook!` macro will return a `Waiter` to send `Order` and recive `Dish`.

## Examples
- Asynchronous counter
```rust
// types of Order
enum CounterOrder {
    Inc,
    Get,
}

let waiter: Waiter<CounterOrder, Option<i32>> = 
cook!(
  {
    let mut x = 1; // initiate the Cook
  },
  |order| match order { // a match expression on how the Cook handle different Order
    CounterOrder::Inc => {
      x += 1;
      None
    },
    CounterOrder::Get => Some(x),
  }
);

waiter.order(CounterOrder::Inc);
assert_eq!(Err(TryRecvError::Empty), waiter.try_serve()); // the Dish will not be finished immediately

thread::sleep(Duration::from_secs(1));
assert_eq!(Ok(None), waiter.try_serve()); // the result of CounterOrder::Inc

waiter.order(CounterOrder::Get);
thread::sleep(Duration::from_secs(1));
assert_eq!(Ok(Some(2)), waiter.try_serve());

```

- Logger
```rust
let logger = cook!(|x| println!("{}", x));
logger.order(1);
thread::sleep(Duration::from_secs(1)); // if the main thread dont sleep, the main thread would ended before the log is printed
```
```rust
let logger = cook!(|x| println!("{}", x));
logger.order(1); // Ok
logger.order("123"); // Compile Error, because rust infered `logger` as `Waiter<i32,()>`, which cause a type error
```
- Finish Cooking
```rust
enum Command {
  Foo,
  Exit,
}
let waiter = cook!(|x| match x {
  Command::Foo => "foo",
  Command::Exit => break, // use break to finish cooking
});
waiter.order(Command::Foo);
thread::sleep(Duration::from_secs(1));
assert_eq!(Ok("foo"), waiter.try_serve());

waiter.order(Command::Exit);
thread::sleep(Duration::from_secs(1));
// `TryRecvError::Disconnected` will be returned if called after finish
assert_eq!(Err(TryRecvError::Disconnected), waiter.try_serve());
```
