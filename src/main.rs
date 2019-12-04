// Authored by Addison Heavner, Brian Sweeney, and Kharon Harrell
// WVU CS 410: Compiler Construction term project
// This is a pure Rust implementation of a Brainfuck interpreter
// To compile you need to install Rust from https://www.rust-lang.org/
// Source code repository located at https://github.com/AHeavner/bfi

use std::io::Read;
use std::fs::File;

extern crate clap;
use clap::{Arg, App};

/// Interpreter starting point.
fn main() {
    // opens file handle from command line input filename
    let mut file = File::open(command_line()).unwrap();

    // reads program from file
    let mut program = String::new();
    file.read_to_string(&mut program).unwrap();
    
    // precompiles code into "instructions"
    let instructions = precompile(program);

    // interprets the program
    run(instructions);
}

/// Handles command line input using the clap crate. Returns the file name input.
fn command_line() -> String {
    let matches = App::new("Brainfuck Interpreter")
                        .version("0.1")
                        .author("Addison Heavner, Brian Sweeney, Kharon Harrell")
                        .about("Interprets BF code")
                        .arg(Arg::with_name("INPUT")
                               .help("Sets the input file to use")
                               .required(true)
                               .index(1))
                        .get_matches();

    matches.value_of("INPUT").unwrap().to_string()
}

/// Lexes the program input string and returns instruction vector
fn precompile(program: String) -> Vec<Instruction> {
    let mut tokens: Vec<Instruction> = Vec::new();
    // Matches input symbol to tokens to be stored in the instructions vector
    for symbol in program.chars() {
        match symbol {
            '>' => tokens.push(Instruction::Right),
            '<' => tokens.push(Instruction::Left),
            '+' => tokens.push(Instruction::Inc),
            '-' => tokens.push(Instruction::Dec),
            '.' => tokens.push(Instruction::Write),
            ',' => tokens.push(Instruction::Read),
            '[' => tokens.push(Instruction::LoopStart),
            ']' => tokens.push(Instruction::LoopEnd),
            _ => (),
	};
    }

    tokens //returns tokens
}

/// Interprets the precompiled BF code. Handles inputs and outputs to the terminal.
fn run(instructions: Vec<Instruction>) {
    let mut data_list: Vec<u8> = vec![0; 100000];
    let mut data_pointer: usize = 0;

    // Executes program stored in instructions vector
    let mut loops = 0;
    let mut code_pointer: usize = 0;
    while code_pointer < instructions.len() {
        match instructions[code_pointer] {
            // Increments pointer
            Instruction::Right => data_pointer += 1,
            // Decrements pointer
            Instruction::Left => {
                if data_pointer == 0 {
                    data_pointer = 0;
                }
                else{
                    data_pointer -= 1;
                }
            }
            // Increments data cell
            Instruction::Inc => {
                if data_list[data_pointer] == 255 {
                    data_list[data_pointer] = 0;
                }
                else{
                    data_list[data_pointer] += 1;
                }
            }
            // Decrements data cell
            Instruction::Dec => {
                if data_list[data_pointer] == 0 {
                    data_list[data_pointer] = 255;
                }
                else{
                    data_list[data_pointer] -= 1;
                }
            }
            // Outputs current data cell to the terminal
            Instruction::Write => print!("{}", data_list[data_pointer] as char),
            // Reads a byte of input from the terminal buffer
            Instruction::Read => {
                let mut data_in: [u8; 1] = [0];
                std::io::stdin().read_exact(&mut data_in).unwrap();
                data_list[data_pointer] = data_in[0];
            }
            // Logic to begin a loop. Keeps track of how many nested loops.
            Instruction::LoopStart => {
                if data_list[data_pointer] == 0 {
                    code_pointer += 1;
                    while loops > 0 || instructions[code_pointer] != Instruction::LoopEnd {
                        if instructions[code_pointer] == Instruction::LoopStart {
                            loops += 1;
                        }
                        if instructions[code_pointer] == Instruction::LoopEnd {
                            loops -= 1;
                        }
                        code_pointer += 1;
                    }
                }
            }
            // Logic to end a loop. Keeps track of how many nested loops.
            Instruction::LoopEnd => {
                if data_list[data_pointer] != 0 {
                    code_pointer -= 1;
                    while loops > 0 || instructions[code_pointer] != Instruction::LoopStart {
                        if instructions[code_pointer] == Instruction::LoopEnd{
                            loops += 1;
                        }
                        if instructions[code_pointer] == Instruction::LoopStart{
                            loops -= 1;
                        }
                        code_pointer -= 1;                        
                    }
                    code_pointer -= 1;
                }
            }
        }
        code_pointer += 1;
    }
}

/// enum containing all of the BF instructions.
#[derive(Debug)]
#[derive(PartialEq)]
enum Instruction {
    Right,
    Left,
    Inc,
    Dec,
    Write,
    Read,
    LoopStart,
    LoopEnd,
}









//                                 ...,:cllllllcccc::;;;,,'. 
//                          .';:lodxkkkkkkkkkOkkkkkkkkkkkxo:'. 
//                     ..';codxkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkko, 
//                  .':ldxkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkOOkkx:. 
//                .;oxkkkxkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkOOkl. 
//               .lxkkkkkxxxxxkkkkkkkkkkkkkkkkkkkkkkkkkOOkkkkkOkkkl. 
//              .lxxkkkxxxxkkkkkkkkkkkkkkkkkkkkkkkkkkkkOOkkkkkkkkkkl. 
//             'lxxxxxkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkOko. 
//            'oxxxxxxxkkkkkkkkkkkkkkkkkkkkkkkxxxxkkkkkkkkkkkkkkkkkkkl.
//           'odddddddxxkkkkkkkkkkkkkkkkkkkkkkxdxxxxxkkkOOOkkkkkkkkkkkc.
//          'lddddooolcodxxxxxxxxxxdddxkkkxxxxxddxxxxxxkkkkxxxxxxxxxxxl. 
//         ,oddoodddoc:lddollloddddoloxkxxxxxdddddollodxxdoodxxxxddddo;. 
//       .;oddddddddddddxd:.,;..,;coddxxdddxxxxxo:;;;;;cddoodlcc;..:ddl. 
//     .'cdddddddoddddddxl..;c. ,c;lddddddddddoc;coddoc;;ldxl';l. .;dxxl. 
//   .;odxdddddddoodddddxl'     :d:ldddddddol:;;ldxxxxxo:;lxc.....'cxxxkdc.  
//  ,lllodddddddddooooooddl'...:llcodddddl:,;:odxxxxxxxxd:;lo:'..,coxxkkkkdc.  
// ;ooc,;lxxddddxxxxxdddddddollolodxdoool;;cclodxxxxxxxxxo;,odollodddxxkkkkkxl,
// odddc,cdxxxxxxxxxxxxxkxxxxxdxxxxkxdooc;ccc::;::codddxxd;'cddddddddxkkkkkkkkd'
// dddxocldxxxxddddxxxxxxxkxxxxxkkxxkkxxl;:clddolc:;;::cc:'.:dxxxxxxxxkkkkkkkkx,
// dddxdddddxdddddddxxxxxxxxxkkkkkxxkkkkxdc:::oxxxxdolc:cc,,lxxxxkkkkkkkkkkkkkkc.
// xddxddooodddddddxxxxxxxxkkkkkOOkkkkkkkkkkxc::lddxxxxxo;,lxkkkkkkkkkkkkkkkkkkxo,.
// dxddddoooooddddxxxddxxkkkkkkkOOkkkkkkkkkkkkdl:;;::::;;:oxkkkkxxxxkkkkkkkkkkkkkxc
// dxxdddddooooddddxxxxxkkkkkxxkkkkkkkkkkkkkkkkkxxdlcccodxxxxxxkxxxkkkkkkkkkkkkkkx:
// xxxxxxddolodddddddddxxxxxxxxxxxxxkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkx
// xxxxxdl::coxxdxddddddxxxxxxxxxxxkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkxxkkkkkkk
// dxxxxdollcldddxxxxddddxxxkxxxxxxxkkkkkkkkkkkkkkkkkkkkkkkkkkxxkkkkkxoccccclxkkkkk
// dddxddddddooddddxxxddxxxkkxxxxkxkkkkkkkkkkkxxxkkkkkkkkxxxddodddxxkdc,,ldddxxxxxk
// ddddddddddddddddddxxxxxxxxxxxxxkkkkxxkkkkxxxxxxxxxxxxdolccccccloddxxddxxxxxxxxkk
// ddddddddddddddddddddxxxxxxdxxxxxxxdlooldxxoloxxxxolc,;:'.',::clodxxxxxxxxxxkkkkk
// ddddxxxddddddddddddddxxxxxxxxxxxxdolllcoddlcodddxoll:,:ccodxxxxddxxxxxxxxxxxxxxx
// dddddddddddddddddddddddxxxxxxxxxxxddddddxddxxddxxxkxxddxxxxxxkxxddxxxxkkkkkkkxxx
// ddddddddddddddddddddddxxxxxxxkkxxxxxxxxxxxxxxxxxxxxkkxxxxxxxxxxxxxxddxxkkkkxxxxx
// dddddddddddddddddddddddxxxxkkkkkkkkxxxxxxxxkkkxxxxxxxxxxxxxxxxxxxxxdxxxxxxxxxxxx
// ddddddddddddddoddddddddxxxxkkkkkkkkkkkxxxxxxxxxxxxxxxxxdddddddxxxxxxxxxxxxxxxxxx
// dddddddddddddddddddddddxxxxxxxxkkkkkkkkxxxxxxxxxxxxxxxddddddddxxxxxxxxxxxxxxxxxx
// ddddddddddxxxxddddddxxxxxxxxxxxxkkkkkkkkkxxxxxxxxxxxxxddddddddxxxxxxxxxxxxxxxxxx
