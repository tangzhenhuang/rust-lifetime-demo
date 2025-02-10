mod missing_lifetime;
mod dangling_ref;
mod recursive_ref;
mod async_ref;
mod lock_lifetime_in_async;
mod closure_lifetime;
mod self_ref;

#[tokio::main]
async fn main() {
    //do nothing
}