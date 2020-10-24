extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use clap::{App, Arg};
#[macro_use]
extern crate prettytable;
use prettytable::{Cell, Row, Table};
use std::path::Path;

mod filename;
mod migrations;
mod reserved;
mod runner;

fn main() {
  pretty_env_logger::init();

  let m = App::new("mitre")
    .version("0.1")
    .author("Lee Hambley <lee.hambley@gmail.com>")
    .about("CLI runner for migrations")
    .subcommand(
      App::new("run")
        .about("runs migrations")
        .arg(
          Arg::with_name("config_file")
            .long("config")
            .short('c')
            .takes_value(true)
            .value_name("CONFIG FILE")
            .about("The configuration file to use"),
        )
        .arg(
          Arg::with_name("directory")
            .long("directory")
            .short('d')
            .takes_value(true)
            .value_name("MIGRATION DIR")
            .about("The directory to use"),
        ),
    )
    .subcommand(
      App::new("reserved-words")
        .about("utilties for reserved words")
        .subcommand(App::new("ls").about("list reserved words")),
    )
    .subcommand(
      App::new("show-migrations")
        .about("for migrations")
        .arg(
          Arg::with_name("config_file")
            .long("config")
            .short('c')
            .takes_value(true)
            .value_name("CONFIG FILE")
            .about("The configuration file to use"),
        )
        .arg(
          Arg::with_name("directory")
            .long("directory")
            .short('d')
            .takes_value(true)
            .value_name("MIGRATION DIR")
            .about("The directory to use"),
        ),
    )
    .get_matches();

  match m.subcommand_name() {
    Some("reserved-words") => {
      let mut table = Table::new();

      table.add_row(row!["Word", "Kind", "Reason"]);

      reserved::words().iter().for_each(|word| {
        table.add_row(Row::new(vec![
          Cell::new(word.word).style_spec("bFy"),
          Cell::new(&word.kind.to_string()).style_spec("Fb"),
          Cell::new(word.reason),
        ]));
      });
      table.printstd();
    }

    Some("show-migrations") => {
      info!("showing migrations");
      if let Some(ref matches) = m.subcommand_matches("show-migrations") {
        assert!(matches.is_present("directory"));
        let path = Path::new(matches.value_of("directory").unwrap());
        let migrations = match migrations::migrations(path) {
          Ok(m) => m,
          Err(_) => panic!("something happen"),
        };

        let mut table = Table::new();
        table.add_row(row!["Filename"]);
        migrations.iter().for_each(|migration| {
          eprintln!("{:?}", migration);
          table.add_row(Row::new(vec![Cell::new(
            migration.parsed.path.to_str().unwrap(),
          )
          .style_spec("bFy")]));
        });
        table.printstd();
      }
    }
    Some("run") => {
      info!("running migrations");
      if let Some(ref matches) = m.subcommand_matches("run") {
        assert!(matches.is_present("directory"));
        let path = Path::new(matches.value_of("directory").unwrap());
        let migrations = match migrations::migrations(path) {
          Ok(m) => m,
          Err(_) => panic!("something happen"),
        };
        let result = match runner::run_migrations(&migrations) {
          Ok(m) => m,
          Err(_) => panic!("something happened during execution"),
        };

        let mut table = Table::new();
        table.add_row(row!["Filename"]);
        result.iter().for_each(|migration| {
          eprintln!("{:?}", migration);
          table.add_row(Row::new(vec![Cell::new(
            migration.parsed.path.to_str().unwrap(),
          )
          .style_spec("bFy")]));
        });
        table.printstd();
      }
    }
    Some("up") => {}   // up was used
    Some("down") => {} // dowm was used
    Some("redo") => {} // dowm was used
    _ => {}            // Either no subcommand or one not tested for...
  }
}
