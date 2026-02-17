use std;
use clap::Parser;
pub trait Console {
    fn print(&self, s: &str);
    fn println(&self, s: &str);
    fn readline(&self) -> Result<String, std::io::Error>;
    fn panic(&self);
}
pub struct HostConsole;
impl Console for HostConsole {
    fn print(&self, s: &str) {
        print!("{}", s);
    }
    fn println(&self, s: &str) {
        println!("{}", s)
    }
    fn readline(&self) -> Result<String, std::io::Error> {
        let mut line = String::new();
        let result = std::io::stdin().read_line(&mut line);
        result.map(|_| line)
    }
    fn panic(&self) {
        panic!("Panicked!");
    }
}
// Entry point for the kernel in the future
pub fn run_shell(console: &dyn Console) {
    //init();

    shell_loop(console);
}


fn shell_loop(console: &dyn Console) { //TODO: Make divergent later
    loop {
        console.print("> ");

        let line = console.readline()
            .unwrap_or_else(|_err| {
                // Handle this error later
                console.panic();
                String::from("error")
            }).trim().to_string();

        let _args = parse(line);

        return;
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

//TODO: Finish later today
fn parse(_s: String) -> Args {


    todo!();
}

fn init() {
    todo!()
}