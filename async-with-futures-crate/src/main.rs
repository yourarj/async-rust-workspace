use futures::executor::block_on;

fn main() {
    println!("Hello, world!");

    // the following line will not do anything
    // async convert a block code into State Machine which implements Future trait.
    // Here is fut is a Future
    // To happen anything this future needs to be run on an executor
    let fut = hello_async_world();

    // blocks_on from futures crate blocks on current thread unitl the
    // given future complete it's excution
    let val = block_on(fut);

    println!("Value returned from the future is {:01}", val);
}

async fn hello_async_world() -> i32 {
    println!("Hello ASYNC world");
    // return my fancy integer
    32
}
