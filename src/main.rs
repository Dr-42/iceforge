/*
* Copyright (c) 2024, Dr. Spandan Roy
*
* This file is part of iceforge.
*
* iceforge is free software: you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* iceforge is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with iceforge.  If not, see <https://www.gnu.org/licenses/>.
*/
use codespan_reporting::term::{
    self,
    termcolor::{ColorChoice, StandardStream},
};

pub mod build_config;
pub mod cli;
pub mod logger;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = match build_config::BuildConfig::load_config("sample.toml") {
        Ok(config) => config,
        Err(e) => {
            let diag = e.diagnostic.unwrap();
            let writer = StandardStream::stderr(ColorChoice::Always);
            let config = codespan_reporting::term::Config::default();

            term::emit(&mut writer.lock(), &config, &e.files, &diag)?;
            std::process::exit(1);
        }
    };
    let _ = config.verify_config();
    cli::parse();
    Ok(())
}
