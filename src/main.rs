mod machine;

fn main() {
    let mut m = machine::Machine::new();

    m.load_program(vec![
        machine::instructions::PUSH, 0x05,
        machine::instructions::CALL,

        machine::instructions::DEC,
        machine::instructions::RET,

        machine::instructions::PUSH, 0x0F,
        machine::instructions::PUSH, 0x03,
        machine::instructions::CALL,
        machine::instructions::PUSH, 0x07,
        machine::instructions::JNZ,
        machine::instructions::DEC,
        machine::instructions::HALT
    ]);
    m.run();
}
