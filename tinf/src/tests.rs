use std::io::Cursor;
use super::*;

#[test]
fn desc_file() {
    use super::cap::*;
    let mut file = Cursor::new(include_bytes!("test-desc").to_vec());
    let desc = Desc::parse(&mut file).unwrap();
    assert_eq!(desc[am], true);
    assert_eq!(desc[cols], 80);
    assert_eq!(&desc[cr], b"\x0d");
    assert_eq!(desc.get_bool_ext(&UserDef::named("userbool")), true);
    assert_eq!(desc.get_num_ext(&UserDef::named("usernum")), 300);
    assert_eq!(&desc.get_str_ext(&UserDef::named("userstr")), b"user");
}

#[test]
fn desc_literal() {
    use super::cap::*;
    let desc = desc![
            "dumb", "80-column dumb tty",
            am => true,
            cols => 80,
            lines => 24,
            bel => b"\x07",
            cr => b"\r",
            cud1 => b"\n",
            ind => b"\n",
    ];
    assert_eq!(desc[bw], false);
    assert_eq!(desc[am], true);
    assert_eq!(desc[xsb], false);
    assert_eq!(desc[cols], 80);
    assert_eq!(desc[it], 0xffff);
    assert_eq!(&desc[cr], b"\x0d");
    assert_eq!(vec!["dumb", "80-column dumb tty"], desc.names());
}

#[test]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn desc_user_literal() {
    use super::cap::{am, cols};
    let tc = UserDef::named("Tc");
    let setb24 = UserDef::named("setb24");
    let setf24 = UserDef::named("setf24");
    let setb24_val =
        "\x1b[48;2;%p1%{65536}%/%d;%p1%{256}%/%{255}%&%d;%p1%{255}%&%dm";
    let setf24_val =
        "\x1b[38;2;%p1%{65536}%/%d;%p1%{256}%/%{255}%&%d;%p1%{255}%&%dm";
    let desc = desc![
        am => true,
        cols => 80,
        &tc => true,
        &setb24 => setb24_val,
        &setf24 => setf24_val,
    ];
    assert_eq!(desc[am], true);
    assert_eq!(desc[cols], 80);
    assert_eq!(desc.get_bool_ext(&tc), true);
    assert_eq!(desc.get_num_ext(&UserDef::named("missing")), 0xffff);
    assert_eq!(desc.get_str_ext(&setb24), setb24_val.as_bytes());
}

#[test]
fn cap_string_names() {
    use super::cap::*;
    let desc = desc![
        "dumb", "80-column dumb tty",
        am => true,
        cols => 80,
        bel => b"\x07",
        cr => b"\r",
        cud1 => b"\n",
        ind => b"\n",
    ];
    assert_eq!(desc[Number::named("cols").unwrap()], 80);
    assert_eq!(desc[Boolean::named("am").unwrap()], true);
    assert_eq!(&desc[String::named("ind").unwrap()], b"\n");
}

#[test]
#[should_panic]
fn cap_string_names_bad() {
    super::cap::Number::named("xyzzy").unwrap();
}

#[test]
fn tparm_basic_setabf() {
    let mut output = Vec::new();
    let _ = tparm(
        &mut output,
        b"\\E[48;5;%p1%dm",
        &mut params!(1),
        &mut Vars::new(),
    );
    assert_eq!(output, b"\\E[48;5;1m");
}

#[test]
fn tparm_multiple_int_constants() {
    let mut output = Vec::new();
    let _ = tparm(
        &mut output,
        b"%{1}%{2}%d%d",
        &mut params!(),
        &mut Vars::new(),
    );
    assert_eq!(output, b"21");
}

#[test]
fn tparm_op_i() {
    let mut output = Vec::new();
    let _ = tparm(
        &mut output,
        b"%p1%d%p2%d%p3%d%i%p1%d%p2%d%p3%d",
        &mut params!(1, 2, 3),
        &mut Vars::new(),
    );
    assert_eq!(output, b"123233");
    output.clear();

    let _ = tparm(&mut output, b"%i%p1%d", &mut params!(1), &mut Vars::new());
    assert_eq!(output, b"2");
}

#[test]
fn tparm_conditionals() {
    let mut output = Vec::new();
    let cap =
        b"\\E[%?%p1%{8}%<%t3%p1%d%e%p1%{16}%<%t9%p1%{8}%-%d%e38;5;%p1%d%;m";
    let _ = tparm(&mut output, cap, &mut params!(1), &mut Vars::new());
    assert_eq!(output, b"\\E[31m");
    output.clear();
    let _ = tparm(&mut output, cap, &mut params!(8), &mut Vars::new());
    assert_eq!(output, b"\\E[90m");
    output.clear();
    let _ = tparm(&mut output, cap, &mut params!(42), &mut Vars::new());
    assert_eq!(output, b"\\E[38;5;42m");
}

#[test]
fn tparm_format() {
    let mut output = Vec::new();
    let _ = tparm(
        &mut output,
        b"%p1%s%p2%2s%p3%2s%p4%.2s",
        &mut params!("foo", "foo", "f", "foo"),
        &mut Vars::new(),
    );
    assert_eq!(output, b"foofoo ffo");
    output.clear();

    let _ = tparm(
        &mut output,
        b"%p1%:-4.2s",
        &mut params!("foo"),
        &mut Vars::new(),
    );
    assert_eq!(output, b"fo  ");
    output.clear();

    let _ = tparm(
        &mut output,
        b"%p1%d%p1%.3d%p1%5d%p1%:d",
        &mut params!(1),
        &mut Vars::new(),
    );
    assert_eq!(output, b"1001    11");
    output.clear();

    let _ = tparm(
        &mut output,
        b"%p1%o%p1%#o%p2%6.4x%p2%#6.4X",
        &mut params!(15, 27),
        &mut Vars::new(),
    );
    assert_eq!(output, b"17017  001b0X001B");
}

#[test]
fn tparm_vars() {
    let mut output = Vec::new();
    let mut vars = Vars::new();
    let cap = b"%?%p1%{1}%=%t%'h'%Pa%e%'l'%Pa%;\
              \\E[?1000%ga%c\\E[?1003%ga%c\\E[?1006%ga%c";

    let _ = tparm(&mut output, cap, &mut params!(1), &mut vars);
    assert_eq!(output, b"\\E[?1000h\\E[?1003h\\E[?1006h");
    output.clear();

    let _ = tparm(&mut output, cap, &mut params!(0), &mut vars);
    assert_eq!(output, b"\\E[?1000l\\E[?1003l\\E[?1006l");
    output.clear();

    let _ = tparm(&mut output, b"%gz%d", &mut params!(), &mut vars);
    assert_eq!(output, b"0");
}

#[test]
fn tputs_flash() {
    let cap = b"\\E[?5h$<2/>\\E[?5l";
    let mut output = Cursor::new(vec![0u8; 0]);
    let _ = tputs(&mut output, cap, 1, 19200, Some(0));
    assert_eq!(output.into_inner(), b"\\E[?5h\0\0\0\0\\E[?5l");
    let mut output = Cursor::new(vec![0u8; 0]);
    let _ = tputs(&mut output, cap, 1, 50, Some(0));
    assert_eq!(output.into_inner(), b"\\E[?5h\\E[?5l");
}
