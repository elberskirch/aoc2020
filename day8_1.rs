use std::env;
use std::fs;
use std::collections::HashSet;

struct Cpu {
    accumulator: i32,
    pos: i32,
}

/*
acc increases or decreases a single global value called the accumulator by the value given in the argument. For example, acc +7 would increase the accumulator by 7. The accumulator starts at 0. After an acc instruction, the instruction immediately below it is executed next.

jmp jumps to a new instruction relative to itself. The next instruction to execute is found using the argument as an offset from the jmp instruction; for example, jmp +2 would skip the next instruction, jmp +1 would continue to the instruction immediately below it, and jmp -20 would cause the instruction 20 lines above to be executed next.

nop stands for No OPeration - it does nothing. The instruction immediately below it is executed next.
*/
impl Cpu {
    fn nop(&mut self, arg: i32) -> i32 {
        self.pos += 1;
        println!("[nop] arg: {} pos: {} acc: {}", arg, self.pos, self.accumulator);
        self.pos
    } 
    
    fn acc(&mut self, arg: i32) -> i32 {
        self.accumulator += arg;
        self.pos += 1;
        println!("[acc] arg: {} pos: {} acc: {}", arg, self.pos, self.accumulator);
        self.pos
    } 
    
    fn jmp(&mut self, arg: i32) -> i32 {
        self.pos += arg;
        println!("[jmp] arg: {} pos: {} acc: {}", arg, self.pos, self.accumulator);
        self.pos
    } 

    fn get_accumulator(&self) -> i32 {
        self.accumulator
    }
    
    fn dispatch(&mut self, op: &str, arg: i32) -> i32 {
        match op {
            "nop" => return self.nop(arg),
            "acc" => return self.acc(arg),
            "jmp" => return self.jmp(arg),
            _ => return 0,
        }
    }
}

struct Instruction<'a> {
    op: &'a str,
    arg: i32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = &args[1];

    let contents = fs::read_to_string(input_filename)
        .expect("Something went wrong reading the file");

    let mut instructions: Vec<Vec<&str>> = contents.split('\n')
        .map(|x: &str| x.split(' ').collect::<Vec<&str>>())
        .collect();
   
    // dirty remove of last "" element
    instructions.pop(); 
    println!("{:?}", instructions);

    let op_list: Vec<Instruction> = instructions.iter().map(|x| Instruction{ op: x[0], arg: x[1].parse::<i32>().unwrap()}).collect();
    // trace vector indizes
    let mut visited = HashSet::new();
    let mut cpu = Cpu { accumulator: 0, pos: 0 };

   
    loop {

        if cpu.pos >= (op_list.len() as i32) {
            println!("Break loop! Instruction out of bounds Acc: {}", cpu.accumulator);
            return 
        }
        if !visited.insert(cpu.pos) {
            println!("Break loop! Acc: {}", cpu.accumulator);
            return 
        }
        let i = &op_list[cpu.pos as usize];
        cpu.dispatch(i.op, i.arg); 
    }

}
