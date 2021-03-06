use module::prelude::*;
use cratesio;
use url::percent_encoding::{utf8_percent_encode, PATH_SEGMENT_ENCODE_SET};
use itertools::Itertools;
use reqwest::StatusCode::NotFound;

pub enum CrateInfo {}

impl Module for CrateInfo {
    fn init(commands: &mut CommandRegistry) {
        commands.set_named_handler("crate", crate_handler);
    }
}

fn crate_handler(ctx: &Context, args: &[&str]) -> Flow {
    let crate_name = match args.get(0) {
        Some(name) => name,
        None => return Flow::Continue,
    };

    let info = match cratesio::crate_info(crate_name) {
        Ok(info) => info,
        // TODO: Use proper error types
        Err(ref err) if err.status() == Some(NotFound) => {
            ctx.reply(format!("Crate '{}' does not exist.", crate_name));
            return Flow::Break
        },
        Err(err) => {
            eprintln!("Error getting crate info for '{}': {:?}", crate_name, err);
            ctx.reply(format!("Failed to get crate info for {}", crate_name));
            return Flow::Break
        }
    };

    let krate = info.krate();
    let output = format!(
        "{name} ({version}) - {description} -> https://crates.io/crates/{urlname} [https://docs.rs/crate/{urlname}]",
        name = krate.name(),
        version = krate.max_version(),
        description = krate.description().split_whitespace().join(" "),
        urlname = utf8_percent_encode(&krate.name(), PATH_SEGMENT_ENCODE_SET).collect::<String>()
    );

    ctx.reply(output);

    Flow::Break
}
