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

    // naive approach could be as following
    // but this would be still synchronous
    // We will be doing synchronously hello -> watching -> snacking
    // Is there any other way to invoke future?
    block_on(watch_favourite_show());
    block_on(have_favourite_snacks());

    println!("Value returned from the future is {:01}", val);
}

async fn hello_async_world() -> i32 {
    println!("Hello ASYNC world");
    // return my fancy integer
    32
}

// Watch your favourite show
async fn watch_favourite_show() {
    println!("I'm watching RAMAYANA");
}

// Have your favourite snacks
async fn have_favourite_snacks() {
    println!("I'm eating BANANA CHIPS");
}
