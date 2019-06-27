
use std::collections::HashMap;
use std::env;

#[derive(Debug)]
struct ArgumentHelp {
    default: Option<String>,
    description: String,
}

pub struct ArgMan {
    args: HashMap<String, String>,
    args_help: HashMap<String, ArgumentHelp>,
}

impl ArgMan {

    pub fn new() -> ArgMan {
        ArgMan {
            args_help: HashMap::new(),
            args: HashMap::new(),
        }
    }

    pub fn add_arg_unset(&mut self, name: &str, description: &str) {
        self.args_help.insert(name.to_string(), ArgumentHelp{
            description: description.to_string(),
            default: None,
        });
    }

    pub fn add_arg(&mut self, name: &str, default: String, description: &str) {
        self.args_help.insert(name.to_string(), ArgumentHelp{
            description: description.to_string(),
            default: Some(default),
        });
    }

    pub fn print_help(&self) {
        println!("\nUSAGE:\n");

        for (name, arg_help) in &self.args_help {
            println!("{}:", name);
            let common_text = format!("    {}", arg_help.description).to_string();
            match &arg_help.default {
                Some(default) => println!("{} (Default: {})", common_text, default),
                None => println!("{}", common_text),
            }
        }
    }

    fn set_arg(&mut self, name: &str, value_to_add: String) {
        self.args.insert(name.to_string(), value_to_add);
    }

    pub fn set_defaults(&mut self) {
        for (name, arg_help) in &self.args_help {

            if !self.args.contains_key(name) {
                match &arg_help.default {
                    None => println!("Warning: No default for unset argument {}", name),
                    Some(default_value) => {
                        println!("Insert default argument : {}: {:?}", name, default_value);
                        self.args.insert(name.to_string(), default_value.to_string());
                    },
                }
            }
        }
    }

    pub fn parse_args(&mut self) -> bool {
        return self.parse_args_vec(env::args().collect());
    }

    pub fn parse_args_vec(&mut self, raw_args: Vec<String>) -> bool {

        println!("\nraw_args: {:?}", raw_args);
        for raw_arg in raw_args.iter().skip(1) {

            if raw_arg == "--help" {
                self.print_help();
                return false;
            }

            let raw_arg_split : Vec<&str> = raw_arg.split("=").collect();
            if raw_arg_split.len() != 2 {
                println!("Incorrect argument syntax: {}\n", raw_arg);
                println!("There must be one and only be one '=' symbol per argument.");
                println!("Try '{} --help'\n", raw_args[0]);
                return false;
            }

            let name = raw_arg_split[0];
            if !self.args_help.contains_key(name) {

                println!("Unknown argument {}\n", name);
                println!("Try '{} --help'\n", raw_args[0]);
                return false;
            } else {
                let value_to_add = raw_arg_split[1].to_string();
                self.set_arg(name, value_to_add);
            }
        }

        // Set defaults last if they haven't been set
        self.set_defaults();

        true
    }

    pub fn is_none(&self, arg_name: &str) -> bool {
        return self.args.get(arg_name).is_none();
    }

    pub fn get(&self, arg_name: &str) -> &str {
        if !self.args_help.contains_key(arg_name) {
            panic!("Argument {} is not defined.", arg_name);
        }

        if self.is_none(arg_name) {
            panic!("Argument {} is not set.", arg_name);
        }
        return self.args.get(arg_name).unwrap();
    }

    pub fn dev_print_selected_args(&self) {
        println!("\nThe following args were selected:\n");
        for (name, arg) in &self.args {
            println!("{}: {:?}", name, arg);
        }
    }
}
