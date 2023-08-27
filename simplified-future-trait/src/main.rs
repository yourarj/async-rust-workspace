/// Simplified version of actual Future trait
trait SimpleFutureTrait {
    /// represents value that the future will return
    type Output;

    /// method to compute and retrieve futures output
    /// A future can only advance by calling poll only
    /// it's inert otherwise
    ///
    /// 🤔 What is wake: fn()
    /// When we poll a future, if computation compeletes Poll::Ready(RETURN_TYPE_VAL)
    /// But this is not the case all the time
    /// In other cases it return POLL::Pending and make arrangements for wake to get invoked
    /// when future can make progress
    /// When wake is Called EXECUTOR will again call poll and make progress
    /// The iteration repeats till future completes it's exectuion.  
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

/// Represents state of future computation
pub enum Poll<T> {
    /// Future completed computation and result is READY
    Ready(T),
    /// Future is under progress
    Pending,
}

fn main() {
    println!("Hello, world!");
}
