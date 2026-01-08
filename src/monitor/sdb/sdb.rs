use std::str::FromStr;
use rustyline::DefaultEditor;
use log::{error, debug};
use rustyline::error::ReadlineError;
use strum::{EnumIter, EnumMessage, IntoEnumIterator};

use crate::monitor::sdb::expr::init_regex;
use crate::monitor::sdb::watchpoint::init_wp;
use crate::monitor::Emulator;

#[derive(Debug, PartialEq, EnumMessage, EnumIter)]
pub enum SdbCommand {
    #[strum(serialize = "help", serialize = "h", message = "help [cmd]: print help message")]
    Help(Option<String>),
    #[strum(serialize = "quit", serialize = "q", message = "quit: quit sdb")]
    Quit,
    #[strum(serialize = "continue", serialize = "c", message = "continue: continue to next breakpoint")]
    Continue,
    #[strum(serialize = "step", serialize = "s", message = "step [num]: step [num] instructions, default is 1")]
    Step(u64),
    #[strum(serialize = "singlestep", serialize = "si", message = "single step, same as step 1")]
    SingleStep,
    #[strum(serialize = "breakpoint", serialize = "b", message = "break: set breakpoint")]
    BreakPoint,
}

#[derive(Debug)]
pub struct ParserSdbCommandError(String);
#[derive(Debug)]
pub struct SdbCommandExecError;

impl std::fmt::Display for ParserSdbCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ParserSdbCommandError {}

#[derive(Debug, Clone, Copy)]
pub enum SdbMainloopState {
    Running,
    Quit
}

impl FromStr for SdbCommand {
    type Err =  ParserSdbCommandError;

    // 优化
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let args: Vec<&str> = s.split_whitespace().collect();
        let op = args[0];

        let res = match op {
            "help" | "h" => {
                match args.len() {
                    1 => Ok(SdbCommand::Help(None)),
                    2 => Ok(SdbCommand::Help(Some(args[1].to_string()))),
                    _ => Err(ParserSdbCommandError("help arg must be 0 or 1".to_string()))
                }
            },
            "quit" | "q" => Ok(SdbCommand::Quit),
            "continue" | "c" => Ok(SdbCommand::Continue),
            "step" | "s" => {
                let step = match args.len() {
                    1 => Ok(1),
                    2 => {
                        let step = args[1].parse::<u64>();
                        step
                    },
                    _ => return Err(ParserSdbCommandError("too many args".to_string()))
                };
                match step {
                    Ok(step) => Ok(SdbCommand::Step(step)),
                    Err(_) => Err(ParserSdbCommandError("step arg must be a number".to_string()))
                }
            },
            "breakpoint" | "b" => Ok(SdbCommand::BreakPoint),
            "si" | "singlestep" => Ok(SdbCommand::SingleStep),
            _ => Err(ParserSdbCommandError(format!("Unknown command: {}", op)))
        };

        res
    }
}

impl SdbCommand {
    pub fn exec(&self, emu: &mut Emulator) -> Result<SdbMainloopState, SdbCommandExecError> {
        match self {
            SdbCommand::Help(arg) => exec_help(arg),
            SdbCommand::Continue => exec_continue(emu),
            SdbCommand::Quit => exec_quit(emu),
            // 这里需要手动解引用
            SdbCommand::Step(step) => exec_steps(emu, *step),
            SdbCommand::SingleStep => exec_singlestep(emu),
            _ => Err(SdbCommandExecError),
        }
    }
}

pub fn init_sdb() {
    init_regex();
    init_wp();
}

pub fn sdb_mainloop(emu: &mut Emulator) {
    let mut rl = DefaultEditor::new().expect("Error in initializing rustyline.");
    #[cfg(feature = "with-file-history")]
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    loop {
        let raw_line = rl.readline("(sdb) ");
        match raw_line {
            Ok(line) => {
                // skip empty line
                if line.is_empty() {
                    continue;
                }
                debug!("{}", line);
                // add cmd to history
                #[cfg(feature = "with-file-history")]
                if let Err(e) = rl.add_history_entry(line.as_str()) {
                    error!("Error in adding history: {}", e);
                }
                
                // parse cmd
                let ret: Result<SdbCommand, _> = line.parse();
                match ret {
                    Ok(cmd) => {
                        // 执行命令
                        match cmd.exec(emu) {
                            Ok(SdbMainloopState::Running) => {
                                // nothing to do
                            }
                            Ok(SdbMainloopState::Quit) => {
                                break;
                            }
                            Err(e) => {
                                debug!("SdbCommandExecError: {:?}", e);
                            }
                        }
                    },
                    Err(e) => {
                        println!("{}", e.0);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                // 处理Ctrl+C
                debug!("CTRL-C");
                break;
            }
            Err(_) => {
                error!("Error in reading line.");
            }
        }
    }

    #[cfg(feature = "with-file-history")]
    rl.save_history("history.txt").expect("Error in saving history.");
}

// Cmd handler
fn exec_help(arg: &Option<String>) -> Result<SdbMainloopState, SdbCommandExecError> {
    debug!("execute help");
    match arg {
        Some(arg) => {
            match SdbCommand::from_str(arg) {
                Ok(cmd) => {
                    if let Some(cmd_info) = cmd.get_message() {
                        let cmd_name = cmd.get_serializations();
                        let alias_str = cmd_name.join("|");
                        println!("  {:<20} - {}", alias_str, cmd_info);
                    }
                }
                Err(e) => { 
                    println!("{}", e.0);
                }
            }
        }
        None => {
            for cmd in SdbCommand::iter() {
                if let Some(cmd_info) = cmd.get_message() {
                    let cmd_name = cmd.get_serializations();
                    let alias_str = cmd_name.join("|");
                    println!("  {:<20} - {}", alias_str, cmd_info);
                }
            }
        }
    }

    Ok(SdbMainloopState::Running)
}

fn exec_continue(emu: &mut Emulator) -> Result<SdbMainloopState, SdbCommandExecError> {
    debug!("execute continue");
    emu.exec_continue();
    Ok(SdbMainloopState::Running)
}

fn exec_quit(emu: &mut Emulator) -> Result<SdbMainloopState, SdbCommandExecError> {
    debug!("execute quit");
    Ok(SdbMainloopState::Quit)
}

fn exec_steps(emu: &mut Emulator, steps: u64) -> Result<SdbMainloopState, SdbCommandExecError> {
    emu.exec(steps);
    Ok(SdbMainloopState::Running)
}

fn exec_singlestep(emu: &mut Emulator) -> Result<SdbMainloopState, SdbCommandExecError> {
    exec_steps(emu, 1)
}