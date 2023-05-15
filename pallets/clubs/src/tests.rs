use crate::{mock::*, Error, Club};
use frame_support::{assert_noop, assert_ok, assert_err, error::BadOrigin};

#[test]
fn it_creates_club() {
	basic_ledger().execute_with(|| {
		let club_id = 1;
		let club = Club::new(CLUB_OWNER, 1000);
		assert_ok!(ClubsModule::add_club(Origin::root(), club_id, club.clone()));
		assert_err!(ClubsModule::add_club(Origin::signed(2), club_id + 1, club), BadOrigin);
	});
}

#[test]
fn throws_duplicate_clubs() {
	basic_ledger().execute_with(|| {
		let club_id = 1;
		let club = Club::new(CLUB_OWNER, 1000);

		assert_ok!(ClubsModule::add_club(Origin::root(), club_id, club.clone()));
		assert_noop!(ClubsModule::add_club(Origin::root(), club_id, club), Error::<Test>::ClubAlreadyExists);
	});
}

#[test]
fn throws_on_not_found_club() {
	basic_ledger().execute_with(|| {
		let club_id = vec![1, 2];

		let club = Club::new(CLUB_OWNER, 1000);

		// Add clubs and assign members
		assert_ok!(ClubsModule::add_club(Origin::root(), club_id[0], club.clone()));
		assert_ok!(ClubsModule::add_club(Origin::root(), club_id[1], club.clone()));

		assert_ok!(ClubsModule::add_member(Origin::signed(10), club_id[0], MEMBER));

		assert_noop!(ClubsModule::add_member(Origin::signed(10), 88888, MEMBER), Error::<Test>::ClubNotFound);
	});
}

#[test]
fn it_extends_membership() {
	basic_ledger().execute_with(|| {
		let club_id = 1;

		let club = Club::new(CLUB_OWNER, 2);

		// Add clubs and assign members
		assert_ok!(ClubsModule::add_club(Origin::root(), club_id, club.clone()));

		assert_ok!(ClubsModule::add_member(Origin::signed(10), club_id, MEMBER));

		// Pay fee 2 * 100 blocks = 200 currency
		assert_ok!(ClubsModule::extend_membership(Origin::signed(MEMBER), club_id, 100));

		// This will assert: 2 * 500 blocks = 1000 (account is out of money)
		assert_noop!(ClubsModule::extend_membership(Origin::signed(MEMBER), club_id, 500), pallet_balances::Error::<Test, _>::InsufficientBalance);
	});
}