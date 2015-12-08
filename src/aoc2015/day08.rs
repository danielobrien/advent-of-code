    let mut code_len = 0;
    let mut mem_len = 0;
    for line in input.lines() {
        //println!("  line is {}", line);
        code_len += line.trim().len() as u32;
        mem_len += get_mem_len(line);
        //println!("{}: {} - {} = {}", line, code_len, mem_len, code_len - mem_len);
    }
    
    vec![Ok(code_len - mem_len)]
}

fn get_mem_len(line: &str) -> u32 {
    let mut offset = 2;
    //println!("    str is: {}", line);
    let mut line = line.replace("\\\"", "\"");
    //println!("    replaced quotes: {}", line);
    line = line.replace("\\\\", "\\"); //"
    //println!("    replaced backslashes: {}", line); 
    while let Some(i) = line.find("\\x") {
        println!("{}", i);
        let v: Vec<char> = line.chars().collect();
        if v.get(i+2).unwrap().is_digit(16) && v.get(i+3).unwrap().is_digit(16) {
            line.replace("\\x", "Z");
            offset += 2;
        } else {
            line.replace("\\x", "ZZ");
        }
        println!("{}", line);
        //println!("    Found hex-escape at {} in {}", i, line);
        
    }
    line.len() as u32 - offset
}
