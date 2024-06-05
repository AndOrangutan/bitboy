# bitboy

Micro:bit embedded project

## Setup Cross Compiling

1. Add support for board.

```bash
rustup target add thumbv7em-none-eabihf
```

2. List targets.

```bash
rustup show
```

## Resources

- <https://infocenter.nordicsemi.com/index.jsp?topic=%2Fstruct_nrf52%2Fstruct%2Fnrf52833.html&cp=3_1>
- <https://github.com/microbit-foundation/microbit-v2-hardware/blob/main/V2.21/MicroBit_V2.2.1_nRF52820%20schematic.PDF>
