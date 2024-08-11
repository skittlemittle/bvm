/*
 brainfuck interpreter
 tape cells are u8
 the tape itself wraps around from the end to the start
*/

struct MachineState {
    tape: [u8; 30_000],
    pointer: usize,
}

impl MachineState {
    pub fn new() -> MachineState {
        MachineState {
            tape: [0; 30_000],
            pointer: 0,
        }
    }

    pub fn print_tape(self: MachineState, till: usize) {
        let stop = if self.tape.len() < till {
            self.tape.len()
        } else {
            till
        };

        for cell in 0..stop {
            print!("{} ", self.tape[cell]);
        }
        println!("");
    }
}

// TODO: slow!, prematch the [] before even running!
fn seek_closing_brace(commands: &Vec<char>, mut i_pointer: usize) -> Result<usize, VMError> {
    let mut stack: Vec<char> = vec![];

    while i_pointer < commands.len() {
        match commands[i_pointer] {
            ']' => {
                if stack.is_empty() {
                    return Ok(i_pointer);
                }
                stack.pop();
            }
            '[' => {
                stack.push('[');
            }
            _ => {}
        }
        i_pointer += 1;
    }

    Err(VMError::UnmatchedBrace)
}

fn main() {
    //let commands: Vec<char> = vec!['+', '+', '>', '+', '+', '[', '-', ']', 'i'];
    let program = String::from("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");
    let commands: Vec<char> = program.chars().collect();

    let mut machine = MachineState::new();
    let mut cmd_stack: Vec<usize> = vec![];
    let mut i_pointer: usize = 0;

    while i_pointer < commands.len() {
        match commands[i_pointer] {
            '+' => {
                machine.tape[machine.pointer] += 1;
                i_pointer += 1;
            }
            '-' => {
                if machine.tape[machine.pointer] == 0 {
                    machine.tape[machine.pointer] = u8::max_value();
                } else {
                    machine.tape[machine.pointer] -= 1;
                }
                i_pointer += 1;
            }
            '>' => {
                machine.pointer = (machine.pointer + 1) % machine.tape.len();
                i_pointer += 1;
            }
            '<' => {
                if machine.pointer == 0 {
                    machine.pointer = machine.tape.len() - 1;
                } else {
                    machine.pointer -= 1;
                }
                i_pointer += 1;
            }
            '.' => {
                print!(
                    "{}",
                    std::ascii::escape_default(machine.tape[machine.pointer])
                );
                i_pointer += 1;
            }
            ',' => i_pointer += 1,
            '[' => {
                if machine.tape[machine.pointer] == 0 {
                    match seek_closing_brace(&commands, i_pointer + 1) {
                        Ok(p) => i_pointer = p + 1,
                        Err(_) => panic!("SYNTAX ERROR: unmatched [ at {}", i_pointer),
                    }
                } else {
                    cmd_stack.push(i_pointer);
                    i_pointer += 1;
                }
            }
            ']' => match cmd_stack.pop() {
                Some(p) => i_pointer = p,
                None => panic!("SYNTAX ERROR: unmatched ] at {}", i_pointer),
            },
            _ => i_pointer += 1,
        }
    }
    MachineState::print_tape(machine, 30);
}

#[derive(Debug)]
enum VMError {
    UnmatchedBrace,
}
