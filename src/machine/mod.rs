pub mod instructions;
use std::process;

pub struct Machine {
    pub stack: Vec<u16>,
    pub program: Vec<u16>,
    pub sp: u16,
    pub ip: u16,
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            stack: vec![0; 1024],
            program: vec![0; 1024],
            sp: 0,
            ip: 0,
        }
    }

    pub fn load_program(&mut self, program: Vec<u16>) {
        for (mut _space, byte) in self.program.iter_mut().zip(program.iter()) {
            *_space = *byte;
        }
    }

    fn fetch(&mut self) -> Result<&u16, &str> {
        let ins = self.program.get(self.ip as usize);
        self.ip = self.ip.wrapping_add(1);
        return ins.ok_or("ip out of range");
    }

    pub fn step(&mut self) {
        let ins = *self.fetch().unwrap_or_else(|_| process::exit(1));
        match ins {
            instructions::PUSH => {
                let value = *self.fetch().unwrap_or_else(|_| process::exit(1));
                self.sp = self.sp.wrapping_add(1);
                self.stack[self.sp as usize] = value;
            },
            instructions::PIP => {
                self.sp = self.sp.wrapping_add(1);
                self.stack[self.sp as usize] = self.ip;
            },
            instructions::PSP => {
                self.sp = self.sp.wrapping_add(1);
                self.stack[self.sp as usize] = self.sp;
            },
            instructions::ADD => {
                let a = self.stack.get(self.sp as usize).unwrap_or_else(|| process::exit(1));
                self.sp = self.sp.wrapping_sub(1);
                let b = self.stack.get(self.sp as usize).unwrap_or_else(|| process::exit(1));
                self.stack[self.sp as usize] = a.wrapping_add(*b);
            },
            instructions::SUB => {
                let a = self.stack.get(self.sp as usize).unwrap_or_else(|| process::exit(1));
                self.sp = self.sp.wrapping_sub(1);
                let b = self.stack.get(self.sp as usize).unwrap_or_else(|| process::exit(1));
                self.stack[self.sp as usize] = a.wrapping_sub(*b);
            },
            instructions::MUL => {
                let a = self.stack.get(self.sp as usize).unwrap_or_else(|| process::exit(1));
                self.sp = self.sp.wrapping_sub(1);
                let b = self.stack.get(self.sp as usize).unwrap_or_else(|| process::exit(1));
                self.stack[self.sp as usize] = a.wrapping_mul(*b);
            },
            instructions::LSF => {
                let s = self.stack.get(self.sp as usize).unwrap_or_else(|| process::exit(1));
                self.sp = self.sp.wrapping_sub(1);
                let v = self.stack.get(self.sp as usize).unwrap_or_else(|| process::exit(1));
                self.stack[self.sp as usize] = v.wrapping_shl(*s as u32);
            },
            instructions::RSF => {
                let s = self.stack.get(self.sp as usize).unwrap_or_else(|| process::exit(1));
                self.sp = self.sp.wrapping_sub(1);
                let v = self.stack.get(self.sp as usize).unwrap_or_else(|| process::exit(1));
                self.stack[self.sp as usize] = v.wrapping_shr(*s as u32);
            },
            instructions::INC => {
                self.stack[self.sp as usize] = self.stack[self.sp as usize].wrapping_add(1);
            },
            instructions::DEC => {
                self.stack[self.sp as usize] = self.stack[self.sp as usize].wrapping_sub(1);
            },
            instructions::IIP => {
                self.ip = self.ip.wrapping_add(1);
            },
            instructions::ISP => {
                self.sp = self.sp.wrapping_add(1);
            },
            instructions::DIP => {
                self.ip = self.ip.wrapping_sub(1);
            },
            instructions::DSP => {
                self.sp = self.sp.wrapping_sub(1);
            },
            instructions::CALL => {
                let return_address = self.ip;
                let jmp_address = *self.stack.get(self.sp as usize).unwrap_or_else(|| process::exit(1));
                let swap_value = *self.stack.get((self.sp - 1) as usize).unwrap_or_else(|| process::exit(1));

                self.stack[(self.sp - 1) as usize] = return_address;
                self.stack[self.sp as usize] = swap_value;
                self.ip = jmp_address;
            },
            instructions::RET => {
                let cur_value = *self.stack.get(self.sp as usize).unwrap_or_else(|| process::exit(1));
                self.sp = self.sp.wrapping_sub(1);
                self.ip = *self.stack.get(self.sp as usize).unwrap_or_else(|| process::exit(1));
                self.stack[self.sp as usize] = cur_value;
            },
            instructions::JNZ => {
                let jmp_address = *self.stack.get(self.sp as usize).unwrap_or_else(|| process::exit(1));
                self.sp = self.sp.wrapping_sub(1);
                let check_value = *self.stack.get(self.sp as usize).unwrap_or_else(|| process::exit(1));

                if check_value != 0 {
                    self.ip = jmp_address;
                }
            },
            instructions::HALT => {
                let cur_value = *self.stack.get(self.sp as usize).unwrap_or_else(|| process::exit(1));
                println!("{}", cur_value);
                process::exit(0);
            },
            instructions::SSP => {
                let new_sp = *self.stack.get(self.sp as usize).unwrap_or_else(|| process::exit(1));
                self.sp = new_sp;
            },
            instructions::JMP => {
                let new_ip = *self.stack.get(self.sp as usize).unwrap_or_else(|| process::exit(1));
                self.ip = new_ip;
            },
            _ => ()
        }
    }

    pub fn run(&mut self) {
        loop {
            self.step();
        }
    }
}
