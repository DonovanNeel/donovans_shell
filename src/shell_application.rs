use std;
trait Console {
    fn print(&self, s: &str);
    fn println(&self, s: &str);
    fn readline(&self) -> Result<String, std::io::Error>;
}
struct HostConsole;
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
}
pub fn run_shell() {

}