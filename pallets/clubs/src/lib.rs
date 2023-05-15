#![cfg_attr(not(feature = "std"), no_std)]


#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;

pub use weights::WeightInfo;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::{*, OptionQuery};
	use frame_system::pallet_prelude::*;
	use frame_support::Blake2_128Concat;
	use frame_support::traits::{Currency, ExistenceRequirement};
	use sp_runtime::traits::Convert;
	use sp_std::prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Currency: Currency<Self::AccountId>;

		type AdminAccount: EnsureOrigin<Self::Origin>;
		
		type WeightInfo: WeightInfo;

		type BlockNumberToBalance: Convert<Self::BlockNumber, BalanceOf<Self>>;
	}

	type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[derive(Encode, Decode, Default, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct Club<AccountId, BalanceOf> {
		pub(super) owner: AccountId,
		pub(super) annual_expense: BalanceOf,
	}

	impl<AccountId, BalanceOf> Club<AccountId, BalanceOf> {
		pub fn new(owner: AccountId, annual_expense: BalanceOf) -> Self {
			Club {
				owner,
				annual_expense,
			}
		}
	}

	#[derive(Encode, Decode, Default, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct Membership<MembershipDuration> {
		pub(super) expiration: MembershipDuration,
	}

	#[pallet::storage]
	#[pallet::getter(fn club)]
	pub(super) type Clubs<T: Config> = StorageMap
	<_,
		Blake2_128Concat,
		u32,
		Club<T::AccountId, BalanceOf<T>>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn membership)]
	pub(super) type Memberships<T: Config> = StorageDoubleMap
	<_,
		Blake2_128Concat,
		u32,
		Blake2_128Concat,
		T::AccountId,
		Membership<T::BlockNumber>,
		OptionQuery
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ClubAdded(u32),
		MemberAdded(u32, T::AccountId),
		MembershipExtended(T::BlockNumber, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		ClubAlreadyExists,
		ClubNotFound,
		ClubNameTooLong,
		AlreadyMember,
		IsNotMember,
		TooManyClubs,
		NotOwner,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Creates a new club with the specified id.
		#[pallet::weight(T::WeightInfo::add_club(*club_id))]
		pub fn add_club(origin: OriginFor<T>, club_id: u32, club: Club<T::AccountId, BalanceOf<T>>) -> DispatchResult {
			T::AdminAccount::ensure_origin(origin)?;

			ensure!(Clubs::<T>::try_get(club_id).is_err(), Error::<T>::ClubAlreadyExists);

			Clubs::<T>::insert(club_id, club);

			Self::deposit_event(Event::<T>::ClubAdded(club_id));
			Ok(())
		}

		/// Adds a member (account) to a club
		#[pallet::weight(T::WeightInfo::add_member(*club_id))]
		pub fn add_member(origin: OriginFor<T>, club_id: u32, member: T::AccountId) -> DispatchResult {
			let caller = ensure_signed(origin)?;

			ensure!(Clubs::<T>::contains_key(club_id), Error::<T>::ClubNotFound);

			let club = Clubs::<T>::get(&club_id).unwrap();

			ensure!(club.owner == caller, Error::<T>::NotOwner);
			ensure!(!Memberships::<T>::contains_key(club_id, &member), Error::<T>::AlreadyMember);

			let membership = Membership {
				expiration: <frame_system::Pallet<T>>::block_number(),
			};

			Memberships::<T>::insert(club_id, &member, membership);
			Self::deposit_event(Event::<T>::MemberAdded(club_id, member));
			Ok(())
		}

		/// Extend the membership by paying the annual expense
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2,1))]
		pub fn extend_membership(origin: OriginFor<T>, club_id: u32, duration: T::BlockNumber) -> DispatchResult {
			let caller = ensure_signed(origin)?;

			ensure!(Clubs::<T>::contains_key(club_id), Error::<T>::ClubNotFound);

			let mut membership = match Memberships::<T>::get(club_id, caller.clone()) {
				Some(membership) => Ok(membership),
				None => Err(Error::<T>::IsNotMember)
			}?;

			let club = Clubs::<T>::get(&club_id).unwrap();
			let block_number_balance = T::BlockNumberToBalance::convert(duration);
			let fee = club.annual_expense * block_number_balance;

			// Transfer the fee amount to the club owner
        	T::Currency::transfer(&caller, &club.owner, fee, ExistenceRequirement::KeepAlive)?;

			// Extend the membership for the duration paid
			let membership_end = membership.expiration + duration;
			membership.expiration = membership_end;

			Memberships::<T>::insert(club_id, &caller, membership);
			Self::deposit_event(Event::<T>::MembershipExtended(membership_end, caller));
			Ok(())
		}
	}
}
