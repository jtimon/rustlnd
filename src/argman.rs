
use std::collections::HashMap;
use std::env;

#[derive(Debug)]
enum ArgType {
    ArgBool,
    ArgMultistr,
    ArgMapStr,
    ArgStr,
}

#[derive(Debug)]
struct ArgumentHelp {
    description: String,
    arg_type: ArgType,
    default: Option<String>,
    default_multi: Vec<String>,
    default_map: HashMap<String, String>,
}

pub struct ArgMan {
    args: HashMap<String, String>,
    args_help: HashMap<String, ArgumentHelp>,
    args_multi: HashMap<String, Vec<String>>,
    args_multi_map: HashMap<String, HashMap<String, String>>,
}

impl ArgMan {

    pub fn new() -> ArgMan {
        ArgMan {
            args_help: HashMap::new(),
            args: HashMap::new(),
            args_multi: HashMap::new(),
            args_multi_map: HashMap::new(),
        }
    }

    pub fn add_arg_unset(&mut self, name: &str, description: &str) {
        self.args_help.insert(name.to_string(), ArgumentHelp{
            arg_type: ArgType::ArgStr,
            default: None,
            default_multi: vec![],
            default_map: HashMap::new(),
            description: description.to_string(),
        });
    }

    pub fn add_arg(&mut self, name: &str, default: String, description: &str) {
        self.args_help.insert(name.to_string(), ArgumentHelp{
            arg_type: ArgType::ArgStr,
            default: Some(default),
            default_multi: vec![],
            default_map: HashMap::new(),
            description: description.to_string(),
        });
    }

    pub fn add_arg_bool(&mut self, name: &str, default: String, description: &str) {
        if default != "0" && default != "1" {
            println!("The default was {}", default);
            panic!("A bool arg can only be 0 or 1 by default (and in general too)");
        }
        self.args_help.insert(name.to_string(), ArgumentHelp{
            description: description.to_string(),
            default: Some(default),
            default_multi: vec![],
            default_map: HashMap::new(),
            arg_type: ArgType::ArgBool,
        });
    }

    pub fn add_arg_multi(&mut self, name: &str, default_multi: Vec<String>, description: &str) {
        self.args_help.insert(name.to_string(), ArgumentHelp{
            description: description.to_string(),
            default: None,
            default_multi,
            default_map: HashMap::new(),
            arg_type: ArgType::ArgMultistr,
        });
    }

    pub fn add_arg_with_category(&mut self, name: &str, default_map: HashMap<String, String>, description: &str) {
        self.args_help.insert(name.to_string(), ArgumentHelp{
            description: description.to_string(),
            default: None,
            default_multi: vec![],
            default_map,
            arg_type: ArgType::ArgMapStr,
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

    fn set_arg(&mut self, name: &str, value_to_add: String) -> bool {

        let (parse_ok, parsed_name, category) = ArgMan::get_parsed_name_cateory(name);
        if !parse_ok {
            return false;
        }

        match self.args_help.get(parsed_name).unwrap().arg_type {

            ArgType::ArgStr => {
                self.args.insert(parsed_name.to_string(), value_to_add);
            },

            ArgType::ArgBool => {
                match &value_to_add[..] {
                    "0" => {},
                    "1"  => {},
                    _ => {
                        println!("'{}' cannot be parsed as bool (only '0' or '1' allowed')", parsed_name);
                        return false;
                    },
                }
                self.args.insert(parsed_name.to_string(), value_to_add);
            },

            ArgType::ArgMultistr => {
                if self.args_multi.contains_key(parsed_name) {
                    self.args_multi.get_mut(parsed_name).unwrap().push(value_to_add);
                } else {
                    self.args_multi.insert(parsed_name.to_string(), vec![value_to_add]);
                }
            },

            ArgType::ArgMapStr => {

                if self.args_multi_map.contains_key(parsed_name) {

                    self.args_multi_map.get_mut(parsed_name).unwrap().insert(category.to_string(), value_to_add);
                } else {

                    let mut per_name_map : HashMap<String, String> = HashMap::new();
                    per_name_map.insert(category.to_string(), value_to_add);
                    self.args_multi_map.insert(parsed_name.to_string(), per_name_map);
                }
            },
        }
        true
    }

    pub fn set_defaults(&mut self) {
        for (name, arg_help) in &self.args_help {
            match arg_help.arg_type {

                ArgType::ArgStr => {
                    if !self.args.contains_key(name) {
                        match &arg_help.default {
                            None => println!("Warning: No default for unset argument {}", name),
                            Some(default_value) => {
                                println!("Insert default argument : {}: {:?}", name, default_value);
                                self.args.insert(name.to_string(), default_value.to_string());
                            },
                        }
                    }
                },

                ArgType::ArgBool => {
                    if arg_help.default.is_none() {
                        panic!("Bool args should always have a default unlike somehow bool arg '{}'", name);
                    } else {
                        if !self.args.contains_key(name) {
                            println!("Insert default argument : {}: {:?}", name, &arg_help.default);
                            self.args.insert(name.to_string(), arg_help.default.clone().unwrap());
                        }
                    }
                },

                ArgType::ArgMultistr => {
                    if !self.args_multi.contains_key(name) {
                        self.args_multi.insert(name.to_string(), arg_help.default_multi.clone());
                    }
                },

                ArgType::ArgMapStr => {
                    if !self.args_multi_map.contains_key(name) {
                        self.args_multi_map.insert(name.to_string(), arg_help.default_map.clone());
                    } else {
                        // TODO set each default independently if not set
                        // for category, cat_val in self.args_multi_map.get(name).items() {
                        // }
                    }
                },
            }
        }
    }

    fn get_parsed_name_cateory(name: &str) -> (bool, &str, &str) {

        let name_split : Vec<&str> = name.split(".").collect();
        if name_split.len() == 2 {

            return (true, name_split[1], name_split[0]);
        } else if name_split.len() != 1 {

            println!("Incorrect argument syntax: {}\n", name);
            println!("There must be one and only one '.' symbol per map argument or none for other arguments.");
            return (false, "", "");
        }

        (true, name, "")
    }

    fn check_defined_argument(&self, name: &str, bin_nme: &str) -> bool {
        if !self.args_help.contains_key(name) {
            println!("Unknown argument {}\n", name);
            println!("Try '{} --help'\n", bin_nme);
            return false;
        }
        true
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
            if raw_arg_split.len() != 1 && raw_arg_split.len() != 2 {
                println!("Incorrect argument syntax: {}\n", raw_arg);
                println!("There cannot be more than one '=' symbol per argument.");
                println!("Try '{} --help'\n", raw_args[0]);
                return false;
            }

            let name = raw_arg_split[0];
            let (parse_ok, parsed_name, _category) = ArgMan::get_parsed_name_cateory(name);
            if !parse_ok || !self.check_defined_argument(parsed_name, &raw_args[0]) {
                return false;
            }

            {
                let value_to_add;
                if raw_arg_split.len() == 1 {
                    match self.args_help.get(parsed_name).unwrap().arg_type {
                        ArgType::ArgBool => {
                            value_to_add = "1".to_string();
                        },
                        _ => {
                            println!("Incorrect argument syntax: {}\n", raw_arg);
                            println!("Argument {} is not a bool and needs an '=' symbol before its value.\n", parsed_name);
                            println!("Try '{} --help'\n", raw_args[0]);
                            return false;
                        },
                    }
                } else {
                    value_to_add = raw_arg_split[1].to_string();
                }

                if !self.set_arg(name, value_to_add) {
                    println!("Try '{} --help'\n", raw_args[0]);
                    return false;
                }
            }
            println!("\nname : {:?}", name);
        }

        // Set defaults last if they haven't been set
        self.set_defaults();

        true
    }

    pub fn is_none(&self, arg_name: &str) -> bool {
        return self.args.get(arg_name).is_none() && self.args_multi.get(arg_name).is_none();
    }

    fn _common_get(&self, arg_name: &str) {
        if !self.args_help.contains_key(arg_name) {
            panic!("Argument {} is not defined.", arg_name);
        }

        match self.args_help.get(arg_name).unwrap().arg_type {
            ArgType::ArgStr => {
                if self.args.get(arg_name).is_none() {
                    panic!("Argument {} is not set.", arg_name);
                }
            },
            ArgType::ArgBool => {
                if self.args.get(arg_name).is_none() {
                    panic!("Argument {} is not set.", arg_name);
                }
            },
            ArgType::ArgMultistr => {
                if self.args_multi.get(arg_name).is_none() {
                    panic!("Argument {} is not set.", arg_name);
                }
            },
            ArgType::ArgMapStr => {
                if self.args_multi_map.get(arg_name).is_none() {
                    panic!("Argument {} is not set.", arg_name);
                }
            },
        }
        // if self.args.get(arg_name).is_none() {
        //     if self.args_multi.get(arg_name).is_none() {
        //         panic!("Argument {} is not set.", arg_name);
        //     } else if self.args_multi_map.get(arg_name).is_none() {
        //         panic!("Argument {} is not set.", arg_name);
        //     } else {
        //         match
        //         panic!("Argument {} is an argument that can be repeated, try 'g_args.get_multi(\"{}\")'.", arg_name, arg_name);
        //     } else {
        //     }
        // }
    }

    pub fn get(&self, arg_name: &str) -> &str {
        self._common_get(arg_name);

        match self.args_help.get(arg_name).unwrap().arg_type {
            ArgType::ArgStr => {
                return &self.args.get(arg_name).unwrap()[..];
            },
            _ => panic!("get is being used for {}, which is not defined as a str arg", arg_name),
        }
    }

    pub fn get_by_category(&self, category: &str, arg_name: &str) -> &str {
        if !self.args_help.contains_key(arg_name) {
            panic!("Argument {} is not defined.", arg_name);
        }

        match self.args_help.get(arg_name).unwrap().arg_type {
            ArgType::ArgMapStr => {

                if !self.args_multi_map.contains_key(arg_name) {
                    panic!("Argument {} is not set.", arg_name);
                }

                if !self.args_multi_map.get(arg_name).unwrap().contains_key(category) {
                    panic!("no {} category for argument {}", category, arg_name);
                }

                return &self.args_multi_map.get(arg_name).unwrap().get(category).unwrap()[..];
            },
            _ => panic!("get is being used for {}, which is not defined as a map arg", arg_name),
        }
    }

    pub fn get_bool(&self, arg_name: &str) -> bool {
        self._common_get(arg_name);

        match self.args_help.get(arg_name).unwrap().arg_type {
            ArgType::ArgBool => {
                let str_val = &self.args.get(arg_name).unwrap()[..];
                match str_val {
                    "0" => return false,
                    "1" => return true,
                    _ => panic!("Argument {} is a bool and can only be 0 or 1"),
                }
            },
            _ => panic!("get_bool is being used for {}, which is not defined as a bool arg", arg_name),
        }
    }

    pub fn get_multi(&self, arg_name: &str) -> &Vec<String> {
        if !self.args_help.contains_key(arg_name) {
            panic!("Argument {} is not defined.", arg_name);
        }

        if self.args_multi.get(arg_name).is_none() {
            if self.args.get(arg_name).is_none() {
                panic!("Argument {} is not set.", arg_name);
            } else {
                panic!("Argument {} is an argument that cannot be repeated, try 'g_args.get(\"{}\")'.", arg_name, arg_name);
            }
        }

        return self.args_multi.get(arg_name).unwrap();
    }

    pub fn dev_print_selected_args(&self) {
        println!("\nThe following args were selected:\n");
        for (name, arg) in &self.args {
            println!("{}: {:?}", name, arg);
        }
        println!("\nThe following args_multi were selected:\n");
        for (name, arg) in &self.args_multi {
            println!("{}: {:?}", name, arg);
        }
    }
}
