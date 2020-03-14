#[allow(dead_code)]
#[rustversion::attr(stable(1.41), test)]
fn tests() {
    let t = trybuild::TestCases::new();

    t.pass("tests/css/css-selector-pass.rs");
}
