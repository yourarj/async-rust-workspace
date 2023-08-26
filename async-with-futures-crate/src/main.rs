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

    // we can also use .await on any type that implements Future trait
    // PROVIDED the method where are awaiting for compeletion of future
    // must be async too.
    //
    // Unlike block_on(), .await will not block current thread.
    // Instead it'll wait asynchronously waits for completion.
    // This approach allows other tasks to run if current task is not able to progress
    // e.g. as in following example
    watch_favourite_show().await;
    have_favourite_snacks().await;

    // NOTE watching and having snack is synchronous
    // untill you complete watching 🎥 the show you can't have snacks 🥲

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
