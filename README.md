# far memory

"download more ram" by seamlessly (almost) extending local memory with memory of remote nodes.

Documentation can be found on [nikitavbv.github.io/far-memory](https://nikitavbv.github.io/far-memory).

features:
- integration via a Rust library (see `client/client.rs` and `client/vec.rs` for client and see `demo/llm_inference.rs` for example of integration into real-world application).
- NBD-based block device. Can be used as ramdisk or as a swap device.
- replication and erasure coding.
- in-memory, ssd and network backends.
- background eviction thread with configurable policies.
