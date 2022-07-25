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
fn create_kitty() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let mut vec = Vec::new();
		vec.push(1);
		assert_ok!(KittiesModuleV2::create_kitty(Origin::signed(1), vec, 1));

		
	});
}
