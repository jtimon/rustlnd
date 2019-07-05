
/// Let's make sure ArgMan behaves the way is supposed to in a way that's simple to read

use rustlnd::argman;

#[test]
fn test_get_str_arg() {
    let raw_args = vec!["binname".to_string(), "-aaa=EXPECTED_STR".to_string()];
    let mut g_args = argman::ArgMan::new();
    g_args.add_arg_unset("-aaa", "Simple string arg");
    assert!(g_args.parse_args_vec(raw_args));
    assert_eq!(g_args.get("-aaa"), "EXPECTED_STR".to_string());
}

#[test]
fn test_help_returns_false() {
    let raw_args = vec!["binname".to_string(), "--help".to_string()];
    let mut g_args = argman::ArgMan::new();
    assert!(!g_args.parse_args_vec(raw_args));
}

#[test]
fn test_2_equals_returns_false() {
    let raw_args = vec!["binname".to_string(), "--aaa=bbb=ccc".to_string()];
    let mut g_args = argman::ArgMan::new();
    g_args.add_arg_unset("-aaa", "Simple string arg");
    assert!(!g_args.parse_args_vec(raw_args));
}

#[test]
fn test_set_0_equals_returns_false() {
    let raw_args = vec!["binname".to_string(), "-aaa".to_string()];
    let mut g_args = argman::ArgMan::new();
    g_args.add_arg_unset("-aaa", "Simple string arg");
    assert!(!g_args.parse_args_vec(raw_args));
}

#[test]
fn test_bool_0_equals_returs_true() {
    let raw_args = vec!["binname".to_string(), "-aaa".to_string()];
    let mut g_args = argman::ArgMan::new();
    g_args.add_arg_bool("-aaa", "0".to_string(), "Simple string arg");
    g_args.parse_args_vec(raw_args.clone());
    assert!(g_args.parse_args_vec(raw_args));
    assert_eq!(true, g_args.get_bool("-aaa"));
}

#[test]
fn test_unknown_argument_returns_false() {
    let raw_args = vec!["binname".to_string(), "-aaa=bbb".to_string()];
    let mut g_args = argman::ArgMan::new();
    assert!(!g_args.parse_args_vec(raw_args));
}

#[test]
#[should_panic(expected = "Argument -aaa is not defined.")]
fn test_undefined() {
    let raw_args = vec!["binname".to_string()];
    let mut g_args = argman::ArgMan::new();
    assert!(g_args.parse_args_vec(raw_args));
    g_args.get("-aaa");
}

#[test]
#[should_panic(expected = "Argument -aaa is not set.")]
fn test_defined_unset() {
    let raw_args = vec!["binname".to_string()];
    let mut g_args = argman::ArgMan::new();
    g_args.add_arg_unset("-aaa", "Simple string arg");
    assert!(g_args.parse_args_vec(raw_args));
    g_args.get("-aaa");
}

#[test]
fn argman_defined_default() {
    let raw_args = vec!["binname".to_string()];
    let mut g_args = argman::ArgMan::new();
    g_args.add_arg("-aaa", "mydefault".to_string().clone(), "Simple string arg");
    assert!(g_args.parse_args_vec(raw_args));
    assert_eq!("mydefault".to_string(), g_args.get("-aaa"));
}

#[test]
fn argman_changed_default() {
    let raw_args = vec!["binname".to_string(), "-aaa=notdefault".to_string()];
    let mut g_args = argman::ArgMan::new();
    let default_str = "mydefault".to_string();
    g_args.add_arg("-aaa", default_str.clone(), "Simple string arg");
    assert!(g_args.parse_args_vec(raw_args));
    assert_eq!("notdefault".to_string(), g_args.get("-aaa"));
}

#[test]
#[should_panic(expected = "A bool arg can only be 0 or 1 by default (and in general too)")]
fn test_bool_invalid_format_default() {
    let raw_args = vec!["binname".to_string()];
    println!("{:?}", raw_args);
    let mut g_args = argman::ArgMan::new();
    g_args.add_arg_bool("-aaa", "bbb".to_string(), "Simple string arg");
    g_args.parse_args_vec(raw_args);
}

#[test]
fn test_bool_invalid_format_selection() {
    let raw_args = vec!["binname".to_string(), "-aaa=bbb".to_string()];
    println!("{:?}", raw_args);
    let mut g_args = argman::ArgMan::new();
    g_args.add_arg_bool("-aaa", "0".to_string(), "Simple string arg");
    assert!(!g_args.parse_args_vec(raw_args));
}

fn str2bool(src: &str) -> bool {
    match src {
        "0" => return false,
        "1"  => return true,
        _ => panic!("str2bool cannot parse {}", src),
    }
}

#[test]
fn test_get_bool_arg_default() {
    for default in vec!["0", "1"] {

        let raw_args = vec!["binname".to_string()];
        println!("{:?}", raw_args);
        let mut g_args = argman::ArgMan::new();
        g_args.add_arg_bool("-aaa", default.to_string(), "Simple string arg");
        assert!(g_args.parse_args_vec(raw_args));
        assert_eq!(g_args.get_bool("-aaa"), str2bool(default));
    }
}

#[test]
fn test_get_bool_arg_selection() {
    for default in vec!["0", "1"] {
        for selection in vec!["0", "1"] {

            let raw_args = vec!["binname".to_string(), format!("-aaa={}", selection).to_string()];
            println!("{:?}", raw_args);
            let mut g_args = argman::ArgMan::new();
            g_args.add_arg_bool("-aaa", default.to_string(), "Simple string arg");
            assert!(g_args.parse_args_vec(raw_args));
            assert_eq!(g_args.get_bool("-aaa"), str2bool(selection));
        }
    }
}
