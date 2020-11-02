#[macro_use]
extern crate clap;

use clap::{App, Arg, ArgMatches};

pub mod kube;

pub fn read_args() -> ArgMatches<'static> {
  App::new(crate_name!())
    .version(crate_version!())
    .about(crate_description!())
    .arg(
      Arg::with_name("secretName")
        .short("s")
        .long("secret")
        .value_name("secret name")
        .help("Name of the Kubernetes secret")
        .takes_value(true)
        .required(true)
        .index(1),
    )
    .arg(
      Arg::with_name("secretKey")
        .value_name("secret key")
        .help("Key within the secret to get")
        .required(false)
        .multiple(true)
        .takes_value(true)
        .index(2),
    )
    .get_matches()
}
