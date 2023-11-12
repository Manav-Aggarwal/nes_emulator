# RISC: Global Domination
<p align="center">
<img width="872" alt="Screen Shot 2023-11-12 at 11 21 03 AM" src="https://github.com/Manav-Aggarwal/nes_emulator/assets/35550889/6fca2903-b93b-43e5-a45b-08f19292be7f" class="center"></p>

Cheating in video games has accelerated in advancement – now, players employ methods such as bots and software assistance to perform automated target acquisition and calibration, lag switches, and world-hacking. The vision of autonomous worlds is enabled only by provability of all in-game actions. Players must be able to prove ownership and milestone-based achievements in order for the autonomous world itself to create digital scarcity and programmable ruleset based environments independent of control from a centralized authority. In this project, we build the foundation for autonomous worlds by using Bonsai to prove gameplay.

A Nintendo Entertainment System (NES) emulator is a software that allows a computer, smartphone, or other device to emulate both the hardware and software of the original Nintendo Entertainment System. This enables the play of classic NES games on platforms other than the original NES console. The emulator translates NES game code so that it can run on modern devices.

RISC: Global Domination is a NES emulator that uses Risc0 Bonsai to generate proofs to verify gameplay. When the user plays the game, the NES emulator has a buffer that keeps track of all instructions executed. In this project, the CPU is the circuirt and the inputs are the operations. The host writes the input, reads the input, and applies the operated return. The host then passes the inputs to the prover. Later, the host can keep writing the CPU and operation to this. Functionally, the project creates a zk NES CPU with Bonsai. 

This delivers the primary benefits:

1) Preserve Fun and Functionality: There are two general approaches to on-chain gaming–crypto native games for crypto native gamers or replicating traditional games on chain. This project caters to the latter and is an example of maintaining playability of traditional games without needlessly inserting a token/tokenomics. It is simply proving reputation (in-game achievements, milestones, actions) of a game with which players are already familiar.

2) Provability: Players can prove that in-game achievements and milestones were honestly accomplished without revealing how different milestones were achieved. 

_Acknowledgements
Project inspired by Tonk’s Dappicom and to takahirox for nes-rust._
<p align="center">
<img width="291" alt="Screen Shot 2023-11-12 at 11 27 10 AM" src="https://github.com/Manav-Aggarwal/nes_emulator/assets/35550889/4926d2a2-a026-4fce-b2ef-fce2060923e9"></p>


# RISC Zero Rust Starter Template

Welcome to the RISC Zero Rust Starter Template! This template is intended to
give you a starting point for building a project using the RISC Zero zkVM.
Throughout the template (including in this README), you'll find comments
labelled `TODO` in places where you'll need to make changes. To better
understand the concepts behind this template, check out the [zkVM
Overview][zkvm-overview].

## Quick Start

First, make sure [rustup] is installed. The
[`rust-toolchain.toml`][rust-toolchain] file will be used by `cargo` to
automatically install the correct version.

To build all methods and execute the method within the zkVM, run the following
command:

```bash
cargo run
```

This is an empty template, and so there is no expected output (until you modify
the code).

### Executing the project locally in development mode

During development, faster iteration upon code changes can be achieved by leveraging [dev-mode], we strongly suggest activating it during your early development phase. Furthermore, you might want to get insights into the execution statistics of your project, and this can be achieved by specifying the environment variable `RUST_LOG="executor=info"` before running your project.

Put together, the command to run your project in development mode while getting execution statistics is:

```bash
RUST_LOG="executor=info" RISC0_DEV_MODE=1 cargo run
```

### Running proofs remotely on Bonsai

_Note: The Bonsai proving service is still in early Alpha; an API key is
required for access. [Click here to request access][bonsai access]._

If you have access to the URL and API key to Bonsai you can run your proofs
remotely. To prove in Bonsai mode, invoke `cargo run` with two additional
environment variables:

```bash
BONSAI_API_KEY="YOUR_API_KEY" BONSAI_API_URL="BONSAI_URL" cargo run
```

## How to create a project based on this template

Search this template for the string `TODO`, and make the necessary changes to
implement the required feature described by the `TODO` comment. Some of these
changes will be complex, and so we have a number of instructional resources to
assist you in learning how to write your own code for the RISC Zero zkVM:

- The [RISC Zero Developer Docs][dev-docs] is a great place to get started.
- Example projects are available in the [examples folder][examples] of
  [`risc0`][risc0-repo] repository.
- Reference documentation is available at [https://docs.rs][docs.rs], including
  [`risc0-zkvm`][risc0-zkvm], [`cargo-risczero`][cargo-risczero],
  [`risc0-build`][risc0-build], and [others][crates].

## Directory Structure

It is possible to organize the files for these components in various ways.
However, in this starter template we use a standard directory structure for zkVM
applications, which we think is a good starting point for your applications.

```text
project_name
├── Cargo.toml
├── host
│   ├── Cargo.toml
│   └── src
│       └── main.rs                        <-- [Host code goes here]
└── methods
    ├── Cargo.toml
    ├── build.rs
    ├── guest
    │   ├── Cargo.toml
    │   └── src
    │       └── bin
    │           └── method_name.rs         <-- [Guest code goes here]
    └── src
        └── lib.rs
```

## Video Tutorial

For a walk-through of how to build with this template, check out this [excerpt
from our workshop at ZK HACK III][zkhack-iii].

## Questions, Feedback, and Collaborations

We'd love to hear from you on [Discord][discord] or [Twitter][twitter].

[bonsai access]: https://bonsai.xyz/apply
[cargo-risczero]: https://docs.rs/cargo-risczero
[crates]: https://github.com/risc0/risc0/blob/main/README.md#rust-binaries
[dev-docs]: https://dev.risczero.com
[dev-mode]: https://dev.risczero.com/api/zkvm/dev-mode
[discord]: https://discord.gg/risczero
[docs.rs]: https://docs.rs/releases/search?query=risc0
[examples]: https://github.com/risc0/risc0/tree/main/examples
[risc0-build]: https://docs.rs/risc0-build
[risc0-repo]: https://www.github.com/risc0/risc0
[risc0-zkvm]: https://docs.rs/risc0-zkvm
[rustup]: https://rustup.rs
[rust-toolchain]: rust-toolchain.toml
[twitter]: https://twitter.com/risczero
[zkvm-overview]: https://dev.risczero.com/zkvm
[zkhack-iii]: https://www.youtube.com/watch?v=Yg_BGqj_6lg&list=PLcPzhUaCxlCgig7ofeARMPwQ8vbuD6hC5&index=5
