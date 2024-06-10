# PNGme

Code for the [PNGme](https://jrdngr.github.io/pngme_book/) book.
This program lets you hide secret messages in PNG files.

## Setup

- `cargo build --release`

## Run

Below are the supported actions, along with an example command. See `./target/release/pngme --help` for more info.

- Encode a message into a PNG file: `./target/release/pngme encode ./pondering_calvin.png ruSt "Hide this message!"`
- Decode a message stored in a PNG file: `./target/release/pngme decode ./pondering_calvin.png ruSt`
- Remove a message from a PNG file: `./target/release/pngme remove ./pondering_calvin.png ruSt`
- Print a list of PNG chunks that can be searched for messages: `./target/release/pngme print ./pondering_calvin.png`
