use clap::{
    builder::{StringValueParser, TypedValueParser},
    crate_authors, crate_description, crate_name, crate_version, Arg, Command,
};
use dnote::{use_cases::AddingNote, value_objects::BookName};

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace"))
        .format_timestamp(None)
        .init();

    if let Err(err) = try_main() {
        log::error!("An error occurred: {:?}", err);
        println!("{:?}", err);
        std::process::exit(1);
    }
}

fn try_main() -> eyre::Result<()> {
    log::trace!("try_main called");
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("add", submatches)) => {
            log::trace!("command 'add'");
            let bookname = submatches.get_one::<BookName>("book").expect("required");
            log::debug!("bookname = {:?}", bookname);

            let content = match submatches.get_one::<String>("content") {
                Some(content) => content,
                None => todo!("open editor"),
            };
            log::debug!("content = '{}'", content);

            let uc = AddingNote::new();
            uc.execute(bookname, content);
        }
        _ => todo!(),
    }
    Ok(())
}

const ADD_EXAMPLE: &str = r#"
Examples:

* Open an editor to write content
 dnoter add git

* Skip the editor by providing content directly
 dnoter add git -c "time is a part of the commit hash"
"#;

fn cli() -> clap::Command {
    Command::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .subcommand(
            Command::new("add")
                .about("Add new note")
                .long_about(ADD_EXAMPLE)
                .arg(
                    Arg::new("book")
                        .required(true)
                        .value_parser(StringValueParser::new().try_map(BookName::try_new)),
                )
                .arg(
                    Arg::new("content")
                        .short('c')
                        .long("content")
                        .help("The new content for the note"),
                ),
        )
}
