//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Kitties;
use frame_benchmarking::account;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	create_kitty {
		// khởi tạo các tham số cho extrinsic benchmark
		let dnas : Vec<u8> = b"vinh".to_vec();

		let caller: T::AccountId = whitelisted_caller();
	}: create_kitty (RawOrigin::Signed(caller), dnas, 1)

	// kiểm tra lại trạng thái storage khi thực hiện extrinsic xem đúng chưa
	verify {
		assert_eq!(KittyId::<T>::get(), 1);
	}

	transfer{
		let dnas: Vec<u8> = b"vinh".to_vec();
		let caller: T::AccountId= whitelisted_caller();
		let to: T::AccountId = account("receiver", 0, 0);
		let from = <T as frame_system::Config>::Origin::from( RawOrigin::Signed(caller.clone()));
		Kitties::<T>::create_kitty(from, dnas.clone(), 1);
	}: transfer(RawOrigin::Signed(caller), dnas, to)


	impl_benchmark_test_suite!(Kitties, crate::mock::new_test_ext(), crate::mock::Test);
}
