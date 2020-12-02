use regex::RegexBuilder;

#[test]
fn regex_clear_field_type() {
    let re = RegexBuilder::new(r"^Option < ([a-z\d\s<>]+) >$")
        .case_insensitive(true)
        .build()
        .unwrap();
    // invalids
    assert!(&re.captures_iter("String").next().is_none());
    assert!(&re.captures_iter("bool").next().is_none());
    assert!(&re.captures_iter("i32").next().is_none());
    assert!(&re.captures_iter("u32").next().is_none());
    assert!(&re.captures_iter("i64").next().is_none());
    assert!(&re.captures_iter("f64").next().is_none());
    assert!(&re.captures_iter("Vec < String >").next().is_none());
    // valids
    assert_eq!(
        "String",
        &re.captures_iter("Option < String >").next().unwrap()[1]
    );
    assert_eq!(
        "bool",
        &re.captures_iter("Option < bool >").next().unwrap()[1]
    );
    assert_eq!(
        "i32",
        &re.captures_iter("Option < i32 >").next().unwrap()[1]
    );
    assert_eq!(
        "u32",
        &re.captures_iter("Option < u32 >").next().unwrap()[1]
    );
    assert_eq!(
        "i64",
        &re.captures_iter("Option < i64 >").next().unwrap()[1]
    );
    assert_eq!(
        "f64",
        &re.captures_iter("Option < f64 >").next().unwrap()[1]
    );
    assert_eq!(
        "Vec < String >",
        &re.captures_iter("Option < Vec < String > >")
            .next()
            .unwrap()[1]
    );
}
