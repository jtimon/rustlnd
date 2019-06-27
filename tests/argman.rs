
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
fn test_0_equals_returns_false() {
    let raw_args = vec!["binname".to_string(), "-aaa".to_string()];
    let mut g_args = argman::ArgMan::new();
    g_args.add_arg_unset("-aaa", "Simple string arg");
    assert!(!g_args.parse_args_vec(raw_args));
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

#[test]
#[should_panic(expected = "Argument -aaa is not defined.")]
fn test_undefined_multi() {
    let raw_args = vec!["binname".to_string()];
    let mut g_args = argman::ArgMan::new();
    assert!(g_args.parse_args_vec(raw_args));
    g_args.get_multi("-aaa");
}

#[test]
#[should_panic(expected = "Argument -aaa is not set.")]
fn test_defined_unset_multi() {
    let raw_args = vec!["binname".to_string()];
    let mut g_args = argman::ArgMan::new();
    g_args.add_arg_unset("-aaa", "Simple string arg");
    assert!(g_args.parse_args_vec(raw_args));
    println!("{:?}", g_args.get_multi("-aaa"));
}

fn is_eq_str_vec(va: &Vec<String>, vb: &Vec<String>) -> bool {
    (va.len() == vb.len()) &&  // zip stops at the shortest
     va.iter()
       .zip(vb)
       .all(|(a,b)| *a == *b)
}

// Who tests the test tools?
#[test]
fn test_is_eq_vec() {
    let v1 = vec!["aaa".to_string(), "bbb".to_string()];
    let v1_repeated = vec!["aaa".to_string(), "bbb".to_string()];
    let v2 = vec!["aaa".to_string(), "ccc".to_string()];
    let v3 = vec!["aaa".to_string(), "bbb".to_string(), "ccc".to_string()];
    assert!(is_eq_str_vec(&v1, &v1_repeated));
    assert!(!is_eq_str_vec(&v1, &v2));
    assert!(!is_eq_str_vec(&v1, &v3));
}

#[test]
fn test_get_multistr_arg_1() {
    let raw_args = vec!["binname".to_string(), "-aaa=AAA".to_string()];
    let expected_vec = vec!["AAA".to_string()];
    let mut g_args = argman::ArgMan::new();
    g_args.add_arg_multi("-aaa", vec![], "Simple string arg");
    assert!(g_args.parse_args_vec(raw_args));
    println!("{:?}", g_args.get_multi("-aaa"));
    assert!(is_eq_str_vec(&expected_vec, g_args.get_multi("-aaa")));
}

#[test]
fn test_get_multistr_arg_2() {
    let raw_args = vec!["binname".to_string(), "-aaa=AAA".to_string(), "-aaa=BBB".to_string()];
    let expected_vec = vec!["AAA".to_string(), "BBB".to_string()];
    let mut g_args = argman::ArgMan::new();
    g_args.add_arg_multi("-aaa", vec![], "Simple string arg");
    assert!(g_args.parse_args_vec(raw_args));
    println!("{:?}", g_args.get_multi("-aaa"));
    assert!(is_eq_str_vec(&expected_vec, g_args.get_multi("-aaa")));
}

#[test]
fn test_defined_default_multistr_1() {
    let raw_args = vec!["binname".to_string()];
    let expected_vec = vec!["mydefault".to_string()];
    let mut g_args = argman::ArgMan::new();
    g_args.add_arg_multi("-aaa", expected_vec.clone(), "Simple string arg");
    assert!(g_args.parse_args_vec(raw_args));
    assert_eq!(1, g_args.get_multi("-aaa").len());
    println!("{:?}", g_args.get_multi("-aaa"));
    assert!(is_eq_str_vec(&expected_vec, g_args.get_multi("-aaa")));
}

#[test]
fn test_defined_default_multistrmulti_2() {
    let raw_args = vec!["binname".to_string()];
    let expected_vec = vec!["mydefault".to_string(), "mydefault2".to_string()];
    let mut g_args = argman::ArgMan::new();
    g_args.add_arg_multi("-aaa", expected_vec.clone(), "Simple string arg");
    assert!(g_args.parse_args_vec(raw_args));
    assert_eq!(2, g_args.get_multi("-aaa").len());
    println!("{:?}", g_args.get_multi("-aaa"));
    assert!(is_eq_str_vec(&expected_vec, g_args.get_multi("-aaa")));
}

#[test]
fn test_defined_default_changed_multistr_1() {
    let raw_args = vec!["binname".to_string(), "-aaa=notdefault1".to_string(), "-aaa=notdefault2".to_string()];
    let default_vec = vec!["mydefault".to_string()];
    let expected_vec = vec!["notdefault1".to_string(), "notdefault2".to_string()];
    let mut g_args = argman::ArgMan::new();
    g_args.add_arg_multi("-aaa", default_vec, "Simple string arg");
    assert!(g_args.parse_args_vec(raw_args));
    assert_eq!(2, g_args.get_multi("-aaa").len());
    println!("{:?}", g_args.get_multi("-aaa"));
    assert!(is_eq_str_vec(&expected_vec, g_args.get_multi("-aaa")));
}

#[test]
fn test_defined_default_changed_multistrmulti_2() {
    let raw_args = vec!["binname".to_string(), "-aaa=notdefault2".to_string()];
    let default_vec = vec!["mydefault".to_string(), "mydefault2".to_string()];
    let expected_vec = vec!["notdefault2".to_string()];
    let mut g_args = argman::ArgMan::new();
    g_args.add_arg_multi("-aaa", default_vec, "Simple string arg");
    assert!(g_args.parse_args_vec(raw_args));
    assert_eq!(1, g_args.get_multi("-aaa").len());
    println!("{:?}", g_args.get_multi("-aaa"));
    assert!(is_eq_str_vec(&expected_vec, g_args.get_multi("-aaa")));
}
