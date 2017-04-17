#[macro_use]
pub mod terminfo;

#[cfg(windows)]
pub mod console;

#[cfg(test)]
mod tests {
    use super::terminfo::{Vars, tparm};

    #[test]
    fn test_basic_setabf() {
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
    fn test_multiple_int_constants() {
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
    fn test_op_i() {
        let mut output = Vec::new();
        let _ = tparm(
            &mut output,
            b"%p1%d%p2%d%p3%d%i%p1%d%p2%d%p3%d",
            &mut params!(1, 2, 3),
            &mut Vars::new(),
        );
        assert_eq!(output, b"123233");
    }

    #[test]
    fn test_conditionals() {
        let mut output = Vec::new();
        let cap =
            b"\\E[%?%p1%{8}%<%t3%p1%d%e%p1%{16}%<%t9%p1%{8}%-%d%e38;5;%p1%d%;m";
        let _ = tparm(&mut output, cap, &mut params!(1), &mut Vars::new());
        assert_eq!(output, b"\\E[31m");
        let _ = tparm(&mut output, cap, &mut params!(8), &mut Vars::new());
        assert_eq!(output, b"\\E[90m");
        let _ = tparm(&mut output, cap, &mut params!(42), &mut Vars::new());
        assert_eq!(output, b"\\E[38;5;42m");
    }

    #[test]
    fn test_format() {
        let mut output = Vec::new();
        let _ = tparm(
            &mut output,
            b"%p1%s%p2%2s%p3%2s%p4%.2s",
            &mut params!(b"foo"[..], b"foo"[..], b"f"[..], b"foo"[..]),
            &mut Vars::new(),
        );
        assert_eq!(output, b"foofoo ffo");

        let _ = tparm(
            &mut output,
            b"%p1%:-4.2s",
            &mut params!(b"foo"[..]),
            &mut Vars::new(),
        );
        assert_eq!(output, b"fo  ");

        let _ = tparm(
            &mut output,
            b"%p1%d%p1%.3d%p1%5d%p1%:d",
            &mut params!(1),
            &mut Vars::new(),
        );
        assert_eq!(output, b"1001    11");

        let _ = tparm(
            &mut output,
            b"%p1%o%p1%#o%p2%6.4x%p2%#6.4X",
            &mut params!(15, 27),
            &mut Vars::new(),
        );
        assert_eq!(output, b"17017  001b0X001B");
    }
}
