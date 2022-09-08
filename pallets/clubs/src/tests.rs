use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, assert_err, error::BadOrigin};

#[test]
fn it_creates_club() {
	basic_ledger().execute_with(|| {
		let club_id = 1;
		
		assert_ok!(ClubsModule::add_club(Origin::signed(1), club_id, "club_a".into()));
		assert_err!(ClubsModule::add_club(Origin::signed(2), club_id + 1, "club_b".into()), BadOrigin);
	});
}

#[test]
fn throws_duplicate_clubs() {
	basic_ledger().execute_with(|| {
		let club_id = 1;
		
		assert_ok!(ClubsModule::add_club(Origin::signed(1), club_id, "club_a".into()));
		assert_noop!(ClubsModule::add_club(Origin::signed(1), club_id, "club_a".into()), Error::<Test>::ClubAlreadyExists);
	});
}

#[test]
fn it_adds_and_removes_members() {
	basic_ledger().execute_with(|| {
		let club_id = vec![1, 2];
		let member_id = 1234567;

		// Add clubs and assign members
		assert_ok!(ClubsModule::add_club(Origin::signed(1), club_id[0], "club_a".into()));
		assert_ok!(ClubsModule::add_club(Origin::signed(1), club_id[1], "club_b".into()));

		assert_ok!(ClubsModule::add_member(Origin::signed(1), club_id[0], member_id));
		assert_ok!(ClubsModule::add_member(Origin::signed(1), club_id[1], member_id));

		// Try non-root add member
		assert_err!(ClubsModule::add_member(Origin::signed(2), club_id[0], member_id), BadOrigin);

		// Remove member
		assert_ok!(ClubsModule::remove_member(Origin::signed(1), club_id[0], member_id));

		// Try non-root remove member
		assert_err!(ClubsModule::remove_member(Origin::signed(2), club_id[1], member_id), BadOrigin);
	});
}

#[test]
fn throws_on_not_found_club() {
	basic_ledger().execute_with(|| {
		let club_id = vec![1, 2];
		let member_id = 1234567;

		// Add clubs and assign members
		assert_ok!(ClubsModule::add_club(Origin::signed(1), club_id[0], "club_a".into()));
		assert_ok!(ClubsModule::add_club(Origin::signed(1), club_id[1], "club_b".into()));

		assert_ok!(ClubsModule::add_member(Origin::signed(1), club_id[0], member_id));

		// Remove member
		assert_noop!(ClubsModule::remove_member(Origin::signed(1), club_id[1], member_id), Error::<Test>::IsNotMember);
	});
}