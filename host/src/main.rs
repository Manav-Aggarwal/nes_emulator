// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use risc0_zkvm::{default_prover, ExecutorEnv};
use nes_emulator_methods::{NES_EMULATOR_INTERP_ELF, NES_EMULATOR_INTERP_ID};
use nes_rust_core::cpu::{Cpu, operation};
use nes_rust_core::default_input::DefaultInput;
use nes_rust_core::default_audio::DefaultAudio;
use nes_rust_core::default_display::DefaultDisplay;

fn operate_return(&mut cpu, Operation op) -> cpu {
    let env = ExecutorEnv::builder()
        .write(&cpu)
        .unwrap()
        .write(&op)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove_elf(env, NES_EMULATOR_INTERP_ELF).unwrap();

    receipt.verify(NES_EMULATOR_INTERP_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct image ID?",
    );
    let result: cpu = receipt.journal.decode().unwrap();

    result
}

fn main() {
    
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use super::*;

    #[test]
    fn nes_emulate() {
        let input = Box::new(DefaultInput::new());
		let display = Box::new(DefaultDisplay::new());
		let audio = Box::new(DefaultAudio::new());
        let mut cpu = Cpu::new(input, display, audio);
        let mut_cpu: &mut Cpu= cpu.get_mut_cpu();

        let op = operation(0x01);
        let new_cpu = cpu.operate_return(&op);
    }
}
