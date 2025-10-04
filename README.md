# RISC-V Bare Metal Rust example (with dynamic memory allocation)

The target `riscv64imac-unknown-none-elf` must be installed:
```
rustup target add riscv64imac-unknown-none-elf
```

Build with:
(Note, it doesn't seem to work for debug mode)
```
cargo build -r
```

Run on QEMU:

```
qemu-system-riscv64 -machine virt -bios target/riscv64imac-unknown-none-elf/release/risc-v-rust-bare-metal -nographic
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
QEMU: Terminated
```
(Note, you have to manually kill QEMU, usually with Ctrl-a x)

Or using [Simmerv](https://github.com/tommythorn/simmerv):
```
sim -c -t -n target/riscv64imac-unknown-none-elf/release/risc-v-rust-bare-metal|head
    1 3         80000000     3117 auipc   sp, 3000                                      80003000
    2 3         80000004 e0010113 addi    sp, sp:80003000, fffffffffffffe00             80002e00
    3 3         80000008   6000ef jal     ra, 8000000e                                  8000000c
    4 3         8000000e     7171 addi    sp, sp:80002e00, ffffffffffffff50             80002d50
    5 3         80000010     f506 sd      ra:8000000c, a8(sp:80002d50)
    6 3         80000012     f122 sd      s0:0, a0(sp:80002d50)
    7 3         80000014     ed26 sd      s1:0, 98(sp:80002d50)
    8 3         80000016     e94a sd      s2:0, 90(sp:80002d50)
    9 3         80000018     e54e sd      s3:0, 88(sp:80002d50)
   10 3         8000001a     e152 sd      s4:0, 80(sp:80002d50)
```

## Article

This code is meant to accompany the article at
http://popovicu.com/posts/bare-metal-rust-risc-v-with-dynamic-memory
