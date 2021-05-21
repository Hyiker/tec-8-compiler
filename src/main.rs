use clap::Clap;
use std::fs::File;
use std::io::prelude::*;

const SPACE_TOKEN: char = ' ';
const COMMA_TOKEN: char = ',';
const BRACKET_1_TOKEN: char = '[';
const BRACKET_2_TOKEN: char = ']';
#[derive(Clap)]
#[clap(version = "0.0.1", author = "Hyiker Hu <hyiker@bupt.edu.cn>")]
struct Opts {
    #[clap(index = 1, value_name = "FILE")]
    src: String,
    #[clap(short, long, default_value = "a.out")]
    out: String,
}

fn read_src(path: &str) -> std::io::Result<String> {
    let mut contents = String::new();
    let mut src = File::open(path)?;
    src.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_register(reg_str: &str) -> Result<u8, String> {
    let n_reg = reg_str.chars().nth(1);
    match n_reg {
        Some(c) => match c.to_digit(10) {
            Some(n) => Ok(n as u8),
            None => Err(format!("无效的寄存器号`{}`", c).to_string()),
        },
        None => Err("没有指定寄存器编号".to_string()),
    }
}

// 汇编转机器码
fn parse_asm(asm: &str, pc: u8) -> Result<u8, String> {
    let opcode: u8;
    let mut opt: u8 = 0;
    let mut buf = String::new();
    let mut asm_iter = asm.chars();
    loop {
        let c = asm_iter.next();
        match c {
            Some(c) => {
                if c == SPACE_TOKEN {
                    break;
                }
                buf.push(c);
            }
            None => break,
        }
    }
    let buf = buf;
    match buf.as_str() {
        "add" => opcode = 1,
        "sub" => opcode = 2,
        "and" => opcode = 3,
        "inc" => opcode = 4,
        "ld" => opcode = 5,
        "st" => opcode = 6,
        "jc" => opcode = 7,
        "jz" => opcode = 8,
        "jmp" => opcode = 9,
        "out" => opcode = 10,
        "iret" => opcode = 11,
        "di" => opcode = 12,
        "ei" => opcode = 13,
        "stp" => opcode = 14,
        _ => return Err(format!("未定义的指令`{}`", buf).to_string()),
    }
    match opcode {
        // 双操作数
        1 | 2 | 3 | 5 | 6 => {
            let mut opt1 = String::new();
            let mut opt2 = String::new();
            loop {
                let c = asm_iter.next();
                match c {
                    Some(c) if c == SPACE_TOKEN => continue,
                    Some(c) if c == COMMA_TOKEN => break,
                    Some(c) => opt1.push(c),
                    None => return Err("指令缺少操作符2".to_string()),
                }
            }
            loop {
                let c = asm_iter.next();
                match c {
                    Some(SPACE_TOKEN) | Some(BRACKET_1_TOKEN) | Some(BRACKET_2_TOKEN) => continue,
                    Some(c) => opt2.push(c),
                    None => break,
                }
            }
            let _opt1;
            let _opt2;
            if opcode == 6 {
                _opt2 = parse_register(&opt1)?;
                _opt1 = parse_register(&opt2)?;
            } else {
                _opt1 = parse_register(&opt1)?;
                _opt2 = parse_register(&opt2)?;
            }
            opt = (_opt1 & 0x3) << 2 | (_opt2 & 0x3);
        }
        // 单操作数
        4 | 7 | 8 | 9 | 10 => {
            let mut opt1 = String::new();
            loop {
                let c = asm_iter.next();
                match c {
                    Some(c) if c == SPACE_TOKEN => continue,
                    Some(c) => opt1.push(c),
                    None => break,
                }
            }
            match opcode {
                4 | 9 => opt = (parse_register(&opt1)? & 0x3) << 2,
                10 => opt = parse_register(&opt1)? & 0x3,
                7 | 8 => match u8::from_str_radix(&opt1.trim_end_matches("h"), 16) {
                    Ok(n) => opt = ((n as i8 - pc as i8) as u8) & 0xF,
                    Err(_) => return Err(format!("转换offset`{}`出错", opt1).to_string()),
                },
                _ => (),
            }
        }
        // 无操作数
        11..=14 => {
            opt = 0;
        }
        _ => (),
    }

    Ok((opcode << 4) | opt)
}

// 将文本格式的指令编译为8比特的机器码
// 每条指令通过行分隔
fn compile(text: &str) -> Result<Vec<(String, u8)>, String> {
    let mut machine_codes = Vec::new();
    for (i, line) in text.lines().enumerate() {
        let machine_code = match parse_asm(&line.to_lowercase(), (i + 1) as u8) {
            Ok(code) => code,
            Err(e) => return Err(format!("编译{}行时发生错误: {}", i + 1, e).to_string()),
        };
        machine_codes.push((line.to_string(), machine_code));
    }
    Ok(machine_codes)
}

fn main() -> std::io::Result<()> {
    let opts: Opts = Opts::parse();
    let contents = read_src(&opts.src)?;
    let mut result_content = String::new();
    match compile(&contents) {
        Ok(codes) => {
            println!("成功编译");
            for (i, code) in codes.iter().enumerate() {
                result_content
                    .push_str(format!("{:04b}:  {:08b} {}\n", i, code.1, code.0).as_str());
            }
        }
        Err(e) => eprintln!("编译时遇到错误: {}", e),
    }
    let mut out = File::create(opts.out)?;
    out.write_all(result_content.as_bytes())?;

    Ok(())
}
