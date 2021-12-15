# ortb-v2_5
This is a rustlang implementation of OpenRTB 2.5.

### Requirements
1) Rustlang 1.56.1 and the cargo ecosystem. use ```rustup```

To build for GNU/Linux:

1) Install cross compile toolchain to build for Linux ```rustup target add x86_64-unknown-linux-musl```
2) Install musl-gcc (OSX) ```brew install filosottile/musl-cross/musl-cross```

### Build with debug symbols and no optimization
```make build-debug```

### Build for release and those delicious rust optimizations
```make```
Or specifically for Linux
```make build-linux```
### Unit Test
```make unit-test```