#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use frame_support::dispatch::fmt;
use frame_support::inherent::Vec;
use frame_support::pallet_prelude::*;
use frame_support::sp_runtime::traits::Hash;
use frame_support::sp_runtime::ArithmeticError;
use frame_support::storage::bounded_vec::BoundedVec;
use frame_support::traits::Currency;
use frame_support::traits::Randomness;
use frame_support::traits::Time;

use frame_system::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_io::hashing::blake2_128;

type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {
	pub use super::*;

	#[derive(Clone, Encode, Decode, TypeInfo, PartialEq)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T: Config> {
		dna: Vec<u8>,
		owner: T::AccountId,
		price: u32,
		gender: Gender,
		//created_date: <<T as Config>::Time as Time>::Moment,
		created_date: u64,
	}

	// impl Debug trait Kitty
	impl<T: Config> fmt::Debug for Kitty<T> {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			f.debug_struct("Kitty")
				.field("dna", &self.dna)
				.field("owner", &self.owner)
				.field("price", &self.price)
				.field("gender", &self.gender)
				.field("created_date", &self.created_date)
				.finish()
		}
	}

	pub type Id = u32;

	#[derive(Clone, Encode, Decode, TypeInfo, PartialEq, Copy, MaxEncodedLen, Debug)]
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

		#[pallet::constant]
		type MaxKittyOwned: Get<u32>;

		type KittyRandomness: Randomness<Self::Hash, Self::BlockNumber>;
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

		#[pallet::storage]
		#[pallet::getter(fn kitty_owned_genesis)]
		pub(super) type KittiesOwnedGenesis<T: Config> =
			StorageMap<_, Blake2_128Concat, T::AccountId, Vec<u8>, ValueQuery>;
	

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::storage]
	pub type Nonce<T> = StorageValue<_, u32>;
	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),

		Created {
			dna: Vec<u8>,
			owner: T::AccountId,
		},
		Transfer {
			from: T::AccountId,
			to: T::AccountId,
			dna: Vec<u8>,
		},
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

		ExceedMaxKittyOwned,
	}


	// === exercise 4 ====
	// the genesis config type
	
	#[pallet::genesis_config]
	pub struct GenesisConfig<T:Config>{
		//pub _kitties: Vec< ( T::AccountId, Vec<u8>) > ,

		pub genesis_kitties: Vec<Vec<u8>>,
		pub owner: Option<T::AccountId>,
		pub current_time: u64
	}

	// the default value for the genesis config type
	#[cfg(feature="std")]
	impl<T:Config> Default for GenesisConfig<T>{
		fn default()-> GenesisConfig<T>{
		GenesisConfig{
			//_kitties: vec![],	
			genesis_kitties: Default::default(),
			owner: Default::default(),
			current_time: 0
		}
		}
	}
	
	// the build of genesis for the pallet
	#[pallet::genesis_build]
	impl<T:Config> GenesisBuild<T> for GenesisConfig<T>{
		fn build(&self){
			// for(b,c) in &self._kitties{
			// 	let k = <Pallet<T>>::fixKittyForAlice(b.clone(),c.clone());
			// }
			
			for item in self.genesis_kitties.iter(){
				let kitty = Kitty::<T>{
					dna: Pallet::<T>::gen_dna(),
					owner: self.owner.clone().unwrap(),
					price: 1,
					gender: Gender::Male,
					created_date: self.current_time
				};
				Kitties::insert(item, kitty);
			}
			
		}
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

		#[pallet::weight(31_000_000 + T::DbWeight::get().reads_writes(6,4))]
		pub fn create_kitty(origin: OriginFor<T>, dna: Vec<u8>, price: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;
			log::info!("total balance {:?}", T::Currency::total_balance(&who));
			let _gender = Self::gen_gender(&dna)?;

			let _kitty = Kitty::<T> {
				dna: Self::gen_dna(),
				owner: who.clone(),
				price,
				gender: _gender,
				//created_date: T::Time::now(),
				created_date: 0,
			};
			// check not exist
			ensure!(!Kitties::<T>::contains_key(&dna), Error::<T>::DuplicateKitty);

			// using debug for log kitty
			log::info!("Kitty {:?}", &_kitty);

			let current_id = KittyId::<T>::get();
			let next_id = current_id.checked_add(1).ok_or(ArithmeticError::Overflow)?;

			Kitties::<T>::insert(&dna, _kitty);
			KittyId::<T>::put(next_id);

			log::info!("MaxKittyOwned {}", T::MaxKittyOwned::get());

			let mut _to_owned = KittiesOwned::<T>::get(&who);
			ensure!(
				(_to_owned.len() as u32) < T::MaxKittyOwned::get(),
				Error::<T>::ExceedMaxKittyOwned
			);

			KittiesOwned::<T>::append(&who, &dna);

			Self::deposit_event(Event::Created { dna, owner: who });

			Ok(())
		}
		#[pallet::weight(21_000_000 + T::DbWeight::get().reads_writes(3,3))]
		pub fn transfer(origin: OriginFor<T>, dna: Vec<u8>, to: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let mut _kitty = Kitties::<T>::get(&dna).ok_or(Error::<T>::NoKitty)?;
			ensure!(_kitty.owner == who, Error::<T>::NotOwner);
			ensure!(who != to, Error::<T>::TransferToSelf);

			let mut _from_owned = KittiesOwned::<T>::get(&who);
			// remove of who
			if let Some(index) = _from_owned.iter().position(|ids| *ids == dna) {
				_from_owned.swap_remove(index);
			} else {
				return Err(Error::<T>::NoKitty.into());
			}
			let mut _to_owned = KittiesOwned::<T>::get(&to);
			log::info!("MaxKittyOwned2 {}", T::MaxKittyOwned::get());
			ensure!(
				(_to_owned.len() as u32) < T::MaxKittyOwned::get(),
				Error::<T>::ExceedMaxKittyOwned
			);

			_to_owned.push(dna.clone());
			_kitty.owner = to.clone();

			Kitties::<T>::insert(&dna, _kitty);
			KittiesOwned::<T>::insert(&to, _to_owned);
			KittiesOwned::<T>::insert(&who, _from_owned);

			Self::deposit_event(Event::Transfer { from: who, to, dna });

			Ok(())
		}

		

	}
}

impl<T: Config> Pallet<T> {

	
	pub fn fixKittyForAlice(who: T::AccountId, dna: Vec<u8>) -> DispatchResult{
		KittiesOwned::<T>::append(who, dna);

		Ok(())
	}

	fn gen_gender(dna: &Vec<u8>) -> Result<Gender, Error<T>> {
		let mut res = Gender::Female;
		if dna.len() % 2 == 0 {
			res = Gender::Male;
		}
		Ok(res)
	}

	fn gen_dna() -> Vec<u8> {
		let nonce = Self::get_and_increment_nonce();
		let rand = T::KittyRandomness::random(&nonce).0;
		log::info!("random {:?}", rand.as_ref().to_vec());
		rand.as_ref().to_vec()

		// let nonce = Self::get_and_increment_nonce();
		// let (randomValue, _) = T::KittyRandomness::random(&nonce);
		// log::info!("randomValue {:?}", randomValue);
		// randomValue as u8
	}

	fn get_and_increment_nonce() -> Vec<u8> {
		let nonce = Nonce::<T>::get();
		match nonce {
			Some(a) => Nonce::<T>::put(nonce.unwrap() + 1),
			None => Nonce::<T>::put(1),
		}
		nonce.encode()
	}

	// pub fn mint(
	// 	owner: &T::AccountId,
	// 	dna: Option<[u8; 16]>,
	// 	gender: Option<Gender>,
	// ) -> Result<T::Hash, Error<T>> {
	// 	let kitty = Kitty::<T> {
	// 		dna: dna.unwrap_or_else(Self::gen_dna),
	// 		price: None,
	// 		gender: gender.unwrap_or_else(Self::gen_gender),
	// 		owner: owner.clone(),
	// 	};
	
	// 	let kitty_id = T::Hashing::hash_of(&kitty);
	
	// 	// Performs this operation first as it may fail
	// 	let new_cnt = Self::kitty_cnt().checked_add(1)
	// 	.ok_or(<Error<T>>::KittyCntOverflow)?;
	
	// 	// Performs this operation first because as it may fail
	// 	<KittiesOwned<T>>::try_mutate(&owner, |kitty_vec| {kitty_vec.try_push(kitty_id)}).map_err(|_| <Error<T>>::ExceedMaxKittyOwned)?;
	
	// 	<Kitties<T>>::insert(kitty_id, kitty);
	// 	<KittyCnt<T>>::put(new_cnt);
	// 	Ok(kitty_id)
	// }
}
