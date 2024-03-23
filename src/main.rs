use std::env;

#[derive(Debug)]
enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}

fn format_size(size: u64) -> String {
    let filesize = match size {
        0           ..=1_023            => FileSize::Bytes(size),
        1_024       ..=1_048_575        => FileSize::Kilobytes(size as f64 / 1024.0),
        1_048_576   ..=1_073_741_823    => FileSize::Megabytes(size as f64 / 1_048_576.0),
        _                               => FileSize::Gigabytes(size as f64 / 1_073_741_824.0),
    };

    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.2} KB", kb),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb),
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("At least an argument is required!\n\tProgram ends"); 
        return();
    }

    let tokens:Vec<&str>= args[1].split_whitespace().collect();
    println!("{:?}", tokens);
    if tokens.len() < 2 {
        println!("At least a string with 2 tokens is required!\n\tProgram ends"); 
        return();
    }

    let mut passed_size = 0i64;
    let turbo_parsed = tokens[0].parse::<i64>();
    if turbo_parsed.is_err() {
        println!("The passed string have to have a parse problem! (Number too big?)\n\tProgram ends"); 
        return();
    } 

    match turbo_parsed.ok() {
        None => {
            println!("The passed string have to have as fist token a number!\n\tProgram ends"); 
            return();
        },
        Some(x) => passed_size=x,
    }
    println!("passed_size = {}", passed_size);

    let mut multiplier = 1i64;
    match tokens[1] {
        "kb" | "Kb" | "KB" | "KByte" => {multiplier=1024i64;},
        "mb" | "Mb" | "MB" | "MByte" => {multiplier=1024i64*1024i64;},
        _ => {multiplier = 1i64; println!("Invalid multiplier: multiplier set to 1");},
    }
    //println!("multiplier = {}", multiplier);

    let raw_size = (multiplier as f64) * (passed_size as f64);

    let result = vec! [FileSize::Bytes(raw_size as u64), 
                                     FileSize::Kilobytes(raw_size/1024.0),
                                     FileSize::Megabytes(raw_size/1024.0/1024.0),
                                     FileSize::Gigabytes(raw_size/1024.0/1024.0/1024.0),
    ];

    println!("{:?}", result);
    let result = format_size(raw_size as u64);
    println!("{}", result)
}
