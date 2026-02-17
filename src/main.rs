mod shell_application;

fn main() {
    let console = shell_application::HostConsole;
    shell_application::run_shell(&console);
}
