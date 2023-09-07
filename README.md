# ZK-Battleship

This repository show an example implementation of the well known Battleship game based on ZK-SNARKS.

## Battleship Game Mechanics

The console application generates a field with randomly placed ships.
And user is trying to guess where the ships located by providing coordinates.
Before the game starts the correctness of the field is verified by Rank-1 Constraint System (R1CS).

Current implementation uses [arkworks](https://github.com/arkworks-rs) zkSNARKs library.

## How To Run

To run the application you need Rust installed, then simply run:

```shell
cargo run
```

## TODO

* Add zkSNARK verification using Groth16 proof system.
* Add proof generation for each shoot i.e. generate ZK proof that the user hit a ship or missed.
