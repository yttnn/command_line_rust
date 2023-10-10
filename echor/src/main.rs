use clap::{Command, Arg, ArgAction};

fn main() {
  let matches = Command::new("echor")
      .version("0.1.0")
      .author("author")
      .about("Rust echo")
      .arg(
        Arg::new("text")
            .value_name("TEXT")
            .help("input text")
            .required(true)
            .action(ArgAction::Append)
      )
      .arg(
        Arg::new("omit_newline")
            .short('n')
            .help("Do not print newline")
            .action(ArgAction::SetTrue)
      )
      .get_matches();

  let text: Vec<_> = matches.get_many("text")
                            .unwrap()
                            .map(String::as_str)
                            .collect();
  let omit_newline = matches.get_flag("omit_newline");

  let ending = if omit_newline { "" } else { "\n" };
  print!("{}{}", text.join(" "), ending);

  println!("{:#?}", matches)
}
