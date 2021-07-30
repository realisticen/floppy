use std::{fs, ops::Index};
use structopt::StructOpt;

fn read_u16(slice: &[u8]) -> u16 {
    let mut b: [u8; 2] = Default::default();
    b.copy_from_slice(&slice[0..2]);
    u16::from_le_bytes(b)
}

fn read_u8(slice: &[u8]) -> u8 {
    let mut b: [u8; 1] = Default::default();
    b.copy_from_slice(&slice[0..1]);
    u8::from_le_bytes(b)
}

fn write_u16(value: u16, slice: &mut[u8]) {
    let bytes = value.to_le_bytes();
    slice[0] = bytes[0];
    slice[1] = bytes[1];
}

#[derive(Debug)]
struct Head {
    // one byte - static value 1
    // nine bytes - static value 0
    // Two bytes - wait small pistol
    // Two bytes - wait big pistol
    // ten bytes - static value 0
    // 22 Bytes in total
    wait_l: u16,
    wait_r: u16,
}

impl Head {
    const SIZE: usize = 22; // Head is 22 bytes

    fn from_bytes(slice: &[u8]) -> Self {
        Head {
            wait_l: read_u16(&slice[10..12]),
            wait_r: read_u16(&slice[12..14]),
        }
    }

    fn to_bytes(&self) -> [u8; Head::SIZE] {
        let mut header: [u8; Head::SIZE] = [0; Head::SIZE];
        header[0] = 1;
        write_u16(self.wait_l, &mut header[10..12]);
        write_u16(self.wait_r, &mut header[12..]);
        header
    }

    fn from_string(str: &str) -> Self {
        let v: Vec<&str> = str.trim().split('|').collect();
        Head {
            wait_l: v.get(0).expect("Could not read delay L").trim().parse().expect("Invalid value for delay L"),
            wait_r: v.get(1).expect("Could not read delay R").trim().parse().expect("Invalid value for delay R")
        }
    }

    fn to_string(&self) -> String {
        format!("{:^7}|{:^7}", self.wait_l, self.wait_r)
    }

    fn pretty_print(&self) {
        println!("Wait L | Wait R");
        println!("{:^7}|{:^7}", self.wait_l, self.wait_r);
    }
}


// First step starts at offset 22
#[derive(Debug)]
struct Step {
    quote: u16, // Maching position in mm?
    // two bytes - 0 value (not sure what they are for)
    p1: u8,
    p2: u8,
    p3: u8,
    p4: u8,
    p5: u8,
    p6: u8,
    p7: u8,
    p8: u8,
    l: u8, // Left  piston set (offsets pistols 5 6 7 8)
    r: u8, // Right piston set (offsets pistols 1 2 3 4)
}

impl Step {
    const SIZE: usize = 14; // Each step is 14 bytes

    fn from_bytes(slice: &[u8]) -> Self {
        Step {
            quote: read_u16(&slice[0..2]),
            p1: read_u8(&slice[4..5]),
            p2: read_u8(&slice[5..6]),
            p3: read_u8(&slice[6..7]),
            p4: read_u8(&slice[7..8]),
            p5: read_u8(&slice[8..9]),
            p6: read_u8(&slice[9..10]),
            p7: read_u8(&slice[10..11]),
            p8: read_u8(&slice[11..12]),
            l: read_u8(&slice[12..13]),
            r: read_u8(&slice[13..14]),
        }
    }

    fn to_bytes(&self) -> [u8; Step::SIZE] {
        let mut step: [u8; Step::SIZE] = [0; Step::SIZE];
        write_u16(self.quote, &mut step);
        step[4] = self.p1;
        step[5] = self.p2;
        step[6] = self.p3;
        step[7] = self.p4;
        step[8] = self.p5;
        step[9] = self.p6;
        step[10] = self.p7;
        step[11] = self.p8;
        step[12] = self.l;
        step[13] = self.r;
        step
    }

    fn vec_from_bytes(slice: &[u8]) -> Vec<Self> {
        slice
            .chunks(Step::SIZE)
            .map(Step::from_bytes)
            .take_while(|s| s.quote != 0)
            .collect()
    }

    fn from_string(str: &str) -> Self {
        let s1: Vec<&str> = str.trim().split('|').collect();
        let s2: Vec<&str> = s1.get(1).expect("Could not get pistol data - missing |").trim().split(' ').collect();
        let s3: Vec<&str> = s1.get(2).expect("Could not get offset data - missing |").trim().split(' ').collect();

        Step {
            quote: s1.get(0).expect("Could not read quote").trim().parse().expect("Invalid value for quote"),
            p1: s2.get(0).expect("Could not read p1").parse().expect("Invalid value for p1"),
            p2: s2.get(1).expect("Could not read p2").parse().expect("Invalid value for p2"),
            p3: s2.get(2).expect("Could not read p3").parse().expect("Invalid value for p3"),
            p4: s2.get(3).expect("Could not read p4").parse().expect("Invalid value for p4"),
            p5: s2.get(4).expect("Could not read p5").parse().expect("Invalid value for p5"),
            p6: s2.get(5).expect("Could not read p6").parse().expect("Invalid value for p6"),
            p7: s2.get(6).expect("Could not read p7").parse().expect("Invalid value for p7"),
            p8: s2.get(7).expect("Could not read p8").parse().expect("Invalid value for p8"),
            l: s3.get(0).expect("Could not read l").parse().expect("Invalid value for l"),
            r: s3.get(1).expect("Could not read r").parse().expect("Invalid value for r"),
        }
    }

    fn to_string(&self) -> String {
        // format!("{:>5},{},{},{},{},{},{},{},{},{},{}", self.quote, self.p1, self.p2, self.p3, self.p4, self.p5, self.p6, self.p7, self.p8, self.l, self.r)
        format!(
            "{:>5} | {} {} {} {} {} {} {} {} | {} {}",
            self.quote,
            self.p1,
            self.p2,
            self.p3,
            self.p4,
            self.p5,
            self.p6,
            self.p7,
            self.p8,
            self.l,
            self.r
        )
    }

    fn vec_from_string(v: &[&str]) -> Vec<Self> {
        v.iter().map(|z| Step::from_string(z)).collect()
    }

    fn pretty_print(&self) {
        println!(
            "{:>5} | {} {} {} {} {} {} {} {} | {} {}",
            self.quote,
            self.p1,
            self.p2,
            self.p3,
            self.p4,
            self.p5,
            self.p6,
            self.p7,
            self.p8,
            self.l,
            self.r
        );
    }

    fn pretty_print_vec(steps: &Vec<Self>) {
        println!("Quote | 1 2 3 4 5 6 7 8 | L R");
        println!("-----------------------------");
        for step in steps {
            step.pretty_print();
        }
    }
}

struct Program {
    head: Head, // Head is 22 bytes
    steps: Vec<Step>, // First step at offset 22, each step is 14 bytes
    read_bin: bool // Was program read from binary file?
}

impl Program {
    fn pretty_print(&self) {
        self.head.pretty_print();

        println!();
        Step::pretty_print_vec(&self.steps);
    }

    fn from_file(path: &std::path::PathBuf) -> Self {
        println!("Loading file: {:?}", path);

        match fs::read_to_string(path) {
            Ok(data) => {
                println!("Reading text format");
                let v: Vec<&str> = data.trim().split('\n').collect();
                let head = Head::from_string(&v.get(1).expect("Header missing"));
                let steps = Step::vec_from_string(&v[5..].iter().as_slice());
                Program { head, steps, read_bin: false}
            },
            Err(_e) => {
                println!("Reading binary format");
                let data = fs::read(path).expect("Unable to read file");
                let head = Head::from_bytes(&data[0..Head::SIZE]);
                let steps = Step::vec_from_bytes(&data[Head::SIZE..]);
                Program { head, steps, read_bin: true}
            }
        }
    }

    fn save_file_string(&self, path: &std::path::PathBuf) {
        let mut txt = "Wait L | Wait R\n".to_owned();
        txt.push_str(self.head.to_string().as_str());
        txt.push('\n');
        txt.push('\n');

        txt.push_str("Quote | 1 2 3 4 5 6 7 8 | L R\n");
        txt.push_str("-----------------------------\n");
        for step in &self.steps {
            txt.push_str(step.to_string().as_str());
            txt.push('\n');
        }

        fs::write(path, txt).unwrap();
        println!("Wrote txt file:{:?}", path);
    }


    fn save_file_bytes(&self, path: &std::path::PathBuf) {
        let mut bytes: Vec<u8> = self.head.to_bytes().to_vec();

        for step in &self.steps {
            bytes.append(&mut step.to_bytes().to_vec());
        }

        if 1422 < bytes.len() {
            panic!("Program is bigger than 1422 bytes!");
        }
        
        bytes.resize(1422, 0);

        fs::write(path, bytes).unwrap();
        println!("Wrote bytes file:{:?}", path);
    }
}

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short, long)]
    pretty_print_off: bool,

    #[structopt(parse(from_os_str))]
    in_file: std::path::PathBuf,
}


fn main() {
    let args = Cli::from_args();
    println!("{:#?}", args);

    let program = Program::from_file(&args.in_file);

    if !args.pretty_print_off {
        program.pretty_print();
    }


    let out_prefix = if program.read_bin {
        "txt"
    } else {
        "prg"
    };

    let mut out_file: std::path::PathBuf = args.in_file.with_extension(out_prefix);

    if out_file.is_file() {
        let file_name = out_file.file_name().unwrap().to_str().unwrap().to_string();
        let index = file_name.find('.').unwrap();
        
        let _ = (0..100).find(|i|{
            let mut new_file_name = file_name.clone();
            new_file_name.insert_str(index, format!("_{}", i.to_string().as_str()).as_str() );
            out_file.set_file_name(new_file_name);
            !out_file.is_file()
        }).expect("Could name new file! (could not add _0 to _99)");
    }
    
    println!("Saving output to: {:?}", out_file);

    if program.read_bin {
        program.save_file_string(&out_file);
    } else {
        program.save_file_bytes(&out_file);
    }

    println!("Done!");
}
