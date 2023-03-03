use bpaf::*;
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Options {
    turbo: bool,
    backing: bool,
    xinerama: bool,
    extensions: Vec<(String, bool)>,
}

fn toggle_options(name: &'static str, help: &'static str) -> impl Parser<bool> {
    any::<String>(name)
        .help(help)
        .parse(move |s| {
            let (state, cur_name) = if let Some(rest) = s.strip_prefix('+') {
                (true, rest)
            } else if let Some(rest) = s.strip_prefix('-') {
                (false, rest)
            } else {
                return Err(format!("{} is not a toggle option", s));
            };
            if cur_name != name {
                Err(format!("{} is not a known toggle option name", cur_name))
            } else {
                Ok(state)
            }
        })
        .anywhere()
        .catch()
}

fn extension() -> impl Parser<(String, bool)> {
    let on = any::<String>("+ext")
        .help("enable ext <EXT>")
        .parse::<_, _, String>(|s| {
            if s == "+ext" {
                Ok(true)
            } else {
                Err(String::new())
            }
        });
    let off = any::<String>("-ext")
        .help("disable ext <EXT>")
        .parse::<_, _, String>(|s| {
            if s == "-ext" {
                Ok(false)
            } else {
                Err(String::new())
            }
        });

    let state = construct!([on, off]);
    let name = positional::<String>("EXT").hide();
    construct!(state, name)
        .map(|(a, b)| (b, a))
        .anywhere()
        .catch()
}

pub fn options() -> OptionParser<Options> {
    let backing = toggle_options("backing", "Backing status").fallback(false);
    let xinerama = toggle_options("xinerama", "Xinerama status").fallback(true);
    let turbo = short('t')
        .long("turbo")
        .help("Engage the turbo mode")
        .switch();
    let extensions = extension().many();
    construct!(Options {
        turbo,
        backing,
        xinerama,
        extensions,
    })
    .to_options()
}

fn main() {
    println!("{:#?}", options().run());
}
