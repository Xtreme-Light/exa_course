use std::ffi::OsStr;
use std::fmt;
use std::fmt::Formatter;
#[derive(Debug)]
pub struct Options {
    show_in_visible: bool,
}
///
pub type ShortArg = u8;
pub type LongArg = &'static str;
pub type Values = &'static [&'static str];
#[derive(Debug)]
pub struct Arg {
    pub short: Option<ShortArg>,
    pub long: LongArg,
    pub args: TakesValue,
}

pub static ALL: Arg = Arg {
    short: Some(b'a'),
    long: "all",
    args: TakesValue::Forbidden,
};
#[derive(Debug)]
pub struct Args(pub &'static [&'static Arg]);
pub static ALL_ARGS: Args = Args(&[&ALL]);

impl Args {
    pub fn parse<'args, I>(&self, inputs: I) -> Result<Matches<'args>, ParseError>
    where
        I: IntoIterator<Item = &'args OsStr>,
    {
        use std::os::unix::ffi::OsStrExt;
        let mut parsing = true;
        let mut frees: Vec<&OsStr> = Vec::new();
        let mut result_flags = Vec::new();

        let mut inputs = inputs.into_iter();
        while let Some(arg) = inputs.next() {
            let bytes = arg.as_bytes();
            if !parsing {
                frees.push(arg);
            } else if arg == "--" {
                parsing = false;
            } else if bytes.starts_with(b"--") {
                let long_arg_name = OsStr::from_bytes(&bytes[2..]);
                let _arg = self.0.iter().find(|e| long_arg_name == e.long).unwrap();
                let _arg: &Arg = _arg;
                result_flags.push(_arg);
            } else if bytes.starts_with(b"-") && arg != "-" {
                let short_arg_name = OsStr::from_bytes(&bytes[1..]);
                let arg_list = short_arg_name.as_bytes();
                for short in arg_list {
                    let _arg = self.0.iter().find(|e| e.short == Some(*short)).unwrap();
                    result_flags.push(_arg);
                }
            } else {
                frees.push(arg);
            }
        }
        Ok(Matches {
            flags: result_flags,
            frees,
        })
    }
}
#[derive(Debug)]
pub struct Matches<'args> {
    pub flags: Vec<(&'args Arg)>,
    pub frees: Vec<&'args OsStr>,
}
#[derive(Debug)]
pub enum TakesValue {
    Necessary(Option<Values>),
    Forbidden,
    Optional(Option<Values>),
}
#[derive(Debug)]
pub enum OptionsResult<'args> {
    Ok(Options, Vec<&'args OsStr>),
    InvalidOptions(OptionsError),
}
#[derive(Debug)]
pub enum OptionsError {
    Parse(ParseError),
}
#[derive(Debug)]
pub enum ParseError {
    ForbiddenValue,
}

impl Options {
    pub fn parse<'args, I>(args: I) -> OptionsResult<'args>
    where
        I: IntoIterator<Item = &'args OsStr>,
    {
        let result = ALL_ARGS.parse(args);
        println!("{:?}", result.unwrap());
        OptionsResult::InvalidOptions(OptionsError::Parse(ParseError::ForbiddenValue))
    }
}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ForbiddenValue => write!(f, "Flag cannot take a value"),
        }
    }
}
