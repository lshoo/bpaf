use crate::{info::Info, parsers::NamedArg, Meta};

#[doc(hidden)]
#[derive(Clone, Debug)]
pub enum Item {
    Positional {
        metavar: &'static str,
        strict: bool,
        help: Option<String>,
    },
    Command {
        name: &'static str,
        short: Option<char>,
        help: Option<String>,
        meta: Box<Meta>,
        info: Box<Info>,
    },
    Flag {
        name: ShortLong,
        shorts: Vec<char>,
        help: Option<String>,
    },
    Argument {
        name: ShortLong,
        shorts: Vec<char>,
        metavar: &'static str,
        env: Option<&'static str>,
        help: Option<String>,
    },
    MultiArg {
        name: ShortLong,
        shorts: Vec<char>,
        help: Option<String>,
        fields: Vec<(&'static str, Option<String>)>,
    },
}

#[doc(hidden)]
#[derive(Copy, Clone, Debug)]
pub enum ShortLong {
    Short(char),
    Long(&'static str),
    ShortLong(char, &'static str),
}

impl From<&NamedArg> for ShortLong {
    fn from(named: &NamedArg) -> Self {
        match (named.short.is_empty(), named.long.is_empty()) {
            (true, true) => unreachable!("Named should have either short or long name"),
            (true, false) => Self::Long(named.long[0]),
            (false, true) => Self::Short(named.short[0]),
            (false, false) => Self::ShortLong(named.short[0], named.long[0]),
        }
    }
}

impl Item {
    #[must_use]
    pub(crate) fn required(self, required: bool) -> Meta {
        if required {
            Meta::Item(self)
        } else {
            Meta::Optional(Box::new(Meta::Item(self)))
        }
    }
}
