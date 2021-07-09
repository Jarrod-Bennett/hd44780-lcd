# hd44780-lcd
Rust driver for HD44780-compatible LCD displays.

Development currently in progress.

## Feature goals
- both current stable embedded-hal (v=0.2.5) and alpha embedded-hal (v1.0.0alphax)
- both full read-write and feature support for the HD44780 and a simplified write-only model
- both 4 and 8 pin data lines
- blocking and non-blocking/async api
