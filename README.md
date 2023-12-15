# Tebex Headless Wrapper

A wrapper library for Tebex's Headless API, aka creating the serverside for your headless Tebex webstore.

- **Fast**: One of our main aims, both when choosing to do this in Rust, and choosing our dependencies, was the fact that this should be blazing fast.
- **Reliable**: This should pretty much be as reliable as possible, as it is a straight/raw wrapper for all Tebex API calls.

[![Crates.io][crates-badge]][crates-url]
[![Documentation][documentation-badge]][documentation-url]
[![Discord chat][discord-badge]][discord-url]

[crates-badge]: https://img.shields.io/crates/v/tebex_headless_rust.svg
[crates-url]: https://crates.io/crates/tebex_headless_rust
[documentation-url]: https://docs.rs/tebex_headless_rust/latest/tebex_headless_rust/
[documentation-badge]: https://img.shields.io/badge/Documentation-blue
[discord-badge]: https://img.shields.io/discord/931629164656734238.svg?logo=discord&style=flat-square
[discord-url]: http://discord.zerio-scripts.com

## Overview

This is a simple but straight forward to use wrapper library for Tebex's Headless API.
If you do not know what that is already, then please read [this](https://docs.tebex.io/developers/headless-api/overview) first.

## Example

```rs
use tebex_headless_rust::handlers::{misc::set_public_api_key, package::get_all_packages};

// tokio is used to allow an async main function
#[tokio::main]
async fn main() {
    // set public api key
    set_public_api_key(String::from("public_api_key_tebex"));

    // fetch packages
    let packages = get_all_packages(None, None).await;

    match packages {
        // if packages successfully fetched
        Ok (packages) => {
            println!("Package amount: {}", packages.len());
        }

        // handle issue with fetching of packages
        Err (err) => {
            println!("Could not fetch pacakges: {}", err);
        }
    }
}
```

## Todo

- Some of the structs are empty as it the structure of them are unknown
- Find some way to generate & **use** auth link via tests, as that would allow us to improve tests drastically

## Issues, suggestions, etcetera

If you encounter an issue with our template, simply open an issue [here](https://github.com/Z3rio/tebex_headless_rust/issues)

## Contribution

We gladly accept all contributions, contributing can be done via forking this
repo and then creating a PR.
