use super::util::*;

macro_rules! t_modeline {
    ($name:ident, $input:literal) => {
        #[test]
        fn $name() {
            let (stdout, stderr) = get_output_from_str($input, ["-m".to_string()]);
            insta::assert_snapshot!(stdout);
            assert!(stderr.is_empty());
        }
    };

    (invalid $name:ident, $input:literal) => {
        #[test]
        fn $name() {
            let stuff = $input;
            eprintln!("{stuff:?}");
            let (stdout, stderr) = get_output_from_str($input, ["-m".to_string()]);
            insta::assert_snapshot!(concat!(stringify!($name), "_stdout"), stdout);
            insta::assert_snapshot!(concat!(stringify!($name), "_stderr"), stderr);
        }
    };
}

t_modeline!(
    test_correct_modeline,
    r"braille -r -3 4 4
-2
0
1

3
4"
);

t_modeline!(invalid test_invalid_modeline_prefix, r"braile -r -3 4 4
1
2
3
");

t_modeline!(invalid test_invalid_modeline_arg, r"braille --foo
1
2
3
");
