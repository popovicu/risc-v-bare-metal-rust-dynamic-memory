# RISC-V Bare Metal Rust example (with dynamic memory allocation)

This example must be built with nightly `cargo`, and the target `riscv64gc-unknown-none-elf` must be installed to the nightly.

1. `rustup toolchain install nightly`
2. `rustup +nightly target add riscv64gc-unknown-none-elf`

Build with:

```
cargo +nightly build --bin risc-v-rust-bare-metal
```

Run on QEMU:

```
qemu-system-riscv64 -machine virt -bios target/riscv64gc-unknown-none-elf/debug/risc-v-rust-bare-metal -nographic
```

Sample output:

```
Hello, world!
Ticks: 1
Ticks: 2
Ticks: 3
Ticks: 4
Ticks: 5
Ticks: 6
Ticks: 7
Ticks: 8
Ticks: 9
Ticks: 10
Ticks: 11
Ticks: 12
Ticks: 13
Ticks: 14
Ticks: 15
Ticks: 16
Ticks: 17
Ticks: 18
```

## Problems with output?

I have personally only seen issues if using the dynamically allocated strings sometimes. The version as it is here works on my machine. *Perhaps* there is some problem with the `talc` library and we end up throwing bad bytes at UART and we throw QEMU off. When I remove any references to the dynamic strings, then things work well.

Therefore, if you see any issues with the UART output, try removing all the dynamic strings and maybe get rid of the custom allocator altogether and try again.

## Article

This code is meant to accompany the article at http://popovicu.com/posts/bare-metal-rust-risc-v-with-dynamic-memory