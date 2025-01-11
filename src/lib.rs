/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/swamp-render
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */

/*!
# future-runner

A lightweight crate for executing Rust futures on different target platforms.

## Features

- Cross-platform future execution support (native and WASM)
- Configurable blocking/non-blocking execution via features
- Simple API with a single `run_future` function

## Usage

```rust
use future_runner::run_future;

async fn my_async_task() {
    // Your async code here
}

fn main() {
    run_future(my_async_task());
}
```
*/

#[cfg(all(not(target_arch = "wasm32"), feature = "threaded"))]
pub fn run_future<F: std::future::Future<Output = ()> + Send + 'static>(future: F) {
    use std::thread;

    thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed to create runtime");

        rt.block_on(future);
    })
    .join()
    .expect("Thread panicked");
}

#[cfg(all(not(target_arch = "wasm32"), not(feature = "threaded")))]
pub fn run_future<F: std::future::Future<Output = ()> + 'static>(future: F) {
    // Existing implementation stays the same
    #[cfg(feature = "block")]
    futures::executor::block_on(future);

    #[cfg(not(feature = "block"))]
    {
        let mut local_pool = futures::executor::LocalPool::new();
        use futures::task::LocalSpawnExt;
        let spawner = local_pool.spawner();
        spawner.spawn_local(future).expect("Failed to spawn future");
        local_pool.run();
    }
}

/*
#[cfg(target_arch = "wasm32")]
pub fn run_future<Fut>(future: Fut)
where
    Fut: std::future::Future<Output = ()> + 'static,
{
    use wasm_bindgen_futures::spawn_local;

    spawn_local(async move {
        info!("Starting future...");

        // Await the provided future
        future.await;

        info!("Future COMPLETED!");
    });
}
*/

#[cfg(target_arch = "wasm32")]
pub fn run_future<Fut>(future: Fut)
where
    Fut: std::future::Future<Output = ()> + 'static,
{
    wasm_bindgen_futures::spawn_local(future);
}
