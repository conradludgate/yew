use yew::prelude::*;

fn compile_pass1() {
    let stylesheet = css! { .foo {} };
    let expected = ".foo {}";
    assert_eq!(stylesheet, expected);
}

fn compile_pass2() {
    let (foo, bar, foo_bar_baz);
    let stylesheet = css! {
        $foo {}
        $bar {}
        $foo_bar_baz {}
    };

    // Can't determine what order the variables get assigned
    // so the numbers at the end cannot be determined
    // The order they appear in the final stylesheet is determined, however
    assert!(foo.starts_with("foo"));
    assert!(bar.starts_with("bar"));
    assert!(foo_bar_baz.starts_with("foo-bar-baz"));

    let expected = format!(
        ".{} {{}}
.{} {{}}
.{} {{}}",
        foo, bar, foo_bar_baz
    );
    assert_eq!(stylesheet, expected);
}

fn compile_pass3() {
    let stylesheet = css! {
        .a & .b, .c {}
    };
    let expected = ".a.b,
.c {}";
    assert_eq!(stylesheet, expected);
}

fn main() {
    compile_pass1();
    compile_pass2();
    compile_pass3();
}
