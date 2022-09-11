# Clubs Pallet

## Intro

The Clubs Pallet gives the ability to create clubs with a designated ID and name. After creating clubs (which is only allowed for the ROOT account) the ROOT account can add accounts to any club or remove members from any club.

## Configuration

The pallet config has some notable variables that can restrict certain capabilities:

`AdminAccount` - the account that has permissions to add clubs, add or remove members from clubs

`MaxLength` - maximum number of clubs that can be created

### Functions

This pallet exports 3 functions:

#### add_club

`add_club(club_id: u32, name: Vec<u8>)` - creates a club with the specific ID and name (string encoded byte array)

#### add_member

`add_member(club_id: u32, member: AccountId)` - adds the account to a previously created club

#### remove_member

`remove_member(club_id: u32, member: AccountId)` - removes the account from a previously created club

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