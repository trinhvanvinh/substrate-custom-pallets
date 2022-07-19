#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use frame_support::inherent::Vec;
use frame_support::dispatch::fmt;
use scale_info::TypeInfo;
pub type Id = u32;
use frame_support::traits::Currency;
use frame_support::traits::Time;
use frame_support::storage::bounded_vec::BoundedVec;

#[frame_support::pallet]
pub mod pallet {
	pub use super::*;

	#[derive(TypeInfo, Default, Encode, Decode)]
	#[scale_info(skip_type_params(T))]
	#[codec(mel_bound())]
	pub struct Kitty<T:Config >{
		dna: Vec<u8>,
		owner: T::AccountId,
		price: u32,
		gender: Gender,
		//exercise 2
		created_date: <<T as Config>::Time as Time>::Moment,
	}

	pub type Id = u32;

	#[derive(TypeInfo, Encode, Decode)]
	pub enum Gender{
		Male,
		Female
	}

	impl Default for Gender{
		fn default()-> Self{
			Gender::Female
		}
	}
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: Currency<Self::AccountId>;

		#[pallet::constant]
		type MaxKittyOwned: Get<u32>;

		type Time: Time;
	}
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	//save count Kitty
	#[pallet::storage]
	#[pallet::getter(fn kitty_id)]
	pub type KittyCount<T> = StorageValue<_, Id, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub(super) type Kitties<T: Config> = StorageMap<_, Blake2_128Concat, Id, Kitty<T>, OptionQuery>;

	// save info kitty use dna
	#[pallet::storage]
	#[pallet::getter(fn Infokitty)]
	pub(super) type InfoKitty<T:Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, Kitty<T>, OptionQuery>;

	// save owner kitty
	#[pallet::storage]
	#[pallet::getter(fn Ownerkitty)]
	pub(super) type OwnerKitty<T:Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Vec<u8>, OptionQuery>;


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		KittyStored(Vec<u8>, u32),
	}

	#[pallet::error]
	pub enum Error<T> {
		NotOwner,
		StorageOverflow,
		KittyNotExist
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_kitty(origin: OriginFor<T>, dna: Vec<u8>, price: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;

			log::info!("total balance {:?}", T::Currency::total_balance(&who));

			let gender =  Self::gen_gender(dna.clone())?;
			let _kitty = Kitty{
				dna: dna.clone(),
				owner: who.clone(),
				price: price,
				gender: gender,
				created_date: T::Time::now()
			};
			let mut current_id = <KittyCount<T>>::get();
			<Kitties<T>>::insert(current_id, &_kitty);
			current_id += 1;
			KittyCount::<T>::put(current_id);

			// Save info kitty follow dna
			<InfoKitty<T>>::insert(&dna,_kitty);

			// owner sở hữu bao nhiêu kitty: map(accountId => vec<dna>)
			 let ownerKitty = OwnerKitty::<T>::get(&who);
			 match ownerKitty{
				 Some(mut vector) =>{
					for val in &dna{
						vector.push(*val);
					}
					OwnerKitty::<T>::insert(who, vector);
				 } ,
				 None => OwnerKitty::<T>::insert(who,&dna) ,
			 }
				
			
			Self::deposit_event(Event::KittyStored(dna, price));
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn change_owner_kitty(origin: OriginFor<T>, _id: Id, _account: T::AccountId)-> DispatchResult{
			let who = ensure_signed(origin)?;
			let _kitties = <Kitties<T>>::get(_id).unwrap();

			// check ensure
			ensure!(who == _kitties.owner, Error::<T>::NotOwner);
			
			let _newKitty = Kitty {
				dna: _kitties.dna.clone(),
				owner: _account.clone(),
				price: _kitties.price,
				gender: _kitties.gender,
				created_date: _kitties.created_date
			};
			
			<Kitties<T>>::insert(_id, &_newKitty);

			// change info kitty
			<InfoKitty<T>>::insert(&_kitties.dna,_newKitty);

			// owner sở hữu bao nhiêu kitty: map(accountId => vec<dna>)
			let ownerKittyOld = OwnerKitty::<T>::get(&who);
			match ownerKittyOld{
				Some(mut vector) =>{
					// remove
					for val in &_kitties.dna{
						let index = vector.iter().position(|x| x == val).unwrap();
						vector.remove(index);
					}
				
				   OwnerKitty::<T>::insert(who, vector);
				} ,
				None => {}
			};

			let ownerKittyNew = OwnerKitty::<T>::get(&_account);
			match ownerKittyNew{
				Some(mut vectornew) =>{
					// add
					for val in &_kitties.dna{
						vectornew.push(*val);
					}

				   OwnerKitty::<T>::insert(_account, vectornew);
				  
				} ,
				None => {
					OwnerKitty::<T>::insert(_account, &_kitties.dna);
				}
			};

			
			Ok(())
		}

	}
}

impl<T> Pallet<T> {
	fn gen_gender(dna: Vec<u8>) -> Result<Gender, Error<T>>{
		let mut res = Gender::Female;
		if dna.len() % 2 == 0{
			res = Gender::Male;
		}
		Ok(res)
	}
}