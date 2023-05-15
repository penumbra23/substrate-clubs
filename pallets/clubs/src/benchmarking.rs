use super::*;

#[allow(unused)]
use crate::Pallet as ClubsPallet;
use frame_benchmarking::{account, benchmarks};
use frame_system::RawOrigin;
use sp_runtime::traits::Bounded;
use frame_support::{
	assert_ok,
    traits::{
        Currency,
    },
};

type DepositBalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

benchmarks! {
	where_clause { where
        BalanceOf<T>: Bounded + From<u128>,
		T::BlockNumber: Bounded + From<u32>
    }

	add_club {
		let s in 0 .. 100;
		let club = Club::new(1, 2);
		let owner: T::AccountId = account("owner", 10, 0);
		let price: BalanceOf::<T> = 2u128.into();
		let club = Club::new(owner.clone(), price);
	}: _(RawOrigin::Root, s, club)
	verify {
		assert_eq!(Clubs::<T>::contains_key(s), true);
	}

	add_member {
		let s in 0 .. 100;
		let club_id = 1;
		let owner: T::AccountId = account("owner", 10, 0);
		let club = Club::new(owner.clone(), 2.into());
		assert_ok!(ClubsPallet::<T>::add_club(RawOrigin::Root.into(), club_id, club));
	}: _(RawOrigin::Signed(owner.clone()), club_id, account("member", s, 0))
	verify {
		assert_eq!(Memberships::<T>::contains_key(club_id, account::<T::AccountId>("member", s, 0)), true);
	}
	
	extend_membership {
		let club_id = 1;
		let owner: T::AccountId = account("owner", 10, 0);
		let member: T::AccountId = account("member", 11, 1);
		let club = Club::new(owner.clone(), DepositBalanceOf::<T>::min_value());
		assert_ok!(ClubsPallet::<T>::add_club(RawOrigin::Root.into(), club_id, club));
		assert_ok!(ClubsPallet::<T>::add_member(RawOrigin::Signed(owner.clone()).into(), club_id, member.clone()));

		<T>::Currency::make_free_balance_be(&member, DepositBalanceOf::<T>::max_value());
	}: _(RawOrigin::Signed(member.clone()), club_id, 1u32.into())
	verify {
		assert_eq!(Memberships::<T>::contains_key(club_id, member), true);
	}

	impl_benchmark_test_suite!(ClubsPallet, crate::mock::basic_ledger(), crate::mock::Test);
}
