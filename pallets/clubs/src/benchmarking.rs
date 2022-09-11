use super::*;

#[allow(unused)]
use crate::Pallet as ClubsPallet;
use frame_benchmarking::{account, benchmarks};
use frame_system::RawOrigin;

benchmarks! {
	add_club {
		let s in 0 .. 100;
	}: _(RawOrigin::Root, s, [1u8, 2u8, s as u8].into())
	verify {
		assert_eq!(*Clubs::<T>::get(s).last().unwrap(), s as u8);
	}

	add_member {
		let s in 0 .. 100;
		let club_id = 1;
		ClubsPallet::<T>::add_club(RawOrigin::Root.into(), club_id, "club_a".into());
	}: _(RawOrigin::Root, club_id, account("member", s, 0))
	verify {
		assert_eq!(Members::<T>::get(account::<T::AccountId>("member", s, 0)).contains(&club_id), true);
	}

	impl_benchmark_test_suite!(ClubsPallet, crate::mock::basic_ledger(), crate::mock::Test);
}
