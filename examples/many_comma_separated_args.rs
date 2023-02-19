use bpaf::*;
use std::str::FromStr;

// --ports 1,2,3 --ports 4,5   => [1,2,3,4,5]
fn args() -> impl Parser<Vec<u16>> {
    long("ports")
        .help("Comma separated list of ports")
        .argument::<String>("PORTS")
        .parse(|s| {
            s.split(",")
                .map(|c| u16::from_str(c))
                .collect::<Result<Vec<_>, _>>()
        })
        .many()
        .map(|nested| nested.into_iter().flatten().collect())
}

fn main() {
    println!("{:?}", args().to_options().run());
}
