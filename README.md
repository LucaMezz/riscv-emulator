# RISC-V 64 Emulator

A lightweight and modular RISC-V 64 emulator implemented in Rust. This project aims to emulate the RISC-V 64-bit instruction set architecture (ISA) with a focus on correctness, performance, and extendability.

## Features

- **Instruction Set Support**: Implements the RV64I base ISA, with plans to support extensions like M (Multiplication), A (Atomic), F (Floating Point), and others.
- **Modular Design**: Easily extendable for new RISC-V features or custom extensions.
- **Performance-Oriented**: Optimized for execution speed while maintaining clarity and correctness.
- **Rust Safety**: Leverages Rust's safety guarantees for memory management and concurrency.
- **Debugging Support**: Includes debugging tools such as step-by-step execution and memory inspection.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/) (latest stable version recommended)

To install Rust, run:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Building the Emulator

Clone the repository:
```bash
git clone https://github.com/your-username/riscv64-emulator.git
cd riscv64-emulator
```

Build the project:
```bash
cargo build --release
```

### Running a Program

1. Assemble or compile a RISC-V program using a RISC-V toolchain, such as [GNU RISC-V](https://github.com/riscv-collab/riscv-gnu-toolchain) or [riscv-tools](https://github.com/riscv/riscv-tools).
2. Run the program in the emulator:

```bash
cargo run -- path/to/your/program.bin
```

### Debugging Mode

To run the emulator in debugging mode, use the `--debug` flag:
```bash
cargo run -- path/to/your/program.bin --debug
```

## Project Structure

```
.
├── src
│   ├── components
│   │   ├── cpu.rs         # CPU implementation, including registers and execution logic
│   │   ├── mod.rs         # Module definition for components
│   │   ├── ram.rs         # RAM implementation and management
│   │   └── rom.rs         # ROM implementation and handling
│   ├── isa
│   │   ├── format.rs      # Instruction format parsing
│   │   ├── instruction.rs # Instruction decoding and execution
│   │   └── mod.rs         # Module definition for ISA
│   ├── test               # Testing utilities and modules
│   ├── main.rs            # Entry point of the emulator
│   └── util.rs            # Utility functions for the emulator
├── target                 # Build output directory
├── .gitignore             # Git ignore file
├── Cargo.lock             # Cargo lock file for dependencies
├── Cargo.toml             # Rust project configuration
└── README.md              # Project documentation
```

## Contributing

Contributions are welcome! If you'd like to contribute, please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature-name`)
3. Commit your changes (`git commit -m 'Add feature'`)
4. Push to the branch (`git push origin feature-name`)
5. Open a Pull Request

## Roadmap

- [ ] Basic RV64I instruction set support
- [ ] Implement M (Multiplication) extension
- [ ] Implement F (Floating Point) extension
- [ ] Add support for privileged instructions
- [ ] Performance optimizations
- [ ] Better debugging tools (e.g., interactive debugger)

## License

This project is licensed under the [MIT License](LICENSE).

## Acknowledgements

- [The RISC-V Foundation](https://riscv.org/) for the open ISA specification.
- [Rust](https://www.rust-lang.org/) for its incredible ecosystem and safety features.