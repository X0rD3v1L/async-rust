## Async Rust

## Included Projects

1. [Block On with Futures](./futures_block_on/). Using `block_on` to start an async runtime.
2. [Block On with Tokio](./tokio_block_on/). Using `block_on` with options to start a Tokio runtime.
3. [Hello Tokio](./hello_tokio/). Start a runtime and `await` your first function.
4. [Tokio Joining](./tokio_join/). Using `join` to run multiple tasks concurrently. Switch between single-threaded and multi-threaded execution to see the difference in output.
5. [Tokio Spawn](./tokio_spawn/). Spawn tasks in the background, either detached or for access later.
6. [Tokio Yield](./tokio_yield/). Yield control from an async task, to ensure another one can run.
7. [Tokio Select](./tokio_select/). Select the first of several futures to return.
8. [Tokio Blocking](./tokio_blocking/). The right way and the wrong way to perform a blocking function in Tokio.
9. [Tokio Spawn Blocking](./tokio_spawn_blocking/). Spawn a blocking function in the background without tying up the Tokio runtime.
10. [Tokio MPSC Channels](./tokio_channels/). Create an async channel between tasks.

###### All the concepts learned and exercises implemented were based on content from Ardan Labs.