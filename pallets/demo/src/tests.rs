use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t: sp_io::TestExternalities =
		frame_system::GenesisConfig::default().build_storage::<Test>().unwrap().into();
	t.execute_with(|| System::set_block_number(1));
	t
}

#[test]
fn create_student() {
	new_test_ext().execute_with(|| {
		let mut vec = Vec::new();
		vec.push(1);
		assert_ok!(DemoModule::create_student(Origin::signed(1), vec, 1));
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {});
}
