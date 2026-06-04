# msp430-hello

This repository contains a blinking light "Hello, World" program in Rust for the MSP430FR5994 Texas Instrument microcontroller. The example code from `rust-embedded` would not compile for this board due to API differences. I could not tell if this was due to a change in APIs across the entire MSP430 ecosystem or something unique to the crate for this board specifically. That being said, this code should just compile if your `rustup` default is set to nightly.

### Please note: Since Rust does not currently support the 20-bit MSP430X instruction set, this code defaults to the legacy 16-bit MSP430 instruction set which does limit the memory address range.

I built the `MSPDLLv3` source on my Ubuntu ARM64 machine to allow `mspdebug` to use `tilib`. That being said, running `mspdebug tilib` was only able to successfully update the firmware of the machine, but was unable to upload the generated binary program. Running `mspdebug ezfet` ran into a similar problem despite not relying on `MSPDLLv3` and also experienced a stack overflow. This was after using the latest upstream commit from `dlbeer`'s GitHub repository for `mspdebug`. I filed a bug report for this issue.

Since this is the case, I will try to upload the program by using a `USB-to-TTL` UART connection to the board alongside a Python module to take care of the actual interfacing. Once I am successfully able to get the program to upload, I will update this README with instructions as well as potentially some pictures and videos of the process and it working on the board.
