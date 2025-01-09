use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("please use either toc or tof with a number");
    }

    let cmd = &args[1];
    let temp: f32 = (&args[2]).parse().expect("please use only numbers");

    if cmd == "toc" {
        let temp_c = (temp - 32.0)*5.0/9.0;
        eprintln! ("Temperature = {temp_c}C");
    }
    else if cmd == "tof" {
        let temp_f = (temp*9.0/5.0) + 32.0;
        eprintln!("Temperature = {temp_f}F")
    }

    else {
        eprintln! ("Invalid Argument, Please use a valid argument")
    }


}
