#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use frame_support::BoundedBTreeSet;
	use sp_std::vec::Vec;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		
		type AdminAccount: EnsureOrigin<Self::Origin>;
		
		#[pallet::constant]
		type MaxLength: Get<u32>;
	}

	#[pallet::storage]
	#[pallet::getter(fn club)]
	pub type Clubs<T: Config> = StorageMap<_, Blake2_128Concat, u32, BoundedVec<u8, T::MaxLength>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn members)]
	pub type Members<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, BoundedBTreeSet<u32, T::MaxLength>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ClubAdded(u32),
		MemberAdded(u32, T::AccountId),
		MemberRemoved(u32, T::AccountId),
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
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Adds a new club with the specified id and a name.
		#[pallet::weight(100_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn add_club(origin: OriginFor<T>, club_id: u32, name: Vec<u8>) -> DispatchResult {
			T::AdminAccount::ensure_origin(origin)?;

			ensure!(Clubs::<T>::try_get(club_id).is_err(), Error::<T>::ClubAlreadyExists);

			let club_name: BoundedVec<_, _> = name.try_into().map_err(|()| Error::<T>::ClubNameTooLong)?;

			Clubs::<T>::insert(club_id, club_name);
			Self::deposit_event(Event::<T>::ClubAdded(club_id));
			Ok(())
		}

		/// Adds a member (account) to a club
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2,1))]
		pub fn add_member(origin: OriginFor<T>, club_id: u32, member: T::AccountId) -> DispatchResult {
			T::AdminAccount::ensure_origin(origin)?;

			ensure!(Clubs::<T>::contains_key(club_id), Error::<T>::ClubNotFound);

			let mut member_clubs = Members::<T>::get(&member);

			ensure!(!member_clubs.contains(&club_id), Error::<T>::AlreadyMember);
			ensure!(member_clubs.try_insert(club_id).is_ok(), Error::<T>::TooManyClubs);

			Members::<T>::insert(&member, member_clubs);
			Self::deposit_event(Event::<T>::MemberAdded(club_id, member));
			Ok(())
		}

		/// Removes the member from the club
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2,1))]
		pub fn remove_member(origin: OriginFor<T>, club_id: u32, member: T::AccountId) -> DispatchResult {
			T::AdminAccount::ensure_origin(origin)?;

			ensure!(Clubs::<T>::contains_key(club_id), Error::<T>::ClubNotFound);

			let mut member_clubs = Members::<T>::get(&member);

			ensure!(member_clubs.contains(&club_id), Error::<T>::IsNotMember);
			member_clubs.remove(&club_id);

			Members::<T>::insert(&member, member_clubs);
			Self::deposit_event(Event::<T>::MemberRemoved(club_id, member));
			Ok(())
		}
	}
}
