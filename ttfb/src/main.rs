/*
MIT License

Copyright (c) 2021 Philipp Schuster

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

#![deny(
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    // clippy::restriction,
    // clippy::pedantic
)]
// now allow a few rules which are denied by the above statement
// --> they are ridiculous and not necessary
#![allow(
    clippy::suboptimal_flops,
    clippy::redundant_pub_crate,
    clippy::fallible_impl_from
)]
// I can't do anything about this; fault of the dependencies
#![allow(clippy::multiple_crate_versions)]
// allow: required because of derive macro.. :(
#![allow(clippy::use_self)]
// Not needed here. We only need this for the library!
// #![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(rustdoc::all)]

use clap::Parser;
use crossterm::style::{Attribute, SetAttribute};
use crossterm::ExecutableCommand;
use std::io::stdout;
use std::process::exit;
use ttfb::TtfbError;
use ttfb::TtfbOutcome;

const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

macro_rules! unwrap_or_exit {
    ($ident:ident) => {
        if let Err(err) = $ident {
            $crate::exit_error(err);
        } else {
            $ident.unwrap()
        }
    };
}

/// CLI Arguments for `clap`.
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "CLI utility to measure the TTFB (time to first byte) of HTTP(S) requests. \
        Additionally, this crate measures the relative and absolute times of DNS \
        lookup, TCP connect, and TLS handshake."
)]
struct TtfbArgs {
    /// Name of the host. An IP address or a URL. "https://"-prefix must be provided for HTTPS/TLS.
    host: String,
    /// Whether insecure TLS-certificates (e.g., expired, wrong domain name) are allowed.
    /// Similar to `-k` of `curl`.
    #[arg(short = 'k', long = "insecure")]
    allow_insecure_certificates: bool,
}

/// Small CLI binary wrapper around the [`ttfb`] lib.
fn main() {
    let input: TtfbArgs = TtfbArgs::parse();
    let res = ttfb::ttfb(input.host, input.allow_insecure_certificates);
    let ttfb = unwrap_or_exit!(res);
    print_outcome(&ttfb).unwrap();
}

fn exit_error(err: TtfbError) -> ! {
    eprint!("\u{1b}[31m");
    eprint!("\u{1b}[1m");
    eprint!("ERROR: ",);
    eprint!("\u{1b}[0m");
    eprint!("{}", err);
    eprintln!();
    exit(-1)
}

fn print_outcome(ttfb: &TtfbOutcome) -> Result<(), String> {
    stdout()
        .execute(SetAttribute(Attribute::Bold))
        .map_err(|err| err.to_string())?;
    println!(
        "TTFB for {url} (by ttfb@v{crate_version})",
        url = ttfb.user_input(),
        crate_version = CRATE_VERSION
    );
    println!("PROPERTY        REL TIME (ms)   ABS TIME (ms)");
    stdout()
        .execute(SetAttribute(Attribute::Reset))
        .map_err(|err| err.to_string())?;
    if ttfb.dns_duration_rel().is_some() {
        print!(
            "{property:<14}: {rel_time:>13.3}   {abs_time:>13.3}",
            property = "DNS Lookup",
            rel_time = ttfb.dns_duration_rel().unwrap().as_secs_f64() * 1000.0,
            // for DNS abs and rel time is the same (because it happens first)
            abs_time = ttfb.dns_duration_rel().unwrap().as_secs_f64() * 1000.0,
        );
        if ttfb.dns_duration_rel().unwrap().as_millis() < 2 {
            print!("  (probably cached)");
        }
        println!();
    }
    println!(
        "{property:<14}: {rel_time:>13.3}   {abs_time:>13.3}",
        property = "TCP connect",
        rel_time = ttfb.tcp_connect_duration_rel().as_secs_f64() * 1000.0,
        abs_time = ttfb.tcp_connect_duration_abs().as_secs_f64() * 1000.0,
    );
    if ttfb.tls_handshake_duration_rel().is_some() {
        println!(
            "{property:<14}: {rel_time:>13.3}   {abs_time:>13.3}",
            property = "TLS Handshake",
            rel_time = ttfb.tls_handshake_duration_rel().unwrap().as_secs_f64() * 1000.0,
            // for DNS abs and rel time is the same (because it happens first)
            abs_time = ttfb.tls_handshake_duration_abs().unwrap().as_secs_f64() * 1000.0,
        );
    }
    println!(
        "{property:<14}: {rel_time:>13.3}   {abs_time:>13.3}",
        property = "HTTP GET Req",
        rel_time = ttfb.http_get_send_duration_rel().as_secs_f64() * 1000.0,
        abs_time = ttfb.http_get_send_duration_abs().as_secs_f64() * 1000.0,
    );

    stdout()
        .execute(SetAttribute(Attribute::Bold))
        .map_err(|err| err.to_string())?;
    println!(
        "{property:<14}: {rel_time:>13.3}   {abs_time:>13.3}",
        property = "HTTP Resp TTFB",
        rel_time = ttfb.http_ttfb_duration_rel().as_secs_f64() * 1000.0,
        abs_time = ttfb.http_ttfb_duration_abs().as_secs_f64() * 1000.0,
    );
    stdout()
        .execute(SetAttribute(Attribute::Reset))
        .map_err(|err| err.to_string())?;

    Ok(())
}
