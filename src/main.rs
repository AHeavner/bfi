use std::env;
use std::io::Read;
use std::fs::File;

fn main() {
    // Collecting arguments from command line
    let args: Vec<String> = env::args().collect();
    // program is a string from the first argument
    let program = args[1].clone();

    let instructions = precompile(program);

    run(instructions);
}

//function lex: lexes the program
fn precompile(program: String) -> Vec<Instruction> {
    let mut tokens: Vec<Instruction> = Vec::new();

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

fn run(instructions: Vec<Instruction>) {
    let mut data_list: Vec<u8> = vec![0; 5000];
    let mut data_pointer: usize = 0;

    // for keeping track of loops
    let mut stack: Vec<usize> = vec![];
    let mut jump = vec![0; instructions.len()];

    // records the code points for the loops
    for (i, instruction) in instructions.iter().enumerate() {
        match instruction {
            Instruction::LoopStart => {
                stack.push(i);
            }
            Instruction::LoopEnd => {
                let temp = stack.pop().unwrap();
                jump[i] = temp;
                jump[temp] = i;
            }
            _ => (),
        }
    }

    // interpreting part
    let mut code_pointer: usize = 0;
    while code_pointer < instructions.len() {
        match instructions[code_pointer] {
            Instruction::Right => data_pointer += 1,
            Instruction::Left => data_pointer -= 1,
            Instruction::Inc => data_list[data_pointer] += 1,
            Instruction::Dec => data_list[data_pointer] -= 1,
            Instruction::Write => print!("{}", data_list[data_pointer] as char),
            Instruction::Read => {
                let mut data_in: [u8; 1] = [0];
                std::io::stdin().read_exact(&mut data_in).unwrap();
                data_list[data_pointer] = data_in[0];
            }
            Instruction::LoopStart => {
                if data_list[data_pointer] == 0 {
                    ();
                }
                else {
                    code_pointer = jump[code_pointer];
                }
            }
            Instruction::LoopEnd => {
                if data_list[data_pointer] != 0 {
                    ();
                }
                else {
                    code_pointer = jump[code_pointer];
                }
            }
        }
        code_pointer += 1;
    }
}

// enum for all of the instructions
enum Instruction {
    // ">" - increment the pointer (move it to the "right")
    Right,
    // "<" - decrement the pointer (move it to the "left")
    Left,
    // "+" - increment the byte at the pointer
    Inc,
    // "-" - decrement the byte at the pointer
    Dec,
    // "." - output the byte at the pointer
    Write,
    // "," - input a byte and store it in the byte at the pointer
    Read,
    // "[" - jump forward past the matching ] if the byte at the pointer is zero
    LoopStart,
        // Store the instruction index of the matching parenthesis
        // Lazily initialized as needed
    // "]" - jump backward to the matching [ unless the byte at the pointer is zero
    LoopEnd,
        // Store the instruction index of the matching parenthesis
        // Strictly initialized when the program is loaded into memory
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
