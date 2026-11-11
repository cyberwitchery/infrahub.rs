//! generate a typed infrahub client from a schema
//!
//! this binary produces a standalone, schema-specific crate with:
//! - typed `types`, `inputs`, and `responses`
//! - a full surface `generated()` client
//! - an ergonomic `api()` layer grouped by schema namespaces
//!
//! command help reference (kept in sync with `infrahub-codegen --help`):
#[doc = concat!("```text\n", include_str!("../infrahub-codegen-help.txt"), "\n```")]
pub const CLI_HELP: &str = include_str!("../infrahub-codegen-help.txt");

mod cli;
mod conventions;
mod emit;
mod schema;
mod selection;
mod type_map;

use cli::{load_schema, parse_args, ParseArgsError};
use emit::generate_client;
use graphql_parser::schema::parse_schema;
use schema::SchemaContext;

fn main() {
    let args = match parse_args(std::env::args().collect()) {
        Ok(args) => args,
        Err(ParseArgsError::Help) => {
            print!("{CLI_HELP}");
            return;
        }
        Err(ParseArgsError::Message(err)) => {
            eprintln!("{err}\n\n{CLI_HELP}");
            std::process::exit(1);
        }
    };

    let schema = match load_schema(&args) {
        Ok(schema) => schema,
        Err(err) => {
            eprintln!("failed to load schema: {err}");
            std::process::exit(1);
        }
    };

    let document = match parse_schema::<String>(&schema) {
        Ok(doc) => doc,
        Err(err) => {
            eprintln!("failed to parse schema: {err}");
            std::process::exit(1);
        }
    };

    let ctx = SchemaContext::new(&document);

    if let Err(err) = generate_client(&args, &ctx) {
        eprintln!("codegen failed: {err}");
        std::process::exit(1);
    }
}
