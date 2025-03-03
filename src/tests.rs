use crate::args::Arg;

#[test]
#[cfg(any(windows, unix))]
fn wtf_shenanigans_1() {
    use crate::args::{split_os_argument, Arg, ArgType};
    use std::ffi::OsString;

    for (i_c, prefix) in [
        (ArgType::Short, "f"),
        (ArgType::Long, "foo"),
        (ArgType::Long, "口水鸡"),
    ] {
        let i_prefix = OsString::from(prefix);
        let i_suffix;
        #[cfg(windows)]
        {
            use std::os::windows::ffi::OsStringExt;
            i_suffix = OsString::from_wide(&[0x0066, 0x006f, 0xD800, 0x006f]);
        }
        #[cfg(not(windows))]
        {
            use std::os::unix::ffi::OsStringExt;
            i_suffix = OsString::from_vec(vec![0x66, 0x6f, 0xD8, 0x6f]);
        }

        let mut os_string = OsString::new();
        match i_c {
            ArgType::Short => os_string.push("-"),
            ArgType::Long => os_string.push("--"),
        }
        os_string.push(&i_prefix);
        os_string.push("=");
        os_string.push(&i_suffix);

        let (o_c, o_prefix, o_suffix) = split_os_argument(&os_string).unwrap();
        assert_eq!(i_c, o_c);
        assert_eq!(i_prefix.to_str().unwrap(), o_prefix);
        assert_eq!(Arg::Word(i_suffix), o_suffix.unwrap());
    }
}

#[test]
fn wtf_shenanigans_2() {
    use crate::args::{split_os_argument, split_os_argument_fallback, ArgType};
    use std::ffi::OsString;

    for (i_c, prefix) in [
        (ArgType::Short, "f"),
        (ArgType::Long, "foo"),
        (ArgType::Long, "口水鸡"),
    ] {
        let i_prefix = OsString::from(prefix);
        let i_suffix = OsString::from("口水鸡");

        let mut os_string = OsString::new();
        match i_c {
            ArgType::Short => os_string.push("-"),
            ArgType::Long => os_string.push("--"),
        }
        os_string.push(&i_prefix);
        os_string.push("=");
        os_string.push(&i_suffix);

        let (o_c, o_prefix, o_suffix) = split_os_argument(&os_string).unwrap();
        assert_eq!(i_c, o_c);
        assert_eq!(i_prefix.to_str().unwrap(), o_prefix);
        assert_eq!(Arg::Word(i_suffix.clone()), o_suffix.unwrap());

        let (o_c, o_prefix, o_suffix) = split_os_argument_fallback(&os_string).unwrap();
        assert_eq!(i_c, o_c);
        assert_eq!(i_prefix.to_str().unwrap(), o_prefix);
        assert_eq!(Arg::Word(i_suffix.clone()), o_suffix.unwrap());
    }
}

#[test]
fn fallback_with_strange_args_produces_same_results() {
    use crate::args::{split_os_argument, split_os_argument_fallback};
    let s = std::ffi::OsString::from("-Obits=2048");
    let r1 = split_os_argument(&s);
    let r2 = split_os_argument_fallback(&s);
    assert_eq!(r1, r2);
}

#[test]
fn different_methods_of_args_cration_are_identical() {
    use crate::Args;
    use std::ffi::OsString;
    let items = ["hello", "--world", "--a=-b"];
    let oitems = items.iter().map(OsString::from).collect::<Vec<_>>();
    let oitems2 = oitems.iter().map(OsString::as_os_str).collect::<Vec<_>>();

    // No Eq instances :)
    let args1 = format!("{:?}", Args::from(&items));
    let args2 = format!("{:?}", Args::from(&items[..])); // not available in 1.56
    let args3 = format!("{:?}", Args::from(oitems.as_slice()));
    let args4 = format!("{:?}", Args::from(oitems2.as_slice()));

    assert_eq!(args1, args2);
    assert_eq!(args1, args3);
    assert_eq!(args1, args4);
}

#[test]
fn de_yoda() {
    use bpaf::*;
    let parser = construct!(a(short('a').switch()), b(short('b').switch())).to_options();

    let r = parser.run_inner(Args::from(&[])).unwrap();
    assert_eq!(r, (false, false));

    let r = parser.run_inner(Args::from(&["-a", "-b"])).unwrap();
    assert_eq!(r, (true, true));
}
