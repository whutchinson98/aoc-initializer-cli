# Aoc Initializer CLI

## Description

Created to easily create a template solution in Rust with some boilerplate for Advent of Code.

## Usage

Build the cli
`cargo build --release`

Move into existing path location or add into your path
`sudo mv ./target/release/aoc-initializer-cli /usr/bin/aoc`

Initialize your config
`aoc config ${YOUR_AOC_SESSION_ID} ${YEAR}`

Create the template for a given day in AOC
`aoc init ${day}`
