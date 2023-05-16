# Clubs Pallet

## Intro

The Clubs Pallet gives the ability to create clubs with a designated ID and name. After creating clubs (which is only allowed for the ROOT account) the club owner can add accounts to any club. The members need to pay some fees to extend their membership in the club.

## Configuration

The pallet config has some notable variables that can restrict certain capabilities:

`AdminAccount` - the account that has permissions to add clubs, add or remove members from clubs

### Functions

This pallet exports 3 functions:

#### add_club

`add_club(club_id: u32, club: club)` - creates a club with the specified ID

#### add_member

`add_member(club_id: u32, member: AccountId)` - adds the account to a previously created club setting the expiration to the current block

#### remove_member

`extend_membership(club_id: u32, duration: BlockNumber)` - extends the membership expiration by paying the fees

### Tests

The Clubs Pallet contains unit and benchmark tests:

To run the tests:
```sh
cd pallets/clubs;
# omit --feature runtime-benchmarks if benchmarks won't be tested
cargo test --features runtime-benchmarks;
```

To run the benchmarks, compile the whole project and run the benchmark command on the node-template:
```sh
cargo build --release --features runtime-benchmarks;
./target/release/node-template benchmark pallet \
--chain dev \
--pallet pallet_clubs \
--extrinsic '*' \
--steps 20 \
--repeat 100 \
--output pallets/clubs/src/weights.rs;
```

### License
MIT