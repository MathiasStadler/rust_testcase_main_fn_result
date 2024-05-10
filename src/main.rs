use std::process::{ExitCode, Termination};

pub enum LinuxExitCode { ExitOK, ExitERR(u8) }

impl Termination for LinuxExitCode {
   fn report(self) -> ExitCode {
     match self {
       LinuxExitCode::ExitOK => ExitCode::SUCCESS,
       LinuxExitCode::ExitERR(v) => ExitCode::from(v)
     }
   }
}
fn main() -> LinuxExitCode {
    LinuxExitCode::ExitERR(3)
}
