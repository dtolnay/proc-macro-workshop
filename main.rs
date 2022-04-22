// Write code here.
//
// To see what the code looks like after macro expansion:
//     $ cargo expand
//
// To run the code:
//     $ cargo run
#![allow(dead_code)]

use derive_builder::Builder;


type Option = ();
type Some = ();
type None = ();
type Result = ();
type Box = ();


#[derive(Builder)]
pub struct Command {
    executable: String,
    #[builder(each = "arg")]
    args: Vec<String>,
    #[builder(each = "env")]
    env: Vec<String>,
    current_dir: std::option::Option<String>,
}

fn main() {}
















/* 
pub struct Command {
    executable: String,
    #[builder(each = "arg")]
    args: Vec<String>,
    #[builder(each = "env")]
    env: Vec<String>,
    current_dir: Option<String>,
}
pub struct CommandBuilder {
    executable: Option<String>,
    args: Option<Vec<String>>,
    env: Option<Vec<String>>,
    current_dir: Option<String>,
}
impl Command {
    pub fn builder() -> CommandBuilder {
        CommandBuilder {
            executable: None,
            args: Some(vec![]),
            env: Some(vec![]),
            current_dir: None,
        }
    }
}
impl CommandBuilder {
    fn executable(&mut self, executable: String) -> &mut Self {
        self.executable = Some(executable);
        self
    }
    fn current_dir(&mut self, current_dir: String) -> &mut Self {
        self.current_dir = Some(current_dir);
        self
    }
    fn arg(&mut self, arg: String) -> &mut Self {
        let new_val = self.args.as_mut().unwrap();
        new_val.push(arg);
        //self.args = Some(new_val);
        self
    }
    fn env(&mut self, env: String) -> &mut Self {
        let new_value: Vec<String> = self.env.unwrap().as_ref();
        new_value.push(env);
        self.env = Some(new_value);
        self
    }
    pub fn build(&mut self) -> Result<Command, Box<dyn std::error::Error>> {
        let executable = std::mem::replace(&mut self.executable, None);
        let args = std::mem::replace(&mut self.args, None);
        let env = std::mem::replace(&mut self.env, None);
        let current_dir = std::mem::replace(&mut self.current_dir, None);
        Ok(Command {
            executable: match executable {
                Some(value) => value,
                None => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Error: field 'executable' haven't been set.",
                    )))
                }
            },
            args: match args {
                Some(value) => value,
                None => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Error: field 'args' haven't been set.",
                    )))
                }
            },
            env: match env {
                Some(value) => value,
                None => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Error: field 'env' haven't been set.",
                    )))
                }
            },
            current_dir,
        })
    }
}
fn main() {}
*/