use arviss_experiment::v2::{
    cpu::Rv32iCpu, cpu_types::Rv32i, memory::BasicMem, trap_handler::BasicTrapHandler};

pub fn main() {
    let mut rv32i_cpu = Rv32iCpu::<BasicMem>::new();
    rv32i_cpu.add(10, 11, 12);

    let mut rv32i_cpu_with_trap_handler = Rv32iCpu::<BasicMem, BasicTrapHandler>::new();
    rv32i_cpu_with_trap_handler.add(10, 11, 12);
}
