use std::collections::HashSet;
use subxt::storage::address::StorageMapKey;
use subxt::storage::{StaticStorageAddress};
use crate::encointer_rococo::api::runtime_types::primitive_types::H256;
use crate::encointer_rococo::api::runtime_types::sp_core::crypto::AccountId32;

mod runtimes;
mod encointer_rococo;

pub struct PermissionedStorage {
	pallet_name: &'static str,
	entry_name: &'static str,
	/// The full storage key **including** the storage root, i.e, the pallet and entry name.
	storage_key: Vec<u8>,
	/// Who tries to access the storage
	account: AccountId32,
}





impl<ReturnTy, Fetchable, Defaultable, Iterable> From<StaticStorageAddress<ReturnTy, Fetchable, Defaultable, Iterable>> for PermissionedStorage {
	fn from(storage: StaticStorageAddress<ReturnTy, Fetchable, Defaultable, Iterable>) -> Self {
		Self {
			pallet_name: storage.pallet_name(),
			entry_name: storage.entry_name(),
			storage_entry_keys: storage.storage_entry_keys()
		}
	}
}

fn main() {

}
