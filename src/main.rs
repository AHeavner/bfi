use std::env;

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

fn main() {
    // Collecting arguments from command line
    let args: Vec<String> = env::args().collect();
    // program is a string from the first argument
    let program = args[1].clone();

    let instructions = lex(program);

}

//function lex: lexes the program
fn lex(program: String) -> Vec<Instruction> {
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
















// MMMMMMMMMMMMMMMMMMMMMMMMMMWXOxdlc:'.................',;:lx0NMMMMMMMMMMMMMMMMMMMM
// MMMMMMMMMMMMMMMMMMMMMNX0xo;.   ...,:cllllllcccc::;;;,,'.  .;dKWMMMMMMMMMMMMMMMMM
// MMMMMMMMMMMMMMMMWNKxl,..  .';:lodxkkkkkkkkkOkkkkkkkkkkkxo:'. 'dNMMMMMMMMMMMMMMMM
// MMMMMMMMMMMMMMWKd;. ..';codxkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkko, .lXMMMMMMMMMMMMMMM
// MMMMMMMMMMMMW0l. .':ldxkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkOOkkx:. :0WMMMMMMMMMMMMM
// MMMMMMMMMMMNx' .;oxkkkxkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkOOkl. ,OWMMMMMMMMMMMM
// MMMMMMMMMMNd. .lxkkkkkxxxxxkkkkkkkkkkkkkkkkkkkkkkkkkOOkkkkkOkkkl. ,OWMMMMMMMMMMM
// MMMMMMMMMKc. .lxxkkkxxxxkkkkkkkkkkkkkkkkkkkkkkkkkkkkOOkkkkkkkkkkl. 'kWMMMMMMMMMM
// MMMMMMMMK:  'lxxxxxkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkOko. ,OWMMMMMMMMM
// MMMMMMMXc  'oxxxxxxxkkkkkkkkkkkkkkkkkkkkkkkxxxxkkkkkkkkkkkkkkkkkkkl. ,OWMMMMMMMM
// MMMMMMXc  'odddddddxxkkkkkkkkkkkkkkkkkkkkkkxdxxxxxkkkOOOkkkkkkkkkkkc. lNMMMMMMMM
// MMMMWK:  'lddddooolcodxxxxxxxxxxdddxkkkxxxxxddxxxxxxkkkkxxxxxxxxxxxl. lNMMMMMMMM
// MMMWO;  ,oddoodddoc:lddollloddddoloxkxxxxxdddddollodxxdoodxxxxddddo;. lNMMMMMMMM
// MXx:. .;oddddddddddddxd:.,;..,;coddxxdddxxxxxo:;;;;;cddoodlcc;..:ddl. 'xNMMMMMMM
// x,  .'cdddddddoddddddxl..;c. ,c;lddddddddddoc;coddoc;;ldxl';l. .;dxxl. .,dXWMMMM
//   .;odxdddddddoodddddxl'     :d:ldddddddol:;;ldxxxxxo:;lxc.....'cxxxkdc.  ,xXWMM
//  ,lllodddddddddooooooddl'...:llcodddddl:,;:odxxxxxxxxd:;lo:'..,coxxkkkkdc.  .c0W
// ;ooc,;lxxddddxxxxxdddddddollolodxdoool;;cclodxxxxxxxxxo;,odollodddxxkkkkkxl,  ,0
// odddc,cdxxxxxxxxxxxxxkxxxxxdxxxxkxdooc;ccc::;::codddxxd;'cddddddddxkkkkkkkkd' .k
// dddxocldxxxxddddxxxxxxxkxxxxxkkxxkkxxl;:clddolc:;;::cc:'.:dxxxxxxxxkkkkkkkkx, .o
// dddxdddddxdddddddxxxxxxxxxkkkkkxxkkkkxdc:::oxxxxdolc:cc,,lxxxxkkkkkkkkkkkkkkc. .
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
