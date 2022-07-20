#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

use frame_support::inherent::Vec;
use frame_support::pallet_prelude::*;
use frame_support::storage::bounded_vec::BoundedVec;
use frame_support::traits::Currency;
use frame_support::traits::Time;
use frame_system::pallet_prelude::*;
use scale_info::TypeInfo;
use frame_support::sp_runtime::ArithmeticError;

type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {
	pub use super::*;

	#[derive(Clone, Encode, Decode, TypeInfo,PartialEq, RuntimeDebug)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T: Config> {
		dna: Vec<u8>,
		owner: T::AccountId,
		price: u32,
		gender: Gender,
		created_date: <<T as Config>::Time as Time>::Moment,
	}

	pub type Id = u32;

	#[derive(Clone, Encode, Decode, TypeInfo, PartialEq, RuntimeDebug, Copy, MaxEncodedLen)]
	pub enum Gender {
		Male,
		Female,
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: Currency<Self::AccountId>;
		type Time: Time;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	//custom
	#[pallet::storage]
	#[pallet::getter(fn kitty_id)]
	pub type KittyId<T> = StorageValue<_, Id, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_kitty)]
	pub(super) type Kitties<T: Config> =
		StorageMap<_, Blake2_128Concat, Vec<u8>, Kitty<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_owned)]
	pub(super) type KittiesOwned<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, Vec<Vec<u8>>, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),

		Created {dna: Vec<u8>, owner: T::AccountId},
		Transfer {from: T::AccountId, to: T::AccountId, dna: Vec<u8>},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,

		DuplicateKitty,
		TooManyOwned,
		NoKitty,
		NotOwner,
		TransferToSelf,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;
			//let who2 = Config::Pallet::<T>::get();

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => return Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_kitty(origin: OriginFor<T>, dna: Vec<u8>, price: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;
			log::info!("total balance {:?}", T::Currency::total_balance(&who));
			let _gender = Self::gen_gender(&dna)?;
			let _kitty = Kitty::<T>{
				dna: dna.clone(),
				owner: who.clone(),
				price: price,
				gender: _gender,
				created_date: T::Time::now()
			};
			// check not exist
			ensure!(!Kitties::<T>::contains_key(&dna), Error::<T>::DuplicateKitty);

			let current_id = KittyId::<T>::get();
			let next_id = current_id.checked_add(1).ok_or(ArithmeticError::Overflow)?;

			Kitties::<T>::insert(&dna, _kitty);
			KittyId::<T>::put(next_id);

			KittiesOwned::<T>::append(&who, &dna);

			Self::deposit_event(Event::Created{dna: dna, owner: who });

			Ok(())
		}
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn transfer(origin: OriginFor<T>, dna: Vec<u8>, to: T::AccountId)-> DispatchResult{
			let who = ensure_signed(origin)?;
			let mut _kitty = Kitties::<T>::get(&dna).ok_or(Error::<T>::NoKitty)?;
			ensure!(_kitty.owner == who, Error::<T>::NotOwner);
			ensure!(who != to , Error::<T>::TransferToSelf);

			let mut _from_owned = KittiesOwned::<T>::get(&who);
			// remove of who
			if let Some(index) = _from_owned.iter().position(|ids| *ids == dna ){
				_from_owned.swap_remove(index);
			}else{
				return Err(Error::<T>::NoKitty.into());
			}
			let mut _to_owned = KittiesOwned::<T>::get(&to);
			_to_owned.push(dna.clone());
			_kitty.owner = to.clone();

			Kitties::<T>::insert(&dna, _kitty);
			KittiesOwned::<T>::insert(&to, _to_owned);
			KittiesOwned::<T>::insert(&who, _from_owned);

			Self::deposit_event(Event::Transfer{from: who , to: to, dna: dna});

			Ok(())
		}
	}
}

impl<T> Pallet<T> {
	fn gen_gender(dna: &Vec<u8>) -> Result<Gender, Error<T>> {
		let mut res = Gender::Female;
		if (dna.len() % 2 == 0) {
			res = Gender::Male;
		}
		Ok((res))
	}
}
