use structopt::StructOpt;
mod sysinfo;

#[derive(Debug, StructOpt)]
#[structopt(name = "SysInfo", about = "Utility to list System Information.")]

struct Options {
    #[structopt(name = "option")]
    /// os | d | mem | proc | cpu | comp | net
    ///
    /// os - List of Operating System Information
    ///
    /// d | disk - List of Disk Information
    ///
    /// mem | memory - System Memory Usage
    ///
    /// proc | processes - Information about all processes on system
    ///
    /// cpu - Number of System Cores
    ///
    /// comp | components - List of System Components
    ///
    /// net | network - Network Usage Statistics
    option: String,
}

fn main() {
    println!("{:?}", Options::from_args().option);
    sysinfo::call(&Options::from_args().option);
}