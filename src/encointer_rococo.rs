#[allow(dead_code, unused_imports, non_camel_case_types)]
pub mod api {
    use super::api as root_mod;
    pub static PALLETS: [&str; 25usize] = [
        "System",
        "ParachainSystem",
        "RandomnessCollectiveFlip",
        "Timestamp",
        "ParachainInfo",
        "Balances",
        "TransactionPayment",
        "AssetTxPayment",
        "Aura",
        "AuraExt",
        "XcmpQueue",
        "PolkadotXcm",
        "CumulusXcm",
        "DmpQueue",
        "Utility",
        "Treasury",
        "Proxy",
        "Scheduler",
        "Collective",
        "Membership",
        "EncointerScheduler",
        "EncointerCeremonies",
        "EncointerCommunities",
        "EncointerBalances",
        "EncointerBazaar",
    ];
    #[derive(:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug)]
    pub enum Event {
        #[codec(index = 0)]
        System(system::Event),
        #[codec(index = 1)]
        ParachainSystem(parachain_system::Event),
        #[codec(index = 10)]
        Balances(balances::Event),
        #[codec(index = 11)]
        TransactionPayment(transaction_payment::Event),
        #[codec(index = 12)]
        AssetTxPayment(asset_tx_payment::Event),
        #[codec(index = 30)]
        XcmpQueue(xcmp_queue::Event),
        #[codec(index = 31)]
        PolkadotXcm(polkadot_xcm::Event),
        #[codec(index = 32)]
        CumulusXcm(cumulus_xcm::Event),
        #[codec(index = 33)]
        DmpQueue(dmp_queue::Event),
        #[codec(index = 40)]
        Utility(utility::Event),
        #[codec(index = 43)]
        Treasury(treasury::Event),
        #[codec(index = 44)]
        Proxy(proxy::Event),
        #[codec(index = 48)]
        Scheduler(scheduler::Event),
        #[codec(index = 50)]
        Collective(collective::Event),
        #[codec(index = 51)]
        Membership(membership::Event),
        #[codec(index = 60)]
        EncointerScheduler(encointer_scheduler::Event),
        #[codec(index = 61)]
        EncointerCeremonies(encointer_ceremonies::Event),
        #[codec(index = 62)]
        EncointerCommunities(encointer_communities::Event),
        #[codec(index = 63)]
        EncointerBalances(encointer_balances::Event),
        #[codec(index = 64)]
        EncointerBazaar(encointer_bazaar::Event),
    }
    pub mod system {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct FillBlock {
                pub ratio: runtime_types::sp_arithmetic::per_things::Perbill,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Remark {
                pub remark: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct SetHeapPages {
                pub pages: ::core::primitive::u64,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SetCode {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SetCodeWithoutChecks {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SetStorage {
                pub items: ::std::vec::Vec<(
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::std::vec::Vec<::core::primitive::u8>,
                )>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct KillStorage {
                pub keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct KillPrefix {
                pub prefix: ::std::vec::Vec<::core::primitive::u8>,
                pub subkeys: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct RemarkWithEvent {
                pub remark: ::std::vec::Vec<::core::primitive::u8>,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "A dispatch that will fill the block weight up to the given ratio."]
                pub fn fill_block(
                    &self,
                    ratio: runtime_types::sp_arithmetic::per_things::Perbill,
                ) -> ::subxt::tx::StaticTxPayload<FillBlock> {
                    ::subxt::tx::StaticTxPayload::new(
                        "System",
                        "fill_block",
                        FillBlock { ratio },
                        [
                            48u8, 18u8, 205u8, 90u8, 222u8, 4u8, 20u8, 251u8, 173u8, 76u8, 167u8,
                            4u8, 83u8, 203u8, 160u8, 89u8, 132u8, 218u8, 191u8, 145u8, 130u8,
                            245u8, 177u8, 201u8, 169u8, 129u8, 173u8, 105u8, 88u8, 45u8, 136u8,
                            191u8,
                        ],
                    )
                }
                #[doc = "Make some on-chain remark."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(1)`"]
                #[doc = "# </weight>"]
                pub fn remark(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<Remark> {
                    ::subxt::tx::StaticTxPayload::new(
                        "System",
                        "remark",
                        Remark { remark },
                        [
                            101u8, 80u8, 195u8, 226u8, 224u8, 247u8, 60u8, 128u8, 3u8, 101u8, 51u8,
                            147u8, 96u8, 126u8, 76u8, 230u8, 194u8, 227u8, 191u8, 73u8, 160u8,
                            146u8, 87u8, 147u8, 243u8, 28u8, 228u8, 116u8, 224u8, 181u8, 129u8,
                            160u8,
                        ],
                    )
                }
                #[doc = "Set the number of pages in the WebAssembly environment's heap."]
                pub fn set_heap_pages(
                    &self,
                    pages: ::core::primitive::u64,
                ) -> ::subxt::tx::StaticTxPayload<SetHeapPages> {
                    ::subxt::tx::StaticTxPayload::new(
                        "System",
                        "set_heap_pages",
                        SetHeapPages { pages },
                        [
                            43u8, 103u8, 128u8, 49u8, 156u8, 136u8, 11u8, 204u8, 80u8, 6u8, 244u8,
                            86u8, 171u8, 44u8, 140u8, 225u8, 142u8, 198u8, 43u8, 87u8, 26u8, 45u8,
                            125u8, 222u8, 165u8, 254u8, 172u8, 158u8, 39u8, 178u8, 86u8, 87u8,
                        ],
                    )
                }
                #[doc = "Set the new runtime code."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(C + S)` where `C` length of `code` and `S` complexity of `can_set_code`"]
                #[doc = "- 1 call to `can_set_code`: `O(S)` (calls `sp_io::misc::runtime_version` which is"]
                #[doc = "  expensive)."]
                #[doc = "- 1 storage write (codec `O(C)`)."]
                #[doc = "- 1 digest item."]
                #[doc = "- 1 event."]
                #[doc = "The weight of this function is dependent on the runtime, but generally this is very"]
                #[doc = "expensive. We will treat this as a full block."]
                #[doc = "# </weight>"]
                pub fn set_code(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<SetCode> {
                    ::subxt::tx::StaticTxPayload::new(
                        "System",
                        "set_code",
                        SetCode { code },
                        [
                            27u8, 104u8, 244u8, 205u8, 188u8, 254u8, 121u8, 13u8, 106u8, 120u8,
                            244u8, 108u8, 97u8, 84u8, 100u8, 68u8, 26u8, 69u8, 93u8, 128u8, 107u8,
                            4u8, 3u8, 142u8, 13u8, 134u8, 196u8, 62u8, 113u8, 181u8, 14u8, 40u8,
                        ],
                    )
                }
                #[doc = "Set the new runtime code without doing any checks of the given `code`."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(C)` where `C` length of `code`"]
                #[doc = "- 1 storage write (codec `O(C)`)."]
                #[doc = "- 1 digest item."]
                #[doc = "- 1 event."]
                #[doc = "The weight of this function is dependent on the runtime. We will treat this as a full"]
                #[doc = "block. # </weight>"]
                pub fn set_code_without_checks(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<SetCodeWithoutChecks> {
                    ::subxt::tx::StaticTxPayload::new(
                        "System",
                        "set_code_without_checks",
                        SetCodeWithoutChecks { code },
                        [
                            102u8, 160u8, 125u8, 235u8, 30u8, 23u8, 45u8, 239u8, 112u8, 148u8,
                            159u8, 158u8, 42u8, 93u8, 206u8, 94u8, 80u8, 250u8, 66u8, 195u8, 60u8,
                            40u8, 142u8, 169u8, 183u8, 80u8, 80u8, 96u8, 3u8, 231u8, 99u8, 216u8,
                        ],
                    )
                }
                #[doc = "Set some items of storage."]
                pub fn set_storage(
                    &self,
                    items: ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                ) -> ::subxt::tx::StaticTxPayload<SetStorage> {
                    ::subxt::tx::StaticTxPayload::new(
                        "System",
                        "set_storage",
                        SetStorage { items },
                        [
                            74u8, 43u8, 106u8, 255u8, 50u8, 151u8, 192u8, 155u8, 14u8, 90u8, 19u8,
                            45u8, 165u8, 16u8, 235u8, 242u8, 21u8, 131u8, 33u8, 172u8, 119u8, 78u8,
                            140u8, 10u8, 107u8, 202u8, 122u8, 235u8, 181u8, 191u8, 22u8, 116u8,
                        ],
                    )
                }
                #[doc = "Kill some items from storage."]
                pub fn kill_storage(
                    &self,
                    keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                ) -> ::subxt::tx::StaticTxPayload<KillStorage> {
                    ::subxt::tx::StaticTxPayload::new(
                        "System",
                        "kill_storage",
                        KillStorage { keys },
                        [
                            174u8, 174u8, 13u8, 174u8, 75u8, 138u8, 128u8, 235u8, 222u8, 216u8,
                            85u8, 18u8, 198u8, 1u8, 138u8, 70u8, 19u8, 108u8, 209u8, 41u8, 228u8,
                            67u8, 130u8, 230u8, 160u8, 207u8, 11u8, 180u8, 139u8, 242u8, 41u8,
                            15u8,
                        ],
                    )
                }
                #[doc = "Kill all storage items with a key that starts with the given prefix."]
                #[doc = ""]
                #[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
                #[doc = "the prefix we are removing to accurately calculate the weight of this function."]
                pub fn kill_prefix(
                    &self,
                    prefix: ::std::vec::Vec<::core::primitive::u8>,
                    subkeys: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<KillPrefix> {
                    ::subxt::tx::StaticTxPayload::new(
                        "System",
                        "kill_prefix",
                        KillPrefix { prefix, subkeys },
                        [
                            203u8, 116u8, 217u8, 42u8, 154u8, 215u8, 77u8, 217u8, 13u8, 22u8,
                            193u8, 2u8, 128u8, 115u8, 179u8, 115u8, 187u8, 218u8, 129u8, 34u8,
                            80u8, 4u8, 173u8, 120u8, 92u8, 35u8, 237u8, 112u8, 201u8, 207u8, 200u8,
                            48u8,
                        ],
                    )
                }
                #[doc = "Make some on-chain remark and emit event."]
                pub fn remark_with_event(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<RemarkWithEvent> {
                    ::subxt::tx::StaticTxPayload::new(
                        "System",
                        "remark_with_event",
                        RemarkWithEvent { remark },
                        [
                            123u8, 225u8, 180u8, 179u8, 144u8, 74u8, 27u8, 85u8, 101u8, 75u8,
                            134u8, 44u8, 181u8, 25u8, 183u8, 158u8, 14u8, 213u8, 56u8, 225u8,
                            136u8, 88u8, 26u8, 114u8, 178u8, 43u8, 176u8, 43u8, 240u8, 84u8, 116u8,
                            46u8,
                        ],
                    )
                }
            }
        }
        #[doc = "Event for the System pallet."]
        pub type Event = runtime_types::frame_system::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "An extrinsic completed successfully."]
            pub struct ExtrinsicSuccess {
                pub dispatch_info: runtime_types::frame_support::weights::DispatchInfo,
            }
            impl ::subxt::events::StaticEvent for ExtrinsicSuccess {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicSuccess";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "An extrinsic failed."]
            pub struct ExtrinsicFailed {
                pub dispatch_error: runtime_types::sp_runtime::DispatchError,
                pub dispatch_info: runtime_types::frame_support::weights::DispatchInfo,
            }
            impl ::subxt::events::StaticEvent for ExtrinsicFailed {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicFailed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "`:code` was updated."]
            pub struct CodeUpdated;
            impl ::subxt::events::StaticEvent for CodeUpdated {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "CodeUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A new account was created."]
            pub struct NewAccount {
                pub account: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            impl ::subxt::events::StaticEvent for NewAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "NewAccount";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "An account was reaped."]
            pub struct KilledAccount {
                pub account: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            impl ::subxt::events::StaticEvent for KilledAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "KilledAccount";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "On on-chain remark happened."]
            pub struct Remarked {
                pub sender: ::subxt::ext::sp_core::crypto::AccountId32,
                pub hash: ::subxt::ext::sp_core::H256,
            }
            impl ::subxt::events::StaticEvent for Remarked {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "Remarked";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The full account information for a particular account ID."]
                pub fn account(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::frame_system::AccountInfo<
                            ::core::primitive::u32,
                            runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "Account",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            176u8, 187u8, 21u8, 220u8, 159u8, 204u8, 127u8, 14u8, 21u8, 69u8, 77u8,
                            114u8, 230u8, 141u8, 107u8, 79u8, 23u8, 16u8, 174u8, 243u8, 252u8,
                            42u8, 65u8, 120u8, 229u8, 38u8, 210u8, 255u8, 22u8, 40u8, 109u8, 223u8,
                        ],
                    )
                }
                #[doc = " The full account information for a particular account ID."]
                pub fn account_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::frame_system::AccountInfo<
                            ::core::primitive::u32,
                            runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
                        >,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "Account",
                        Vec::new(),
                        [
                            176u8, 187u8, 21u8, 220u8, 159u8, 204u8, 127u8, 14u8, 21u8, 69u8, 77u8,
                            114u8, 230u8, 141u8, 107u8, 79u8, 23u8, 16u8, 174u8, 243u8, 252u8,
                            42u8, 65u8, 120u8, 229u8, 38u8, 210u8, 255u8, 22u8, 40u8, 109u8, 223u8,
                        ],
                    )
                }
                #[doc = " Total extrinsics count for the current block."]
                pub fn extrinsic_count(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "ExtrinsicCount",
                        vec![],
                        [
                            223u8, 60u8, 201u8, 120u8, 36u8, 44u8, 180u8, 210u8, 242u8, 53u8,
                            222u8, 154u8, 123u8, 176u8, 249u8, 8u8, 225u8, 28u8, 232u8, 4u8, 136u8,
                            41u8, 151u8, 82u8, 189u8, 149u8, 49u8, 166u8, 139u8, 9u8, 163u8, 231u8,
                        ],
                    )
                }
                #[doc = " The current weight for the block."]
                pub fn block_weight(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::frame_support::weights::PerDispatchClass<
                            ::core::primitive::u64,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "BlockWeight",
                        vec![],
                        [
                            91u8, 211u8, 177u8, 36u8, 147u8, 249u8, 55u8, 164u8, 48u8, 49u8, 55u8,
                            11u8, 121u8, 193u8, 103u8, 69u8, 38u8, 142u8, 148u8, 36u8, 137u8, 41u8,
                            115u8, 195u8, 31u8, 174u8, 163u8, 125u8, 69u8, 5u8, 94u8, 79u8,
                        ],
                    )
                }
                #[doc = " Total length (in bytes) for all extrinsics put together, for the current block."]
                pub fn all_extrinsics_len(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "AllExtrinsicsLen",
                        vec![],
                        [
                            202u8, 145u8, 209u8, 225u8, 40u8, 220u8, 174u8, 74u8, 93u8, 164u8,
                            254u8, 248u8, 254u8, 192u8, 32u8, 117u8, 96u8, 149u8, 53u8, 145u8,
                            219u8, 64u8, 234u8, 18u8, 217u8, 200u8, 203u8, 141u8, 145u8, 28u8,
                            134u8, 60u8,
                        ],
                    )
                }
                #[doc = " Map of block numbers to block hashes."]
                pub fn block_hash(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::H256>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "BlockHash",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Twox64Concat,
                        )],
                        [
                            50u8, 112u8, 176u8, 239u8, 175u8, 18u8, 205u8, 20u8, 241u8, 195u8,
                            21u8, 228u8, 186u8, 57u8, 200u8, 25u8, 38u8, 44u8, 106u8, 20u8, 168u8,
                            80u8, 76u8, 235u8, 12u8, 51u8, 137u8, 149u8, 200u8, 4u8, 220u8, 237u8,
                        ],
                    )
                }
                #[doc = " Map of block numbers to block hashes."]
                pub fn block_hash_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::H256>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "BlockHash",
                        Vec::new(),
                        [
                            50u8, 112u8, 176u8, 239u8, 175u8, 18u8, 205u8, 20u8, 241u8, 195u8,
                            21u8, 228u8, 186u8, 57u8, 200u8, 25u8, 38u8, 44u8, 106u8, 20u8, 168u8,
                            80u8, 76u8, 235u8, 12u8, 51u8, 137u8, 149u8, 200u8, 4u8, 220u8, 237u8,
                        ],
                    )
                }
                #[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
                pub fn extrinsic_data(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "ExtrinsicData",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Twox64Concat,
                        )],
                        [
                            210u8, 224u8, 211u8, 186u8, 118u8, 210u8, 185u8, 194u8, 238u8, 211u8,
                            254u8, 73u8, 67u8, 184u8, 31u8, 229u8, 168u8, 125u8, 98u8, 23u8, 241u8,
                            59u8, 49u8, 86u8, 126u8, 9u8, 114u8, 163u8, 160u8, 62u8, 50u8, 67u8,
                        ],
                    )
                }
                #[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
                pub fn extrinsic_data_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "ExtrinsicData",
                        Vec::new(),
                        [
                            210u8, 224u8, 211u8, 186u8, 118u8, 210u8, 185u8, 194u8, 238u8, 211u8,
                            254u8, 73u8, 67u8, 184u8, 31u8, 229u8, 168u8, 125u8, 98u8, 23u8, 241u8,
                            59u8, 49u8, 86u8, 126u8, 9u8, 114u8, 163u8, 160u8, 62u8, 50u8, 67u8,
                        ],
                    )
                }
                #[doc = " The current block number being processed. Set by `execute_block`."]
                pub fn number(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "Number",
                        vec![],
                        [
                            228u8, 96u8, 102u8, 190u8, 252u8, 130u8, 239u8, 172u8, 126u8, 235u8,
                            246u8, 139u8, 208u8, 15u8, 88u8, 245u8, 141u8, 232u8, 43u8, 204u8,
                            36u8, 87u8, 211u8, 141u8, 187u8, 68u8, 236u8, 70u8, 193u8, 235u8,
                            164u8, 191u8,
                        ],
                    )
                }
                #[doc = " Hash of the previous block."]
                pub fn parent_hash(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::H256>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "ParentHash",
                        vec![],
                        [
                            232u8, 206u8, 177u8, 119u8, 38u8, 57u8, 233u8, 50u8, 225u8, 49u8,
                            169u8, 176u8, 210u8, 51u8, 231u8, 176u8, 234u8, 186u8, 188u8, 112u8,
                            15u8, 152u8, 195u8, 232u8, 201u8, 97u8, 208u8, 249u8, 9u8, 163u8, 69u8,
                            36u8,
                        ],
                    )
                }
                #[doc = " Digest of the current block, also part of the block header."]
                pub fn digest(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::sp_runtime::generic::digest::Digest,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "Digest",
                        vec![],
                        [
                            83u8, 141u8, 200u8, 132u8, 182u8, 55u8, 197u8, 122u8, 13u8, 159u8,
                            31u8, 42u8, 60u8, 191u8, 89u8, 221u8, 242u8, 47u8, 199u8, 213u8, 48u8,
                            216u8, 131u8, 168u8, 245u8, 82u8, 56u8, 190u8, 62u8, 69u8, 96u8, 37u8,
                        ],
                    )
                }
                #[doc = " Events deposited for the current block."]
                #[doc = ""]
                #[doc = " NOTE: The item is unbound and should therefore never be read on chain."]
                #[doc = " It could otherwise inflate the PoV size of a block."]
                #[doc = ""]
                #[doc = " Events have a large in-memory size. Box the events to not go out-of-memory"]
                #[doc = " just in case someone still reads them from within the runtime."]
                pub fn events(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<
                            runtime_types::frame_system::EventRecord<
                                runtime_types::encointer_runtime::Event,
                                ::subxt::ext::sp_core::H256,
                            >,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "Events",
                        vec![],
                        [
                            179u8, 63u8, 187u8, 202u8, 7u8, 70u8, 97u8, 135u8, 37u8, 42u8, 134u8,
                            35u8, 101u8, 164u8, 169u8, 172u8, 182u8, 19u8, 221u8, 137u8, 62u8,
                            135u8, 184u8, 119u8, 122u8, 110u8, 189u8, 238u8, 67u8, 55u8, 168u8,
                            165u8,
                        ],
                    )
                }
                #[doc = " The number of events in the `Events<T>` list."]
                pub fn event_count(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "EventCount",
                        vec![],
                        [
                            236u8, 93u8, 90u8, 177u8, 250u8, 211u8, 138u8, 187u8, 26u8, 208u8,
                            203u8, 113u8, 221u8, 233u8, 227u8, 9u8, 249u8, 25u8, 202u8, 185u8,
                            161u8, 144u8, 167u8, 104u8, 127u8, 187u8, 38u8, 18u8, 52u8, 61u8, 66u8,
                            112u8,
                        ],
                    )
                }
                #[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
                #[doc = " of events in the `<Events<T>>` list."]
                #[doc = ""]
                #[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
                #[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
                #[doc = " in case of changes fetch the list of events of interest."]
                #[doc = ""]
                #[doc = " The value has the type `(T::BlockNumber, EventIndex)` because if we used only just"]
                #[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
                #[doc = " no notification will be triggered thus the event might be lost."]
                pub fn event_topics(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::H256>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "EventTopics",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            205u8, 90u8, 142u8, 190u8, 176u8, 37u8, 94u8, 82u8, 98u8, 1u8, 129u8,
                            63u8, 246u8, 101u8, 130u8, 58u8, 216u8, 16u8, 139u8, 196u8, 154u8,
                            111u8, 110u8, 178u8, 24u8, 44u8, 183u8, 176u8, 232u8, 82u8, 223u8,
                            38u8,
                        ],
                    )
                }
                #[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
                #[doc = " of events in the `<Events<T>>` list."]
                #[doc = ""]
                #[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
                #[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
                #[doc = " in case of changes fetch the list of events of interest."]
                #[doc = ""]
                #[doc = " The value has the type `(T::BlockNumber, EventIndex)` because if we used only just"]
                #[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
                #[doc = " no notification will be triggered thus the event might be lost."]
                pub fn event_topics_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "EventTopics",
                        Vec::new(),
                        [
                            205u8, 90u8, 142u8, 190u8, 176u8, 37u8, 94u8, 82u8, 98u8, 1u8, 129u8,
                            63u8, 246u8, 101u8, 130u8, 58u8, 216u8, 16u8, 139u8, 196u8, 154u8,
                            111u8, 110u8, 178u8, 24u8, 44u8, 183u8, 176u8, 232u8, 82u8, 223u8,
                            38u8,
                        ],
                    )
                }
                #[doc = " Stores the `spec_version` and `spec_name` of when the last runtime upgrade happened."]
                pub fn last_runtime_upgrade(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::frame_system::LastRuntimeUpgradeInfo,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "LastRuntimeUpgrade",
                        vec![],
                        [
                            52u8, 37u8, 117u8, 111u8, 57u8, 130u8, 196u8, 14u8, 99u8, 77u8, 91u8,
                            126u8, 178u8, 249u8, 78u8, 34u8, 9u8, 194u8, 92u8, 105u8, 113u8, 81u8,
                            185u8, 127u8, 245u8, 184u8, 60u8, 29u8, 234u8, 182u8, 96u8, 196u8,
                        ],
                    )
                }
                #[doc = " True if we have upgraded so that `type RefCount` is `u32`. False (default) if not."]
                pub fn upgraded_to_u32_ref_count(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "UpgradedToU32RefCount",
                        vec![],
                        [
                            171u8, 88u8, 244u8, 92u8, 122u8, 67u8, 27u8, 18u8, 59u8, 175u8, 175u8,
                            178u8, 20u8, 150u8, 213u8, 59u8, 222u8, 141u8, 32u8, 107u8, 3u8, 114u8,
                            83u8, 250u8, 180u8, 233u8, 152u8, 54u8, 187u8, 99u8, 131u8, 204u8,
                        ],
                    )
                }
                #[doc = " True if we have upgraded so that AccountInfo contains three types of `RefCount`. False"]
                #[doc = " (default) if not."]
                pub fn upgraded_to_triple_ref_count(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "UpgradedToTripleRefCount",
                        vec![],
                        [
                            90u8, 33u8, 56u8, 86u8, 90u8, 101u8, 89u8, 133u8, 203u8, 56u8, 201u8,
                            210u8, 244u8, 232u8, 150u8, 18u8, 51u8, 105u8, 14u8, 230u8, 103u8,
                            155u8, 246u8, 99u8, 53u8, 207u8, 225u8, 128u8, 186u8, 76u8, 40u8,
                            185u8,
                        ],
                    )
                }
                #[doc = " The execution phase of the block."]
                pub fn execution_phase(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<runtime_types::frame_system::Phase>,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "ExecutionPhase",
                        vec![],
                        [
                            230u8, 183u8, 221u8, 135u8, 226u8, 223u8, 55u8, 104u8, 138u8, 224u8,
                            103u8, 156u8, 222u8, 99u8, 203u8, 199u8, 164u8, 168u8, 193u8, 133u8,
                            201u8, 155u8, 63u8, 95u8, 17u8, 206u8, 165u8, 123u8, 161u8, 33u8,
                            172u8, 93u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " Block & extrinsics weights: base values and limits."]
                pub fn block_weights(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::frame_system::limits::BlockWeights,
                    >,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "System",
                        "BlockWeights",
                        [
                            153u8, 164u8, 86u8, 79u8, 97u8, 114u8, 248u8, 181u8, 179u8, 186u8,
                            214u8, 124u8, 215u8, 96u8, 116u8, 109u8, 215u8, 182u8, 61u8, 10u8,
                            77u8, 74u8, 29u8, 125u8, 131u8, 111u8, 249u8, 208u8, 233u8, 170u8,
                            11u8, 14u8,
                        ],
                    )
                }
                #[doc = " The maximum length of a block (in bytes)."]
                pub fn block_length(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::frame_system::limits::BlockLength,
                    >,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "System",
                        "BlockLength",
                        [
                            116u8, 184u8, 225u8, 228u8, 207u8, 203u8, 4u8, 220u8, 234u8, 198u8,
                            150u8, 108u8, 205u8, 87u8, 194u8, 131u8, 229u8, 51u8, 140u8, 4u8, 47u8,
                            12u8, 200u8, 144u8, 153u8, 62u8, 51u8, 39u8, 138u8, 205u8, 203u8,
                            236u8,
                        ],
                    )
                }
                #[doc = " Maximum number of block number to block hash mappings to keep (oldest pruned first)."]
                pub fn block_hash_count(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "System",
                        "BlockHashCount",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The weight of runtime database operations the runtime can invoke."]
                pub fn db_weight(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::frame_support::weights::RuntimeDbWeight,
                    >,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "System",
                        "DbWeight",
                        [
                            124u8, 162u8, 190u8, 149u8, 49u8, 177u8, 162u8, 231u8, 62u8, 167u8,
                            199u8, 181u8, 43u8, 232u8, 185u8, 116u8, 195u8, 51u8, 233u8, 223u8,
                            20u8, 129u8, 246u8, 13u8, 65u8, 180u8, 64u8, 9u8, 157u8, 59u8, 245u8,
                            118u8,
                        ],
                    )
                }
                #[doc = " Get the chain's current version."]
                pub fn version(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<runtime_types::sp_version::RuntimeVersion>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "System",
                        "Version",
                        [
                            93u8, 98u8, 57u8, 243u8, 229u8, 8u8, 234u8, 231u8, 72u8, 230u8, 139u8,
                            47u8, 63u8, 181u8, 17u8, 2u8, 220u8, 231u8, 104u8, 237u8, 185u8, 143u8,
                            165u8, 253u8, 188u8, 76u8, 147u8, 12u8, 170u8, 26u8, 74u8, 200u8,
                        ],
                    )
                }
                #[doc = " The designated SS85 prefix of this chain."]
                #[doc = ""]
                #[doc = " This replaces the \"ss58Format\" property declared in the chain spec. Reason is"]
                #[doc = " that the runtime should know about the prefix in order to make use of it as"]
                #[doc = " an identifier of the chain."]
                pub fn ss58_prefix(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u16>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "System",
                        "SS58Prefix",
                        [
                            116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8, 169u8, 167u8, 227u8,
                            41u8, 144u8, 11u8, 236u8, 82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8,
                            90u8, 208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8, 193u8, 29u8, 70u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod parachain_system {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SetValidationData {
                pub data:
                    runtime_types::cumulus_primitives_parachain_inherent::ParachainInherentData,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SudoSendUpwardMessage {
                pub message: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct AuthorizeUpgrade {
                pub code_hash: ::subxt::ext::sp_core::H256,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct EnactAuthorizedUpgrade {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Set the current validation data."]
                #[doc = ""]
                #[doc = "This should be invoked exactly once per block. It will panic at the finalization"]
                #[doc = "phase if the call was not invoked."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be `Inherent`"]
                #[doc = ""]
                #[doc = "As a side effect, this function upgrades the current validation function"]
                #[doc = "if the appropriate time has come."]
                pub fn set_validation_data(
                    &self,
                    data : runtime_types :: cumulus_primitives_parachain_inherent :: ParachainInherentData,
                ) -> ::subxt::tx::StaticTxPayload<SetValidationData> {
                    ::subxt::tx::StaticTxPayload::new(
                        "ParachainSystem",
                        "set_validation_data",
                        SetValidationData { data },
                        [
                            200u8, 80u8, 163u8, 177u8, 184u8, 117u8, 61u8, 203u8, 244u8, 214u8,
                            106u8, 151u8, 128u8, 131u8, 254u8, 120u8, 254u8, 76u8, 104u8, 39u8,
                            215u8, 227u8, 233u8, 254u8, 26u8, 62u8, 17u8, 42u8, 19u8, 127u8, 108u8,
                            242u8,
                        ],
                    )
                }
                pub fn sudo_send_upward_message(
                    &self,
                    message: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<SudoSendUpwardMessage> {
                    ::subxt::tx::StaticTxPayload::new(
                        "ParachainSystem",
                        "sudo_send_upward_message",
                        SudoSendUpwardMessage { message },
                        [
                            127u8, 79u8, 45u8, 183u8, 190u8, 205u8, 184u8, 169u8, 255u8, 191u8,
                            86u8, 154u8, 134u8, 25u8, 249u8, 63u8, 47u8, 194u8, 108u8, 62u8, 60u8,
                            170u8, 81u8, 240u8, 113u8, 48u8, 181u8, 171u8, 95u8, 63u8, 26u8, 222u8,
                        ],
                    )
                }
                pub fn authorize_upgrade(
                    &self,
                    code_hash: ::subxt::ext::sp_core::H256,
                ) -> ::subxt::tx::StaticTxPayload<AuthorizeUpgrade> {
                    ::subxt::tx::StaticTxPayload::new(
                        "ParachainSystem",
                        "authorize_upgrade",
                        AuthorizeUpgrade { code_hash },
                        [
                            52u8, 152u8, 69u8, 207u8, 143u8, 113u8, 163u8, 11u8, 181u8, 182u8,
                            124u8, 101u8, 207u8, 19u8, 59u8, 81u8, 129u8, 29u8, 79u8, 115u8, 90u8,
                            83u8, 225u8, 124u8, 21u8, 108u8, 99u8, 194u8, 78u8, 83u8, 252u8, 163u8,
                        ],
                    )
                }
                pub fn enact_authorized_upgrade(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<EnactAuthorizedUpgrade> {
                    ::subxt::tx::StaticTxPayload::new(
                        "ParachainSystem",
                        "enact_authorized_upgrade",
                        EnactAuthorizedUpgrade { code },
                        [
                            43u8, 157u8, 1u8, 230u8, 134u8, 72u8, 230u8, 35u8, 159u8, 13u8, 201u8,
                            134u8, 184u8, 94u8, 167u8, 13u8, 108u8, 157u8, 145u8, 166u8, 119u8,
                            37u8, 51u8, 121u8, 252u8, 255u8, 48u8, 251u8, 126u8, 152u8, 247u8, 5u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::cumulus_pallet_parachain_system::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "The validation function has been scheduled to apply."]
            pub struct ValidationFunctionStored;
            impl ::subxt::events::StaticEvent for ValidationFunctionStored {
                const PALLET: &'static str = "ParachainSystem";
                const EVENT: &'static str = "ValidationFunctionStored";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            #[doc = "The validation function was applied as of the contained relay chain block number."]
            pub struct ValidationFunctionApplied {
                pub relay_chain_block_num: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for ValidationFunctionApplied {
                const PALLET: &'static str = "ParachainSystem";
                const EVENT: &'static str = "ValidationFunctionApplied";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "The relay-chain aborted the upgrade process."]
            pub struct ValidationFunctionDiscarded;
            impl ::subxt::events::StaticEvent for ValidationFunctionDiscarded {
                const PALLET: &'static str = "ParachainSystem";
                const EVENT: &'static str = "ValidationFunctionDiscarded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "An upgrade has been authorized."]
            pub struct UpgradeAuthorized {
                pub code_hash: ::subxt::ext::sp_core::H256,
            }
            impl ::subxt::events::StaticEvent for UpgradeAuthorized {
                const PALLET: &'static str = "ParachainSystem";
                const EVENT: &'static str = "UpgradeAuthorized";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            #[doc = "Some downward messages have been received and will be processed."]
            pub struct DownwardMessagesReceived {
                pub count: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for DownwardMessagesReceived {
                const PALLET: &'static str = "ParachainSystem";
                const EVENT: &'static str = "DownwardMessagesReceived";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Downward messages were processed using the given weight."]
            pub struct DownwardMessagesProcessed {
                pub weight_used: ::core::primitive::u64,
                pub dmq_head: ::subxt::ext::sp_core::H256,
            }
            impl ::subxt::events::StaticEvent for DownwardMessagesProcessed {
                const PALLET: &'static str = "ParachainSystem";
                const EVENT: &'static str = "DownwardMessagesProcessed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " In case of a scheduled upgrade, this storage field contains the validation code to be applied."]
                #[doc = ""]
                #[doc = " As soon as the relay chain gives us the go-ahead signal, we will overwrite the [`:code`][well_known_keys::CODE]"]
                #[doc = " which will result the next block process with the new validation code. This concludes the upgrade process."]
                #[doc = ""]
                #[doc = " [well_known_keys::CODE]: sp_core::storage::well_known_keys::CODE"]
                pub fn pending_validation_code(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ParachainSystem",
                        "PendingValidationCode",
                        vec![],
                        [
                            162u8, 35u8, 108u8, 76u8, 160u8, 93u8, 215u8, 84u8, 20u8, 249u8, 57u8,
                            187u8, 88u8, 161u8, 15u8, 131u8, 213u8, 89u8, 140u8, 20u8, 227u8,
                            204u8, 79u8, 176u8, 114u8, 119u8, 8u8, 7u8, 64u8, 15u8, 90u8, 92u8,
                        ],
                    )
                }
                #[doc = " Validation code that is set by the parachain and is to be communicated to collator and"]
                #[doc = " consequently the relay-chain."]
                #[doc = ""]
                #[doc = " This will be cleared in `on_initialize` of each new block if no other pallet already set"]
                #[doc = " the value."]
                pub fn new_validation_code(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ParachainSystem",
                        "NewValidationCode",
                        vec![],
                        [
                            224u8, 174u8, 53u8, 106u8, 240u8, 49u8, 48u8, 79u8, 219u8, 74u8, 142u8,
                            166u8, 92u8, 204u8, 244u8, 200u8, 43u8, 169u8, 177u8, 207u8, 190u8,
                            106u8, 180u8, 65u8, 245u8, 131u8, 134u8, 4u8, 53u8, 45u8, 76u8, 3u8,
                        ],
                    )
                }
                #[doc = " The [`PersistedValidationData`] set for this block."]
                #[doc = " This value is expected to be set only once per block and it's never stored"]
                #[doc = " in the trie."]
                pub fn validation_data(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::polkadot_primitives::v2::PersistedValidationData<
                            ::subxt::ext::sp_core::H256,
                            ::core::primitive::u32,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ParachainSystem",
                        "ValidationData",
                        vec![],
                        [
                            112u8, 58u8, 240u8, 81u8, 219u8, 110u8, 244u8, 186u8, 251u8, 90u8,
                            195u8, 217u8, 229u8, 102u8, 233u8, 24u8, 109u8, 96u8, 219u8, 72u8,
                            139u8, 93u8, 58u8, 140u8, 40u8, 110u8, 167u8, 98u8, 199u8, 12u8, 138u8,
                            131u8,
                        ],
                    )
                }
                #[doc = " Were the validation data set to notify the relay chain?"]
                pub fn did_set_validation_code(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ParachainSystem",
                        "DidSetValidationCode",
                        vec![],
                        [
                            89u8, 83u8, 74u8, 174u8, 234u8, 188u8, 149u8, 78u8, 140u8, 17u8, 92u8,
                            165u8, 243u8, 87u8, 59u8, 97u8, 135u8, 81u8, 192u8, 86u8, 193u8, 187u8,
                            113u8, 22u8, 108u8, 83u8, 242u8, 208u8, 174u8, 40u8, 49u8, 245u8,
                        ],
                    )
                }
                #[doc = " The relay chain block number associated with the last parachain block."]
                pub fn last_relay_chain_block_number(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ParachainSystem",
                        "LastRelayChainBlockNumber",
                        vec![],
                        [
                            68u8, 121u8, 6u8, 159u8, 181u8, 94u8, 151u8, 215u8, 225u8, 244u8, 4u8,
                            158u8, 216u8, 85u8, 55u8, 228u8, 197u8, 35u8, 200u8, 33u8, 29u8, 182u8,
                            17u8, 83u8, 59u8, 63u8, 25u8, 180u8, 132u8, 23u8, 97u8, 252u8,
                        ],
                    )
                }
                #[doc = " An option which indicates if the relay-chain restricts signalling a validation code upgrade."]
                #[doc = " In other words, if this is `Some` and [`NewValidationCode`] is `Some` then the produced"]
                #[doc = " candidate will be invalid."]
                #[doc = ""]
                #[doc = " This storage item is a mirror of the corresponding value for the current parachain from the"]
                #[doc = " relay-chain. This value is ephemeral which means it doesn't hit the storage. This value is"]
                #[doc = " set after the inherent."]
                pub fn upgrade_restriction_signal(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::core::option::Option<
                            runtime_types::polkadot_primitives::v2::UpgradeRestriction,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ParachainSystem",
                        "UpgradeRestrictionSignal",
                        vec![],
                        [
                            61u8, 3u8, 26u8, 6u8, 88u8, 114u8, 109u8, 63u8, 7u8, 115u8, 245u8,
                            198u8, 73u8, 234u8, 28u8, 228u8, 126u8, 27u8, 151u8, 18u8, 133u8, 54u8,
                            144u8, 149u8, 246u8, 43u8, 83u8, 47u8, 77u8, 238u8, 10u8, 196u8,
                        ],
                    )
                }
                #[doc = " The state proof for the last relay parent block."]
                #[doc = ""]
                #[doc = " This field is meant to be updated each block with the validation data inherent. Therefore,"]
                #[doc = " before processing of the inherent, e.g. in `on_initialize` this data may be stale."]
                #[doc = ""]
                #[doc = " This data is also absent from the genesis."]
                pub fn relay_state_proof(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::sp_trie::storage_proof::StorageProof,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ParachainSystem",
                        "RelayStateProof",
                        vec![],
                        [
                            35u8, 124u8, 167u8, 221u8, 162u8, 145u8, 158u8, 186u8, 57u8, 154u8,
                            225u8, 6u8, 176u8, 13u8, 178u8, 195u8, 209u8, 122u8, 221u8, 26u8,
                            155u8, 126u8, 153u8, 246u8, 101u8, 221u8, 61u8, 145u8, 211u8, 236u8,
                            48u8, 130u8,
                        ],
                    )
                }
                #[doc = " The snapshot of some state related to messaging relevant to the current parachain as per"]
                #[doc = " the relay parent."]
                #[doc = ""]
                #[doc = " This field is meant to be updated each block with the validation data inherent. Therefore,"]
                #[doc = " before processing of the inherent, e.g. in `on_initialize` this data may be stale."]
                #[doc = ""]
                #[doc = " This data is also absent from the genesis."]                pub fn relevant_messaging_state (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: cumulus_pallet_parachain_system :: relay_state_snapshot :: MessagingStateSnapshot > , :: subxt :: storage :: address :: Yes , () , () >{
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ParachainSystem",
                        "RelevantMessagingState",
                        vec![],
                        [
                            68u8, 241u8, 114u8, 83u8, 200u8, 99u8, 8u8, 244u8, 110u8, 134u8, 106u8,
                            153u8, 17u8, 90u8, 184u8, 157u8, 100u8, 140u8, 157u8, 83u8, 25u8,
                            166u8, 173u8, 31u8, 221u8, 24u8, 236u8, 85u8, 176u8, 223u8, 237u8,
                            65u8,
                        ],
                    )
                }
                #[doc = " The parachain host configuration that was obtained from the relay parent."]
                #[doc = ""]
                #[doc = " This field is meant to be updated each block with the validation data inherent. Therefore,"]
                #[doc = " before processing of the inherent, e.g. in `on_initialize` this data may be stale."]
                #[doc = ""]
                #[doc = " This data is also absent from the genesis."]
                pub fn host_configuration(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::polkadot_primitives::v2::AbridgedHostConfiguration,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ParachainSystem",
                        "HostConfiguration",
                        vec![],
                        [
                            104u8, 200u8, 30u8, 202u8, 119u8, 204u8, 233u8, 20u8, 67u8, 199u8,
                            47u8, 166u8, 254u8, 152u8, 10u8, 187u8, 240u8, 255u8, 148u8, 201u8,
                            134u8, 41u8, 130u8, 201u8, 112u8, 65u8, 68u8, 103u8, 56u8, 123u8,
                            178u8, 113u8,
                        ],
                    )
                }
                #[doc = " The last downward message queue chain head we have observed."]
                #[doc = ""]
                #[doc = " This value is loaded before and saved after processing inbound downward messages carried"]
                #[doc = " by the system inherent."]
                pub fn last_dmq_mqc_head(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::cumulus_primitives_parachain_inherent::MessageQueueChain,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ParachainSystem",
                        "LastDmqMqcHead",
                        vec![],
                        [
                            176u8, 255u8, 246u8, 125u8, 36u8, 120u8, 24u8, 44u8, 26u8, 64u8, 236u8,
                            210u8, 189u8, 237u8, 50u8, 78u8, 45u8, 139u8, 58u8, 141u8, 112u8,
                            253u8, 178u8, 198u8, 87u8, 71u8, 77u8, 248u8, 21u8, 145u8, 187u8, 52u8,
                        ],
                    )
                }
                #[doc = " The message queue chain heads we have observed per each channel incoming channel."]
                #[doc = ""]
                #[doc = " This value is loaded before and saved after processing inbound downward messages carried"]
                #[doc = " by the system inherent."]
                pub fn last_hrmp_mqc_heads(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::subxt::utils::KeyedVec<
                            runtime_types::polkadot_parachain::primitives::Id,
                            runtime_types::cumulus_primitives_parachain_inherent::MessageQueueChain,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ParachainSystem",
                        "LastHrmpMqcHeads",
                        vec![],
                        [
                            55u8, 179u8, 35u8, 16u8, 173u8, 0u8, 122u8, 179u8, 236u8, 98u8, 9u8,
                            112u8, 11u8, 219u8, 241u8, 89u8, 131u8, 198u8, 64u8, 139u8, 103u8,
                            158u8, 77u8, 107u8, 83u8, 236u8, 255u8, 208u8, 47u8, 61u8, 219u8,
                            240u8,
                        ],
                    )
                }
                #[doc = " Number of downward messages processed in a block."]
                #[doc = ""]
                #[doc = " This will be cleared in `on_initialize` of each new block."]
                pub fn processed_downward_messages(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ParachainSystem",
                        "ProcessedDownwardMessages",
                        vec![],
                        [
                            48u8, 177u8, 84u8, 228u8, 101u8, 235u8, 181u8, 27u8, 66u8, 55u8, 50u8,
                            146u8, 245u8, 223u8, 77u8, 132u8, 178u8, 80u8, 74u8, 90u8, 166u8, 81u8,
                            109u8, 25u8, 91u8, 69u8, 5u8, 69u8, 123u8, 197u8, 160u8, 146u8,
                        ],
                    )
                }
                #[doc = " HRMP watermark that was set in a block."]
                #[doc = ""]
                #[doc = " This will be cleared in `on_initialize` of each new block."]
                pub fn hrmp_watermark(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ParachainSystem",
                        "HrmpWatermark",
                        vec![],
                        [
                            189u8, 59u8, 183u8, 195u8, 69u8, 185u8, 241u8, 226u8, 62u8, 204u8,
                            230u8, 77u8, 102u8, 75u8, 86u8, 157u8, 249u8, 140u8, 219u8, 72u8, 94u8,
                            64u8, 176u8, 72u8, 34u8, 205u8, 114u8, 103u8, 231u8, 233u8, 206u8,
                            111u8,
                        ],
                    )
                }
                #[doc = " HRMP messages that were sent in a block."]
                #[doc = ""]
                #[doc = " This will be cleared in `on_initialize` of each new block."]
                pub fn hrmp_outbound_messages(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<
                            runtime_types::polkadot_core_primitives::OutboundHrmpMessage<
                                runtime_types::polkadot_parachain::primitives::Id,
                            >,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ParachainSystem",
                        "HrmpOutboundMessages",
                        vec![],
                        [
                            74u8, 86u8, 173u8, 248u8, 90u8, 230u8, 71u8, 225u8, 127u8, 164u8,
                            221u8, 62u8, 146u8, 13u8, 73u8, 9u8, 98u8, 168u8, 6u8, 14u8, 97u8,
                            166u8, 45u8, 70u8, 62u8, 210u8, 9u8, 32u8, 83u8, 18u8, 4u8, 201u8,
                        ],
                    )
                }
                #[doc = " Upward messages that were sent in a block."]
                #[doc = ""]
                #[doc = " This will be cleared in `on_initialize` of each new block."]
                pub fn upward_messages(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ParachainSystem",
                        "UpwardMessages",
                        vec![],
                        [
                            129u8, 208u8, 187u8, 36u8, 48u8, 108u8, 135u8, 56u8, 204u8, 60u8,
                            100u8, 158u8, 113u8, 238u8, 46u8, 92u8, 228u8, 41u8, 178u8, 177u8,
                            208u8, 195u8, 148u8, 149u8, 127u8, 21u8, 93u8, 92u8, 29u8, 115u8, 10u8,
                            248u8,
                        ],
                    )
                }
                #[doc = " Upward messages that are still pending and not yet send to the relay chain."]
                pub fn pending_upward_messages(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ParachainSystem",
                        "PendingUpwardMessages",
                        vec![],
                        [
                            223u8, 46u8, 224u8, 227u8, 222u8, 119u8, 225u8, 244u8, 59u8, 87u8,
                            127u8, 19u8, 217u8, 237u8, 103u8, 61u8, 6u8, 210u8, 107u8, 201u8,
                            117u8, 25u8, 85u8, 248u8, 36u8, 231u8, 28u8, 202u8, 41u8, 140u8, 208u8,
                            254u8,
                        ],
                    )
                }
                #[doc = " The number of HRMP messages we observed in `on_initialize` and thus used that number for"]
                #[doc = " announcing the weight of `on_initialize` and `on_finalize`."]
                pub fn announced_hrmp_messages_per_candidate(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ParachainSystem",
                        "AnnouncedHrmpMessagesPerCandidate",
                        vec![],
                        [
                            132u8, 61u8, 162u8, 129u8, 251u8, 243u8, 20u8, 144u8, 162u8, 73u8,
                            237u8, 51u8, 248u8, 41u8, 127u8, 171u8, 180u8, 79u8, 137u8, 23u8, 66u8,
                            134u8, 106u8, 222u8, 182u8, 154u8, 0u8, 145u8, 184u8, 156u8, 36u8,
                            97u8,
                        ],
                    )
                }
                #[doc = " The weight we reserve at the beginning of the block for processing XCMP messages. This"]
                #[doc = " overrides the amount set in the Config trait."]
                pub fn reserved_xcmp_weight_override(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ParachainSystem",
                        "ReservedXcmpWeightOverride",
                        vec![],
                        [
                            250u8, 177u8, 18u8, 183u8, 23u8, 84u8, 14u8, 178u8, 92u8, 60u8, 210u8,
                            155u8, 63u8, 58u8, 105u8, 196u8, 184u8, 235u8, 145u8, 11u8, 215u8,
                            121u8, 60u8, 140u8, 14u8, 50u8, 185u8, 101u8, 210u8, 230u8, 180u8,
                            250u8,
                        ],
                    )
                }
                #[doc = " The weight we reserve at the beginning of the block for processing DMP messages. This"]
                #[doc = " overrides the amount set in the Config trait."]
                pub fn reserved_dmp_weight_override(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ParachainSystem",
                        "ReservedDmpWeightOverride",
                        vec![],
                        [
                            20u8, 145u8, 152u8, 245u8, 73u8, 101u8, 125u8, 190u8, 151u8, 180u8,
                            22u8, 157u8, 58u8, 115u8, 165u8, 167u8, 117u8, 166u8, 201u8, 10u8,
                            206u8, 255u8, 206u8, 40u8, 40u8, 63u8, 228u8, 53u8, 58u8, 47u8, 121u8,
                            76u8,
                        ],
                    )
                }
                #[doc = " The next authorized upgrade, if there is one."]
                pub fn authorized_upgrade(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::H256>,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ParachainSystem",
                        "AuthorizedUpgrade",
                        vec![],
                        [
                            136u8, 238u8, 241u8, 144u8, 252u8, 61u8, 101u8, 171u8, 234u8, 160u8,
                            145u8, 210u8, 69u8, 29u8, 204u8, 166u8, 250u8, 101u8, 254u8, 32u8,
                            96u8, 197u8, 222u8, 212u8, 50u8, 189u8, 25u8, 7u8, 48u8, 183u8, 234u8,
                            95u8,
                        ],
                    )
                }
                #[doc = " A custom head data that should be returned as result of `validate_block`."]
                #[doc = ""]
                #[doc = " See [`Pallet::set_custom_validation_head_data`] for more information."]
                pub fn custom_validation_head_data(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ParachainSystem",
                        "CustomValidationHeadData",
                        vec![],
                        [
                            189u8, 150u8, 234u8, 128u8, 111u8, 27u8, 173u8, 92u8, 109u8, 4u8, 98u8,
                            103u8, 158u8, 19u8, 16u8, 5u8, 107u8, 135u8, 126u8, 170u8, 62u8, 64u8,
                            149u8, 80u8, 33u8, 17u8, 83u8, 22u8, 176u8, 118u8, 26u8, 223u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod randomness_collective_flip {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Series of block headers from the last 81 blocks that acts as random seed material. This"]
                #[doc = " is arranged as a ring buffer with `block_number % 81` being the index into the `Vec` of"]
                #[doc = " the oldest hash."]
                pub fn random_material(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<
                            ::subxt::ext::sp_core::H256,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "RandomnessCollectiveFlip",
                        "RandomMaterial",
                        vec![],
                        [
                            152u8, 126u8, 73u8, 88u8, 54u8, 147u8, 6u8, 19u8, 214u8, 40u8, 159u8,
                            30u8, 236u8, 61u8, 240u8, 65u8, 178u8, 94u8, 146u8, 152u8, 135u8,
                            252u8, 160u8, 86u8, 123u8, 114u8, 251u8, 140u8, 98u8, 143u8, 217u8,
                            242u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod timestamp {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Set {
                #[codec(compact)]
                pub now: ::core::primitive::u64,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Set the current time."]
                #[doc = ""]
                #[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
                #[doc = "phase, if this call hasn't been invoked by that time."]
                #[doc = ""]
                #[doc = "The timestamp should be greater than the previous one by the amount specified by"]
                #[doc = "`MinimumPeriod`."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be `Inherent`."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
                #[doc = "- 1 storage read and 1 storage mutation (codec `O(1)`). (because of `DidUpdate::take` in"]
                #[doc = "  `on_finalize`)"]
                #[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
                #[doc = "# </weight>"]
                pub fn set(
                    &self,
                    now: ::core::primitive::u64,
                ) -> ::subxt::tx::StaticTxPayload<Set> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Timestamp",
                        "set",
                        Set { now },
                        [
                            6u8, 97u8, 172u8, 236u8, 118u8, 238u8, 228u8, 114u8, 15u8, 115u8,
                            102u8, 85u8, 66u8, 151u8, 16u8, 33u8, 187u8, 17u8, 166u8, 88u8, 127u8,
                            214u8, 182u8, 51u8, 168u8, 88u8, 43u8, 101u8, 185u8, 8u8, 1u8, 28u8,
                        ],
                    )
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Current time for the current block."]
                pub fn now(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Timestamp",
                        "Now",
                        vec![],
                        [
                            148u8, 53u8, 50u8, 54u8, 13u8, 161u8, 57u8, 150u8, 16u8, 83u8, 144u8,
                            221u8, 59u8, 75u8, 158u8, 130u8, 39u8, 123u8, 106u8, 134u8, 202u8,
                            185u8, 83u8, 85u8, 60u8, 41u8, 120u8, 96u8, 210u8, 34u8, 2u8, 250u8,
                        ],
                    )
                }
                #[doc = " Did the timestamp get updated in this block?"]
                pub fn did_update(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Timestamp",
                        "DidUpdate",
                        vec![],
                        [
                            70u8, 13u8, 92u8, 186u8, 80u8, 151u8, 167u8, 90u8, 158u8, 232u8, 175u8,
                            13u8, 103u8, 135u8, 2u8, 78u8, 16u8, 6u8, 39u8, 158u8, 167u8, 85u8,
                            27u8, 47u8, 122u8, 73u8, 127u8, 26u8, 35u8, 168u8, 72u8, 204u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The minimum period between blocks. Beware that this is different to the *expected*"]
                #[doc = " period that the block production apparatus provides. Your chosen consensus system will"]
                #[doc = " generally work with this to determine a sensible block time. e.g. For Aura, it will be"]
                #[doc = " double this period on default settings."]
                pub fn minimum_period(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Timestamp",
                        "MinimumPeriod",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod parachain_info {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                pub fn parachain_id(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::polkadot_parachain::primitives::Id,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ParachainInfo",
                        "ParachainId",
                        vec![],
                        [
                            151u8, 191u8, 241u8, 118u8, 192u8, 47u8, 166u8, 151u8, 217u8, 240u8,
                            165u8, 232u8, 51u8, 113u8, 243u8, 1u8, 89u8, 240u8, 11u8, 1u8, 77u8,
                            104u8, 12u8, 56u8, 17u8, 135u8, 214u8, 19u8, 114u8, 135u8, 66u8, 76u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod balances {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Transfer {
                pub dest: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SetBalance {
                pub who: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                #[codec(compact)]
                pub new_free: ::core::primitive::u128,
                #[codec(compact)]
                pub new_reserved: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ForceTransfer {
                pub source: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                pub dest: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct TransferKeepAlive {
                pub dest: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct TransferAll {
                pub dest: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                pub keep_alive: ::core::primitive::bool,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ForceUnreserve {
                pub who: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
                pub amount: ::core::primitive::u128,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Transfer some liquid free balance to another account."]
                #[doc = ""]
                #[doc = "`transfer` will set the `FreeBalance` of the sender and receiver."]
                #[doc = "If the sender's account is below the existential deposit as a result"]
                #[doc = "of the transfer, the account will be reaped."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- Dependent on arguments but not critical, given proper implementations for input config"]
                #[doc = "  types. See related functions below."]
                #[doc = "- It contains a limited number of reads and writes internally and no complex"]
                #[doc = "  computation."]
                #[doc = ""]
                #[doc = "Related functions:"]
                #[doc = ""]
                #[doc = "  - `ensure_can_withdraw` is always called internally but has a bounded complexity."]
                #[doc = "  - Transferring balances to accounts that did not exist before will cause"]
                #[doc = "    `T::OnNewAccount::on_new_account` to be called."]
                #[doc = "  - Removing enough funds from an account will trigger `T::DustRemoval::on_unbalanced`."]
                #[doc = "  - `transfer_keep_alive` works the same way as `transfer`, but has an additional check"]
                #[doc = "    that the transfer will not kill the origin account."]
                #[doc = "---------------------------------"]
                #[doc = "- Origin account is already in memory, so no DB operations for them."]
                #[doc = "# </weight>"]
                pub fn transfer(
                    &self,
                    dest: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<Transfer> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Balances",
                        "transfer",
                        Transfer { dest, value },
                        [
                            111u8, 222u8, 32u8, 56u8, 171u8, 77u8, 252u8, 29u8, 194u8, 155u8,
                            200u8, 192u8, 198u8, 81u8, 23u8, 115u8, 236u8, 91u8, 218u8, 114u8,
                            107u8, 141u8, 138u8, 100u8, 237u8, 21u8, 58u8, 172u8, 3u8, 20u8, 216u8,
                            38u8,
                        ],
                    )
                }
                #[doc = "Set the balances of a given account."]
                #[doc = ""]
                #[doc = "This will alter `FreeBalance` and `ReservedBalance` in storage. it will"]
                #[doc = "also alter the total issuance of the system (`TotalIssuance`) appropriately."]
                #[doc = "If the new free or reserved balance is below the existential deposit,"]
                #[doc = "it will reset the account nonce (`frame_system::AccountNonce`)."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call is `root`."]
                pub fn set_balance(
                    &self,
                    who: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    new_free: ::core::primitive::u128,
                    new_reserved: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<SetBalance> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Balances",
                        "set_balance",
                        SetBalance {
                            who,
                            new_free,
                            new_reserved,
                        },
                        [
                            234u8, 215u8, 97u8, 98u8, 243u8, 199u8, 57u8, 76u8, 59u8, 161u8, 118u8,
                            207u8, 34u8, 197u8, 198u8, 61u8, 231u8, 210u8, 169u8, 235u8, 150u8,
                            137u8, 173u8, 49u8, 28u8, 77u8, 84u8, 149u8, 143u8, 210u8, 139u8,
                            193u8,
                        ],
                    )
                }
                #[doc = "Exactly as `transfer`, except the origin must be root and the source account may be"]
                #[doc = "specified."]
                #[doc = "# <weight>"]
                #[doc = "- Same as transfer, but additional read and write because the source account is not"]
                #[doc = "  assumed to be in the overlay."]
                #[doc = "# </weight>"]
                pub fn force_transfer(
                    &self,
                    source: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    dest: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<ForceTransfer> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Balances",
                        "force_transfer",
                        ForceTransfer {
                            source,
                            dest,
                            value,
                        },
                        [
                            79u8, 174u8, 212u8, 108u8, 184u8, 33u8, 170u8, 29u8, 232u8, 254u8,
                            195u8, 218u8, 221u8, 134u8, 57u8, 99u8, 6u8, 70u8, 181u8, 227u8, 56u8,
                            239u8, 243u8, 158u8, 157u8, 245u8, 36u8, 162u8, 11u8, 237u8, 147u8,
                            15u8,
                        ],
                    )
                }
                #[doc = "Same as the [`transfer`] call, but with a check that the transfer will not kill the"]
                #[doc = "origin account."]
                #[doc = ""]
                #[doc = "99% of the time you want [`transfer`] instead."]
                #[doc = ""]
                #[doc = "[`transfer`]: struct.Pallet.html#method.transfer"]
                pub fn transfer_keep_alive(
                    &self,
                    dest: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<TransferKeepAlive> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Balances",
                        "transfer_keep_alive",
                        TransferKeepAlive { dest, value },
                        [
                            112u8, 179u8, 75u8, 168u8, 193u8, 221u8, 9u8, 82u8, 190u8, 113u8,
                            253u8, 13u8, 130u8, 134u8, 170u8, 216u8, 136u8, 111u8, 242u8, 220u8,
                            202u8, 112u8, 47u8, 79u8, 73u8, 244u8, 226u8, 59u8, 240u8, 188u8,
                            210u8, 208u8,
                        ],
                    )
                }
                #[doc = "Transfer the entire transferable balance from the caller account."]
                #[doc = ""]
                #[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
                #[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
                #[doc = "transferred by this function. To ensure that this function results in a killed account,"]
                #[doc = "you might need to prepare the account by removing any reference counters, storage"]
                #[doc = "deposits, etc..."]
                #[doc = ""]
                #[doc = "The dispatch origin of this call must be Signed."]
                #[doc = ""]
                #[doc = "- `dest`: The recipient of the transfer."]
                #[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
                #[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
                #[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
                #[doc = "  keep the sender account alive (true). # <weight>"]
                #[doc = "- O(1). Just like transfer, but reading the user's transferable balance first."]
                #[doc = "  #</weight>"]
                pub fn transfer_all(
                    &self,
                    dest: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    keep_alive: ::core::primitive::bool,
                ) -> ::subxt::tx::StaticTxPayload<TransferAll> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Balances",
                        "transfer_all",
                        TransferAll { dest, keep_alive },
                        [
                            46u8, 129u8, 29u8, 177u8, 221u8, 107u8, 245u8, 69u8, 238u8, 126u8,
                            145u8, 26u8, 219u8, 208u8, 14u8, 80u8, 149u8, 1u8, 214u8, 63u8, 67u8,
                            201u8, 144u8, 45u8, 129u8, 145u8, 174u8, 71u8, 238u8, 113u8, 208u8,
                            34u8,
                        ],
                    )
                }
                #[doc = "Unreserve some balance from a user by force."]
                #[doc = ""]
                #[doc = "Can only be called by ROOT."]
                pub fn force_unreserve(
                    &self,
                    who: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<ForceUnreserve> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Balances",
                        "force_unreserve",
                        ForceUnreserve { who, amount },
                        [
                            160u8, 146u8, 137u8, 76u8, 157u8, 187u8, 66u8, 148u8, 207u8, 76u8,
                            32u8, 254u8, 82u8, 215u8, 35u8, 161u8, 213u8, 52u8, 32u8, 98u8, 102u8,
                            106u8, 234u8, 123u8, 6u8, 175u8, 184u8, 188u8, 174u8, 106u8, 176u8,
                            78u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_balances::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "An account was created with some free balance."]
            pub struct Endowed {
                pub account: ::subxt::ext::sp_core::crypto::AccountId32,
                pub free_balance: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Endowed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Endowed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
            #[doc = "resulting in an outright loss."]
            pub struct DustLost {
                pub account: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for DustLost {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "DustLost";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Transfer succeeded."]
            pub struct Transfer {
                pub from: ::subxt::ext::sp_core::crypto::AccountId32,
                pub to: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Transfer {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Transfer";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A balance was set by root."]
            pub struct BalanceSet {
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
                pub free: ::core::primitive::u128,
                pub reserved: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for BalanceSet {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "BalanceSet";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Some balance was reserved (moved from free to reserved)."]
            pub struct Reserved {
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Reserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Reserved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Some balance was unreserved (moved from reserved to free)."]
            pub struct Unreserved {
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Unreserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Unreserved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Some balance was moved from the reserve of the first account to the second account."]
            #[doc = "Final argument indicates the destination balance type."]
            pub struct ReserveRepatriated {
                pub from: ::subxt::ext::sp_core::crypto::AccountId32,
                pub to: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
                pub destination_status:
                    runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
            }
            impl ::subxt::events::StaticEvent for ReserveRepatriated {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "ReserveRepatriated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Some amount was deposited (e.g. for transaction fees)."]
            pub struct Deposit {
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Deposit {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Deposit";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
            pub struct Withdraw {
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Withdraw {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Withdraw";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
            pub struct Slashed {
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Slashed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Slashed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The total units issued in the system."]
                pub fn total_issuance(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Balances",
                        "TotalIssuance",
                        vec![],
                        [
                            1u8, 206u8, 252u8, 237u8, 6u8, 30u8, 20u8, 232u8, 164u8, 115u8, 51u8,
                            156u8, 156u8, 206u8, 241u8, 187u8, 44u8, 84u8, 25u8, 164u8, 235u8,
                            20u8, 86u8, 242u8, 124u8, 23u8, 28u8, 140u8, 26u8, 73u8, 231u8, 51u8,
                        ],
                    )
                }
                #[doc = " The Balances pallet example of storing the balance of an account."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " You can also store the balance of an account in the `System` pallet."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "   type AccountStore = System"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
                #[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
                #[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
                #[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
                pub fn account(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Balances",
                        "Account",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            246u8, 154u8, 253u8, 71u8, 192u8, 192u8, 192u8, 236u8, 128u8, 80u8,
                            40u8, 252u8, 201u8, 43u8, 3u8, 131u8, 19u8, 49u8, 141u8, 240u8, 172u8,
                            217u8, 215u8, 109u8, 87u8, 135u8, 248u8, 57u8, 98u8, 185u8, 22u8, 4u8,
                        ],
                    )
                }
                #[doc = " The Balances pallet example of storing the balance of an account."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " You can also store the balance of an account in the `System` pallet."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "   type AccountStore = System"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
                #[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
                #[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
                #[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
                pub fn account_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Balances",
                        "Account",
                        Vec::new(),
                        [
                            246u8, 154u8, 253u8, 71u8, 192u8, 192u8, 192u8, 236u8, 128u8, 80u8,
                            40u8, 252u8, 201u8, 43u8, 3u8, 131u8, 19u8, 49u8, 141u8, 240u8, 172u8,
                            217u8, 215u8, 109u8, 87u8, 135u8, 248u8, 57u8, 98u8, 185u8, 22u8, 4u8,
                        ],
                    )
                }
                #[doc = " Any liquidity locks on some account balances."]
                #[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
                pub fn locks(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::sp_runtime::bounded::weak_bounded_vec::WeakBoundedVec<
                            runtime_types::pallet_balances::BalanceLock<::core::primitive::u128>,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Balances",
                        "Locks",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            216u8, 253u8, 87u8, 73u8, 24u8, 218u8, 35u8, 0u8, 244u8, 134u8, 195u8,
                            58u8, 255u8, 64u8, 153u8, 212u8, 210u8, 232u8, 4u8, 122u8, 90u8, 212u8,
                            136u8, 14u8, 127u8, 232u8, 8u8, 192u8, 40u8, 233u8, 18u8, 250u8,
                        ],
                    )
                }
                #[doc = " Any liquidity locks on some account balances."]
                #[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
                pub fn locks_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::sp_runtime::bounded::weak_bounded_vec::WeakBoundedVec<
                            runtime_types::pallet_balances::BalanceLock<::core::primitive::u128>,
                        >,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Balances",
                        "Locks",
                        Vec::new(),
                        [
                            216u8, 253u8, 87u8, 73u8, 24u8, 218u8, 35u8, 0u8, 244u8, 134u8, 195u8,
                            58u8, 255u8, 64u8, 153u8, 212u8, 210u8, 232u8, 4u8, 122u8, 90u8, 212u8,
                            136u8, 14u8, 127u8, 232u8, 8u8, 192u8, 40u8, 233u8, 18u8, 250u8,
                        ],
                    )
                }
                #[doc = " Named reserves on some account balances."]
                pub fn reserves(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<
                            runtime_types::pallet_balances::ReserveData<
                                [::core::primitive::u8; 8usize],
                                ::core::primitive::u128,
                            >,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Balances",
                        "Reserves",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            17u8, 32u8, 191u8, 46u8, 76u8, 220u8, 101u8, 100u8, 42u8, 250u8, 128u8,
                            167u8, 117u8, 44u8, 85u8, 96u8, 105u8, 216u8, 16u8, 147u8, 74u8, 55u8,
                            183u8, 94u8, 160u8, 177u8, 26u8, 187u8, 71u8, 197u8, 187u8, 163u8,
                        ],
                    )
                }
                #[doc = " Named reserves on some account balances."]
                pub fn reserves_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<
                            runtime_types::pallet_balances::ReserveData<
                                [::core::primitive::u8; 8usize],
                                ::core::primitive::u128,
                            >,
                        >,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Balances",
                        "Reserves",
                        Vec::new(),
                        [
                            17u8, 32u8, 191u8, 46u8, 76u8, 220u8, 101u8, 100u8, 42u8, 250u8, 128u8,
                            167u8, 117u8, 44u8, 85u8, 96u8, 105u8, 216u8, 16u8, 147u8, 74u8, 55u8,
                            183u8, 94u8, 160u8, 177u8, 26u8, 187u8, 71u8, 197u8, 187u8, 163u8,
                        ],
                    )
                }
                #[doc = " Storage version of the pallet."]
                #[doc = ""]
                #[doc = " This is set to v2.0.0 for new networks."]
                pub fn storage_version(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<runtime_types::pallet_balances::Releases>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Balances",
                        "StorageVersion",
                        vec![],
                        [
                            135u8, 96u8, 28u8, 234u8, 124u8, 212u8, 56u8, 140u8, 40u8, 101u8,
                            235u8, 128u8, 136u8, 221u8, 182u8, 81u8, 17u8, 9u8, 184u8, 228u8,
                            174u8, 165u8, 200u8, 162u8, 214u8, 178u8, 227u8, 72u8, 34u8, 5u8,
                            173u8, 96u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The minimum amount required to keep an account open."]
                pub fn existential_deposit(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Balances",
                        "ExistentialDeposit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The maximum number of locks that should exist on an account."]
                #[doc = " Not strictly enforced, but used for weight estimation."]
                pub fn max_locks(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Balances",
                        "MaxLocks",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum number of named reserves that can exist on an account."]
                pub fn max_reserves(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Balances",
                        "MaxReserves",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod transaction_payment {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_transaction_payment::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,"]
            #[doc = "has been paid by `who`."]
            pub struct TransactionFeePaid {
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
                pub actual_fee: ::core::primitive::u128,
                pub tip: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for TransactionFeePaid {
                const PALLET: &'static str = "TransactionPayment";
                const EVENT: &'static str = "TransactionFeePaid";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                pub fn next_fee_multiplier(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::sp_arithmetic::fixed_point::FixedU128,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TransactionPayment",
                        "NextFeeMultiplier",
                        vec![],
                        [
                            210u8, 0u8, 206u8, 165u8, 183u8, 10u8, 206u8, 52u8, 14u8, 90u8, 218u8,
                            197u8, 189u8, 125u8, 113u8, 216u8, 52u8, 161u8, 45u8, 24u8, 245u8,
                            237u8, 121u8, 41u8, 106u8, 29u8, 45u8, 129u8, 250u8, 203u8, 206u8,
                            180u8,
                        ],
                    )
                }
                pub fn storage_version(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_transaction_payment::Releases,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TransactionPayment",
                        "StorageVersion",
                        vec![],
                        [
                            219u8, 243u8, 82u8, 176u8, 65u8, 5u8, 132u8, 114u8, 8u8, 82u8, 176u8,
                            200u8, 97u8, 150u8, 177u8, 164u8, 166u8, 11u8, 34u8, 12u8, 12u8, 198u8,
                            58u8, 191u8, 186u8, 221u8, 221u8, 119u8, 181u8, 253u8, 154u8, 228u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " A fee mulitplier for `Operational` extrinsics to compute \"virtual tip\" to boost their"]
                #[doc = " `priority`"]
                #[doc = ""]
                #[doc = " This value is multipled by the `final_fee` to obtain a \"virtual tip\" that is later"]
                #[doc = " added to a tip component in regular `priority` calculations."]
                #[doc = " It means that a `Normal` transaction can front-run a similarly-sized `Operational`"]
                #[doc = " extrinsic (with no tip), by including a tip value greater than the virtual tip."]
                #[doc = ""]
                #[doc = " ```rust,ignore"]
                #[doc = " // For `Normal`"]
                #[doc = " let priority = priority_calc(tip);"]
                #[doc = ""]
                #[doc = " // For `Operational`"]
                #[doc = " let virtual_tip = (inclusion_fee + tip) * OperationalFeeMultiplier;"]
                #[doc = " let priority = priority_calc(tip + virtual_tip);"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " Note that since we use `final_fee` the multiplier applies also to the regular `tip`"]
                #[doc = " sent with the transaction. So, not only does the transaction get a priority bump based"]
                #[doc = " on the `inclusion_fee`, but we also amplify the impact of tips applied to `Operational`"]
                #[doc = " transactions."]
                pub fn operational_fee_multiplier(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u8>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "TransactionPayment",
                        "OperationalFeeMultiplier",
                        [
                            141u8, 130u8, 11u8, 35u8, 226u8, 114u8, 92u8, 179u8, 168u8, 110u8,
                            28u8, 91u8, 221u8, 64u8, 4u8, 148u8, 201u8, 193u8, 185u8, 66u8, 226u8,
                            114u8, 97u8, 79u8, 62u8, 212u8, 202u8, 114u8, 237u8, 228u8, 183u8,
                            165u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod asset_tx_payment {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_asset_tx_payment::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,"]
            #[doc = "has been paid by `who` in an asset `asset_id`."]
            pub struct AssetTxFeePaid {
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
                pub actual_fee: ::core::primitive::u128,
                pub tip: ::core::primitive::u128,
                pub asset_id: ::core::option::Option<
                    runtime_types::encointer_primitives::communities::CommunityIdentifier,
                >,
            }
            impl ::subxt::events::StaticEvent for AssetTxFeePaid {
                const PALLET: &'static str = "AssetTxPayment";
                const EVENT: &'static str = "AssetTxFeePaid";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {}
        }
    }
    pub mod aura {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The current authority set."]
                pub fn authorities(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<
                            runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Aura",
                        "Authorities",
                        vec![],
                        [
                            199u8, 89u8, 94u8, 48u8, 249u8, 35u8, 105u8, 90u8, 15u8, 86u8, 218u8,
                            85u8, 22u8, 236u8, 228u8, 36u8, 137u8, 64u8, 236u8, 171u8, 242u8,
                            217u8, 91u8, 240u8, 205u8, 205u8, 226u8, 16u8, 147u8, 235u8, 181u8,
                            41u8,
                        ],
                    )
                }
                #[doc = " The current slot of this block."]
                #[doc = ""]
                #[doc = " This will be set in `on_initialize`."]
                pub fn current_slot(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<runtime_types::sp_consensus_slots::Slot>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Aura",
                        "CurrentSlot",
                        vec![],
                        [
                            139u8, 237u8, 185u8, 137u8, 251u8, 179u8, 69u8, 167u8, 133u8, 168u8,
                            204u8, 64u8, 178u8, 123u8, 92u8, 250u8, 119u8, 190u8, 208u8, 178u8,
                            208u8, 176u8, 124u8, 187u8, 74u8, 165u8, 33u8, 78u8, 161u8, 206u8, 8u8,
                            108u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod aura_ext {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Serves as cache for the authorities."]
                #[doc = ""]
                #[doc = " The authorities in AuRa are overwritten in `on_initialize` when we switch to a new session,"]
                #[doc = " but we require the old authorities to verify the seal when validating a PoV. This will always"]
                #[doc = " be updated to the latest AuRa authorities in `on_finalize`."]
                pub fn authorities(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<
                            runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "AuraExt",
                        "Authorities",
                        vec![],
                        [
                            199u8, 89u8, 94u8, 48u8, 249u8, 35u8, 105u8, 90u8, 15u8, 86u8, 218u8,
                            85u8, 22u8, 236u8, 228u8, 36u8, 137u8, 64u8, 236u8, 171u8, 242u8,
                            217u8, 91u8, 240u8, 205u8, 205u8, 226u8, 16u8, 147u8, 235u8, 181u8,
                            41u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod xcmp_queue {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ServiceOverweight {
                pub index: ::core::primitive::u64,
                pub weight_limit: ::core::primitive::u64,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SuspendXcmExecution;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ResumeXcmExecution;
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct UpdateSuspendThreshold {
                pub new: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct UpdateDropThreshold {
                pub new: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct UpdateResumeThreshold {
                pub new: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct UpdateThresholdWeight {
                pub new: ::core::primitive::u64,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct UpdateWeightRestrictDecay {
                pub new: ::core::primitive::u64,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct UpdateXcmpMaxIndividualWeight {
                pub new: ::core::primitive::u64,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Services a single overweight XCM."]
                #[doc = ""]
                #[doc = "- `origin`: Must pass `ExecuteOverweightOrigin`."]
                #[doc = "- `index`: The index of the overweight XCM to service"]
                #[doc = "- `weight_limit`: The amount of weight that XCM execution may take."]
                #[doc = ""]
                #[doc = "Errors:"]
                #[doc = "- `BadOverweightIndex`: XCM under `index` is not found in the `Overweight` storage map."]
                #[doc = "- `BadXcm`: XCM under `index` cannot be properly decoded into a valid XCM format."]
                #[doc = "- `WeightOverLimit`: XCM execution may use greater `weight_limit`."]
                #[doc = ""]
                #[doc = "Events:"]
                #[doc = "- `OverweightServiced`: On success."]
                pub fn service_overweight(
                    &self,
                    index: ::core::primitive::u64,
                    weight_limit: ::core::primitive::u64,
                ) -> ::subxt::tx::StaticTxPayload<ServiceOverweight> {
                    ::subxt::tx::StaticTxPayload::new(
                        "XcmpQueue",
                        "service_overweight",
                        ServiceOverweight {
                            index,
                            weight_limit,
                        },
                        [
                            225u8, 41u8, 132u8, 91u8, 28u8, 116u8, 89u8, 197u8, 194u8, 131u8, 28u8,
                            217u8, 102u8, 241u8, 122u8, 230u8, 242u8, 75u8, 83u8, 67u8, 104u8,
                            55u8, 133u8, 129u8, 91u8, 25u8, 185u8, 131u8, 22u8, 253u8, 84u8, 221u8,
                        ],
                    )
                }
                #[doc = "Suspends all XCM executions for the XCMP queue, regardless of the sender's origin."]
                #[doc = ""]
                #[doc = "- `origin`: Must pass `ControllerOrigin`."]
                pub fn suspend_xcm_execution(
                    &self,
                ) -> ::subxt::tx::StaticTxPayload<SuspendXcmExecution> {
                    ::subxt::tx::StaticTxPayload::new(
                        "XcmpQueue",
                        "suspend_xcm_execution",
                        SuspendXcmExecution {},
                        [
                            139u8, 76u8, 166u8, 86u8, 106u8, 144u8, 16u8, 47u8, 105u8, 185u8, 7u8,
                            7u8, 63u8, 14u8, 250u8, 236u8, 99u8, 121u8, 101u8, 143u8, 28u8, 175u8,
                            108u8, 197u8, 226u8, 43u8, 103u8, 92u8, 186u8, 12u8, 51u8, 153u8,
                        ],
                    )
                }
                #[doc = "Resumes all XCM executions for the XCMP queue."]
                #[doc = ""]
                #[doc = "Note that this function doesn't change the status of the in/out bound channels."]
                #[doc = ""]
                #[doc = "- `origin`: Must pass `ControllerOrigin`."]
                pub fn resume_xcm_execution(
                    &self,
                ) -> ::subxt::tx::StaticTxPayload<ResumeXcmExecution> {
                    ::subxt::tx::StaticTxPayload::new(
                        "XcmpQueue",
                        "resume_xcm_execution",
                        ResumeXcmExecution {},
                        [
                            67u8, 111u8, 47u8, 237u8, 79u8, 42u8, 90u8, 56u8, 245u8, 2u8, 20u8,
                            23u8, 33u8, 121u8, 135u8, 50u8, 204u8, 147u8, 195u8, 80u8, 177u8,
                            202u8, 8u8, 160u8, 164u8, 138u8, 64u8, 252u8, 178u8, 63u8, 102u8,
                            245u8,
                        ],
                    )
                }
                #[doc = "Overwrites the number of pages of messages which must be in the queue for the other side to be told to"]
                #[doc = "suspend their sending."]
                #[doc = ""]
                #[doc = "- `origin`: Must pass `Root`."]
                #[doc = "- `new`: Desired value for `QueueConfigData.suspend_value`"]
                pub fn update_suspend_threshold(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<UpdateSuspendThreshold> {
                    ::subxt::tx::StaticTxPayload::new(
                        "XcmpQueue",
                        "update_suspend_threshold",
                        UpdateSuspendThreshold { new },
                        [
                            155u8, 120u8, 9u8, 228u8, 110u8, 62u8, 233u8, 36u8, 57u8, 85u8, 19u8,
                            67u8, 246u8, 88u8, 81u8, 116u8, 243u8, 236u8, 174u8, 130u8, 8u8, 246u8,
                            254u8, 97u8, 155u8, 207u8, 123u8, 60u8, 164u8, 14u8, 196u8, 97u8,
                        ],
                    )
                }
                #[doc = "Overwrites the number of pages of messages which must be in the queue after which we drop any further"]
                #[doc = "messages from the channel."]
                #[doc = ""]
                #[doc = "- `origin`: Must pass `Root`."]
                #[doc = "- `new`: Desired value for `QueueConfigData.drop_threshold`"]
                pub fn update_drop_threshold(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<UpdateDropThreshold> {
                    ::subxt::tx::StaticTxPayload::new(
                        "XcmpQueue",
                        "update_drop_threshold",
                        UpdateDropThreshold { new },
                        [
                            146u8, 177u8, 164u8, 96u8, 247u8, 182u8, 229u8, 175u8, 194u8, 101u8,
                            186u8, 168u8, 94u8, 114u8, 172u8, 119u8, 35u8, 222u8, 175u8, 21u8,
                            67u8, 61u8, 216u8, 144u8, 194u8, 10u8, 181u8, 62u8, 166u8, 198u8,
                            138u8, 243u8,
                        ],
                    )
                }
                #[doc = "Overwrites the number of pages of messages which the queue must be reduced to before it signals that"]
                #[doc = "message sending may recommence after it has been suspended."]
                #[doc = ""]
                #[doc = "- `origin`: Must pass `Root`."]
                #[doc = "- `new`: Desired value for `QueueConfigData.resume_threshold`"]
                pub fn update_resume_threshold(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<UpdateResumeThreshold> {
                    ::subxt::tx::StaticTxPayload::new(
                        "XcmpQueue",
                        "update_resume_threshold",
                        UpdateResumeThreshold { new },
                        [
                            231u8, 128u8, 80u8, 179u8, 61u8, 50u8, 103u8, 209u8, 103u8, 55u8,
                            101u8, 113u8, 150u8, 10u8, 202u8, 7u8, 0u8, 77u8, 58u8, 4u8, 227u8,
                            17u8, 225u8, 112u8, 121u8, 203u8, 184u8, 113u8, 231u8, 156u8, 174u8,
                            154u8,
                        ],
                    )
                }
                #[doc = "Overwrites the amount of remaining weight under which we stop processing messages."]
                #[doc = ""]
                #[doc = "- `origin`: Must pass `Root`."]
                #[doc = "- `new`: Desired value for `QueueConfigData.threshold_weight`"]
                pub fn update_threshold_weight(
                    &self,
                    new: ::core::primitive::u64,
                ) -> ::subxt::tx::StaticTxPayload<UpdateThresholdWeight> {
                    ::subxt::tx::StaticTxPayload::new(
                        "XcmpQueue",
                        "update_threshold_weight",
                        UpdateThresholdWeight { new },
                        [
                            129u8, 208u8, 93u8, 179u8, 45u8, 236u8, 84u8, 209u8, 37u8, 226u8, 88u8,
                            123u8, 156u8, 101u8, 93u8, 84u8, 110u8, 61u8, 56u8, 45u8, 14u8, 120u8,
                            181u8, 71u8, 174u8, 104u8, 225u8, 36u8, 17u8, 74u8, 94u8, 59u8,
                        ],
                    )
                }
                #[doc = "Overwrites the speed to which the available weight approaches the maximum weight."]
                #[doc = "A lower number results in a faster progression. A value of 1 makes the entire weight available initially."]
                #[doc = ""]
                #[doc = "- `origin`: Must pass `Root`."]
                #[doc = "- `new`: Desired value for `QueueConfigData.weight_restrict_decay`."]
                pub fn update_weight_restrict_decay(
                    &self,
                    new: ::core::primitive::u64,
                ) -> ::subxt::tx::StaticTxPayload<UpdateWeightRestrictDecay> {
                    ::subxt::tx::StaticTxPayload::new(
                        "XcmpQueue",
                        "update_weight_restrict_decay",
                        UpdateWeightRestrictDecay { new },
                        [
                            73u8, 98u8, 189u8, 10u8, 137u8, 162u8, 71u8, 54u8, 24u8, 117u8, 15u8,
                            137u8, 251u8, 121u8, 86u8, 5u8, 123u8, 42u8, 151u8, 244u8, 200u8,
                            140u8, 104u8, 149u8, 101u8, 14u8, 58u8, 163u8, 208u8, 205u8, 177u8,
                            142u8,
                        ],
                    )
                }
                #[doc = "Overwrite the maximum amount of weight any individual message may consume."]
                #[doc = "Messages above this weight go into the overweight queue and may only be serviced explicitly."]
                #[doc = ""]
                #[doc = "- `origin`: Must pass `Root`."]
                #[doc = "- `new`: Desired value for `QueueConfigData.xcmp_max_individual_weight`."]
                pub fn update_xcmp_max_individual_weight(
                    &self,
                    new: ::core::primitive::u64,
                ) -> ::subxt::tx::StaticTxPayload<UpdateXcmpMaxIndividualWeight> {
                    ::subxt::tx::StaticTxPayload::new(
                        "XcmpQueue",
                        "update_xcmp_max_individual_weight",
                        UpdateXcmpMaxIndividualWeight { new },
                        [
                            52u8, 93u8, 25u8, 215u8, 36u8, 235u8, 88u8, 49u8, 142u8, 132u8, 57u8,
                            2u8, 204u8, 195u8, 166u8, 254u8, 235u8, 247u8, 142u8, 207u8, 224u8,
                            43u8, 7u8, 106u8, 142u8, 3u8, 188u8, 101u8, 9u8, 75u8, 57u8, 39u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::cumulus_pallet_xcmp_queue::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Some XCM was executed ok."]
            pub struct Success {
                pub message_hash: ::core::option::Option<::subxt::ext::sp_core::H256>,
                pub weight: ::core::primitive::u64,
            }
            impl ::subxt::events::StaticEvent for Success {
                const PALLET: &'static str = "XcmpQueue";
                const EVENT: &'static str = "Success";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Some XCM failed."]
            pub struct Fail {
                pub message_hash: ::core::option::Option<::subxt::ext::sp_core::H256>,
                pub error: runtime_types::xcm::v2::traits::Error,
                pub weight: ::core::primitive::u64,
            }
            impl ::subxt::events::StaticEvent for Fail {
                const PALLET: &'static str = "XcmpQueue";
                const EVENT: &'static str = "Fail";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Bad XCM version used."]
            pub struct BadVersion {
                pub message_hash: ::core::option::Option<::subxt::ext::sp_core::H256>,
            }
            impl ::subxt::events::StaticEvent for BadVersion {
                const PALLET: &'static str = "XcmpQueue";
                const EVENT: &'static str = "BadVersion";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Bad XCM format used."]
            pub struct BadFormat {
                pub message_hash: ::core::option::Option<::subxt::ext::sp_core::H256>,
            }
            impl ::subxt::events::StaticEvent for BadFormat {
                const PALLET: &'static str = "XcmpQueue";
                const EVENT: &'static str = "BadFormat";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "An upward message was sent to the relay chain."]
            pub struct UpwardMessageSent {
                pub message_hash: ::core::option::Option<::subxt::ext::sp_core::H256>,
            }
            impl ::subxt::events::StaticEvent for UpwardMessageSent {
                const PALLET: &'static str = "XcmpQueue";
                const EVENT: &'static str = "UpwardMessageSent";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "An HRMP message was sent to a sibling parachain."]
            pub struct XcmpMessageSent {
                pub message_hash: ::core::option::Option<::subxt::ext::sp_core::H256>,
            }
            impl ::subxt::events::StaticEvent for XcmpMessageSent {
                const PALLET: &'static str = "XcmpQueue";
                const EVENT: &'static str = "XcmpMessageSent";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "An XCM exceeded the individual message weight budget."]
            pub struct OverweightEnqueued {
                pub sender: runtime_types::polkadot_parachain::primitives::Id,
                pub sent_at: ::core::primitive::u32,
                pub index: ::core::primitive::u64,
                pub required: ::core::primitive::u64,
            }
            impl ::subxt::events::StaticEvent for OverweightEnqueued {
                const PALLET: &'static str = "XcmpQueue";
                const EVENT: &'static str = "OverweightEnqueued";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "An XCM from the overweight queue was executed with the given actual weight used."]
            pub struct OverweightServiced {
                pub index: ::core::primitive::u64,
                pub used: ::core::primitive::u64,
            }
            impl ::subxt::events::StaticEvent for OverweightServiced {
                const PALLET: &'static str = "XcmpQueue";
                const EVENT: &'static str = "OverweightServiced";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Status of the inbound XCMP channels."]
                pub fn inbound_xcmp_status(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<
                            runtime_types::cumulus_pallet_xcmp_queue::InboundChannelDetails,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "XcmpQueue",
                        "InboundXcmpStatus",
                        vec![],
                        [
                            183u8, 198u8, 237u8, 153u8, 132u8, 201u8, 87u8, 182u8, 121u8, 164u8,
                            129u8, 241u8, 58u8, 192u8, 115u8, 152u8, 7u8, 33u8, 95u8, 51u8, 2u8,
                            176u8, 144u8, 12u8, 125u8, 83u8, 92u8, 198u8, 211u8, 101u8, 28u8, 50u8,
                        ],
                    )
                }
                #[doc = " Inbound aggregate XCMP messages. It can only be one per ParaId/block."]
                pub fn inbound_xcmp_messages(
                    &self,
                    _0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "XcmpQueue",
                        "InboundXcmpMessages",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Twox64Concat,
                            ),
                        ],
                        [
                            157u8, 232u8, 222u8, 97u8, 218u8, 96u8, 96u8, 90u8, 216u8, 205u8, 39u8,
                            130u8, 109u8, 152u8, 127u8, 57u8, 54u8, 63u8, 104u8, 135u8, 33u8,
                            175u8, 197u8, 166u8, 238u8, 22u8, 137u8, 162u8, 226u8, 199u8, 87u8,
                            25u8,
                        ],
                    )
                }
                #[doc = " Inbound aggregate XCMP messages. It can only be one per ParaId/block."]
                pub fn inbound_xcmp_messages_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "XcmpQueue",
                        "InboundXcmpMessages",
                        Vec::new(),
                        [
                            157u8, 232u8, 222u8, 97u8, 218u8, 96u8, 96u8, 90u8, 216u8, 205u8, 39u8,
                            130u8, 109u8, 152u8, 127u8, 57u8, 54u8, 63u8, 104u8, 135u8, 33u8,
                            175u8, 197u8, 166u8, 238u8, 22u8, 137u8, 162u8, 226u8, 199u8, 87u8,
                            25u8,
                        ],
                    )
                }
                #[doc = " The non-empty XCMP channels in order of becoming non-empty, and the index of the first"]
                #[doc = " and last outbound message. If the two indices are equal, then it indicates an empty"]
                #[doc = " queue and there must be a non-`Ok` `OutboundStatus`. We assume queues grow no greater"]
                #[doc = " than 65535 items. Queue indices for normal messages begin at one; zero is reserved in"]
                #[doc = " case of the need to send a high-priority signal message this block."]
                #[doc = " The bool is true if there is a signal message waiting to be sent."]
                pub fn outbound_xcmp_status(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<
                            runtime_types::cumulus_pallet_xcmp_queue::OutboundChannelDetails,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "XcmpQueue",
                        "OutboundXcmpStatus",
                        vec![],
                        [
                            238u8, 120u8, 185u8, 141u8, 82u8, 159u8, 41u8, 68u8, 204u8, 15u8, 46u8,
                            152u8, 144u8, 74u8, 250u8, 83u8, 71u8, 105u8, 54u8, 53u8, 226u8, 87u8,
                            14u8, 202u8, 58u8, 160u8, 54u8, 162u8, 239u8, 248u8, 227u8, 116u8,
                        ],
                    )
                }
                #[doc = " The messages outbound in a given XCMP channel."]
                pub fn outbound_xcmp_messages(
                    &self,
                    _0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u16>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "XcmpQueue",
                        "OutboundXcmpMessages",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Twox64Concat,
                            ),
                        ],
                        [
                            50u8, 182u8, 237u8, 191u8, 106u8, 67u8, 54u8, 1u8, 17u8, 107u8, 70u8,
                            90u8, 202u8, 8u8, 63u8, 184u8, 171u8, 111u8, 192u8, 196u8, 7u8, 31u8,
                            186u8, 68u8, 31u8, 63u8, 71u8, 61u8, 83u8, 223u8, 79u8, 200u8,
                        ],
                    )
                }
                #[doc = " The messages outbound in a given XCMP channel."]
                pub fn outbound_xcmp_messages_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "XcmpQueue",
                        "OutboundXcmpMessages",
                        Vec::new(),
                        [
                            50u8, 182u8, 237u8, 191u8, 106u8, 67u8, 54u8, 1u8, 17u8, 107u8, 70u8,
                            90u8, 202u8, 8u8, 63u8, 184u8, 171u8, 111u8, 192u8, 196u8, 7u8, 31u8,
                            186u8, 68u8, 31u8, 63u8, 71u8, 61u8, 83u8, 223u8, 79u8, 200u8,
                        ],
                    )
                }
                #[doc = " Any signal messages waiting to be sent."]
                pub fn signal_messages(
                    &self,
                    _0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "XcmpQueue",
                        "SignalMessages",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            156u8, 242u8, 186u8, 89u8, 177u8, 195u8, 90u8, 121u8, 94u8, 106u8,
                            222u8, 78u8, 19u8, 162u8, 179u8, 96u8, 38u8, 113u8, 209u8, 148u8, 29u8,
                            110u8, 106u8, 167u8, 162u8, 96u8, 221u8, 20u8, 33u8, 179u8, 168u8,
                            142u8,
                        ],
                    )
                }
                #[doc = " Any signal messages waiting to be sent."]
                pub fn signal_messages_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "XcmpQueue",
                        "SignalMessages",
                        Vec::new(),
                        [
                            156u8, 242u8, 186u8, 89u8, 177u8, 195u8, 90u8, 121u8, 94u8, 106u8,
                            222u8, 78u8, 19u8, 162u8, 179u8, 96u8, 38u8, 113u8, 209u8, 148u8, 29u8,
                            110u8, 106u8, 167u8, 162u8, 96u8, 221u8, 20u8, 33u8, 179u8, 168u8,
                            142u8,
                        ],
                    )
                }
                #[doc = " The configuration which controls the dynamics of the outbound queue."]
                pub fn queue_config(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::cumulus_pallet_xcmp_queue::QueueConfigData,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "XcmpQueue",
                        "QueueConfig",
                        vec![],
                        [
                            19u8, 251u8, 183u8, 11u8, 104u8, 50u8, 57u8, 111u8, 143u8, 89u8, 220u8,
                            164u8, 171u8, 188u8, 161u8, 46u8, 70u8, 6u8, 12u8, 88u8, 151u8, 92u8,
                            140u8, 28u8, 113u8, 51u8, 147u8, 57u8, 150u8, 206u8, 179u8, 152u8,
                        ],
                    )
                }
                #[doc = " The messages that exceeded max individual message weight budget."]
                #[doc = ""]
                #[doc = " These message stay in this storage map until they are manually dispatched via"]
                #[doc = " `service_overweight`."]
                pub fn overweight(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u64>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<(
                        runtime_types::polkadot_parachain::primitives::Id,
                        ::core::primitive::u32,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "XcmpQueue",
                        "Overweight",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Twox64Concat,
                        )],
                        [
                            222u8, 249u8, 232u8, 110u8, 117u8, 229u8, 165u8, 164u8, 219u8, 219u8,
                            149u8, 204u8, 25u8, 78u8, 204u8, 116u8, 111u8, 114u8, 120u8, 222u8,
                            56u8, 77u8, 122u8, 147u8, 108u8, 15u8, 94u8, 161u8, 212u8, 50u8, 7u8,
                            7u8,
                        ],
                    )
                }
                #[doc = " The messages that exceeded max individual message weight budget."]
                #[doc = ""]
                #[doc = " These message stay in this storage map until they are manually dispatched via"]
                #[doc = " `service_overweight`."]
                pub fn overweight_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<(
                        runtime_types::polkadot_parachain::primitives::Id,
                        ::core::primitive::u32,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "XcmpQueue",
                        "Overweight",
                        Vec::new(),
                        [
                            222u8, 249u8, 232u8, 110u8, 117u8, 229u8, 165u8, 164u8, 219u8, 219u8,
                            149u8, 204u8, 25u8, 78u8, 204u8, 116u8, 111u8, 114u8, 120u8, 222u8,
                            56u8, 77u8, 122u8, 147u8, 108u8, 15u8, 94u8, 161u8, 212u8, 50u8, 7u8,
                            7u8,
                        ],
                    )
                }
                #[doc = " The number of overweight messages ever recorded in `Overweight`. Also doubles as the next"]
                #[doc = " available free overweight index."]
                pub fn overweight_count(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "XcmpQueue",
                        "OverweightCount",
                        vec![],
                        [
                            102u8, 180u8, 196u8, 148u8, 115u8, 62u8, 46u8, 238u8, 97u8, 116u8,
                            117u8, 42u8, 14u8, 5u8, 72u8, 237u8, 230u8, 46u8, 150u8, 126u8, 89u8,
                            64u8, 233u8, 166u8, 180u8, 137u8, 52u8, 233u8, 252u8, 255u8, 36u8,
                            20u8,
                        ],
                    )
                }
                #[doc = " Whether or not the XCMP queue is suspended from executing incoming XCMs or not."]
                pub fn queue_suspended(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "XcmpQueue",
                        "QueueSuspended",
                        vec![],
                        [
                            23u8, 37u8, 48u8, 112u8, 222u8, 17u8, 252u8, 65u8, 160u8, 217u8, 218u8,
                            30u8, 2u8, 1u8, 204u8, 0u8, 251u8, 17u8, 138u8, 197u8, 164u8, 50u8,
                            122u8, 0u8, 31u8, 238u8, 147u8, 213u8, 30u8, 132u8, 184u8, 215u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod polkadot_xcm {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Send {
                pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                pub message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct TeleportAssets {
                pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
                pub fee_asset_item: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ReserveTransferAssets {
                pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
                pub fee_asset_item: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Execute {
                pub message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
                pub max_weight: ::core::primitive::u64,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ForceXcmVersion {
                pub location:
                    ::std::boxed::Box<runtime_types::xcm::v1::multilocation::MultiLocation>,
                pub xcm_version: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ForceDefaultXcmVersion {
                pub maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ForceSubscribeVersionNotify {
                pub location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ForceUnsubscribeVersionNotify {
                pub location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct LimitedReserveTransferAssets {
                pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
                pub fee_asset_item: ::core::primitive::u32,
                pub weight_limit: runtime_types::xcm::v2::WeightLimit,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct LimitedTeleportAssets {
                pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
                pub fee_asset_item: ::core::primitive::u32,
                pub weight_limit: runtime_types::xcm::v2::WeightLimit,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                pub fn send(
                    &self,
                    dest: runtime_types::xcm::VersionedMultiLocation,
                    message: runtime_types::xcm::VersionedXcm,
                ) -> ::subxt::tx::StaticTxPayload<Send> {
                    ::subxt::tx::StaticTxPayload::new(
                        "PolkadotXcm",
                        "send",
                        Send {
                            dest: ::std::boxed::Box::new(dest),
                            message: ::std::boxed::Box::new(message),
                        },
                        [
                            190u8, 88u8, 197u8, 248u8, 111u8, 198u8, 199u8, 206u8, 39u8, 121u8,
                            23u8, 121u8, 93u8, 82u8, 22u8, 61u8, 96u8, 210u8, 142u8, 249u8, 195u8,
                            78u8, 44u8, 8u8, 118u8, 120u8, 113u8, 168u8, 99u8, 94u8, 232u8, 4u8,
                        ],
                    )
                }
                #[doc = "Teleport some assets from the local chain to some destination chain."]
                #[doc = ""]
                #[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
                #[doc = "index `fee_asset_item`. The weight limit for fees is not provided and thus is unlimited,"]
                #[doc = "with all fees taken as needed from the asset."]
                #[doc = ""]
                #[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
                #[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
                #[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
                #[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
                #[doc = "  an `AccountId32` value."]
                #[doc = "- `assets`: The assets to be withdrawn. The first item should be the currency used to to pay the fee on the"]
                #[doc = "  `dest` side. May not be empty."]
                #[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
                #[doc = "  fees."]
                pub fn teleport_assets(
                    &self,
                    dest: runtime_types::xcm::VersionedMultiLocation,
                    beneficiary: runtime_types::xcm::VersionedMultiLocation,
                    assets: runtime_types::xcm::VersionedMultiAssets,
                    fee_asset_item: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<TeleportAssets> {
                    ::subxt::tx::StaticTxPayload::new(
                        "PolkadotXcm",
                        "teleport_assets",
                        TeleportAssets {
                            dest: ::std::boxed::Box::new(dest),
                            beneficiary: ::std::boxed::Box::new(beneficiary),
                            assets: ::std::boxed::Box::new(assets),
                            fee_asset_item,
                        },
                        [
                            255u8, 5u8, 68u8, 38u8, 44u8, 181u8, 75u8, 221u8, 239u8, 103u8, 88u8,
                            47u8, 136u8, 90u8, 253u8, 55u8, 0u8, 122u8, 217u8, 126u8, 13u8, 77u8,
                            209u8, 41u8, 7u8, 35u8, 235u8, 171u8, 150u8, 235u8, 202u8, 240u8,
                        ],
                    )
                }
                #[doc = "Transfer some assets from the local chain to the sovereign account of a destination"]
                #[doc = "chain and forward a notification XCM."]
                #[doc = ""]
                #[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
                #[doc = "index `fee_asset_item`. The weight limit for fees is not provided and thus is unlimited,"]
                #[doc = "with all fees taken as needed from the asset."]
                #[doc = ""]
                #[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
                #[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
                #[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
                #[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
                #[doc = "  an `AccountId32` value."]
                #[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the fee on the"]
                #[doc = "  `dest` side."]
                #[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
                #[doc = "  fees."]
                pub fn reserve_transfer_assets(
                    &self,
                    dest: runtime_types::xcm::VersionedMultiLocation,
                    beneficiary: runtime_types::xcm::VersionedMultiLocation,
                    assets: runtime_types::xcm::VersionedMultiAssets,
                    fee_asset_item: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<ReserveTransferAssets> {
                    ::subxt::tx::StaticTxPayload::new(
                        "PolkadotXcm",
                        "reserve_transfer_assets",
                        ReserveTransferAssets {
                            dest: ::std::boxed::Box::new(dest),
                            beneficiary: ::std::boxed::Box::new(beneficiary),
                            assets: ::std::boxed::Box::new(assets),
                            fee_asset_item,
                        },
                        [
                            177u8, 160u8, 188u8, 106u8, 153u8, 135u8, 121u8, 12u8, 83u8, 233u8,
                            43u8, 161u8, 133u8, 26u8, 104u8, 79u8, 113u8, 8u8, 33u8, 128u8, 82u8,
                            62u8, 30u8, 46u8, 203u8, 199u8, 175u8, 193u8, 55u8, 130u8, 206u8, 28u8,
                        ],
                    )
                }
                #[doc = "Execute an XCM message from a local, signed, origin."]
                #[doc = ""]
                #[doc = "An event is deposited indicating whether `msg` could be executed completely or only"]
                #[doc = "partially."]
                #[doc = ""]
                #[doc = "No more than `max_weight` will be used in its attempted execution. If this is less than the"]
                #[doc = "maximum amount of weight that the message could take to be executed, then no execution"]
                #[doc = "attempt will be made."]
                #[doc = ""]
                #[doc = "NOTE: A successful return to this does *not* imply that the `msg` was executed successfully"]
                #[doc = "to completion; only that *some* of it was executed."]
                pub fn execute(
                    &self,
                    message: runtime_types::xcm::VersionedXcm,
                    max_weight: ::core::primitive::u64,
                ) -> ::subxt::tx::StaticTxPayload<Execute> {
                    ::subxt::tx::StaticTxPayload::new(
                        "PolkadotXcm",
                        "execute",
                        Execute {
                            message: ::std::boxed::Box::new(message),
                            max_weight,
                        },
                        [
                            191u8, 177u8, 39u8, 21u8, 1u8, 110u8, 39u8, 58u8, 94u8, 27u8, 44u8,
                            18u8, 253u8, 135u8, 100u8, 205u8, 0u8, 231u8, 68u8, 247u8, 5u8, 140u8,
                            131u8, 184u8, 251u8, 197u8, 100u8, 113u8, 253u8, 255u8, 120u8, 206u8,
                        ],
                    )
                }
                #[doc = "Extoll that a particular destination can be communicated with through a particular"]
                #[doc = "version of XCM."]
                #[doc = ""]
                #[doc = "- `origin`: Must be Root."]
                #[doc = "- `location`: The destination that is being described."]
                #[doc = "- `xcm_version`: The latest version of XCM that `location` supports."]
                pub fn force_xcm_version(
                    &self,
                    location: runtime_types::xcm::v1::multilocation::MultiLocation,
                    xcm_version: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<ForceXcmVersion> {
                    ::subxt::tx::StaticTxPayload::new(
                        "PolkadotXcm",
                        "force_xcm_version",
                        ForceXcmVersion {
                            location: ::std::boxed::Box::new(location),
                            xcm_version,
                        },
                        [
                            231u8, 106u8, 60u8, 226u8, 31u8, 25u8, 20u8, 115u8, 107u8, 246u8,
                            248u8, 11u8, 71u8, 183u8, 93u8, 3u8, 219u8, 21u8, 97u8, 188u8, 119u8,
                            121u8, 239u8, 72u8, 200u8, 81u8, 6u8, 177u8, 111u8, 188u8, 168u8, 86u8,
                        ],
                    )
                }
                #[doc = "Set a safe XCM version (the version that XCM should be encoded with if the most recent"]
                #[doc = "version a destination can accept is unknown)."]
                #[doc = ""]
                #[doc = "- `origin`: Must be Root."]
                #[doc = "- `maybe_xcm_version`: The default XCM encoding version, or `None` to disable."]
                pub fn force_default_xcm_version(
                    &self,
                    maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
                ) -> ::subxt::tx::StaticTxPayload<ForceDefaultXcmVersion> {
                    ::subxt::tx::StaticTxPayload::new(
                        "PolkadotXcm",
                        "force_default_xcm_version",
                        ForceDefaultXcmVersion { maybe_xcm_version },
                        [
                            38u8, 36u8, 59u8, 231u8, 18u8, 79u8, 76u8, 9u8, 200u8, 125u8, 214u8,
                            166u8, 37u8, 99u8, 111u8, 161u8, 135u8, 2u8, 133u8, 157u8, 165u8, 18u8,
                            152u8, 81u8, 209u8, 255u8, 137u8, 237u8, 28u8, 126u8, 224u8, 141u8,
                        ],
                    )
                }
                #[doc = "Ask a location to notify us regarding their XCM version and any changes to it."]
                #[doc = ""]
                #[doc = "- `origin`: Must be Root."]
                #[doc = "- `location`: The location to which we should subscribe for XCM version notifications."]
                pub fn force_subscribe_version_notify(
                    &self,
                    location: runtime_types::xcm::VersionedMultiLocation,
                ) -> ::subxt::tx::StaticTxPayload<ForceSubscribeVersionNotify> {
                    ::subxt::tx::StaticTxPayload::new(
                        "PolkadotXcm",
                        "force_subscribe_version_notify",
                        ForceSubscribeVersionNotify {
                            location: ::std::boxed::Box::new(location),
                        },
                        [
                            136u8, 216u8, 207u8, 51u8, 42u8, 153u8, 92u8, 70u8, 140u8, 169u8,
                            172u8, 89u8, 69u8, 28u8, 200u8, 100u8, 209u8, 226u8, 194u8, 240u8,
                            71u8, 38u8, 18u8, 6u8, 6u8, 83u8, 103u8, 254u8, 248u8, 241u8, 62u8,
                            189u8,
                        ],
                    )
                }
                #[doc = "Require that a particular destination should no longer notify us regarding any XCM"]
                #[doc = "version changes."]
                #[doc = ""]
                #[doc = "- `origin`: Must be Root."]
                #[doc = "- `location`: The location to which we are currently subscribed for XCM version"]
                #[doc = "  notifications which we no longer desire."]
                pub fn force_unsubscribe_version_notify(
                    &self,
                    location: runtime_types::xcm::VersionedMultiLocation,
                ) -> ::subxt::tx::StaticTxPayload<ForceUnsubscribeVersionNotify> {
                    ::subxt::tx::StaticTxPayload::new(
                        "PolkadotXcm",
                        "force_unsubscribe_version_notify",
                        ForceUnsubscribeVersionNotify {
                            location: ::std::boxed::Box::new(location),
                        },
                        [
                            51u8, 72u8, 5u8, 227u8, 251u8, 243u8, 199u8, 9u8, 8u8, 213u8, 191u8,
                            52u8, 21u8, 215u8, 170u8, 6u8, 53u8, 242u8, 225u8, 89u8, 150u8, 142u8,
                            104u8, 249u8, 225u8, 209u8, 142u8, 234u8, 161u8, 100u8, 153u8, 120u8,
                        ],
                    )
                }
                #[doc = "Transfer some assets from the local chain to the sovereign account of a destination"]
                #[doc = "chain and forward a notification XCM."]
                #[doc = ""]
                #[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
                #[doc = "index `fee_asset_item`, up to enough to pay for `weight_limit` of weight. If more weight"]
                #[doc = "is needed than `weight_limit`, then the operation will fail and the assets send may be"]
                #[doc = "at risk."]
                #[doc = ""]
                #[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
                #[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
                #[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
                #[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
                #[doc = "  an `AccountId32` value."]
                #[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the fee on the"]
                #[doc = "  `dest` side."]
                #[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
                #[doc = "  fees."]
                #[doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."]
                pub fn limited_reserve_transfer_assets(
                    &self,
                    dest: runtime_types::xcm::VersionedMultiLocation,
                    beneficiary: runtime_types::xcm::VersionedMultiLocation,
                    assets: runtime_types::xcm::VersionedMultiAssets,
                    fee_asset_item: ::core::primitive::u32,
                    weight_limit: runtime_types::xcm::v2::WeightLimit,
                ) -> ::subxt::tx::StaticTxPayload<LimitedReserveTransferAssets> {
                    ::subxt::tx::StaticTxPayload::new(
                        "PolkadotXcm",
                        "limited_reserve_transfer_assets",
                        LimitedReserveTransferAssets {
                            dest: ::std::boxed::Box::new(dest),
                            beneficiary: ::std::boxed::Box::new(beneficiary),
                            assets: ::std::boxed::Box::new(assets),
                            fee_asset_item,
                            weight_limit,
                        },
                        [
                            191u8, 81u8, 68u8, 116u8, 196u8, 125u8, 226u8, 154u8, 144u8, 126u8,
                            159u8, 149u8, 17u8, 124u8, 205u8, 60u8, 249u8, 106u8, 38u8, 251u8,
                            136u8, 128u8, 81u8, 201u8, 164u8, 242u8, 216u8, 80u8, 21u8, 234u8,
                            20u8, 70u8,
                        ],
                    )
                }
                #[doc = "Teleport some assets from the local chain to some destination chain."]
                #[doc = ""]
                #[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
                #[doc = "index `fee_asset_item`, up to enough to pay for `weight_limit` of weight. If more weight"]
                #[doc = "is needed than `weight_limit`, then the operation will fail and the assets send may be"]
                #[doc = "at risk."]
                #[doc = ""]
                #[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
                #[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
                #[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
                #[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
                #[doc = "  an `AccountId32` value."]
                #[doc = "- `assets`: The assets to be withdrawn. The first item should be the currency used to to pay the fee on the"]
                #[doc = "  `dest` side. May not be empty."]
                #[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
                #[doc = "  fees."]
                #[doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."]
                pub fn limited_teleport_assets(
                    &self,
                    dest: runtime_types::xcm::VersionedMultiLocation,
                    beneficiary: runtime_types::xcm::VersionedMultiLocation,
                    assets: runtime_types::xcm::VersionedMultiAssets,
                    fee_asset_item: ::core::primitive::u32,
                    weight_limit: runtime_types::xcm::v2::WeightLimit,
                ) -> ::subxt::tx::StaticTxPayload<LimitedTeleportAssets> {
                    ::subxt::tx::StaticTxPayload::new(
                        "PolkadotXcm",
                        "limited_teleport_assets",
                        LimitedTeleportAssets {
                            dest: ::std::boxed::Box::new(dest),
                            beneficiary: ::std::boxed::Box::new(beneficiary),
                            assets: ::std::boxed::Box::new(assets),
                            fee_asset_item,
                            weight_limit,
                        },
                        [
                            29u8, 31u8, 229u8, 83u8, 40u8, 60u8, 36u8, 185u8, 169u8, 74u8, 30u8,
                            47u8, 118u8, 118u8, 22u8, 15u8, 246u8, 220u8, 169u8, 135u8, 72u8,
                            154u8, 109u8, 192u8, 195u8, 58u8, 121u8, 240u8, 166u8, 243u8, 29u8,
                            29u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_xcm::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Execution of an XCM message was attempted."]
            #[doc = ""]
            #[doc = "\\[ outcome \\]"]
            pub struct Attempted(pub runtime_types::xcm::v2::traits::Outcome);
            impl ::subxt::events::StaticEvent for Attempted {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "Attempted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A XCM message was sent."]
            #[doc = ""]
            #[doc = "\\[ origin, destination, message \\]"]
            pub struct Sent(
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub runtime_types::xcm::v2::Xcm,
            );
            impl ::subxt::events::StaticEvent for Sent {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "Sent";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Query response received which does not match a registered query. This may be because a"]
            #[doc = "matching query was never registered, it may be because it is a duplicate response, or"]
            #[doc = "because the query timed out."]
            #[doc = ""]
            #[doc = "\\[ origin location, id \\]"]
            pub struct UnexpectedResponse(
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub ::core::primitive::u64,
            );
            impl ::subxt::events::StaticEvent for UnexpectedResponse {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "UnexpectedResponse";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Query response has been received and is ready for taking with `take_response`. There is"]
            #[doc = "no registered notification call."]
            #[doc = ""]
            #[doc = "\\[ id, response \\]"]
            pub struct ResponseReady(
                pub ::core::primitive::u64,
                pub runtime_types::xcm::v2::Response,
            );
            impl ::subxt::events::StaticEvent for ResponseReady {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "ResponseReady";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Query response has been received and query is removed. The registered notification has"]
            #[doc = "been dispatched and executed successfully."]
            #[doc = ""]
            #[doc = "\\[ id, pallet index, call index \\]"]
            pub struct Notified(
                pub ::core::primitive::u64,
                pub ::core::primitive::u8,
                pub ::core::primitive::u8,
            );
            impl ::subxt::events::StaticEvent for Notified {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "Notified";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Query response has been received and query is removed. The registered notification could"]
            #[doc = "not be dispatched because the dispatch weight is greater than the maximum weight"]
            #[doc = "originally budgeted by this runtime for the query result."]
            #[doc = ""]
            #[doc = "\\[ id, pallet index, call index, actual weight, max budgeted weight \\]"]
            pub struct NotifyOverweight(
                pub ::core::primitive::u64,
                pub ::core::primitive::u8,
                pub ::core::primitive::u8,
                pub ::core::primitive::u64,
                pub ::core::primitive::u64,
            );
            impl ::subxt::events::StaticEvent for NotifyOverweight {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "NotifyOverweight";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Query response has been received and query is removed. There was a general error with"]
            #[doc = "dispatching the notification call."]
            #[doc = ""]
            #[doc = "\\[ id, pallet index, call index \\]"]
            pub struct NotifyDispatchError(
                pub ::core::primitive::u64,
                pub ::core::primitive::u8,
                pub ::core::primitive::u8,
            );
            impl ::subxt::events::StaticEvent for NotifyDispatchError {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "NotifyDispatchError";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Query response has been received and query is removed. The dispatch was unable to be"]
            #[doc = "decoded into a `Call`; this might be due to dispatch function having a signature which"]
            #[doc = "is not `(origin, QueryId, Response)`."]
            #[doc = ""]
            #[doc = "\\[ id, pallet index, call index \\]"]
            pub struct NotifyDecodeFailed(
                pub ::core::primitive::u64,
                pub ::core::primitive::u8,
                pub ::core::primitive::u8,
            );
            impl ::subxt::events::StaticEvent for NotifyDecodeFailed {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "NotifyDecodeFailed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Expected query response has been received but the origin location of the response does"]
            #[doc = "not match that expected. The query remains registered for a later, valid, response to"]
            #[doc = "be received and acted upon."]
            #[doc = ""]
            #[doc = "\\[ origin location, id, expected location \\]"]
            pub struct InvalidResponder(
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub ::core::primitive::u64,
                pub ::core::option::Option<runtime_types::xcm::v1::multilocation::MultiLocation>,
            );
            impl ::subxt::events::StaticEvent for InvalidResponder {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "InvalidResponder";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Expected query response has been received but the expected origin location placed in"]
            #[doc = "storage by this runtime previously cannot be decoded. The query remains registered."]
            #[doc = ""]
            #[doc = "This is unexpected (since a location placed in storage in a previously executing"]
            #[doc = "runtime should be readable prior to query timeout) and dangerous since the possibly"]
            #[doc = "valid response will be dropped. Manual governance intervention is probably going to be"]
            #[doc = "needed."]
            #[doc = ""]
            #[doc = "\\[ origin location, id \\]"]
            pub struct InvalidResponderVersion(
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub ::core::primitive::u64,
            );
            impl ::subxt::events::StaticEvent for InvalidResponderVersion {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "InvalidResponderVersion";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            #[doc = "Received query response has been read and removed."]
            #[doc = ""]
            #[doc = "\\[ id \\]"]
            pub struct ResponseTaken(pub ::core::primitive::u64);
            impl ::subxt::events::StaticEvent for ResponseTaken {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "ResponseTaken";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Some assets have been placed in an asset trap."]
            #[doc = ""]
            #[doc = "\\[ hash, origin, assets \\]"]
            pub struct AssetsTrapped(
                pub ::subxt::ext::sp_core::H256,
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub runtime_types::xcm::VersionedMultiAssets,
            );
            impl ::subxt::events::StaticEvent for AssetsTrapped {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "AssetsTrapped";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "An XCM version change notification message has been attempted to be sent."]
            #[doc = ""]
            #[doc = "\\[ destination, result \\]"]
            pub struct VersionChangeNotified(
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub ::core::primitive::u32,
            );
            impl ::subxt::events::StaticEvent for VersionChangeNotified {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "VersionChangeNotified";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "The supported version of a location has been changed. This might be through an"]
            #[doc = "automatic notification or a manual intervention."]
            #[doc = ""]
            #[doc = "\\[ location, XCM version \\]"]
            pub struct SupportedVersionChanged(
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub ::core::primitive::u32,
            );
            impl ::subxt::events::StaticEvent for SupportedVersionChanged {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "SupportedVersionChanged";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A given location which had a version change subscription was dropped owing to an error"]
            #[doc = "sending the notification to it."]
            #[doc = ""]
            #[doc = "\\[ location, query ID, error \\]"]
            pub struct NotifyTargetSendFail(
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub ::core::primitive::u64,
                pub runtime_types::xcm::v2::traits::Error,
            );
            impl ::subxt::events::StaticEvent for NotifyTargetSendFail {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "NotifyTargetSendFail";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A given location which had a version change subscription was dropped owing to an error"]
            #[doc = "migrating the location to our new XCM format."]
            #[doc = ""]
            #[doc = "\\[ location, query ID \\]"]
            pub struct NotifyTargetMigrationFail(
                pub runtime_types::xcm::VersionedMultiLocation,
                pub ::core::primitive::u64,
            );
            impl ::subxt::events::StaticEvent for NotifyTargetMigrationFail {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "NotifyTargetMigrationFail";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The latest available query index."]
                pub fn query_counter(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "PolkadotXcm",
                        "QueryCounter",
                        vec![],
                        [
                            137u8, 58u8, 184u8, 88u8, 247u8, 22u8, 151u8, 64u8, 50u8, 77u8, 49u8,
                            10u8, 234u8, 84u8, 213u8, 156u8, 26u8, 200u8, 214u8, 225u8, 125u8,
                            231u8, 42u8, 93u8, 159u8, 168u8, 86u8, 201u8, 116u8, 153u8, 41u8,
                            127u8,
                        ],
                    )
                }
                #[doc = " The ongoing queries."]
                pub fn queries(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u64>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_xcm::pallet::QueryStatus<::core::primitive::u32>,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "PolkadotXcm",
                        "Queries",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            251u8, 97u8, 131u8, 135u8, 93u8, 68u8, 156u8, 25u8, 181u8, 231u8,
                            124u8, 93u8, 170u8, 114u8, 250u8, 177u8, 172u8, 51u8, 59u8, 44u8,
                            148u8, 189u8, 199u8, 62u8, 118u8, 89u8, 75u8, 29u8, 71u8, 49u8, 248u8,
                            48u8,
                        ],
                    )
                }
                #[doc = " The ongoing queries."]
                pub fn queries_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_xcm::pallet::QueryStatus<::core::primitive::u32>,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "PolkadotXcm",
                        "Queries",
                        Vec::new(),
                        [
                            251u8, 97u8, 131u8, 135u8, 93u8, 68u8, 156u8, 25u8, 181u8, 231u8,
                            124u8, 93u8, 170u8, 114u8, 250u8, 177u8, 172u8, 51u8, 59u8, 44u8,
                            148u8, 189u8, 199u8, 62u8, 118u8, 89u8, 75u8, 29u8, 71u8, 49u8, 248u8,
                            48u8,
                        ],
                    )
                }
                #[doc = " The existing asset traps."]
                #[doc = ""]
                #[doc = " Key is the blake2 256 hash of (origin, versioned `MultiAssets`) pair. Value is the number of"]
                #[doc = " times this pair has been trapped (usually just 1 if it exists at all)."]
                pub fn asset_traps(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::H256>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "PolkadotXcm",
                        "AssetTraps",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Identity,
                        )],
                        [
                            4u8, 185u8, 92u8, 4u8, 7u8, 71u8, 214u8, 1u8, 141u8, 59u8, 87u8, 55u8,
                            149u8, 26u8, 125u8, 8u8, 88u8, 31u8, 240u8, 138u8, 133u8, 28u8, 37u8,
                            131u8, 107u8, 218u8, 86u8, 152u8, 147u8, 44u8, 19u8, 239u8,
                        ],
                    )
                }
                #[doc = " The existing asset traps."]
                #[doc = ""]
                #[doc = " Key is the blake2 256 hash of (origin, versioned `MultiAssets`) pair. Value is the number of"]
                #[doc = " times this pair has been trapped (usually just 1 if it exists at all)."]
                pub fn asset_traps_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "PolkadotXcm",
                        "AssetTraps",
                        Vec::new(),
                        [
                            4u8, 185u8, 92u8, 4u8, 7u8, 71u8, 214u8, 1u8, 141u8, 59u8, 87u8, 55u8,
                            149u8, 26u8, 125u8, 8u8, 88u8, 31u8, 240u8, 138u8, 133u8, 28u8, 37u8,
                            131u8, 107u8, 218u8, 86u8, 152u8, 147u8, 44u8, 19u8, 239u8,
                        ],
                    )
                }
                #[doc = " Default version to encode XCM when latest version of destination is unknown. If `None`,"]
                #[doc = " then the destinations whose XCM version is unknown are considered unreachable."]
                pub fn safe_xcm_version(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "PolkadotXcm",
                        "SafeXcmVersion",
                        vec![],
                        [
                            1u8, 223u8, 218u8, 204u8, 222u8, 129u8, 137u8, 237u8, 197u8, 142u8,
                            233u8, 66u8, 229u8, 153u8, 138u8, 222u8, 113u8, 164u8, 135u8, 213u8,
                            233u8, 34u8, 24u8, 23u8, 215u8, 59u8, 40u8, 188u8, 45u8, 244u8, 205u8,
                            199u8,
                        ],
                    )
                }
                #[doc = " The Latest versions that we know various locations support."]
                pub fn supported_version(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                    _1: impl ::std::borrow::Borrow<runtime_types::xcm::VersionedMultiLocation>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "PolkadotXcm",
                        "SupportedVersion",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Twox64Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                        ],
                        [
                            112u8, 34u8, 251u8, 179u8, 217u8, 54u8, 125u8, 242u8, 190u8, 8u8, 44u8,
                            14u8, 138u8, 76u8, 241u8, 95u8, 233u8, 96u8, 141u8, 26u8, 151u8, 196u8,
                            219u8, 137u8, 165u8, 27u8, 87u8, 128u8, 19u8, 35u8, 222u8, 202u8,
                        ],
                    )
                }
                #[doc = " The Latest versions that we know various locations support."]
                pub fn supported_version_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "PolkadotXcm",
                        "SupportedVersion",
                        Vec::new(),
                        [
                            112u8, 34u8, 251u8, 179u8, 217u8, 54u8, 125u8, 242u8, 190u8, 8u8, 44u8,
                            14u8, 138u8, 76u8, 241u8, 95u8, 233u8, 96u8, 141u8, 26u8, 151u8, 196u8,
                            219u8, 137u8, 165u8, 27u8, 87u8, 128u8, 19u8, 35u8, 222u8, 202u8,
                        ],
                    )
                }
                #[doc = " All locations that we have requested version notifications from."]
                pub fn version_notifiers(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                    _1: impl ::std::borrow::Borrow<runtime_types::xcm::VersionedMultiLocation>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "PolkadotXcm",
                        "VersionNotifiers",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Twox64Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                        ],
                        [
                            233u8, 217u8, 119u8, 102u8, 41u8, 77u8, 198u8, 24u8, 161u8, 22u8,
                            104u8, 149u8, 204u8, 128u8, 123u8, 166u8, 17u8, 36u8, 202u8, 92u8,
                            190u8, 44u8, 73u8, 239u8, 88u8, 17u8, 92u8, 41u8, 236u8, 80u8, 154u8,
                            10u8,
                        ],
                    )
                }
                #[doc = " All locations that we have requested version notifications from."]
                pub fn version_notifiers_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "PolkadotXcm",
                        "VersionNotifiers",
                        Vec::new(),
                        [
                            233u8, 217u8, 119u8, 102u8, 41u8, 77u8, 198u8, 24u8, 161u8, 22u8,
                            104u8, 149u8, 204u8, 128u8, 123u8, 166u8, 17u8, 36u8, 202u8, 92u8,
                            190u8, 44u8, 73u8, 239u8, 88u8, 17u8, 92u8, 41u8, 236u8, 80u8, 154u8,
                            10u8,
                        ],
                    )
                }
                #[doc = " The target locations that are subscribed to our version changes, as well as the most recent"]
                #[doc = " of our versions we informed them of."]
                pub fn version_notify_targets(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                    _1: impl ::std::borrow::Borrow<runtime_types::xcm::VersionedMultiLocation>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<(
                        ::core::primitive::u64,
                        ::core::primitive::u64,
                        ::core::primitive::u32,
                    )>,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "PolkadotXcm",
                        "VersionNotifyTargets",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Twox64Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                        ],
                        [
                            108u8, 104u8, 137u8, 191u8, 2u8, 2u8, 240u8, 174u8, 32u8, 174u8, 150u8,
                            136u8, 33u8, 84u8, 30u8, 74u8, 95u8, 94u8, 20u8, 112u8, 101u8, 204u8,
                            15u8, 47u8, 136u8, 56u8, 40u8, 66u8, 1u8, 42u8, 16u8, 247u8,
                        ],
                    )
                }
                #[doc = " The target locations that are subscribed to our version changes, as well as the most recent"]
                #[doc = " of our versions we informed them of."]
                pub fn version_notify_targets_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<(
                        ::core::primitive::u64,
                        ::core::primitive::u64,
                        ::core::primitive::u32,
                    )>,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "PolkadotXcm",
                        "VersionNotifyTargets",
                        Vec::new(),
                        [
                            108u8, 104u8, 137u8, 191u8, 2u8, 2u8, 240u8, 174u8, 32u8, 174u8, 150u8,
                            136u8, 33u8, 84u8, 30u8, 74u8, 95u8, 94u8, 20u8, 112u8, 101u8, 204u8,
                            15u8, 47u8, 136u8, 56u8, 40u8, 66u8, 1u8, 42u8, 16u8, 247u8,
                        ],
                    )
                }
                #[doc = " Destinations whose latest XCM version we would like to know. Duplicates not allowed, and"]
                #[doc = " the `u32` counter is the number of times that a send to the destination has been attempted,"]
                #[doc = " which is used as a prioritization."]
                pub fn version_discovery_queue(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<(
                            runtime_types::xcm::VersionedMultiLocation,
                            ::core::primitive::u32,
                        )>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "PolkadotXcm",
                        "VersionDiscoveryQueue",
                        vec![],
                        [
                            30u8, 163u8, 210u8, 133u8, 30u8, 63u8, 36u8, 9u8, 162u8, 133u8, 99u8,
                            170u8, 34u8, 205u8, 27u8, 41u8, 226u8, 141u8, 165u8, 151u8, 46u8,
                            140u8, 150u8, 242u8, 178u8, 88u8, 164u8, 12u8, 129u8, 118u8, 25u8,
                            79u8,
                        ],
                    )
                }
                #[doc = " The current migration's stage, if any."]
                pub fn current_migration(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_xcm::pallet::VersionMigrationStage,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "PolkadotXcm",
                        "CurrentMigration",
                        vec![],
                        [
                            137u8, 144u8, 168u8, 185u8, 158u8, 90u8, 127u8, 243u8, 227u8, 134u8,
                            150u8, 73u8, 15u8, 99u8, 23u8, 47u8, 68u8, 18u8, 39u8, 16u8, 24u8,
                            43u8, 161u8, 56u8, 66u8, 111u8, 16u8, 7u8, 252u8, 125u8, 100u8, 225u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod cumulus_xcm {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::cumulus_pallet_xcm::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Downward message is invalid XCM."]
            #[doc = "\\[ id \\]"]
            pub struct InvalidFormat(pub [::core::primitive::u8; 8usize]);
            impl ::subxt::events::StaticEvent for InvalidFormat {
                const PALLET: &'static str = "CumulusXcm";
                const EVENT: &'static str = "InvalidFormat";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Downward message is unsupported version of XCM."]
            #[doc = "\\[ id \\]"]
            pub struct UnsupportedVersion(pub [::core::primitive::u8; 8usize]);
            impl ::subxt::events::StaticEvent for UnsupportedVersion {
                const PALLET: &'static str = "CumulusXcm";
                const EVENT: &'static str = "UnsupportedVersion";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Downward message executed with the given outcome."]
            #[doc = "\\[ id, outcome \\]"]
            pub struct ExecutedDownward(
                pub [::core::primitive::u8; 8usize],
                pub runtime_types::xcm::v2::traits::Outcome,
            );
            impl ::subxt::events::StaticEvent for ExecutedDownward {
                const PALLET: &'static str = "CumulusXcm";
                const EVENT: &'static str = "ExecutedDownward";
            }
        }
    }
    pub mod dmp_queue {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ServiceOverweight {
                pub index: ::core::primitive::u64,
                pub weight_limit: ::core::primitive::u64,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Service a single overweight message."]
                #[doc = ""]
                #[doc = "- `origin`: Must pass `ExecuteOverweightOrigin`."]
                #[doc = "- `index`: The index of the overweight message to service."]
                #[doc = "- `weight_limit`: The amount of weight that message execution may take."]
                #[doc = ""]
                #[doc = "Errors:"]
                #[doc = "- `Unknown`: Message of `index` is unknown."]
                #[doc = "- `OverLimit`: Message execution may use greater than `weight_limit`."]
                #[doc = ""]
                #[doc = "Events:"]
                #[doc = "- `OverweightServiced`: On success."]
                pub fn service_overweight(
                    &self,
                    index: ::core::primitive::u64,
                    weight_limit: ::core::primitive::u64,
                ) -> ::subxt::tx::StaticTxPayload<ServiceOverweight> {
                    ::subxt::tx::StaticTxPayload::new(
                        "DmpQueue",
                        "service_overweight",
                        ServiceOverweight {
                            index,
                            weight_limit,
                        },
                        [
                            225u8, 41u8, 132u8, 91u8, 28u8, 116u8, 89u8, 197u8, 194u8, 131u8, 28u8,
                            217u8, 102u8, 241u8, 122u8, 230u8, 242u8, 75u8, 83u8, 67u8, 104u8,
                            55u8, 133u8, 129u8, 91u8, 25u8, 185u8, 131u8, 22u8, 253u8, 84u8, 221u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::cumulus_pallet_dmp_queue::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Downward message is invalid XCM."]
            pub struct InvalidFormat {
                pub message_id: [::core::primitive::u8; 32usize],
            }
            impl ::subxt::events::StaticEvent for InvalidFormat {
                const PALLET: &'static str = "DmpQueue";
                const EVENT: &'static str = "InvalidFormat";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Downward message is unsupported version of XCM."]
            pub struct UnsupportedVersion {
                pub message_id: [::core::primitive::u8; 32usize],
            }
            impl ::subxt::events::StaticEvent for UnsupportedVersion {
                const PALLET: &'static str = "DmpQueue";
                const EVENT: &'static str = "UnsupportedVersion";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Downward message executed with the given outcome."]
            pub struct ExecutedDownward {
                pub message_id: [::core::primitive::u8; 32usize],
                pub outcome: runtime_types::xcm::v2::traits::Outcome,
            }
            impl ::subxt::events::StaticEvent for ExecutedDownward {
                const PALLET: &'static str = "DmpQueue";
                const EVENT: &'static str = "ExecutedDownward";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "The weight limit for handling downward messages was reached."]
            pub struct WeightExhausted {
                pub message_id: [::core::primitive::u8; 32usize],
                pub remaining_weight: ::core::primitive::u64,
                pub required_weight: ::core::primitive::u64,
            }
            impl ::subxt::events::StaticEvent for WeightExhausted {
                const PALLET: &'static str = "DmpQueue";
                const EVENT: &'static str = "WeightExhausted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Downward message is overweight and was placed in the overweight queue."]
            pub struct OverweightEnqueued {
                pub message_id: [::core::primitive::u8; 32usize],
                pub overweight_index: ::core::primitive::u64,
                pub required_weight: ::core::primitive::u64,
            }
            impl ::subxt::events::StaticEvent for OverweightEnqueued {
                const PALLET: &'static str = "DmpQueue";
                const EVENT: &'static str = "OverweightEnqueued";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Downward message from the overweight queue was executed."]
            pub struct OverweightServiced {
                pub overweight_index: ::core::primitive::u64,
                pub weight_used: ::core::primitive::u64,
            }
            impl ::subxt::events::StaticEvent for OverweightServiced {
                const PALLET: &'static str = "DmpQueue";
                const EVENT: &'static str = "OverweightServiced";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The configuration."]
                pub fn configuration(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::cumulus_pallet_dmp_queue::ConfigData,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DmpQueue",
                        "Configuration",
                        vec![],
                        [
                            1u8, 54u8, 187u8, 187u8, 248u8, 220u8, 44u8, 119u8, 173u8, 91u8, 236u8,
                            102u8, 123u8, 199u8, 153u8, 26u8, 188u8, 102u8, 123u8, 180u8, 149u8,
                            239u8, 184u8, 96u8, 100u8, 52u8, 150u8, 23u8, 26u8, 144u8, 0u8, 224u8,
                        ],
                    )
                }
                #[doc = " The page index."]
                pub fn page_index(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::cumulus_pallet_dmp_queue::PageIndexData,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DmpQueue",
                        "PageIndex",
                        vec![],
                        [
                            94u8, 132u8, 34u8, 67u8, 10u8, 22u8, 235u8, 96u8, 168u8, 26u8, 57u8,
                            200u8, 130u8, 218u8, 37u8, 71u8, 28u8, 119u8, 78u8, 107u8, 209u8,
                            120u8, 190u8, 2u8, 101u8, 215u8, 122u8, 187u8, 94u8, 38u8, 255u8,
                            234u8,
                        ],
                    )
                }
                #[doc = " The queue pages."]
                pub fn pages(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<(
                            ::core::primitive::u32,
                            ::std::vec::Vec<::core::primitive::u8>,
                        )>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DmpQueue",
                        "Pages",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            228u8, 86u8, 33u8, 107u8, 248u8, 4u8, 223u8, 175u8, 222u8, 25u8, 204u8,
                            42u8, 235u8, 21u8, 215u8, 91u8, 167u8, 14u8, 133u8, 151u8, 190u8, 57u8,
                            138u8, 208u8, 79u8, 244u8, 132u8, 14u8, 48u8, 247u8, 171u8, 108u8,
                        ],
                    )
                }
                #[doc = " The queue pages."]
                pub fn pages_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<(
                            ::core::primitive::u32,
                            ::std::vec::Vec<::core::primitive::u8>,
                        )>,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DmpQueue",
                        "Pages",
                        Vec::new(),
                        [
                            228u8, 86u8, 33u8, 107u8, 248u8, 4u8, 223u8, 175u8, 222u8, 25u8, 204u8,
                            42u8, 235u8, 21u8, 215u8, 91u8, 167u8, 14u8, 133u8, 151u8, 190u8, 57u8,
                            138u8, 208u8, 79u8, 244u8, 132u8, 14u8, 48u8, 247u8, 171u8, 108u8,
                        ],
                    )
                }
                #[doc = " The overweight messages."]
                pub fn overweight(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u64>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<(
                        ::core::primitive::u32,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DmpQueue",
                        "Overweight",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            222u8, 85u8, 143u8, 49u8, 42u8, 248u8, 138u8, 163u8, 46u8, 199u8,
                            188u8, 61u8, 137u8, 135u8, 127u8, 146u8, 210u8, 254u8, 121u8, 42u8,
                            112u8, 114u8, 22u8, 228u8, 207u8, 207u8, 245u8, 175u8, 152u8, 140u8,
                            225u8, 237u8,
                        ],
                    )
                }
                #[doc = " The overweight messages."]
                pub fn overweight_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<(
                        ::core::primitive::u32,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "DmpQueue",
                        "Overweight",
                        Vec::new(),
                        [
                            222u8, 85u8, 143u8, 49u8, 42u8, 248u8, 138u8, 163u8, 46u8, 199u8,
                            188u8, 61u8, 137u8, 135u8, 127u8, 146u8, 210u8, 254u8, 121u8, 42u8,
                            112u8, 114u8, 22u8, 228u8, 207u8, 207u8, 245u8, 175u8, 152u8, 140u8,
                            225u8, 237u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod utility {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Batch {
                pub calls: ::std::vec::Vec<runtime_types::encointer_runtime::Call>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct AsDerivative {
                pub index: ::core::primitive::u16,
                pub call: ::std::boxed::Box<runtime_types::encointer_runtime::Call>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct BatchAll {
                pub calls: ::std::vec::Vec<runtime_types::encointer_runtime::Call>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct DispatchAs {
                pub as_origin: ::std::boxed::Box<runtime_types::encointer_runtime::OriginCaller>,
                pub call: ::std::boxed::Box<runtime_types::encointer_runtime::Call>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ForceBatch {
                pub calls: ::std::vec::Vec<runtime_types::encointer_runtime::Call>,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Send a batch of dispatch calls."]
                #[doc = ""]
                #[doc = "May be called from any origin."]
                #[doc = ""]
                #[doc = "- `calls`: The calls to be dispatched from the same origin. The number of call must not"]
                #[doc = "  exceed the constant: `batched_calls_limit` (available in constant metadata)."]
                #[doc = ""]
                #[doc = "If origin is root then call are dispatch without checking origin filter. (This includes"]
                #[doc = "bypassing `frame_system::Config::BaseCallFilter`)."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- Complexity: O(C) where C is the number of calls to be batched."]
                #[doc = "# </weight>"]
                #[doc = ""]
                #[doc = "This will return `Ok` in all circumstances. To determine the success of the batch, an"]
                #[doc = "event is deposited. If a call failed and the batch was interrupted, then the"]
                #[doc = "`BatchInterrupted` event is deposited, along with the number of successful calls made"]
                #[doc = "and the error of the failed call. If all were successful, then the `BatchCompleted`"]
                #[doc = "event is deposited."]
                pub fn batch(
                    &self,
                    calls: ::std::vec::Vec<runtime_types::encointer_runtime::Call>,
                ) -> ::subxt::tx::StaticTxPayload<Batch> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Utility",
                        "batch",
                        Batch { calls },
                        [
                            103u8, 12u8, 106u8, 189u8, 146u8, 73u8, 196u8, 122u8, 134u8, 244u8,
                            11u8, 199u8, 106u8, 159u8, 24u8, 36u8, 50u8, 84u8, 162u8, 236u8, 35u8,
                            242u8, 42u8, 218u8, 22u8, 179u8, 193u8, 0u8, 152u8, 130u8, 245u8,
                            187u8,
                        ],
                    )
                }
                #[doc = "Send a call through an indexed pseudonym of the sender."]
                #[doc = ""]
                #[doc = "Filter from origin are passed along. The call will be dispatched with an origin which"]
                #[doc = "use the same filter as the origin of this call."]
                #[doc = ""]
                #[doc = "NOTE: If you need to ensure that any account-based filtering is not honored (i.e."]
                #[doc = "because you expect `proxy` to have been used prior in the call stack and you do not want"]
                #[doc = "the call restrictions to apply to any sub-accounts), then use `as_multi_threshold_1`"]
                #[doc = "in the Multisig pallet instead."]
                #[doc = ""]
                #[doc = "NOTE: Prior to version *12, this was called `as_limited_sub`."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                pub fn as_derivative(
                    &self,
                    index: ::core::primitive::u16,
                    call: runtime_types::encointer_runtime::Call,
                ) -> ::subxt::tx::StaticTxPayload<AsDerivative> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Utility",
                        "as_derivative",
                        AsDerivative {
                            index,
                            call: ::std::boxed::Box::new(call),
                        },
                        [
                            113u8, 126u8, 83u8, 126u8, 92u8, 161u8, 249u8, 158u8, 42u8, 190u8,
                            114u8, 213u8, 151u8, 137u8, 63u8, 219u8, 210u8, 228u8, 220u8, 15u8,
                            0u8, 202u8, 48u8, 174u8, 54u8, 7u8, 64u8, 127u8, 109u8, 56u8, 61u8,
                            100u8,
                        ],
                    )
                }
                #[doc = "Send a batch of dispatch calls and atomically execute them."]
                #[doc = "The whole transaction will rollback and fail if any of the calls failed."]
                #[doc = ""]
                #[doc = "May be called from any origin."]
                #[doc = ""]
                #[doc = "- `calls`: The calls to be dispatched from the same origin. The number of call must not"]
                #[doc = "  exceed the constant: `batched_calls_limit` (available in constant metadata)."]
                #[doc = ""]
                #[doc = "If origin is root then call are dispatch without checking origin filter. (This includes"]
                #[doc = "bypassing `frame_system::Config::BaseCallFilter`)."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- Complexity: O(C) where C is the number of calls to be batched."]
                #[doc = "# </weight>"]
                pub fn batch_all(
                    &self,
                    calls: ::std::vec::Vec<runtime_types::encointer_runtime::Call>,
                ) -> ::subxt::tx::StaticTxPayload<BatchAll> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Utility",
                        "batch_all",
                        BatchAll { calls },
                        [
                            121u8, 96u8, 48u8, 178u8, 153u8, 187u8, 239u8, 161u8, 193u8, 183u8,
                            232u8, 84u8, 111u8, 236u8, 76u8, 173u8, 55u8, 226u8, 205u8, 142u8,
                            27u8, 215u8, 41u8, 159u8, 210u8, 153u8, 160u8, 79u8, 29u8, 41u8, 184u8,
                            143u8,
                        ],
                    )
                }
                #[doc = "Dispatches a function call with a provided origin."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Root_."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- O(1)."]
                #[doc = "- Limited storage reads."]
                #[doc = "- One DB write (event)."]
                #[doc = "- Weight of derivative `call` execution + T::WeightInfo::dispatch_as()."]
                #[doc = "# </weight>"]
                pub fn dispatch_as(
                    &self,
                    as_origin: runtime_types::encointer_runtime::OriginCaller,
                    call: runtime_types::encointer_runtime::Call,
                ) -> ::subxt::tx::StaticTxPayload<DispatchAs> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Utility",
                        "dispatch_as",
                        DispatchAs {
                            as_origin: ::std::boxed::Box::new(as_origin),
                            call: ::std::boxed::Box::new(call),
                        },
                        [
                            128u8, 50u8, 143u8, 196u8, 167u8, 25u8, 195u8, 19u8, 76u8, 152u8, 31u8,
                            174u8, 148u8, 245u8, 57u8, 136u8, 34u8, 231u8, 98u8, 105u8, 139u8,
                            122u8, 19u8, 177u8, 106u8, 20u8, 54u8, 211u8, 64u8, 159u8, 171u8,
                            179u8,
                        ],
                    )
                }
                #[doc = "Send a batch of dispatch calls."]
                #[doc = "Unlike `batch`, it allows errors and won't interrupt."]
                #[doc = ""]
                #[doc = "May be called from any origin."]
                #[doc = ""]
                #[doc = "- `calls`: The calls to be dispatched from the same origin. The number of call must not"]
                #[doc = "  exceed the constant: `batched_calls_limit` (available in constant metadata)."]
                #[doc = ""]
                #[doc = "If origin is root then call are dispatch without checking origin filter. (This includes"]
                #[doc = "bypassing `frame_system::Config::BaseCallFilter`)."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- Complexity: O(C) where C is the number of calls to be batched."]
                #[doc = "# </weight>"]
                pub fn force_batch(
                    &self,
                    calls: ::std::vec::Vec<runtime_types::encointer_runtime::Call>,
                ) -> ::subxt::tx::StaticTxPayload<ForceBatch> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Utility",
                        "force_batch",
                        ForceBatch { calls },
                        [
                            184u8, 22u8, 122u8, 129u8, 55u8, 192u8, 255u8, 170u8, 206u8, 194u8,
                            244u8, 9u8, 148u8, 241u8, 12u8, 212u8, 57u8, 210u8, 85u8, 161u8, 131u8,
                            56u8, 35u8, 93u8, 141u8, 109u8, 226u8, 178u8, 243u8, 57u8, 171u8,
                            148u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_utility::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Batch of dispatches did not complete fully. Index of first failing dispatch given, as"]
            #[doc = "well as the error."]
            pub struct BatchInterrupted {
                pub index: ::core::primitive::u32,
                pub error: runtime_types::sp_runtime::DispatchError,
            }
            impl ::subxt::events::StaticEvent for BatchInterrupted {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "BatchInterrupted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Batch of dispatches completed fully with no error."]
            pub struct BatchCompleted;
            impl ::subxt::events::StaticEvent for BatchCompleted {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "BatchCompleted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Batch of dispatches completed but has errors."]
            pub struct BatchCompletedWithErrors;
            impl ::subxt::events::StaticEvent for BatchCompletedWithErrors {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "BatchCompletedWithErrors";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A single item within a Batch of dispatches has completed with no error."]
            pub struct ItemCompleted;
            impl ::subxt::events::StaticEvent for ItemCompleted {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "ItemCompleted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A single item within a Batch of dispatches has completed with error."]
            pub struct ItemFailed {
                pub error: runtime_types::sp_runtime::DispatchError,
            }
            impl ::subxt::events::StaticEvent for ItemFailed {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "ItemFailed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A call was dispatched."]
            pub struct DispatchedAs {
                pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::events::StaticEvent for DispatchedAs {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "DispatchedAs";
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The limit on the number of batched calls."]
                pub fn batched_calls_limit(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Utility",
                        "batched_calls_limit",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod treasury {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ProposeSpend {
                #[codec(compact)]
                pub value: ::core::primitive::u128,
                pub beneficiary: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct RejectProposal {
                #[codec(compact)]
                pub proposal_id: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ApproveProposal {
                #[codec(compact)]
                pub proposal_id: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Spend {
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
                pub beneficiary: ::subxt::ext::sp_runtime::MultiAddress<
                    ::subxt::ext::sp_core::crypto::AccountId32,
                    (),
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct RemoveApproval {
                #[codec(compact)]
                pub proposal_id: ::core::primitive::u32,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Put forward a suggestion for spending. A deposit proportional to the value"]
                #[doc = "is reserved and slashed if the proposal is rejected. It is returned once the"]
                #[doc = "proposal is awarded."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- Complexity: O(1)"]
                #[doc = "- DbReads: `ProposalCount`, `origin account`"]
                #[doc = "- DbWrites: `ProposalCount`, `Proposals`, `origin account`"]
                #[doc = "# </weight>"]
                pub fn propose_spend(
                    &self,
                    value: ::core::primitive::u128,
                    beneficiary: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::tx::StaticTxPayload<ProposeSpend> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Treasury",
                        "propose_spend",
                        ProposeSpend { value, beneficiary },
                        [
                            109u8, 46u8, 8u8, 159u8, 127u8, 79u8, 27u8, 100u8, 92u8, 244u8, 78u8,
                            46u8, 105u8, 246u8, 169u8, 210u8, 149u8, 7u8, 108u8, 153u8, 203u8,
                            223u8, 8u8, 117u8, 126u8, 250u8, 255u8, 52u8, 245u8, 69u8, 45u8, 136u8,
                        ],
                    )
                }
                #[doc = "Reject a proposed spend. The original deposit will be slashed."]
                #[doc = ""]
                #[doc = "May only be called from `T::RejectOrigin`."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- Complexity: O(1)"]
                #[doc = "- DbReads: `Proposals`, `rejected proposer account`"]
                #[doc = "- DbWrites: `Proposals`, `rejected proposer account`"]
                #[doc = "# </weight>"]
                pub fn reject_proposal(
                    &self,
                    proposal_id: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<RejectProposal> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Treasury",
                        "reject_proposal",
                        RejectProposal { proposal_id },
                        [
                            106u8, 223u8, 97u8, 22u8, 111u8, 208u8, 128u8, 26u8, 198u8, 140u8,
                            118u8, 126u8, 187u8, 51u8, 193u8, 50u8, 193u8, 68u8, 143u8, 144u8,
                            34u8, 132u8, 44u8, 244u8, 105u8, 186u8, 223u8, 234u8, 17u8, 145u8,
                            209u8, 145u8,
                        ],
                    )
                }
                #[doc = "Approve a proposal. At a later time, the proposal will be allocated to the beneficiary"]
                #[doc = "and the original deposit will be returned."]
                #[doc = ""]
                #[doc = "May only be called from `T::ApproveOrigin`."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- Complexity: O(1)."]
                #[doc = "- DbReads: `Proposals`, `Approvals`"]
                #[doc = "- DbWrite: `Approvals`"]
                #[doc = "# </weight>"]
                pub fn approve_proposal(
                    &self,
                    proposal_id: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<ApproveProposal> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Treasury",
                        "approve_proposal",
                        ApproveProposal { proposal_id },
                        [
                            164u8, 229u8, 172u8, 98u8, 129u8, 62u8, 84u8, 128u8, 47u8, 108u8, 33u8,
                            120u8, 89u8, 79u8, 57u8, 121u8, 4u8, 197u8, 170u8, 153u8, 156u8, 17u8,
                            59u8, 164u8, 123u8, 227u8, 175u8, 195u8, 220u8, 160u8, 60u8, 186u8,
                        ],
                    )
                }
                #[doc = "Propose and approve a spend of treasury funds."]
                #[doc = ""]
                #[doc = "- `origin`: Must be `SpendOrigin` with the `Success` value being at least `amount`."]
                #[doc = "- `amount`: The amount to be transferred from the treasury to the `beneficiary`."]
                #[doc = "- `beneficiary`: The destination account for the transfer."]
                #[doc = ""]
                #[doc = "NOTE: For record-keeping purposes, the proposer is deemed to be equivalent to the"]
                #[doc = "beneficiary."]
                pub fn spend(
                    &self,
                    amount: ::core::primitive::u128,
                    beneficiary: ::subxt::ext::sp_runtime::MultiAddress<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::tx::StaticTxPayload<Spend> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Treasury",
                        "spend",
                        Spend {
                            amount,
                            beneficiary,
                        },
                        [
                            177u8, 178u8, 242u8, 136u8, 135u8, 237u8, 114u8, 71u8, 233u8, 239u8,
                            7u8, 84u8, 14u8, 228u8, 58u8, 31u8, 158u8, 185u8, 25u8, 91u8, 70u8,
                            33u8, 19u8, 92u8, 100u8, 162u8, 5u8, 48u8, 20u8, 120u8, 9u8, 109u8,
                        ],
                    )
                }
                #[doc = "Force a previously approved proposal to be removed from the approval queue."]
                #[doc = "The original deposit will no longer be returned."]
                #[doc = ""]
                #[doc = "May only be called from `T::RejectOrigin`."]
                #[doc = "- `proposal_id`: The index of a proposal"]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- Complexity: O(A) where `A` is the number of approvals"]
                #[doc = "- Db reads and writes: `Approvals`"]
                #[doc = "# </weight>"]
                #[doc = ""]
                #[doc = "Errors:"]
                #[doc = "- `ProposalNotApproved`: The `proposal_id` supplied was not found in the approval queue,"]
                #[doc = "i.e., the proposal has not been approved. This could also mean the proposal does not"]
                #[doc = "exist altogether, thus there is no way it would have been approved in the first place."]
                pub fn remove_approval(
                    &self,
                    proposal_id: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<RemoveApproval> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Treasury",
                        "remove_approval",
                        RemoveApproval { proposal_id },
                        [
                            133u8, 126u8, 181u8, 47u8, 196u8, 243u8, 7u8, 46u8, 25u8, 251u8, 154u8,
                            125u8, 217u8, 77u8, 54u8, 245u8, 240u8, 180u8, 97u8, 34u8, 186u8, 53u8,
                            225u8, 144u8, 155u8, 107u8, 172u8, 54u8, 250u8, 184u8, 178u8, 86u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_treasury::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            #[doc = "New proposal."]
            pub struct Proposed {
                pub proposal_index: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for Proposed {
                const PALLET: &'static str = "Treasury";
                const EVENT: &'static str = "Proposed";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            #[doc = "We have ended a spend period and will now allocate funds."]
            pub struct Spending {
                pub budget_remaining: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Spending {
                const PALLET: &'static str = "Treasury";
                const EVENT: &'static str = "Spending";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Some funds have been allocated."]
            pub struct Awarded {
                pub proposal_index: ::core::primitive::u32,
                pub award: ::core::primitive::u128,
                pub account: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            impl ::subxt::events::StaticEvent for Awarded {
                const PALLET: &'static str = "Treasury";
                const EVENT: &'static str = "Awarded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A proposal was rejected; funds were slashed."]
            pub struct Rejected {
                pub proposal_index: ::core::primitive::u32,
                pub slashed: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Rejected {
                const PALLET: &'static str = "Treasury";
                const EVENT: &'static str = "Rejected";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            #[doc = "Some of our funds have been burnt."]
            pub struct Burnt {
                pub burnt_funds: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Burnt {
                const PALLET: &'static str = "Treasury";
                const EVENT: &'static str = "Burnt";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            #[doc = "Spending has finished; this is the amount that rolls over until next spend."]
            pub struct Rollover {
                pub rollover_balance: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Rollover {
                const PALLET: &'static str = "Treasury";
                const EVENT: &'static str = "Rollover";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            #[doc = "Some funds have been deposited."]
            pub struct Deposit {
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Deposit {
                const PALLET: &'static str = "Treasury";
                const EVENT: &'static str = "Deposit";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A new spend proposal has been approved."]
            pub struct SpendApproved {
                pub proposal_index: ::core::primitive::u32,
                pub amount: ::core::primitive::u128,
                pub beneficiary: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            impl ::subxt::events::StaticEvent for SpendApproved {
                const PALLET: &'static str = "Treasury";
                const EVENT: &'static str = "SpendApproved";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Number of proposals that have been made."]
                pub fn proposal_count(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Treasury",
                        "ProposalCount",
                        vec![],
                        [
                            132u8, 145u8, 78u8, 218u8, 51u8, 189u8, 55u8, 172u8, 143u8, 33u8,
                            140u8, 99u8, 124u8, 208u8, 57u8, 232u8, 154u8, 110u8, 32u8, 142u8,
                            24u8, 149u8, 109u8, 105u8, 30u8, 83u8, 39u8, 177u8, 127u8, 160u8, 34u8,
                            70u8,
                        ],
                    )
                }
                #[doc = " Proposals that have been made."]
                pub fn proposals(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_treasury::Proposal<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Treasury",
                        "Proposals",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Twox64Concat,
                        )],
                        [
                            62u8, 223u8, 55u8, 209u8, 151u8, 134u8, 122u8, 65u8, 207u8, 38u8,
                            113u8, 213u8, 237u8, 48u8, 129u8, 32u8, 91u8, 228u8, 108u8, 91u8, 37u8,
                            49u8, 94u8, 4u8, 75u8, 122u8, 25u8, 34u8, 198u8, 224u8, 246u8, 160u8,
                        ],
                    )
                }
                #[doc = " Proposals that have been made."]
                pub fn proposals_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_treasury::Proposal<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Treasury",
                        "Proposals",
                        Vec::new(),
                        [
                            62u8, 223u8, 55u8, 209u8, 151u8, 134u8, 122u8, 65u8, 207u8, 38u8,
                            113u8, 213u8, 237u8, 48u8, 129u8, 32u8, 91u8, 228u8, 108u8, 91u8, 37u8,
                            49u8, 94u8, 4u8, 75u8, 122u8, 25u8, 34u8, 198u8, 224u8, 246u8, 160u8,
                        ],
                    )
                }
                #[doc = " Proposal indices that have been approved but not yet awarded."]
                pub fn approvals(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<
                            ::core::primitive::u32,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Treasury",
                        "Approvals",
                        vec![],
                        [
                            202u8, 106u8, 189u8, 40u8, 127u8, 172u8, 108u8, 50u8, 193u8, 4u8,
                            248u8, 226u8, 176u8, 101u8, 212u8, 222u8, 64u8, 206u8, 244u8, 175u8,
                            111u8, 106u8, 86u8, 96u8, 19u8, 109u8, 218u8, 152u8, 30u8, 59u8, 96u8,
                            1u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " Fraction of a proposal's value that should be bonded in order to place the proposal."]
                #[doc = " An accepted proposal gets these back. A rejected proposal does not."]
                pub fn proposal_bond(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::sp_arithmetic::per_things::Permill,
                    >,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Treasury",
                        "ProposalBond",
                        [
                            225u8, 236u8, 95u8, 157u8, 90u8, 94u8, 106u8, 192u8, 254u8, 19u8, 87u8,
                            80u8, 16u8, 62u8, 42u8, 204u8, 136u8, 106u8, 225u8, 53u8, 212u8, 52u8,
                            177u8, 79u8, 4u8, 116u8, 201u8, 104u8, 222u8, 75u8, 86u8, 227u8,
                        ],
                    )
                }
                #[doc = " Minimum amount of funds that should be placed in a deposit for making a proposal."]
                pub fn proposal_bond_minimum(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Treasury",
                        "ProposalBondMinimum",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                #[doc = " Maximum amount of funds that should be placed in a deposit for making a proposal."]
                pub fn proposal_bond_maximum(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::core::option::Option<::core::primitive::u128>,
                    >,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Treasury",
                        "ProposalBondMaximum",
                        [
                            84u8, 154u8, 218u8, 83u8, 84u8, 189u8, 32u8, 20u8, 120u8, 194u8, 88u8,
                            205u8, 109u8, 216u8, 114u8, 193u8, 120u8, 198u8, 154u8, 237u8, 134u8,
                            204u8, 102u8, 247u8, 52u8, 103u8, 231u8, 43u8, 243u8, 122u8, 60u8,
                            216u8,
                        ],
                    )
                }
                #[doc = " Period between successive spends."]
                pub fn spend_period(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Treasury",
                        "SpendPeriod",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " Percentage of spare funds (if any) that are burnt per spend period."]
                pub fn burn(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::sp_arithmetic::per_things::Permill,
                    >,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Treasury",
                        "Burn",
                        [
                            225u8, 236u8, 95u8, 157u8, 90u8, 94u8, 106u8, 192u8, 254u8, 19u8, 87u8,
                            80u8, 16u8, 62u8, 42u8, 204u8, 136u8, 106u8, 225u8, 53u8, 212u8, 52u8,
                            177u8, 79u8, 4u8, 116u8, 201u8, 104u8, 222u8, 75u8, 86u8, 227u8,
                        ],
                    )
                }
                #[doc = " The treasury's pallet id, used for deriving its sovereign account ID."]
                pub fn pallet_id(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<runtime_types::frame_support::PalletId>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Treasury",
                        "PalletId",
                        [
                            139u8, 109u8, 228u8, 151u8, 252u8, 32u8, 130u8, 69u8, 112u8, 154u8,
                            174u8, 45u8, 83u8, 245u8, 51u8, 132u8, 173u8, 5u8, 186u8, 24u8, 243u8,
                            9u8, 12u8, 214u8, 80u8, 74u8, 69u8, 189u8, 30u8, 94u8, 22u8, 39u8,
                        ],
                    )
                }
                #[doc = " The maximum number of approvals that can wait in the spending queue."]
                #[doc = ""]
                #[doc = " NOTE: This parameter is also used within the Bounties Pallet extension if enabled."]
                pub fn max_approvals(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Treasury",
                        "MaxApprovals",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod proxy {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Proxy {
                pub real: ::subxt::ext::sp_core::crypto::AccountId32,
                pub force_proxy_type:
                    ::core::option::Option<runtime_types::encointer_runtime::ProxyType>,
                pub call: ::std::boxed::Box<runtime_types::encointer_runtime::Call>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct AddProxy {
                pub delegate: ::subxt::ext::sp_core::crypto::AccountId32,
                pub proxy_type: runtime_types::encointer_runtime::ProxyType,
                pub delay: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct RemoveProxy {
                pub delegate: ::subxt::ext::sp_core::crypto::AccountId32,
                pub proxy_type: runtime_types::encointer_runtime::ProxyType,
                pub delay: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct RemoveProxies;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Anonymous {
                pub proxy_type: runtime_types::encointer_runtime::ProxyType,
                pub delay: ::core::primitive::u32,
                pub index: ::core::primitive::u16,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct KillAnonymous {
                pub spawner: ::subxt::ext::sp_core::crypto::AccountId32,
                pub proxy_type: runtime_types::encointer_runtime::ProxyType,
                pub index: ::core::primitive::u16,
                #[codec(compact)]
                pub height: ::core::primitive::u32,
                #[codec(compact)]
                pub ext_index: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Announce {
                pub real: ::subxt::ext::sp_core::crypto::AccountId32,
                pub call_hash: ::subxt::ext::sp_core::H256,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct RemoveAnnouncement {
                pub real: ::subxt::ext::sp_core::crypto::AccountId32,
                pub call_hash: ::subxt::ext::sp_core::H256,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct RejectAnnouncement {
                pub delegate: ::subxt::ext::sp_core::crypto::AccountId32,
                pub call_hash: ::subxt::ext::sp_core::H256,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ProxyAnnounced {
                pub delegate: ::subxt::ext::sp_core::crypto::AccountId32,
                pub real: ::subxt::ext::sp_core::crypto::AccountId32,
                pub force_proxy_type:
                    ::core::option::Option<runtime_types::encointer_runtime::ProxyType>,
                pub call: ::std::boxed::Box<runtime_types::encointer_runtime::Call>,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Dispatch the given `call` from an account that the sender is authorised for through"]
                #[doc = "`add_proxy`."]
                #[doc = ""]
                #[doc = "Removes any corresponding announcement(s)."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "Parameters:"]
                #[doc = "- `real`: The account that the proxy will make a call on behalf of."]
                #[doc = "- `force_proxy_type`: Specify the exact proxy type to be used and checked for this call."]
                #[doc = "- `call`: The call to be made by the `real` account."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "Weight is a function of the number of proxies the user has (P)."]
                #[doc = "# </weight>"]
                pub fn proxy(
                    &self,
                    real: ::subxt::ext::sp_core::crypto::AccountId32,
                    force_proxy_type: ::core::option::Option<
                        runtime_types::encointer_runtime::ProxyType,
                    >,
                    call: runtime_types::encointer_runtime::Call,
                ) -> ::subxt::tx::StaticTxPayload<Proxy> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Proxy",
                        "proxy",
                        Proxy {
                            real,
                            force_proxy_type,
                            call: ::std::boxed::Box::new(call),
                        },
                        [
                            225u8, 186u8, 151u8, 89u8, 217u8, 214u8, 52u8, 232u8, 69u8, 90u8,
                            161u8, 56u8, 37u8, 216u8, 75u8, 90u8, 42u8, 47u8, 72u8, 73u8, 154u8,
                            135u8, 152u8, 33u8, 132u8, 142u8, 252u8, 217u8, 238u8, 217u8, 14u8,
                            207u8,
                        ],
                    )
                }
                #[doc = "Register a proxy account for the sender that is able to make calls on its behalf."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "Parameters:"]
                #[doc = "- `proxy`: The account that the `caller` would like to make a proxy."]
                #[doc = "- `proxy_type`: The permissions allowed for this proxy account."]
                #[doc = "- `delay`: The announcement period required of the initial proxy. Will generally be"]
                #[doc = "zero."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "Weight is a function of the number of proxies the user has (P)."]
                #[doc = "# </weight>"]
                pub fn add_proxy(
                    &self,
                    delegate: ::subxt::ext::sp_core::crypto::AccountId32,
                    proxy_type: runtime_types::encointer_runtime::ProxyType,
                    delay: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<AddProxy> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Proxy",
                        "add_proxy",
                        AddProxy {
                            delegate,
                            proxy_type,
                            delay,
                        },
                        [
                            77u8, 114u8, 88u8, 104u8, 209u8, 170u8, 131u8, 178u8, 233u8, 89u8,
                            109u8, 154u8, 15u8, 237u8, 51u8, 81u8, 34u8, 163u8, 20u8, 64u8, 248u8,
                            193u8, 92u8, 29u8, 243u8, 36u8, 38u8, 110u8, 125u8, 112u8, 249u8,
                            211u8,
                        ],
                    )
                }
                #[doc = "Unregister a proxy account for the sender."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "Parameters:"]
                #[doc = "- `proxy`: The account that the `caller` would like to remove as a proxy."]
                #[doc = "- `proxy_type`: The permissions currently enabled for the removed proxy account."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "Weight is a function of the number of proxies the user has (P)."]
                #[doc = "# </weight>"]
                pub fn remove_proxy(
                    &self,
                    delegate: ::subxt::ext::sp_core::crypto::AccountId32,
                    proxy_type: runtime_types::encointer_runtime::ProxyType,
                    delay: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<RemoveProxy> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Proxy",
                        "remove_proxy",
                        RemoveProxy {
                            delegate,
                            proxy_type,
                            delay,
                        },
                        [
                            85u8, 22u8, 210u8, 77u8, 219u8, 230u8, 148u8, 124u8, 108u8, 184u8,
                            251u8, 158u8, 58u8, 75u8, 232u8, 8u8, 28u8, 167u8, 80u8, 161u8, 240u8,
                            120u8, 54u8, 113u8, 80u8, 191u8, 200u8, 99u8, 177u8, 225u8, 72u8, 61u8,
                        ],
                    )
                }
                #[doc = "Unregister all proxy accounts for the sender."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "WARNING: This may be called on accounts created by `anonymous`, however if done, then"]
                #[doc = "the unreserved fees will be inaccessible. **All access to this account will be lost.**"]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "Weight is a function of the number of proxies the user has (P)."]
                #[doc = "# </weight>"]
                pub fn remove_proxies(&self) -> ::subxt::tx::StaticTxPayload<RemoveProxies> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Proxy",
                        "remove_proxies",
                        RemoveProxies {},
                        [
                            15u8, 237u8, 27u8, 166u8, 254u8, 218u8, 92u8, 5u8, 213u8, 239u8, 99u8,
                            59u8, 1u8, 26u8, 73u8, 252u8, 81u8, 94u8, 214u8, 227u8, 169u8, 58u8,
                            40u8, 253u8, 187u8, 225u8, 192u8, 26u8, 19u8, 23u8, 121u8, 129u8,
                        ],
                    )
                }
                #[doc = "Spawn a fresh new account that is guaranteed to be otherwise inaccessible, and"]
                #[doc = "initialize it with a proxy of `proxy_type` for `origin` sender."]
                #[doc = ""]
                #[doc = "Requires a `Signed` origin."]
                #[doc = ""]
                #[doc = "- `proxy_type`: The type of the proxy that the sender will be registered as over the"]
                #[doc = "new account. This will almost always be the most permissive `ProxyType` possible to"]
                #[doc = "allow for maximum flexibility."]
                #[doc = "- `index`: A disambiguation index, in case this is called multiple times in the same"]
                #[doc = "transaction (e.g. with `utility::batch`). Unless you're using `batch` you probably just"]
                #[doc = "want to use `0`."]
                #[doc = "- `delay`: The announcement period required of the initial proxy. Will generally be"]
                #[doc = "zero."]
                #[doc = ""]
                #[doc = "Fails with `Duplicate` if this has already been called in this transaction, from the"]
                #[doc = "same sender, with the same parameters."]
                #[doc = ""]
                #[doc = "Fails if there are insufficient funds to pay for deposit."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "Weight is a function of the number of proxies the user has (P)."]
                #[doc = "# </weight>"]
                #[doc = "TODO: Might be over counting 1 read"]
                pub fn anonymous(
                    &self,
                    proxy_type: runtime_types::encointer_runtime::ProxyType,
                    delay: ::core::primitive::u32,
                    index: ::core::primitive::u16,
                ) -> ::subxt::tx::StaticTxPayload<Anonymous> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Proxy",
                        "anonymous",
                        Anonymous {
                            proxy_type,
                            delay,
                            index,
                        },
                        [
                            172u8, 157u8, 251u8, 88u8, 238u8, 36u8, 136u8, 254u8, 113u8, 51u8,
                            47u8, 20u8, 18u8, 156u8, 24u8, 231u8, 44u8, 26u8, 44u8, 136u8, 102u8,
                            51u8, 187u8, 245u8, 73u8, 74u8, 97u8, 43u8, 239u8, 230u8, 13u8, 155u8,
                        ],
                    )
                }
                #[doc = "Removes a previously spawned anonymous proxy."]
                #[doc = ""]
                #[doc = "WARNING: **All access to this account will be lost.** Any funds held in it will be"]
                #[doc = "inaccessible."]
                #[doc = ""]
                #[doc = "Requires a `Signed` origin, and the sender account must have been created by a call to"]
                #[doc = "`anonymous` with corresponding parameters."]
                #[doc = ""]
                #[doc = "- `spawner`: The account that originally called `anonymous` to create this account."]
                #[doc = "- `index`: The disambiguation index originally passed to `anonymous`. Probably `0`."]
                #[doc = "- `proxy_type`: The proxy type originally passed to `anonymous`."]
                #[doc = "- `height`: The height of the chain when the call to `anonymous` was processed."]
                #[doc = "- `ext_index`: The extrinsic index in which the call to `anonymous` was processed."]
                #[doc = ""]
                #[doc = "Fails with `NoPermission` in case the caller is not a previously created anonymous"]
                #[doc = "account whose `anonymous` call has corresponding parameters."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "Weight is a function of the number of proxies the user has (P)."]
                #[doc = "# </weight>"]
                pub fn kill_anonymous(
                    &self,
                    spawner: ::subxt::ext::sp_core::crypto::AccountId32,
                    proxy_type: runtime_types::encointer_runtime::ProxyType,
                    index: ::core::primitive::u16,
                    height: ::core::primitive::u32,
                    ext_index: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<KillAnonymous> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Proxy",
                        "kill_anonymous",
                        KillAnonymous {
                            spawner,
                            proxy_type,
                            index,
                            height,
                            ext_index,
                        },
                        [
                            245u8, 128u8, 201u8, 84u8, 151u8, 170u8, 203u8, 109u8, 14u8, 155u8,
                            45u8, 112u8, 173u8, 146u8, 141u8, 236u8, 22u8, 183u8, 189u8, 214u8,
                            200u8, 136u8, 123u8, 33u8, 62u8, 12u8, 97u8, 197u8, 92u8, 234u8, 208u8,
                            241u8,
                        ],
                    )
                }
                #[doc = "Publish the hash of a proxy-call that will be made in the future."]
                #[doc = ""]
                #[doc = "This must be called some number of blocks before the corresponding `proxy` is attempted"]
                #[doc = "if the delay associated with the proxy relationship is greater than zero."]
                #[doc = ""]
                #[doc = "No more than `MaxPending` announcements may be made at any one time."]
                #[doc = ""]
                #[doc = "This will take a deposit of `AnnouncementDepositFactor` as well as"]
                #[doc = "`AnnouncementDepositBase` if there are no other pending announcements."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_ and a proxy of `real`."]
                #[doc = ""]
                #[doc = "Parameters:"]
                #[doc = "- `real`: The account that the proxy will make a call on behalf of."]
                #[doc = "- `call_hash`: The hash of the call to be made by the `real` account."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "Weight is a function of:"]
                #[doc = "- A: the number of announcements made."]
                #[doc = "- P: the number of proxies the user has."]
                #[doc = "# </weight>"]
                pub fn announce(
                    &self,
                    real: ::subxt::ext::sp_core::crypto::AccountId32,
                    call_hash: ::subxt::ext::sp_core::H256,
                ) -> ::subxt::tx::StaticTxPayload<Announce> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Proxy",
                        "announce",
                        Announce { real, call_hash },
                        [
                            99u8, 237u8, 158u8, 131u8, 185u8, 119u8, 88u8, 167u8, 253u8, 29u8,
                            82u8, 216u8, 225u8, 33u8, 181u8, 244u8, 85u8, 176u8, 106u8, 66u8,
                            166u8, 174u8, 218u8, 98u8, 119u8, 86u8, 218u8, 89u8, 150u8, 255u8,
                            86u8, 40u8,
                        ],
                    )
                }
                #[doc = "Remove a given announcement."]
                #[doc = ""]
                #[doc = "May be called by a proxy account to remove a call they previously announced and return"]
                #[doc = "the deposit."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "Parameters:"]
                #[doc = "- `real`: The account that the proxy will make a call on behalf of."]
                #[doc = "- `call_hash`: The hash of the call to be made by the `real` account."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "Weight is a function of:"]
                #[doc = "- A: the number of announcements made."]
                #[doc = "- P: the number of proxies the user has."]
                #[doc = "# </weight>"]
                pub fn remove_announcement(
                    &self,
                    real: ::subxt::ext::sp_core::crypto::AccountId32,
                    call_hash: ::subxt::ext::sp_core::H256,
                ) -> ::subxt::tx::StaticTxPayload<RemoveAnnouncement> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Proxy",
                        "remove_announcement",
                        RemoveAnnouncement { real, call_hash },
                        [
                            197u8, 54u8, 240u8, 51u8, 65u8, 218u8, 154u8, 165u8, 24u8, 54u8, 157u8,
                            30u8, 144u8, 22u8, 247u8, 177u8, 105u8, 38u8, 9u8, 25u8, 127u8, 36u8,
                            97u8, 84u8, 18u8, 3u8, 246u8, 238u8, 60u8, 17u8, 236u8, 69u8,
                        ],
                    )
                }
                #[doc = "Remove the given announcement of a delegate."]
                #[doc = ""]
                #[doc = "May be called by a target (proxied) account to remove a call that one of their delegates"]
                #[doc = "(`delegate`) has announced they want to execute. The deposit is returned."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "Parameters:"]
                #[doc = "- `delegate`: The account that previously announced the call."]
                #[doc = "- `call_hash`: The hash of the call to be made."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "Weight is a function of:"]
                #[doc = "- A: the number of announcements made."]
                #[doc = "- P: the number of proxies the user has."]
                #[doc = "# </weight>"]
                pub fn reject_announcement(
                    &self,
                    delegate: ::subxt::ext::sp_core::crypto::AccountId32,
                    call_hash: ::subxt::ext::sp_core::H256,
                ) -> ::subxt::tx::StaticTxPayload<RejectAnnouncement> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Proxy",
                        "reject_announcement",
                        RejectAnnouncement {
                            delegate,
                            call_hash,
                        },
                        [
                            205u8, 123u8, 102u8, 30u8, 196u8, 250u8, 247u8, 50u8, 243u8, 55u8,
                            67u8, 66u8, 160u8, 147u8, 92u8, 204u8, 75u8, 69u8, 68u8, 140u8, 40u8,
                            250u8, 53u8, 203u8, 228u8, 239u8, 62u8, 66u8, 254u8, 30u8, 126u8,
                            206u8,
                        ],
                    )
                }
                #[doc = "Dispatch the given `call` from an account that the sender is authorized for through"]
                #[doc = "`add_proxy`."]
                #[doc = ""]
                #[doc = "Removes any corresponding announcement(s)."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "Parameters:"]
                #[doc = "- `real`: The account that the proxy will make a call on behalf of."]
                #[doc = "- `force_proxy_type`: Specify the exact proxy type to be used and checked for this call."]
                #[doc = "- `call`: The call to be made by the `real` account."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "Weight is a function of:"]
                #[doc = "- A: the number of announcements made."]
                #[doc = "- P: the number of proxies the user has."]
                #[doc = "# </weight>"]
                pub fn proxy_announced(
                    &self,
                    delegate: ::subxt::ext::sp_core::crypto::AccountId32,
                    real: ::subxt::ext::sp_core::crypto::AccountId32,
                    force_proxy_type: ::core::option::Option<
                        runtime_types::encointer_runtime::ProxyType,
                    >,
                    call: runtime_types::encointer_runtime::Call,
                ) -> ::subxt::tx::StaticTxPayload<ProxyAnnounced> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Proxy",
                        "proxy_announced",
                        ProxyAnnounced {
                            delegate,
                            real,
                            force_proxy_type,
                            call: ::std::boxed::Box::new(call),
                        },
                        [
                            39u8, 208u8, 204u8, 67u8, 162u8, 240u8, 69u8, 106u8, 2u8, 150u8, 100u8,
                            192u8, 48u8, 147u8, 32u8, 246u8, 135u8, 137u8, 85u8, 61u8, 94u8, 142u8,
                            30u8, 236u8, 158u8, 84u8, 1u8, 134u8, 234u8, 80u8, 143u8, 68u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_proxy::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A proxy was executed correctly, with the given."]
            pub struct ProxyExecuted {
                pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::events::StaticEvent for ProxyExecuted {
                const PALLET: &'static str = "Proxy";
                const EVENT: &'static str = "ProxyExecuted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Anonymous account has been created by new proxy with given"]
            #[doc = "disambiguation index and proxy type."]
            pub struct AnonymousCreated {
                pub anonymous: ::subxt::ext::sp_core::crypto::AccountId32,
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
                pub proxy_type: runtime_types::encointer_runtime::ProxyType,
                pub disambiguation_index: ::core::primitive::u16,
            }
            impl ::subxt::events::StaticEvent for AnonymousCreated {
                const PALLET: &'static str = "Proxy";
                const EVENT: &'static str = "AnonymousCreated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "An announcement was placed to make a call in the future."]
            pub struct Announced {
                pub real: ::subxt::ext::sp_core::crypto::AccountId32,
                pub proxy: ::subxt::ext::sp_core::crypto::AccountId32,
                pub call_hash: ::subxt::ext::sp_core::H256,
            }
            impl ::subxt::events::StaticEvent for Announced {
                const PALLET: &'static str = "Proxy";
                const EVENT: &'static str = "Announced";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A proxy was added."]
            pub struct ProxyAdded {
                pub delegator: ::subxt::ext::sp_core::crypto::AccountId32,
                pub delegatee: ::subxt::ext::sp_core::crypto::AccountId32,
                pub proxy_type: runtime_types::encointer_runtime::ProxyType,
                pub delay: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for ProxyAdded {
                const PALLET: &'static str = "Proxy";
                const EVENT: &'static str = "ProxyAdded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A proxy was removed."]
            pub struct ProxyRemoved {
                pub delegator: ::subxt::ext::sp_core::crypto::AccountId32,
                pub delegatee: ::subxt::ext::sp_core::crypto::AccountId32,
                pub proxy_type: runtime_types::encointer_runtime::ProxyType,
                pub delay: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for ProxyRemoved {
                const PALLET: &'static str = "Proxy";
                const EVENT: &'static str = "ProxyRemoved";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The set of account proxies. Maps the account which has delegated to the accounts"]
                #[doc = " which are being delegated to, together with the amount held on deposit."]
                pub fn proxies(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<(
                        runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<
                            runtime_types::pallet_proxy::ProxyDefinition<
                                ::subxt::ext::sp_core::crypto::AccountId32,
                                runtime_types::encointer_runtime::ProxyType,
                                ::core::primitive::u32,
                            >,
                        >,
                        ::core::primitive::u128,
                    )>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Proxy",
                        "Proxies",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Twox64Concat,
                        )],
                        [
                            60u8, 251u8, 35u8, 25u8, 17u8, 253u8, 125u8, 28u8, 160u8, 153u8, 218u8,
                            21u8, 96u8, 240u8, 187u8, 70u8, 103u8, 111u8, 77u8, 215u8, 156u8,
                            221u8, 223u8, 167u8, 172u8, 96u8, 200u8, 114u8, 192u8, 137u8, 83u8,
                            106u8,
                        ],
                    )
                }
                #[doc = " The set of account proxies. Maps the account which has delegated to the accounts"]
                #[doc = " which are being delegated to, together with the amount held on deposit."]
                pub fn proxies_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<(
                        runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<
                            runtime_types::pallet_proxy::ProxyDefinition<
                                ::subxt::ext::sp_core::crypto::AccountId32,
                                runtime_types::encointer_runtime::ProxyType,
                                ::core::primitive::u32,
                            >,
                        >,
                        ::core::primitive::u128,
                    )>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Proxy",
                        "Proxies",
                        Vec::new(),
                        [
                            60u8, 251u8, 35u8, 25u8, 17u8, 253u8, 125u8, 28u8, 160u8, 153u8, 218u8,
                            21u8, 96u8, 240u8, 187u8, 70u8, 103u8, 111u8, 77u8, 215u8, 156u8,
                            221u8, 223u8, 167u8, 172u8, 96u8, 200u8, 114u8, 192u8, 137u8, 83u8,
                            106u8,
                        ],
                    )
                }
                #[doc = " The announcements made by the proxy (key)."]
                pub fn announcements(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<(
                        runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<
                            runtime_types::pallet_proxy::Announcement<
                                ::subxt::ext::sp_core::crypto::AccountId32,
                                ::subxt::ext::sp_core::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        ::core::primitive::u128,
                    )>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Proxy",
                        "Announcements",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Twox64Concat,
                        )],
                        [
                            233u8, 38u8, 249u8, 89u8, 103u8, 87u8, 64u8, 52u8, 140u8, 228u8, 110u8,
                            37u8, 8u8, 92u8, 48u8, 7u8, 46u8, 99u8, 179u8, 83u8, 232u8, 171u8,
                            160u8, 45u8, 37u8, 23u8, 151u8, 198u8, 237u8, 103u8, 217u8, 53u8,
                        ],
                    )
                }
                #[doc = " The announcements made by the proxy (key)."]
                pub fn announcements_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<(
                        runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<
                            runtime_types::pallet_proxy::Announcement<
                                ::subxt::ext::sp_core::crypto::AccountId32,
                                ::subxt::ext::sp_core::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        ::core::primitive::u128,
                    )>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Proxy",
                        "Announcements",
                        Vec::new(),
                        [
                            233u8, 38u8, 249u8, 89u8, 103u8, 87u8, 64u8, 52u8, 140u8, 228u8, 110u8,
                            37u8, 8u8, 92u8, 48u8, 7u8, 46u8, 99u8, 179u8, 83u8, 232u8, 171u8,
                            160u8, 45u8, 37u8, 23u8, 151u8, 198u8, 237u8, 103u8, 217u8, 53u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The base amount of currency needed to reserve for creating a proxy."]
                #[doc = ""]
                #[doc = " This is held for an additional storage item whose value size is"]
                #[doc = " `sizeof(Balance)` bytes and whose key size is `sizeof(AccountId)` bytes."]
                pub fn proxy_deposit_base(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Proxy",
                        "ProxyDepositBase",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The amount of currency needed per proxy added."]
                #[doc = ""]
                #[doc = " This is held for adding 32 bytes plus an instance of `ProxyType` more into a"]
                #[doc = " pre-existing storage value. Thus, when configuring `ProxyDepositFactor` one should take"]
                #[doc = " into account `32 + proxy_type.encode().len()` bytes of data."]
                pub fn proxy_deposit_factor(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Proxy",
                        "ProxyDepositFactor",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The maximum amount of proxies allowed for a single account."]
                pub fn max_proxies(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Proxy",
                        "MaxProxies",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum amount of time-delayed announcements that are allowed to be pending."]
                pub fn max_pending(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Proxy",
                        "MaxPending",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The base amount of currency needed to reserve for creating an announcement."]
                #[doc = ""]
                #[doc = " This is held when a new storage item holding a `Balance` is created (typically 16"]
                #[doc = " bytes)."]
                pub fn announcement_deposit_base(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Proxy",
                        "AnnouncementDepositBase",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The amount of currency needed per announcement made."]
                #[doc = ""]
                #[doc = " This is held for adding an `AccountId`, `Hash` and `BlockNumber` (typically 68 bytes)"]
                #[doc = " into a pre-existing storage value."]
                pub fn announcement_deposit_factor(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Proxy",
                        "AnnouncementDepositFactor",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod scheduler {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Schedule {
                pub when: ::core::primitive::u32,
                pub maybe_periodic:
                    ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                pub priority: ::core::primitive::u8,
                pub call: ::std::boxed::Box<
                    runtime_types::frame_support::traits::schedule::MaybeHashed<
                        runtime_types::encointer_runtime::Call,
                        ::subxt::ext::sp_core::H256,
                    >,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Cancel {
                pub when: ::core::primitive::u32,
                pub index: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ScheduleNamed {
                pub id: ::std::vec::Vec<::core::primitive::u8>,
                pub when: ::core::primitive::u32,
                pub maybe_periodic:
                    ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                pub priority: ::core::primitive::u8,
                pub call: ::std::boxed::Box<
                    runtime_types::frame_support::traits::schedule::MaybeHashed<
                        runtime_types::encointer_runtime::Call,
                        ::subxt::ext::sp_core::H256,
                    >,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct CancelNamed {
                pub id: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ScheduleAfter {
                pub after: ::core::primitive::u32,
                pub maybe_periodic:
                    ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                pub priority: ::core::primitive::u8,
                pub call: ::std::boxed::Box<
                    runtime_types::frame_support::traits::schedule::MaybeHashed<
                        runtime_types::encointer_runtime::Call,
                        ::subxt::ext::sp_core::H256,
                    >,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ScheduleNamedAfter {
                pub id: ::std::vec::Vec<::core::primitive::u8>,
                pub after: ::core::primitive::u32,
                pub maybe_periodic:
                    ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                pub priority: ::core::primitive::u8,
                pub call: ::std::boxed::Box<
                    runtime_types::frame_support::traits::schedule::MaybeHashed<
                        runtime_types::encointer_runtime::Call,
                        ::subxt::ext::sp_core::H256,
                    >,
                >,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Anonymously schedule a task."]
                pub fn schedule(
                    &self,
                    when: ::core::primitive::u32,
                    maybe_periodic: ::core::option::Option<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    priority: ::core::primitive::u8,
                    call: runtime_types::frame_support::traits::schedule::MaybeHashed<
                        runtime_types::encointer_runtime::Call,
                        ::subxt::ext::sp_core::H256,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<Schedule> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Scheduler",
                        "schedule",
                        Schedule {
                            when,
                            maybe_periodic,
                            priority,
                            call: ::std::boxed::Box::new(call),
                        },
                        [
                            29u8, 153u8, 241u8, 227u8, 49u8, 185u8, 35u8, 116u8, 49u8, 27u8, 179u8,
                            94u8, 47u8, 185u8, 206u8, 19u8, 4u8, 67u8, 29u8, 211u8, 101u8, 249u8,
                            23u8, 45u8, 160u8, 173u8, 60u8, 61u8, 123u8, 20u8, 162u8, 34u8,
                        ],
                    )
                }
                #[doc = "Cancel an anonymously scheduled task."]
                pub fn cancel(
                    &self,
                    when: ::core::primitive::u32,
                    index: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<Cancel> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Scheduler",
                        "cancel",
                        Cancel { when, index },
                        [
                            81u8, 251u8, 234u8, 17u8, 214u8, 75u8, 19u8, 59u8, 19u8, 30u8, 89u8,
                            74u8, 6u8, 216u8, 238u8, 165u8, 7u8, 19u8, 153u8, 253u8, 161u8, 103u8,
                            178u8, 227u8, 152u8, 180u8, 80u8, 156u8, 82u8, 126u8, 132u8, 120u8,
                        ],
                    )
                }
                #[doc = "Schedule a named task."]
                pub fn schedule_named(
                    &self,
                    id: ::std::vec::Vec<::core::primitive::u8>,
                    when: ::core::primitive::u32,
                    maybe_periodic: ::core::option::Option<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    priority: ::core::primitive::u8,
                    call: runtime_types::frame_support::traits::schedule::MaybeHashed<
                        runtime_types::encointer_runtime::Call,
                        ::subxt::ext::sp_core::H256,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<ScheduleNamed> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Scheduler",
                        "schedule_named",
                        ScheduleNamed {
                            id,
                            when,
                            maybe_periodic,
                            priority,
                            call: ::std::boxed::Box::new(call),
                        },
                        [
                            250u8, 105u8, 207u8, 153u8, 231u8, 42u8, 198u8, 167u8, 84u8, 206u8,
                            45u8, 46u8, 41u8, 165u8, 7u8, 67u8, 3u8, 156u8, 199u8, 12u8, 2u8,
                            133u8, 192u8, 216u8, 102u8, 48u8, 214u8, 104u8, 202u8, 142u8, 133u8,
                            220u8,
                        ],
                    )
                }
                #[doc = "Cancel a named scheduled task."]
                pub fn cancel_named(
                    &self,
                    id: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<CancelNamed> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Scheduler",
                        "cancel_named",
                        CancelNamed { id },
                        [
                            42u8, 232u8, 92u8, 167u8, 113u8, 136u8, 7u8, 215u8, 88u8, 117u8, 74u8,
                            26u8, 225u8, 230u8, 244u8, 106u8, 150u8, 112u8, 46u8, 228u8, 96u8,
                            252u8, 78u8, 126u8, 39u8, 207u8, 36u8, 110u8, 83u8, 62u8, 84u8, 241u8,
                        ],
                    )
                }
                #[doc = "Anonymously schedule a task after a delay."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "Same as [`schedule`]."]
                #[doc = "# </weight>"]
                pub fn schedule_after(
                    &self,
                    after: ::core::primitive::u32,
                    maybe_periodic: ::core::option::Option<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    priority: ::core::primitive::u8,
                    call: runtime_types::frame_support::traits::schedule::MaybeHashed<
                        runtime_types::encointer_runtime::Call,
                        ::subxt::ext::sp_core::H256,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<ScheduleAfter> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Scheduler",
                        "schedule_after",
                        ScheduleAfter {
                            after,
                            maybe_periodic,
                            priority,
                            call: ::std::boxed::Box::new(call),
                        },
                        [
                            95u8, 16u8, 208u8, 96u8, 214u8, 20u8, 233u8, 150u8, 72u8, 124u8, 3u8,
                            181u8, 205u8, 95u8, 239u8, 84u8, 7u8, 39u8, 12u8, 8u8, 40u8, 141u8,
                            66u8, 237u8, 158u8, 86u8, 4u8, 54u8, 220u8, 29u8, 154u8, 195u8,
                        ],
                    )
                }
                #[doc = "Schedule a named task after a delay."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "Same as [`schedule_named`](Self::schedule_named)."]
                #[doc = "# </weight>"]
                pub fn schedule_named_after(
                    &self,
                    id: ::std::vec::Vec<::core::primitive::u8>,
                    after: ::core::primitive::u32,
                    maybe_periodic: ::core::option::Option<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    priority: ::core::primitive::u8,
                    call: runtime_types::frame_support::traits::schedule::MaybeHashed<
                        runtime_types::encointer_runtime::Call,
                        ::subxt::ext::sp_core::H256,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<ScheduleNamedAfter> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Scheduler",
                        "schedule_named_after",
                        ScheduleNamedAfter {
                            id,
                            after,
                            maybe_periodic,
                            priority,
                            call: ::std::boxed::Box::new(call),
                        },
                        [
                            88u8, 145u8, 60u8, 171u8, 183u8, 134u8, 41u8, 208u8, 21u8, 167u8,
                            101u8, 175u8, 174u8, 67u8, 29u8, 16u8, 248u8, 85u8, 67u8, 148u8, 175u8,
                            3u8, 86u8, 47u8, 245u8, 173u8, 124u8, 229u8, 230u8, 127u8, 146u8, 63u8,
                        ],
                    )
                }
            }
        }
        #[doc = "Events type."]
        pub type Event = runtime_types::pallet_scheduler::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Scheduled some task."]
            pub struct Scheduled {
                pub when: ::core::primitive::u32,
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for Scheduled {
                const PALLET: &'static str = "Scheduler";
                const EVENT: &'static str = "Scheduled";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Canceled some task."]
            pub struct Canceled {
                pub when: ::core::primitive::u32,
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for Canceled {
                const PALLET: &'static str = "Scheduler";
                const EVENT: &'static str = "Canceled";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Dispatched some task."]
            pub struct Dispatched {
                pub task: (::core::primitive::u32, ::core::primitive::u32),
                pub id: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::events::StaticEvent for Dispatched {
                const PALLET: &'static str = "Scheduler";
                const EVENT: &'static str = "Dispatched";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "The call for the provided hash was not found so the task has been aborted."]
            pub struct CallLookupFailed {
                pub task: (::core::primitive::u32, ::core::primitive::u32),
                pub id: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                pub error: runtime_types::frame_support::traits::schedule::LookupError,
            }
            impl ::subxt::events::StaticEvent for CallLookupFailed {
                const PALLET: &'static str = "Scheduler";
                const EVENT: &'static str = "CallLookupFailed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Items to be executed, indexed by the block number that they should be executed on."]
                pub fn agenda(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<
                            ::core::option::Option<
                                runtime_types::pallet_scheduler::ScheduledV3<
                                    runtime_types::frame_support::traits::schedule::MaybeHashed<
                                        runtime_types::encointer_runtime::Call,
                                        ::subxt::ext::sp_core::H256,
                                    >,
                                    ::core::primitive::u32,
                                    runtime_types::encointer_runtime::OriginCaller,
                                    ::subxt::ext::sp_core::crypto::AccountId32,
                                >,
                            >,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Scheduler",
                        "Agenda",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Twox64Concat,
                        )],
                        [
                            28u8, 250u8, 5u8, 109u8, 7u8, 184u8, 221u8, 16u8, 203u8, 110u8, 146u8,
                            151u8, 169u8, 171u8, 134u8, 207u8, 220u8, 94u8, 125u8, 212u8, 72u8,
                            177u8, 96u8, 77u8, 187u8, 23u8, 91u8, 106u8, 42u8, 231u8, 67u8, 47u8,
                        ],
                    )
                }
                #[doc = " Items to be executed, indexed by the block number that they should be executed on."]
                pub fn agenda_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<
                            ::core::option::Option<
                                runtime_types::pallet_scheduler::ScheduledV3<
                                    runtime_types::frame_support::traits::schedule::MaybeHashed<
                                        runtime_types::encointer_runtime::Call,
                                        ::subxt::ext::sp_core::H256,
                                    >,
                                    ::core::primitive::u32,
                                    runtime_types::encointer_runtime::OriginCaller,
                                    ::subxt::ext::sp_core::crypto::AccountId32,
                                >,
                            >,
                        >,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Scheduler",
                        "Agenda",
                        Vec::new(),
                        [
                            28u8, 250u8, 5u8, 109u8, 7u8, 184u8, 221u8, 16u8, 203u8, 110u8, 146u8,
                            151u8, 169u8, 171u8, 134u8, 207u8, 220u8, 94u8, 125u8, 212u8, 72u8,
                            177u8, 96u8, 77u8, 187u8, 23u8, 91u8, 106u8, 42u8, 231u8, 67u8, 47u8,
                        ],
                    )
                }
                #[doc = " Lookup from identity to the block number and index of the task."]
                pub fn lookup(
                    &self,
                    _0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Scheduler",
                        "Lookup",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Twox64Concat,
                        )],
                        [
                            56u8, 105u8, 156u8, 110u8, 251u8, 141u8, 219u8, 56u8, 131u8, 57u8,
                            180u8, 33u8, 48u8, 30u8, 193u8, 194u8, 169u8, 182u8, 168u8, 43u8, 36u8,
                            202u8, 222u8, 182u8, 41u8, 216u8, 222u8, 1u8, 72u8, 165u8, 62u8, 166u8,
                        ],
                    )
                }
                #[doc = " Lookup from identity to the block number and index of the task."]
                pub fn lookup_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Scheduler",
                        "Lookup",
                        Vec::new(),
                        [
                            56u8, 105u8, 156u8, 110u8, 251u8, 141u8, 219u8, 56u8, 131u8, 57u8,
                            180u8, 33u8, 48u8, 30u8, 193u8, 194u8, 169u8, 182u8, 168u8, 43u8, 36u8,
                            202u8, 222u8, 182u8, 41u8, 216u8, 222u8, 1u8, 72u8, 165u8, 62u8, 166u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The maximum weight that may be scheduled per block for any dispatchables of less"]
                #[doc = " priority than `schedule::HARD_DEADLINE`."]
                pub fn maximum_weight(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Scheduler",
                        "MaximumWeight",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
                #[doc = " The maximum number of scheduled calls in the queue for a single block."]
                #[doc = " Not strictly enforced, but used for weight estimation."]
                pub fn max_scheduled_per_block(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Scheduler",
                        "MaxScheduledPerBlock",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod collective {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SetMembers {
                pub new_members: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
                pub prime: ::core::option::Option<::subxt::ext::sp_core::crypto::AccountId32>,
                pub old_count: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Execute {
                pub proposal: ::std::boxed::Box<runtime_types::encointer_runtime::Call>,
                #[codec(compact)]
                pub length_bound: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Propose {
                #[codec(compact)]
                pub threshold: ::core::primitive::u32,
                pub proposal: ::std::boxed::Box<runtime_types::encointer_runtime::Call>,
                #[codec(compact)]
                pub length_bound: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Vote {
                pub proposal: ::subxt::ext::sp_core::H256,
                #[codec(compact)]
                pub index: ::core::primitive::u32,
                pub approve: ::core::primitive::bool,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Close {
                pub proposal_hash: ::subxt::ext::sp_core::H256,
                #[codec(compact)]
                pub index: ::core::primitive::u32,
                #[codec(compact)]
                pub proposal_weight_bound: ::core::primitive::u64,
                #[codec(compact)]
                pub length_bound: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct DisapproveProposal {
                pub proposal_hash: ::subxt::ext::sp_core::H256,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Set the collective's membership."]
                #[doc = ""]
                #[doc = "- `new_members`: The new member list. Be nice to the chain and provide it sorted."]
                #[doc = "- `prime`: The prime member whose vote sets the default."]
                #[doc = "- `old_count`: The upper bound for the previous number of members in storage. Used for"]
                #[doc = "  weight estimation."]
                #[doc = ""]
                #[doc = "Requires root origin."]
                #[doc = ""]
                #[doc = "NOTE: Does not enforce the expected `MaxMembers` limit on the amount of members, but"]
                #[doc = "      the weight estimations rely on it to estimate dispatchable weight."]
                #[doc = ""]
                #[doc = "# WARNING:"]
                #[doc = ""]
                #[doc = "The `pallet-collective` can also be managed by logic outside of the pallet through the"]
                #[doc = "implementation of the trait [`ChangeMembers`]."]
                #[doc = "Any call to `set_members` must be careful that the member set doesn't get out of sync"]
                #[doc = "with other logic managing the member set."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "## Weight"]
                #[doc = "- `O(MP + N)` where:"]
                #[doc = "  - `M` old-members-count (code- and governance-bounded)"]
                #[doc = "  - `N` new-members-count (code- and governance-bounded)"]
                #[doc = "  - `P` proposals-count (code-bounded)"]
                #[doc = "- DB:"]
                #[doc = "  - 1 storage mutation (codec `O(M)` read, `O(N)` write) for reading and writing the"]
                #[doc = "    members"]
                #[doc = "  - 1 storage read (codec `O(P)`) for reading the proposals"]
                #[doc = "  - `P` storage mutations (codec `O(M)`) for updating the votes for each proposal"]
                #[doc = "  - 1 storage write (codec `O(1)`) for deleting the old `prime` and setting the new one"]
                #[doc = "# </weight>"]
                pub fn set_members(
                    &self,
                    new_members: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
                    prime: ::core::option::Option<::subxt::ext::sp_core::crypto::AccountId32>,
                    old_count: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<SetMembers> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Collective",
                        "set_members",
                        SetMembers {
                            new_members,
                            prime,
                            old_count,
                        },
                        [
                            196u8, 103u8, 123u8, 125u8, 226u8, 177u8, 126u8, 37u8, 160u8, 114u8,
                            34u8, 136u8, 219u8, 84u8, 199u8, 94u8, 242u8, 20u8, 126u8, 126u8,
                            166u8, 190u8, 198u8, 33u8, 162u8, 113u8, 237u8, 222u8, 90u8, 1u8, 2u8,
                            234u8,
                        ],
                    )
                }
                #[doc = "Dispatch a proposal from a member using the `Member` origin."]
                #[doc = ""]
                #[doc = "Origin must be a member of the collective."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "## Weight"]
                #[doc = "- `O(M + P)` where `M` members-count (code-bounded) and `P` complexity of dispatching"]
                #[doc = "  `proposal`"]
                #[doc = "- DB: 1 read (codec `O(M)`) + DB access of `proposal`"]
                #[doc = "- 1 event"]
                #[doc = "# </weight>"]
                pub fn execute(
                    &self,
                    proposal: runtime_types::encointer_runtime::Call,
                    length_bound: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<Execute> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Collective",
                        "execute",
                        Execute {
                            proposal: ::std::boxed::Box::new(proposal),
                            length_bound,
                        },
                        [
                            26u8, 104u8, 157u8, 244u8, 60u8, 68u8, 4u8, 69u8, 237u8, 73u8, 88u8,
                            113u8, 251u8, 65u8, 157u8, 86u8, 16u8, 87u8, 186u8, 184u8, 150u8, 0u8,
                            79u8, 197u8, 168u8, 166u8, 126u8, 37u8, 170u8, 80u8, 173u8, 156u8,
                        ],
                    )
                }
                #[doc = "Add a new proposal to either be voted on or executed directly."]
                #[doc = ""]
                #[doc = "Requires the sender to be member."]
                #[doc = ""]
                #[doc = "`threshold` determines whether `proposal` is executed directly (`threshold < 2`)"]
                #[doc = "or put up for voting."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "## Weight"]
                #[doc = "- `O(B + M + P1)` or `O(B + M + P2)` where:"]
                #[doc = "  - `B` is `proposal` size in bytes (length-fee-bounded)"]
                #[doc = "  - `M` is members-count (code- and governance-bounded)"]
                #[doc = "  - branching is influenced by `threshold` where:"]
                #[doc = "    - `P1` is proposal execution complexity (`threshold < 2`)"]
                #[doc = "    - `P2` is proposals-count (code-bounded) (`threshold >= 2`)"]
                #[doc = "- DB:"]
                #[doc = "  - 1 storage read `is_member` (codec `O(M)`)"]
                #[doc = "  - 1 storage read `ProposalOf::contains_key` (codec `O(1)`)"]
                #[doc = "  - DB accesses influenced by `threshold`:"]
                #[doc = "    - EITHER storage accesses done by `proposal` (`threshold < 2`)"]
                #[doc = "    - OR proposal insertion (`threshold <= 2`)"]
                #[doc = "      - 1 storage mutation `Proposals` (codec `O(P2)`)"]
                #[doc = "      - 1 storage mutation `ProposalCount` (codec `O(1)`)"]
                #[doc = "      - 1 storage write `ProposalOf` (codec `O(B)`)"]
                #[doc = "      - 1 storage write `Voting` (codec `O(M)`)"]
                #[doc = "  - 1 event"]
                #[doc = "# </weight>"]
                pub fn propose(
                    &self,
                    threshold: ::core::primitive::u32,
                    proposal: runtime_types::encointer_runtime::Call,
                    length_bound: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<Propose> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Collective",
                        "propose",
                        Propose {
                            threshold,
                            proposal: ::std::boxed::Box::new(proposal),
                            length_bound,
                        },
                        [
                            161u8, 201u8, 17u8, 83u8, 165u8, 196u8, 158u8, 74u8, 12u8, 93u8, 73u8,
                            149u8, 107u8, 187u8, 73u8, 101u8, 127u8, 177u8, 51u8, 131u8, 164u8,
                            201u8, 118u8, 76u8, 41u8, 183u8, 83u8, 91u8, 32u8, 118u8, 2u8, 162u8,
                        ],
                    )
                }
                #[doc = "Add an aye or nay vote for the sender to the given proposal."]
                #[doc = ""]
                #[doc = "Requires the sender to be a member."]
                #[doc = ""]
                #[doc = "Transaction fees will be waived if the member is voting on any particular proposal"]
                #[doc = "for the first time and the call is successful. Subsequent vote changes will charge a"]
                #[doc = "fee."]
                #[doc = "# <weight>"]
                #[doc = "## Weight"]
                #[doc = "- `O(M)` where `M` is members-count (code- and governance-bounded)"]
                #[doc = "- DB:"]
                #[doc = "  - 1 storage read `Members` (codec `O(M)`)"]
                #[doc = "  - 1 storage mutation `Voting` (codec `O(M)`)"]
                #[doc = "- 1 event"]
                #[doc = "# </weight>"]
                pub fn vote(
                    &self,
                    proposal: ::subxt::ext::sp_core::H256,
                    index: ::core::primitive::u32,
                    approve: ::core::primitive::bool,
                ) -> ::subxt::tx::StaticTxPayload<Vote> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Collective",
                        "vote",
                        Vote {
                            proposal,
                            index,
                            approve,
                        },
                        [
                            108u8, 46u8, 180u8, 148u8, 145u8, 24u8, 173u8, 56u8, 36u8, 100u8,
                            216u8, 43u8, 178u8, 202u8, 26u8, 136u8, 93u8, 84u8, 80u8, 134u8, 14u8,
                            42u8, 248u8, 205u8, 68u8, 92u8, 79u8, 11u8, 113u8, 115u8, 157u8, 100u8,
                        ],
                    )
                }
                #[doc = "Close a vote that is either approved, disapproved or whose voting period has ended."]
                #[doc = ""]
                #[doc = "May be called by any signed account in order to finish voting and close the proposal."]
                #[doc = ""]
                #[doc = "If called before the end of the voting period it will only close the vote if it is"]
                #[doc = "has enough votes to be approved or disapproved."]
                #[doc = ""]
                #[doc = "If called after the end of the voting period abstentions are counted as rejections"]
                #[doc = "unless there is a prime member set and the prime member cast an approval."]
                #[doc = ""]
                #[doc = "If the close operation completes successfully with disapproval, the transaction fee will"]
                #[doc = "be waived. Otherwise execution of the approved operation will be charged to the caller."]
                #[doc = ""]
                #[doc = "+ `proposal_weight_bound`: The maximum amount of weight consumed by executing the closed"]
                #[doc = "proposal."]
                #[doc = "+ `length_bound`: The upper bound for the length of the proposal in storage. Checked via"]
                #[doc = "`storage::read` so it is `size_of::<u32>() == 4` larger than the pure length."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "## Weight"]
                #[doc = "- `O(B + M + P1 + P2)` where:"]
                #[doc = "  - `B` is `proposal` size in bytes (length-fee-bounded)"]
                #[doc = "  - `M` is members-count (code- and governance-bounded)"]
                #[doc = "  - `P1` is the complexity of `proposal` preimage."]
                #[doc = "  - `P2` is proposal-count (code-bounded)"]
                #[doc = "- DB:"]
                #[doc = " - 2 storage reads (`Members`: codec `O(M)`, `Prime`: codec `O(1)`)"]
                #[doc = " - 3 mutations (`Voting`: codec `O(M)`, `ProposalOf`: codec `O(B)`, `Proposals`: codec"]
                #[doc = "   `O(P2)`)"]
                #[doc = " - any mutations done while executing `proposal` (`P1`)"]
                #[doc = "- up to 3 events"]
                #[doc = "# </weight>"]
                pub fn close(
                    &self,
                    proposal_hash: ::subxt::ext::sp_core::H256,
                    index: ::core::primitive::u32,
                    proposal_weight_bound: ::core::primitive::u64,
                    length_bound: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<Close> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Collective",
                        "close",
                        Close {
                            proposal_hash,
                            index,
                            proposal_weight_bound,
                            length_bound,
                        },
                        [
                            88u8, 8u8, 33u8, 184u8, 4u8, 97u8, 120u8, 237u8, 43u8, 183u8, 130u8,
                            139u8, 65u8, 74u8, 166u8, 119u8, 246u8, 65u8, 132u8, 219u8, 118u8,
                            69u8, 182u8, 195u8, 111u8, 204u8, 107u8, 78u8, 152u8, 218u8, 181u8,
                            208u8,
                        ],
                    )
                }
                #[doc = "Disapprove a proposal, close, and remove it from the system, regardless of its current"]
                #[doc = "state."]
                #[doc = ""]
                #[doc = "Must be called by the Root origin."]
                #[doc = ""]
                #[doc = "Parameters:"]
                #[doc = "* `proposal_hash`: The hash of the proposal that should be disapproved."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "Complexity: O(P) where P is the number of max proposals"]
                #[doc = "DB Weight:"]
                #[doc = "* Reads: Proposals"]
                #[doc = "* Writes: Voting, Proposals, ProposalOf"]
                #[doc = "# </weight>"]
                pub fn disapprove_proposal(
                    &self,
                    proposal_hash: ::subxt::ext::sp_core::H256,
                ) -> ::subxt::tx::StaticTxPayload<DisapproveProposal> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Collective",
                        "disapprove_proposal",
                        DisapproveProposal { proposal_hash },
                        [
                            25u8, 123u8, 1u8, 8u8, 74u8, 37u8, 3u8, 40u8, 97u8, 37u8, 175u8, 224u8,
                            72u8, 155u8, 123u8, 109u8, 104u8, 43u8, 91u8, 125u8, 199u8, 51u8, 17u8,
                            225u8, 133u8, 38u8, 120u8, 76u8, 164u8, 5u8, 194u8, 201u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_collective::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A motion (given hash) has been proposed (by given account) with a threshold (given"]
            #[doc = "`MemberCount`)."]
            pub struct Proposed {
                pub account: ::subxt::ext::sp_core::crypto::AccountId32,
                pub proposal_index: ::core::primitive::u32,
                pub proposal_hash: ::subxt::ext::sp_core::H256,
                pub threshold: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for Proposed {
                const PALLET: &'static str = "Collective";
                const EVENT: &'static str = "Proposed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A motion (given hash) has been voted on by given account, leaving"]
            #[doc = "a tally (yes votes and no votes given respectively as `MemberCount`)."]
            pub struct Voted {
                pub account: ::subxt::ext::sp_core::crypto::AccountId32,
                pub proposal_hash: ::subxt::ext::sp_core::H256,
                pub voted: ::core::primitive::bool,
                pub yes: ::core::primitive::u32,
                pub no: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for Voted {
                const PALLET: &'static str = "Collective";
                const EVENT: &'static str = "Voted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A motion was approved by the required threshold."]
            pub struct Approved {
                pub proposal_hash: ::subxt::ext::sp_core::H256,
            }
            impl ::subxt::events::StaticEvent for Approved {
                const PALLET: &'static str = "Collective";
                const EVENT: &'static str = "Approved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A motion was not approved by the required threshold."]
            pub struct Disapproved {
                pub proposal_hash: ::subxt::ext::sp_core::H256,
            }
            impl ::subxt::events::StaticEvent for Disapproved {
                const PALLET: &'static str = "Collective";
                const EVENT: &'static str = "Disapproved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A motion was executed; result will be `Ok` if it returned without error."]
            pub struct Executed {
                pub proposal_hash: ::subxt::ext::sp_core::H256,
                pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::events::StaticEvent for Executed {
                const PALLET: &'static str = "Collective";
                const EVENT: &'static str = "Executed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A single member did some action; result will be `Ok` if it returned without error."]
            pub struct MemberExecuted {
                pub proposal_hash: ::subxt::ext::sp_core::H256,
                pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::events::StaticEvent for MemberExecuted {
                const PALLET: &'static str = "Collective";
                const EVENT: &'static str = "MemberExecuted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A proposal was closed because its threshold was reached or after its duration was up."]
            pub struct Closed {
                pub proposal_hash: ::subxt::ext::sp_core::H256,
                pub yes: ::core::primitive::u32,
                pub no: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for Closed {
                const PALLET: &'static str = "Collective";
                const EVENT: &'static str = "Closed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The hashes of the active proposals."]
                pub fn proposals(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<
                            ::subxt::ext::sp_core::H256,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Collective",
                        "Proposals",
                        vec![],
                        [
                            10u8, 133u8, 82u8, 54u8, 193u8, 41u8, 253u8, 159u8, 56u8, 96u8, 249u8,
                            148u8, 43u8, 57u8, 116u8, 43u8, 222u8, 243u8, 237u8, 231u8, 238u8,
                            60u8, 26u8, 225u8, 19u8, 203u8, 213u8, 220u8, 114u8, 217u8, 100u8,
                            27u8,
                        ],
                    )
                }
                #[doc = " Actual proposal for a given hash, if it's current."]
                pub fn proposal_of(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::H256>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<runtime_types::encointer_runtime::Call>,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Collective",
                        "ProposalOf",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Identity,
                        )],
                        [
                            134u8, 207u8, 216u8, 79u8, 51u8, 164u8, 152u8, 73u8, 84u8, 216u8,
                            181u8, 233u8, 243u8, 120u8, 166u8, 12u8, 235u8, 141u8, 48u8, 99u8,
                            83u8, 53u8, 3u8, 64u8, 236u8, 154u8, 29u8, 11u8, 216u8, 246u8, 21u8,
                            188u8,
                        ],
                    )
                }
                #[doc = " Actual proposal for a given hash, if it's current."]
                pub fn proposal_of_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<runtime_types::encointer_runtime::Call>,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Collective",
                        "ProposalOf",
                        Vec::new(),
                        [
                            134u8, 207u8, 216u8, 79u8, 51u8, 164u8, 152u8, 73u8, 84u8, 216u8,
                            181u8, 233u8, 243u8, 120u8, 166u8, 12u8, 235u8, 141u8, 48u8, 99u8,
                            83u8, 53u8, 3u8, 64u8, 236u8, 154u8, 29u8, 11u8, 216u8, 246u8, 21u8,
                            188u8,
                        ],
                    )
                }
                #[doc = " Votes on a given proposal, if it is ongoing."]
                pub fn voting(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::H256>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_collective::Votes<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Collective",
                        "Voting",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Identity,
                        )],
                        [
                            89u8, 108u8, 65u8, 58u8, 60u8, 116u8, 54u8, 68u8, 179u8, 73u8, 161u8,
                            168u8, 78u8, 213u8, 208u8, 54u8, 244u8, 58u8, 70u8, 209u8, 170u8,
                            136u8, 215u8, 3u8, 2u8, 105u8, 229u8, 217u8, 240u8, 230u8, 107u8,
                            221u8,
                        ],
                    )
                }
                #[doc = " Votes on a given proposal, if it is ongoing."]
                pub fn voting_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_collective::Votes<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                        >,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Collective",
                        "Voting",
                        Vec::new(),
                        [
                            89u8, 108u8, 65u8, 58u8, 60u8, 116u8, 54u8, 68u8, 179u8, 73u8, 161u8,
                            168u8, 78u8, 213u8, 208u8, 54u8, 244u8, 58u8, 70u8, 209u8, 170u8,
                            136u8, 215u8, 3u8, 2u8, 105u8, 229u8, 217u8, 240u8, 230u8, 107u8,
                            221u8,
                        ],
                    )
                }
                #[doc = " Proposals so far."]
                pub fn proposal_count(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Collective",
                        "ProposalCount",
                        vec![],
                        [
                            132u8, 145u8, 78u8, 218u8, 51u8, 189u8, 55u8, 172u8, 143u8, 33u8,
                            140u8, 99u8, 124u8, 208u8, 57u8, 232u8, 154u8, 110u8, 32u8, 142u8,
                            24u8, 149u8, 109u8, 105u8, 30u8, 83u8, 39u8, 177u8, 127u8, 160u8, 34u8,
                            70u8,
                        ],
                    )
                }
                #[doc = " The current members of the collective. This is stored sorted (just by value)."]
                pub fn members(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Collective",
                        "Members",
                        vec![],
                        [
                            162u8, 72u8, 174u8, 204u8, 140u8, 105u8, 205u8, 176u8, 197u8, 117u8,
                            206u8, 134u8, 157u8, 110u8, 139u8, 54u8, 43u8, 233u8, 25u8, 51u8, 36u8,
                            238u8, 94u8, 124u8, 221u8, 52u8, 237u8, 71u8, 125u8, 56u8, 129u8,
                            222u8,
                        ],
                    )
                }
                #[doc = " The prime member that helps determine the default vote behavior in case of absentations."]
                pub fn prime(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::crypto::AccountId32>,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Collective",
                        "Prime",
                        vec![],
                        [
                            108u8, 118u8, 54u8, 193u8, 207u8, 227u8, 119u8, 97u8, 23u8, 239u8,
                            157u8, 69u8, 56u8, 142u8, 106u8, 17u8, 215u8, 159u8, 48u8, 42u8, 185u8,
                            209u8, 49u8, 159u8, 32u8, 168u8, 111u8, 158u8, 159u8, 217u8, 244u8,
                            158u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod membership {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct AddMember {
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct RemoveMember {
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SwapMember {
                pub remove: ::subxt::ext::sp_core::crypto::AccountId32,
                pub add: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ResetMembers {
                pub members: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ChangeKey {
                pub new: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SetPrime {
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ClearPrime;
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Add a member `who` to the set."]
                #[doc = ""]
                #[doc = "May only be called from `T::AddOrigin`."]
                pub fn add_member(
                    &self,
                    who: ::subxt::ext::sp_core::crypto::AccountId32,
                ) -> ::subxt::tx::StaticTxPayload<AddMember> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Membership",
                        "add_member",
                        AddMember { who },
                        [
                            106u8, 33u8, 171u8, 114u8, 223u8, 105u8, 71u8, 15u8, 77u8, 253u8, 40u8,
                            204u8, 244u8, 142u8, 103u8, 177u8, 200u8, 243u8, 114u8, 241u8, 36u8,
                            135u8, 175u8, 255u8, 124u8, 193u8, 30u8, 46u8, 186u8, 172u8, 176u8,
                            98u8,
                        ],
                    )
                }
                #[doc = "Remove a member `who` from the set."]
                #[doc = ""]
                #[doc = "May only be called from `T::RemoveOrigin`."]
                pub fn remove_member(
                    &self,
                    who: ::subxt::ext::sp_core::crypto::AccountId32,
                ) -> ::subxt::tx::StaticTxPayload<RemoveMember> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Membership",
                        "remove_member",
                        RemoveMember { who },
                        [
                            100u8, 17u8, 75u8, 92u8, 58u8, 100u8, 34u8, 187u8, 41u8, 160u8, 137u8,
                            58u8, 78u8, 166u8, 161u8, 116u8, 1u8, 67u8, 201u8, 144u8, 103u8, 84u8,
                            55u8, 246u8, 133u8, 180u8, 148u8, 86u8, 175u8, 175u8, 70u8, 73u8,
                        ],
                    )
                }
                #[doc = "Swap out one member `remove` for another `add`."]
                #[doc = ""]
                #[doc = "May only be called from `T::SwapOrigin`."]
                #[doc = ""]
                #[doc = "Prime membership is *not* passed from `remove` to `add`, if extant."]
                pub fn swap_member(
                    &self,
                    remove: ::subxt::ext::sp_core::crypto::AccountId32,
                    add: ::subxt::ext::sp_core::crypto::AccountId32,
                ) -> ::subxt::tx::StaticTxPayload<SwapMember> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Membership",
                        "swap_member",
                        SwapMember { remove, add },
                        [
                            66u8, 84u8, 183u8, 29u8, 104u8, 163u8, 220u8, 217u8, 103u8, 234u8,
                            233u8, 138u8, 191u8, 147u8, 51u8, 98u8, 46u8, 51u8, 179u8, 200u8, 23u8,
                            59u8, 112u8, 53u8, 8u8, 75u8, 135u8, 232u8, 116u8, 201u8, 60u8, 249u8,
                        ],
                    )
                }
                #[doc = "Change the membership to a new set, disregarding the existing membership. Be nice and"]
                #[doc = "pass `members` pre-sorted."]
                #[doc = ""]
                #[doc = "May only be called from `T::ResetOrigin`."]
                pub fn reset_members(
                    &self,
                    members: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
                ) -> ::subxt::tx::StaticTxPayload<ResetMembers> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Membership",
                        "reset_members",
                        ResetMembers { members },
                        [
                            9u8, 35u8, 28u8, 59u8, 158u8, 232u8, 89u8, 78u8, 101u8, 53u8, 240u8,
                            98u8, 13u8, 104u8, 235u8, 161u8, 201u8, 150u8, 117u8, 32u8, 75u8,
                            209u8, 166u8, 252u8, 57u8, 131u8, 96u8, 215u8, 51u8, 81u8, 42u8, 123u8,
                        ],
                    )
                }
                #[doc = "Swap out the sending member for some other key `new`."]
                #[doc = ""]
                #[doc = "May only be called from `Signed` origin of a current member."]
                #[doc = ""]
                #[doc = "Prime membership is passed from the origin account to `new`, if extant."]
                pub fn change_key(
                    &self,
                    new: ::subxt::ext::sp_core::crypto::AccountId32,
                ) -> ::subxt::tx::StaticTxPayload<ChangeKey> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Membership",
                        "change_key",
                        ChangeKey { new },
                        [
                            53u8, 60u8, 54u8, 231u8, 151u8, 0u8, 27u8, 175u8, 250u8, 80u8, 74u8,
                            184u8, 184u8, 63u8, 90u8, 216u8, 186u8, 136u8, 74u8, 214u8, 111u8,
                            186u8, 137u8, 140u8, 108u8, 194u8, 128u8, 97u8, 168u8, 184u8, 112u8,
                            60u8,
                        ],
                    )
                }
                #[doc = "Set the prime member. Must be a current member."]
                #[doc = ""]
                #[doc = "May only be called from `T::PrimeOrigin`."]
                pub fn set_prime(
                    &self,
                    who: ::subxt::ext::sp_core::crypto::AccountId32,
                ) -> ::subxt::tx::StaticTxPayload<SetPrime> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Membership",
                        "set_prime",
                        SetPrime { who },
                        [
                            123u8, 95u8, 75u8, 129u8, 19u8, 34u8, 192u8, 65u8, 169u8, 47u8, 184u8,
                            246u8, 55u8, 250u8, 31u8, 158u8, 57u8, 197u8, 22u8, 112u8, 167u8,
                            198u8, 136u8, 17u8, 15u8, 203u8, 101u8, 149u8, 15u8, 39u8, 16u8, 232u8,
                        ],
                    )
                }
                #[doc = "Remove the prime member if it exists."]
                #[doc = ""]
                #[doc = "May only be called from `T::PrimeOrigin`."]
                pub fn clear_prime(&self) -> ::subxt::tx::StaticTxPayload<ClearPrime> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Membership",
                        "clear_prime",
                        ClearPrime {},
                        [
                            186u8, 182u8, 225u8, 90u8, 71u8, 124u8, 69u8, 100u8, 234u8, 25u8, 53u8,
                            23u8, 182u8, 32u8, 176u8, 81u8, 54u8, 140u8, 235u8, 126u8, 247u8, 7u8,
                            155u8, 62u8, 35u8, 135u8, 48u8, 61u8, 88u8, 160u8, 183u8, 72u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_membership::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "The given member was added; see the transaction for who."]
            pub struct MemberAdded;
            impl ::subxt::events::StaticEvent for MemberAdded {
                const PALLET: &'static str = "Membership";
                const EVENT: &'static str = "MemberAdded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "The given member was removed; see the transaction for who."]
            pub struct MemberRemoved;
            impl ::subxt::events::StaticEvent for MemberRemoved {
                const PALLET: &'static str = "Membership";
                const EVENT: &'static str = "MemberRemoved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Two members were swapped; see the transaction for who."]
            pub struct MembersSwapped;
            impl ::subxt::events::StaticEvent for MembersSwapped {
                const PALLET: &'static str = "Membership";
                const EVENT: &'static str = "MembersSwapped";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "The membership was reset; see the transaction for who the new set is."]
            pub struct MembersReset;
            impl ::subxt::events::StaticEvent for MembersReset {
                const PALLET: &'static str = "Membership";
                const EVENT: &'static str = "MembersReset";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "One of the members' keys changed."]
            pub struct KeyChanged;
            impl ::subxt::events::StaticEvent for KeyChanged {
                const PALLET: &'static str = "Membership";
                const EVENT: &'static str = "KeyChanged";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Phantom member, never used."]
            pub struct Dummy;
            impl ::subxt::events::StaticEvent for Dummy {
                const PALLET: &'static str = "Membership";
                const EVENT: &'static str = "Dummy";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The current membership, stored as an ordered Vec."]
                pub fn members(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Membership",
                        "Members",
                        vec![],
                        [
                            56u8, 56u8, 29u8, 90u8, 26u8, 115u8, 252u8, 185u8, 37u8, 108u8, 16u8,
                            46u8, 136u8, 139u8, 30u8, 19u8, 235u8, 78u8, 176u8, 129u8, 180u8, 57u8,
                            178u8, 239u8, 211u8, 6u8, 64u8, 129u8, 195u8, 46u8, 178u8, 157u8,
                        ],
                    )
                }
                #[doc = " The current prime member, if one exists."]
                pub fn prime(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::crypto::AccountId32>,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Membership",
                        "Prime",
                        vec![],
                        [
                            108u8, 118u8, 54u8, 193u8, 207u8, 227u8, 119u8, 97u8, 23u8, 239u8,
                            157u8, 69u8, 56u8, 142u8, 106u8, 17u8, 215u8, 159u8, 48u8, 42u8, 185u8,
                            209u8, 49u8, 159u8, 32u8, 168u8, 111u8, 158u8, 159u8, 217u8, 244u8,
                            158u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod encointer_scheduler {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct NextPhase;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct PushByOneDay;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SetPhaseDuration {
                pub ceremony_phase:
                    runtime_types::encointer_primitives::scheduler::CeremonyPhaseType,
                pub duration: ::core::primitive::u64,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct SetNextPhaseTimestamp {
                pub timestamp: ::core::primitive::u64,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Manually transition to next phase without affecting the ceremony rhythm"]
                #[doc = ""]
                #[doc = "May only be called from `T::CeremonyMaster`."]
                pub fn next_phase(&self) -> ::subxt::tx::StaticTxPayload<NextPhase> {
                    ::subxt::tx::StaticTxPayload::new(
                        "EncointerScheduler",
                        "next_phase",
                        NextPhase {},
                        [
                            39u8, 14u8, 163u8, 6u8, 137u8, 228u8, 192u8, 7u8, 54u8, 169u8, 49u8,
                            246u8, 12u8, 98u8, 93u8, 160u8, 25u8, 234u8, 17u8, 166u8, 162u8, 63u8,
                            6u8, 242u8, 118u8, 197u8, 136u8, 74u8, 129u8, 39u8, 132u8, 30u8,
                        ],
                    )
                }
                #[doc = "Push next phase change by one entire day"]
                #[doc = ""]
                #[doc = "May only be called from `T::CeremonyMaster`."]
                pub fn push_by_one_day(&self) -> ::subxt::tx::StaticTxPayload<PushByOneDay> {
                    ::subxt::tx::StaticTxPayload::new(
                        "EncointerScheduler",
                        "push_by_one_day",
                        PushByOneDay {},
                        [
                            14u8, 194u8, 45u8, 134u8, 225u8, 47u8, 67u8, 10u8, 211u8, 82u8, 95u8,
                            192u8, 35u8, 95u8, 139u8, 52u8, 202u8, 171u8, 178u8, 106u8, 124u8, 9u8,
                            8u8, 114u8, 170u8, 70u8, 50u8, 215u8, 182u8, 155u8, 180u8, 244u8,
                        ],
                    )
                }
                pub fn set_phase_duration(
                    &self,
                    ceremony_phase : runtime_types :: encointer_primitives :: scheduler :: CeremonyPhaseType,
                    duration: ::core::primitive::u64,
                ) -> ::subxt::tx::StaticTxPayload<SetPhaseDuration> {
                    ::subxt::tx::StaticTxPayload::new(
                        "EncointerScheduler",
                        "set_phase_duration",
                        SetPhaseDuration {
                            ceremony_phase,
                            duration,
                        },
                        [
                            228u8, 104u8, 207u8, 114u8, 222u8, 222u8, 160u8, 211u8, 134u8, 215u8,
                            145u8, 16u8, 206u8, 126u8, 249u8, 38u8, 240u8, 58u8, 162u8, 137u8,
                            182u8, 141u8, 115u8, 144u8, 217u8, 224u8, 125u8, 215u8, 63u8, 190u8,
                            138u8, 75u8,
                        ],
                    )
                }
                pub fn set_next_phase_timestamp(
                    &self,
                    timestamp: ::core::primitive::u64,
                ) -> ::subxt::tx::StaticTxPayload<SetNextPhaseTimestamp> {
                    ::subxt::tx::StaticTxPayload::new(
                        "EncointerScheduler",
                        "set_next_phase_timestamp",
                        SetNextPhaseTimestamp { timestamp },
                        [
                            80u8, 104u8, 132u8, 254u8, 99u8, 182u8, 2u8, 197u8, 73u8, 48u8, 143u8,
                            138u8, 194u8, 12u8, 8u8, 250u8, 89u8, 182u8, 197u8, 180u8, 73u8, 227u8,
                            37u8, 208u8, 33u8, 202u8, 125u8, 160u8, 198u8, 220u8, 66u8, 30u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_encointer_scheduler::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Phase changed to `[new phase]`"]
            pub struct PhaseChangedTo(
                pub runtime_types::encointer_primitives::scheduler::CeremonyPhaseType,
            );
            impl ::subxt::events::StaticEvent for PhaseChangedTo {
                const PALLET: &'static str = "EncointerScheduler";
                const EVENT: &'static str = "PhaseChangedTo";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct CeremonySchedulePushedByOneDay;
            impl ::subxt::events::StaticEvent for CeremonySchedulePushedByOneDay {
                const PALLET: &'static str = "EncointerScheduler";
                const EVENT: &'static str = "CeremonySchedulePushedByOneDay";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                pub fn current_ceremony_index(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerScheduler",
                        "CurrentCeremonyIndex",
                        vec![],
                        [
                            171u8, 198u8, 241u8, 115u8, 220u8, 243u8, 15u8, 169u8, 126u8, 132u8,
                            81u8, 40u8, 19u8, 176u8, 121u8, 129u8, 116u8, 155u8, 58u8, 56u8, 111u8,
                            211u8, 130u8, 99u8, 175u8, 10u8, 176u8, 169u8, 57u8, 56u8, 118u8, 2u8,
                        ],
                    )
                }
                pub fn last_ceremony_block(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerScheduler",
                        "LastCeremonyBlock",
                        vec![],
                        [
                            83u8, 244u8, 134u8, 100u8, 68u8, 197u8, 189u8, 220u8, 127u8, 178u8,
                            25u8, 32u8, 87u8, 1u8, 70u8, 108u8, 188u8, 22u8, 86u8, 184u8, 193u8,
                            228u8, 180u8, 196u8, 165u8, 146u8, 219u8, 119u8, 241u8, 147u8, 43u8,
                            71u8,
                        ],
                    )
                }
                pub fn current_phase(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::encointer_primitives::scheduler::CeremonyPhaseType,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerScheduler",
                        "CurrentPhase",
                        vec![],
                        [
                            160u8, 186u8, 39u8, 151u8, 122u8, 48u8, 160u8, 110u8, 210u8, 223u8,
                            182u8, 187u8, 176u8, 70u8, 3u8, 198u8, 183u8, 98u8, 93u8, 107u8, 129u8,
                            96u8, 55u8, 156u8, 169u8, 103u8, 14u8, 23u8, 173u8, 209u8, 189u8,
                            226u8,
                        ],
                    )
                }
                pub fn next_phase_timestamp(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerScheduler",
                        "NextPhaseTimestamp",
                        vec![],
                        [
                            55u8, 199u8, 204u8, 92u8, 159u8, 238u8, 203u8, 143u8, 241u8, 152u8,
                            242u8, 118u8, 209u8, 26u8, 223u8, 63u8, 37u8, 184u8, 201u8, 192u8,
                            248u8, 214u8, 95u8, 174u8, 240u8, 204u8, 156u8, 159u8, 165u8, 219u8,
                            6u8, 14u8,
                        ],
                    )
                }
                pub fn phase_durations(
                    &self,
                    _0: impl ::std::borrow::Borrow<
                        runtime_types::encointer_primitives::scheduler::CeremonyPhaseType,
                    >,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerScheduler",
                        "PhaseDurations",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            215u8, 220u8, 192u8, 144u8, 230u8, 188u8, 56u8, 23u8, 125u8, 242u8,
                            245u8, 81u8, 153u8, 74u8, 157u8, 181u8, 147u8, 161u8, 68u8, 36u8, 54u8,
                            51u8, 183u8, 74u8, 234u8, 39u8, 166u8, 6u8, 193u8, 149u8, 183u8, 123u8,
                        ],
                    )
                }
                pub fn phase_durations_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerScheduler",
                        "PhaseDurations",
                        Vec::new(),
                        [
                            215u8, 220u8, 192u8, 144u8, 230u8, 188u8, 56u8, 23u8, 125u8, 242u8,
                            245u8, 81u8, 153u8, 74u8, 157u8, 181u8, 147u8, 161u8, 68u8, 36u8, 54u8,
                            51u8, 183u8, 74u8, 234u8, 39u8, 166u8, 6u8, 193u8, 149u8, 183u8, 123u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn moments_per_day(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "EncointerScheduler",
                        "MomentsPerDay",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod encointer_ceremonies {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct RegisterParticipant {
                pub cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                pub proof: ::core::option::Option<
                    runtime_types::encointer_primitives::ceremonies::ProofOfAttendance<
                        runtime_types::sp_runtime::MultiSignature,
                        ::subxt::ext::sp_core::crypto::AccountId32,
                    >,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct UpgradeRegistration {
                pub cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                pub proof: runtime_types::encointer_primitives::ceremonies::ProofOfAttendance<
                    runtime_types::sp_runtime::MultiSignature,
                    ::subxt::ext::sp_core::crypto::AccountId32,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct UnregisterParticipant {
                pub cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                pub maybe_reputation_community_ceremony: ::core::option::Option<(
                    runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    ::core::primitive::u32,
                )>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct AttestAttendees {
                pub cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                pub number_of_participants_vote: ::core::primitive::u32,
                pub attestations: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct EndorseNewcomer {
                pub cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                pub newbie: ::subxt::ext::sp_core::crypto::AccountId32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ClaimRewards {
                pub cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                pub maybe_meetup_index: ::core::option::Option<::core::primitive::u64>,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct SetInactivityTimeout {
                pub inactivity_timeout: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct SetEndorsementTicketsPerBootstrapper {
                pub endorsement_tickets_per_bootstrapper: ::core::primitive::u8,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct SetEndorsementTicketsPerReputable {
                pub endorsement_tickets_per_reputable: ::core::primitive::u8,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct SetReputationLifetime {
                pub reputation_lifetime: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SetMeetupTimeOffset {
                pub meetup_time_offset: ::core::primitive::i32,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct SetTimeTolerance {
                pub time_tolerance: ::core::primitive::u64,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct SetLocationTolerance {
                pub location_tolerance: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct PurgeCommunityCeremony {
                pub community_ceremony: (
                    runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    ::core::primitive::u32,
                ),
            }
            pub struct TransactionApi;
            impl TransactionApi {
                pub fn register_participant(
                    &self,
                    cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    proof: ::core::option::Option<
                        runtime_types::encointer_primitives::ceremonies::ProofOfAttendance<
                            runtime_types::sp_runtime::MultiSignature,
                            ::subxt::ext::sp_core::crypto::AccountId32,
                        >,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<RegisterParticipant> {
                    ::subxt::tx::StaticTxPayload::new(
                        "EncointerCeremonies",
                        "register_participant",
                        RegisterParticipant { cid, proof },
                        [
                            205u8, 241u8, 23u8, 50u8, 229u8, 62u8, 78u8, 220u8, 65u8, 94u8, 215u8,
                            68u8, 99u8, 15u8, 245u8, 76u8, 164u8, 122u8, 42u8, 173u8, 29u8, 4u8,
                            53u8, 124u8, 13u8, 216u8, 165u8, 72u8, 201u8, 104u8, 70u8, 203u8,
                        ],
                    )
                }
                pub fn upgrade_registration(
                    &self,
                    cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    proof: runtime_types::encointer_primitives::ceremonies::ProofOfAttendance<
                        runtime_types::sp_runtime::MultiSignature,
                        ::subxt::ext::sp_core::crypto::AccountId32,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<UpgradeRegistration> {
                    ::subxt::tx::StaticTxPayload::new(
                        "EncointerCeremonies",
                        "upgrade_registration",
                        UpgradeRegistration { cid, proof },
                        [
                            170u8, 82u8, 103u8, 136u8, 229u8, 189u8, 11u8, 224u8, 7u8, 207u8,
                            173u8, 10u8, 40u8, 184u8, 60u8, 160u8, 100u8, 188u8, 208u8, 226u8,
                            237u8, 29u8, 118u8, 159u8, 221u8, 79u8, 121u8, 89u8, 161u8, 44u8, 85u8,
                            108u8,
                        ],
                    )
                }
                pub fn unregister_participant(
                    &self,
                    cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    maybe_reputation_community_ceremony: ::core::option::Option<(
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        ::core::primitive::u32,
                    )>,
                ) -> ::subxt::tx::StaticTxPayload<UnregisterParticipant> {
                    ::subxt::tx::StaticTxPayload::new(
                        "EncointerCeremonies",
                        "unregister_participant",
                        UnregisterParticipant {
                            cid,
                            maybe_reputation_community_ceremony,
                        },
                        [
                            229u8, 41u8, 99u8, 229u8, 36u8, 79u8, 51u8, 11u8, 208u8, 250u8, 179u8,
                            38u8, 93u8, 98u8, 130u8, 223u8, 6u8, 90u8, 71u8, 52u8, 123u8, 29u8,
                            196u8, 55u8, 96u8, 225u8, 88u8, 193u8, 61u8, 194u8, 215u8, 252u8,
                        ],
                    )
                }
                pub fn attest_attendees(
                    &self,
                    cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    number_of_participants_vote: ::core::primitive::u32,
                    attestations: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
                ) -> ::subxt::tx::StaticTxPayload<AttestAttendees> {
                    ::subxt::tx::StaticTxPayload::new(
                        "EncointerCeremonies",
                        "attest_attendees",
                        AttestAttendees {
                            cid,
                            number_of_participants_vote,
                            attestations,
                        },
                        [
                            62u8, 181u8, 244u8, 27u8, 128u8, 196u8, 105u8, 165u8, 139u8, 169u8,
                            203u8, 178u8, 0u8, 129u8, 178u8, 222u8, 136u8, 254u8, 249u8, 102u8,
                            236u8, 23u8, 213u8, 221u8, 124u8, 95u8, 42u8, 28u8, 68u8, 217u8, 9u8,
                            26u8,
                        ],
                    )
                }
                pub fn endorse_newcomer(
                    &self,
                    cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    newbie: ::subxt::ext::sp_core::crypto::AccountId32,
                ) -> ::subxt::tx::StaticTxPayload<EndorseNewcomer> {
                    ::subxt::tx::StaticTxPayload::new(
                        "EncointerCeremonies",
                        "endorse_newcomer",
                        EndorseNewcomer { cid, newbie },
                        [
                            218u8, 220u8, 78u8, 157u8, 196u8, 58u8, 206u8, 145u8, 125u8, 112u8,
                            42u8, 221u8, 199u8, 136u8, 100u8, 178u8, 212u8, 9u8, 70u8, 136u8, 35u8,
                            59u8, 225u8, 78u8, 106u8, 223u8, 88u8, 180u8, 82u8, 236u8, 66u8, 234u8,
                        ],
                    )
                }
                pub fn claim_rewards(
                    &self,
                    cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    maybe_meetup_index: ::core::option::Option<::core::primitive::u64>,
                ) -> ::subxt::tx::StaticTxPayload<ClaimRewards> {
                    ::subxt::tx::StaticTxPayload::new(
                        "EncointerCeremonies",
                        "claim_rewards",
                        ClaimRewards {
                            cid,
                            maybe_meetup_index,
                        },
                        [
                            217u8, 244u8, 60u8, 193u8, 137u8, 38u8, 62u8, 137u8, 248u8, 234u8,
                            39u8, 54u8, 192u8, 120u8, 136u8, 241u8, 224u8, 182u8, 234u8, 151u8,
                            106u8, 234u8, 110u8, 212u8, 185u8, 232u8, 223u8, 233u8, 216u8, 233u8,
                            236u8, 226u8,
                        ],
                    )
                }
                pub fn set_inactivity_timeout(
                    &self,
                    inactivity_timeout: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<SetInactivityTimeout> {
                    ::subxt::tx::StaticTxPayload::new(
                        "EncointerCeremonies",
                        "set_inactivity_timeout",
                        SetInactivityTimeout { inactivity_timeout },
                        [
                            37u8, 244u8, 64u8, 144u8, 252u8, 225u8, 232u8, 39u8, 229u8, 24u8,
                            165u8, 69u8, 64u8, 106u8, 65u8, 119u8, 229u8, 125u8, 86u8, 111u8, 49u8,
                            97u8, 57u8, 132u8, 63u8, 26u8, 195u8, 192u8, 227u8, 175u8, 252u8, 62u8,
                        ],
                    )
                }
                pub fn set_endorsement_tickets_per_bootstrapper(
                    &self,
                    endorsement_tickets_per_bootstrapper: ::core::primitive::u8,
                ) -> ::subxt::tx::StaticTxPayload<SetEndorsementTicketsPerBootstrapper>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "EncointerCeremonies",
                        "set_endorsement_tickets_per_bootstrapper",
                        SetEndorsementTicketsPerBootstrapper {
                            endorsement_tickets_per_bootstrapper,
                        },
                        [
                            38u8, 218u8, 147u8, 167u8, 153u8, 180u8, 212u8, 226u8, 135u8, 37u8,
                            216u8, 61u8, 149u8, 103u8, 122u8, 210u8, 175u8, 184u8, 137u8, 155u8,
                            206u8, 125u8, 28u8, 254u8, 209u8, 126u8, 17u8, 151u8, 106u8, 137u8,
                            48u8, 9u8,
                        ],
                    )
                }
                pub fn set_endorsement_tickets_per_reputable(
                    &self,
                    endorsement_tickets_per_reputable: ::core::primitive::u8,
                ) -> ::subxt::tx::StaticTxPayload<SetEndorsementTicketsPerReputable>
                {
                    ::subxt::tx::StaticTxPayload::new(
                        "EncointerCeremonies",
                        "set_endorsement_tickets_per_reputable",
                        SetEndorsementTicketsPerReputable {
                            endorsement_tickets_per_reputable,
                        },
                        [
                            143u8, 237u8, 16u8, 34u8, 162u8, 103u8, 24u8, 14u8, 68u8, 80u8, 203u8,
                            232u8, 197u8, 35u8, 9u8, 129u8, 212u8, 27u8, 6u8, 222u8, 22u8, 244u8,
                            106u8, 1u8, 255u8, 113u8, 9u8, 8u8, 123u8, 115u8, 89u8, 178u8,
                        ],
                    )
                }
                pub fn set_reputation_lifetime(
                    &self,
                    reputation_lifetime: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<SetReputationLifetime> {
                    ::subxt::tx::StaticTxPayload::new(
                        "EncointerCeremonies",
                        "set_reputation_lifetime",
                        SetReputationLifetime {
                            reputation_lifetime,
                        },
                        [
                            152u8, 56u8, 40u8, 189u8, 28u8, 216u8, 21u8, 12u8, 2u8, 88u8, 212u8,
                            90u8, 12u8, 165u8, 79u8, 77u8, 134u8, 33u8, 11u8, 192u8, 10u8, 187u8,
                            0u8, 183u8, 137u8, 3u8, 3u8, 7u8, 68u8, 183u8, 51u8, 247u8,
                        ],
                    )
                }
                pub fn set_meetup_time_offset(
                    &self,
                    meetup_time_offset: ::core::primitive::i32,
                ) -> ::subxt::tx::StaticTxPayload<SetMeetupTimeOffset> {
                    ::subxt::tx::StaticTxPayload::new(
                        "EncointerCeremonies",
                        "set_meetup_time_offset",
                        SetMeetupTimeOffset { meetup_time_offset },
                        [
                            236u8, 42u8, 242u8, 22u8, 239u8, 186u8, 12u8, 34u8, 186u8, 55u8, 15u8,
                            75u8, 59u8, 224u8, 192u8, 16u8, 100u8, 114u8, 138u8, 218u8, 216u8,
                            54u8, 18u8, 185u8, 11u8, 77u8, 231u8, 183u8, 2u8, 162u8, 138u8, 214u8,
                        ],
                    )
                }
                pub fn set_time_tolerance(
                    &self,
                    time_tolerance: ::core::primitive::u64,
                ) -> ::subxt::tx::StaticTxPayload<SetTimeTolerance> {
                    ::subxt::tx::StaticTxPayload::new(
                        "EncointerCeremonies",
                        "set_time_tolerance",
                        SetTimeTolerance { time_tolerance },
                        [
                            74u8, 187u8, 50u8, 62u8, 34u8, 231u8, 184u8, 5u8, 193u8, 19u8, 85u8,
                            124u8, 104u8, 33u8, 65u8, 8u8, 253u8, 171u8, 193u8, 226u8, 161u8,
                            221u8, 45u8, 94u8, 102u8, 100u8, 240u8, 224u8, 114u8, 12u8, 187u8,
                            156u8,
                        ],
                    )
                }
                pub fn set_location_tolerance(
                    &self,
                    location_tolerance: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<SetLocationTolerance> {
                    ::subxt::tx::StaticTxPayload::new(
                        "EncointerCeremonies",
                        "set_location_tolerance",
                        SetLocationTolerance { location_tolerance },
                        [
                            158u8, 54u8, 171u8, 110u8, 74u8, 59u8, 70u8, 158u8, 126u8, 90u8, 12u8,
                            117u8, 115u8, 235u8, 185u8, 244u8, 160u8, 11u8, 83u8, 173u8, 196u8,
                            18u8, 142u8, 144u8, 220u8, 50u8, 128u8, 123u8, 244u8, 222u8, 228u8,
                            47u8,
                        ],
                    )
                }
                pub fn purge_community_ceremony(
                    &self,
                    community_ceremony: (
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        ::core::primitive::u32,
                    ),
                ) -> ::subxt::tx::StaticTxPayload<PurgeCommunityCeremony> {
                    ::subxt::tx::StaticTxPayload::new(
                        "EncointerCeremonies",
                        "purge_community_ceremony",
                        PurgeCommunityCeremony { community_ceremony },
                        [
                            99u8, 187u8, 173u8, 11u8, 85u8, 181u8, 27u8, 171u8, 85u8, 216u8, 134u8,
                            89u8, 67u8, 0u8, 184u8, 89u8, 151u8, 0u8, 23u8, 197u8, 120u8, 2u8,
                            238u8, 134u8, 233u8, 68u8, 149u8, 245u8, 149u8, 198u8, 120u8, 255u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_encointer_ceremonies::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Participant registered for next ceremony [community, participant type, who]"]
            pub struct ParticipantRegistered(
                pub runtime_types::encointer_primitives::communities::CommunityIdentifier,
                pub runtime_types::encointer_primitives::ceremonies::ParticipantType,
                pub ::subxt::ext::sp_core::crypto::AccountId32,
            );
            impl ::subxt::events::StaticEvent for ParticipantRegistered {
                const PALLET: &'static str = "EncointerCeremonies";
                const EVENT: &'static str = "ParticipantRegistered";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A bootstrapper (first accountid) has endorsed a participant (second accountid) who can now register as endorsee for this ceremony"]
            pub struct EndorsedParticipant(
                pub runtime_types::encointer_primitives::communities::CommunityIdentifier,
                pub ::subxt::ext::sp_core::crypto::AccountId32,
                pub ::subxt::ext::sp_core::crypto::AccountId32,
            );
            impl ::subxt::events::StaticEvent for EndorsedParticipant {
                const PALLET: &'static str = "EncointerCeremonies";
                const EVENT: &'static str = "EndorsedParticipant";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A participant has registered N attestations for fellow meetup participants"]
            pub struct AttestationsRegistered(
                pub runtime_types::encointer_primitives::communities::CommunityIdentifier,
                pub ::core::primitive::u64,
                pub ::core::primitive::u32,
                pub ::subxt::ext::sp_core::crypto::AccountId32,
            );
            impl ::subxt::events::StaticEvent for AttestationsRegistered {
                const PALLET: &'static str = "EncointerCeremonies";
                const EVENT: &'static str = "AttestationsRegistered";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "rewards have been claimed and issued successfully for N participants for their meetup at the previous ceremony"]
            pub struct RewardsIssued(
                pub runtime_types::encointer_primitives::communities::CommunityIdentifier,
                pub ::core::primitive::u64,
                pub ::core::primitive::u8,
            );
            impl ::subxt::events::StaticEvent for RewardsIssued {
                const PALLET: &'static str = "EncointerCeremonies";
                const EVENT: &'static str = "RewardsIssued";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            #[doc = "inactivity timeout has changed. affects how many ceremony cycles a community can be idle before getting purged"]
            pub struct InactivityTimeoutUpdated(pub ::core::primitive::u32);
            impl ::subxt::events::StaticEvent for InactivityTimeoutUpdated {
                const PALLET: &'static str = "EncointerCeremonies";
                const EVENT: &'static str = "InactivityTimeoutUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            #[doc = "The number of endorsement tickets which bootstrappers can give out has changed"]
            pub struct EndorsementTicketsPerBootstrapperUpdated(pub ::core::primitive::u8);
            impl ::subxt::events::StaticEvent for EndorsementTicketsPerBootstrapperUpdated {
                const PALLET: &'static str = "EncointerCeremonies";
                const EVENT: &'static str = "EndorsementTicketsPerBootstrapperUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            #[doc = "The number of endorsement tickets which bootstrappers can give out has changed"]
            pub struct EndorsementTicketsPerReputableUpdated(pub ::core::primitive::u8);
            impl ::subxt::events::StaticEvent for EndorsementTicketsPerReputableUpdated {
                const PALLET: &'static str = "EncointerCeremonies";
                const EVENT: &'static str = "EndorsementTicketsPerReputableUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            #[doc = "reputation lifetime has changed. After this many ceremony cycles, reputations is outdated"]
            pub struct ReputationLifetimeUpdated(pub ::core::primitive::u32);
            impl ::subxt::events::StaticEvent for ReputationLifetimeUpdated {
                const PALLET: &'static str = "EncointerCeremonies";
                const EVENT: &'static str = "ReputationLifetimeUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "meetup time offset has changed. affects the exact time the upcoming ceremony meetups will take place"]
            pub struct MeetupTimeOffsetUpdated(pub ::core::primitive::i32);
            impl ::subxt::events::StaticEvent for MeetupTimeOffsetUpdated {
                const PALLET: &'static str = "EncointerCeremonies";
                const EVENT: &'static str = "MeetupTimeOffsetUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            #[doc = "meetup time tolerance has changed"]
            pub struct TimeToleranceUpdated(pub ::core::primitive::u64);
            impl ::subxt::events::StaticEvent for TimeToleranceUpdated {
                const PALLET: &'static str = "EncointerCeremonies";
                const EVENT: &'static str = "TimeToleranceUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            #[doc = "meetup location tolerance changed [m]"]
            pub struct LocationToleranceUpdated(pub ::core::primitive::u32);
            impl ::subxt::events::StaticEvent for LocationToleranceUpdated {
                const PALLET: &'static str = "EncointerCeremonies";
                const EVENT: &'static str = "LocationToleranceUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "the registry for given ceremony index and community has been purged"]
            pub struct CommunityCeremonyHistoryPurged(
                pub runtime_types::encointer_primitives::communities::CommunityIdentifier,
                pub ::core::primitive::u32,
            );
            impl ::subxt::events::StaticEvent for CommunityCeremonyHistoryPurged {
                const PALLET: &'static str = "EncointerCeremonies";
                const EVENT: &'static str = "CommunityCeremonyHistoryPurged";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct NoReward {
                pub cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                pub cindex: ::core::primitive::u32,
                pub meetup_index: ::core::primitive::u64,
                pub account: ::subxt::ext::sp_core::crypto::AccountId32,
                pub reason: runtime_types::encointer_meetup_validation::ExclusionReason,
            }
            impl ::subxt::events::StaticEvent for NoReward {
                const PALLET: &'static str = "EncointerCeremonies";
                const EVENT: &'static str = "NoReward";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "The inactivity counter of a community has been increased"]
            pub struct InactivityCounterUpdated(
                pub runtime_types::encointer_primitives::communities::CommunityIdentifier,
                pub ::core::primitive::u32,
            );
            impl ::subxt::events::StaticEvent for InactivityCounterUpdated {
                const PALLET: &'static str = "EncointerCeremonies";
                const EVENT: &'static str = "InactivityCounterUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Result of the meetup at the previous ceremony"]
            pub struct MeetupEvaluated(
                pub runtime_types::encointer_primitives::communities::CommunityIdentifier,
                pub ::core::primitive::u64,
                pub runtime_types::encointer_primitives::ceremonies::MeetupResult,
            );
            impl ::subxt::events::StaticEvent for MeetupEvaluated {
                const PALLET: &'static str = "EncointerCeremonies";
                const EVENT: &'static str = "MeetupEvaluated";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                pub fn burned_bootstrapper_newbie_tickets(
                    &self,
                    _0: impl ::std::borrow::Borrow<
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    >,
                    _1: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u8>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "BurnedBootstrapperNewbieTickets",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                        ],
                        [
                            253u8, 76u8, 41u8, 232u8, 28u8, 73u8, 174u8, 34u8, 148u8, 78u8, 102u8,
                            59u8, 74u8, 163u8, 151u8, 59u8, 195u8, 19u8, 170u8, 33u8, 94u8, 225u8,
                            106u8, 223u8, 77u8, 183u8, 9u8, 211u8, 186u8, 162u8, 108u8, 126u8,
                        ],
                    )
                }
                pub fn burned_bootstrapper_newbie_tickets_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u8>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "BurnedBootstrapperNewbieTickets",
                        Vec::new(),
                        [
                            253u8, 76u8, 41u8, 232u8, 28u8, 73u8, 174u8, 34u8, 148u8, 78u8, 102u8,
                            59u8, 74u8, 163u8, 151u8, 59u8, 195u8, 19u8, 170u8, 33u8, 94u8, 225u8,
                            106u8, 223u8, 77u8, 183u8, 9u8, 211u8, 186u8, 162u8, 108u8, 126u8,
                        ],
                    )
                }
                pub fn burned_reputable_newbie_tickets(
                    &self,
                    _0: impl ::std::borrow::Borrow<(
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        ::core::primitive::u32,
                    )>,
                    _1: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u8>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "BurnedReputableNewbieTickets",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                        ],
                        [
                            138u8, 253u8, 224u8, 224u8, 173u8, 250u8, 10u8, 172u8, 248u8, 200u8,
                            250u8, 195u8, 206u8, 104u8, 22u8, 79u8, 222u8, 180u8, 246u8, 0u8, 88u8,
                            88u8, 114u8, 248u8, 126u8, 18u8, 145u8, 102u8, 199u8, 141u8, 62u8,
                            51u8,
                        ],
                    )
                }
                pub fn burned_reputable_newbie_tickets_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u8>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "BurnedReputableNewbieTickets",
                        Vec::new(),
                        [
                            138u8, 253u8, 224u8, 224u8, 173u8, 250u8, 10u8, 172u8, 248u8, 200u8,
                            250u8, 195u8, 206u8, 104u8, 22u8, 79u8, 222u8, 180u8, 246u8, 0u8, 88u8,
                            88u8, 114u8, 248u8, 126u8, 18u8, 145u8, 102u8, 199u8, 141u8, 62u8,
                            51u8,
                        ],
                    )
                }
                pub fn bootstrapper_registry(
                    &self,
                    _0: impl ::std::borrow::Borrow<(
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        ::core::primitive::u32,
                    )>,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u64>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::crypto::AccountId32>,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "BootstrapperRegistry",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                        ],
                        [
                            125u8, 176u8, 92u8, 104u8, 50u8, 147u8, 210u8, 208u8, 9u8, 152u8,
                            232u8, 96u8, 228u8, 147u8, 156u8, 158u8, 254u8, 2u8, 205u8, 237u8,
                            40u8, 234u8, 129u8, 247u8, 53u8, 12u8, 57u8, 234u8, 146u8, 213u8, 50u8,
                            115u8,
                        ],
                    )
                }
                pub fn bootstrapper_registry_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::crypto::AccountId32>,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "BootstrapperRegistry",
                        Vec::new(),
                        [
                            125u8, 176u8, 92u8, 104u8, 50u8, 147u8, 210u8, 208u8, 9u8, 152u8,
                            232u8, 96u8, 228u8, 147u8, 156u8, 158u8, 254u8, 2u8, 205u8, 237u8,
                            40u8, 234u8, 129u8, 247u8, 53u8, 12u8, 57u8, 234u8, 146u8, 213u8, 50u8,
                            115u8,
                        ],
                    )
                }
                pub fn bootstrapper_index(
                    &self,
                    _0: impl ::std::borrow::Borrow<(
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        ::core::primitive::u32,
                    )>,
                    _1: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "BootstrapperIndex",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                        ],
                        [
                            195u8, 200u8, 209u8, 216u8, 5u8, 122u8, 39u8, 211u8, 13u8, 2u8, 102u8,
                            137u8, 159u8, 170u8, 174u8, 251u8, 8u8, 255u8, 168u8, 123u8, 8u8,
                            236u8, 228u8, 182u8, 83u8, 12u8, 199u8, 188u8, 87u8, 117u8, 69u8, 39u8,
                        ],
                    )
                }
                pub fn bootstrapper_index_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "BootstrapperIndex",
                        Vec::new(),
                        [
                            195u8, 200u8, 209u8, 216u8, 5u8, 122u8, 39u8, 211u8, 13u8, 2u8, 102u8,
                            137u8, 159u8, 170u8, 174u8, 251u8, 8u8, 255u8, 168u8, 123u8, 8u8,
                            236u8, 228u8, 182u8, 83u8, 12u8, 199u8, 188u8, 87u8, 117u8, 69u8, 39u8,
                        ],
                    )
                }
                pub fn bootstrapper_count(
                    &self,
                    _0: impl ::std::borrow::Borrow<
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    >,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "BootstrapperCount",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            &(_0.borrow(), _1.borrow()),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            247u8, 217u8, 91u8, 25u8, 62u8, 252u8, 89u8, 217u8, 96u8, 17u8, 47u8,
                            225u8, 239u8, 233u8, 208u8, 130u8, 171u8, 243u8, 83u8, 51u8, 29u8,
                            131u8, 252u8, 231u8, 39u8, 141u8, 53u8, 229u8, 23u8, 183u8, 203u8,
                            76u8,
                        ],
                    )
                }
                pub fn bootstrapper_count_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "BootstrapperCount",
                        Vec::new(),
                        [
                            247u8, 217u8, 91u8, 25u8, 62u8, 252u8, 89u8, 217u8, 96u8, 17u8, 47u8,
                            225u8, 239u8, 233u8, 208u8, 130u8, 171u8, 243u8, 83u8, 51u8, 29u8,
                            131u8, 252u8, 231u8, 39u8, 141u8, 53u8, 229u8, 23u8, 183u8, 203u8,
                            76u8,
                        ],
                    )
                }
                pub fn reputable_registry(
                    &self,
                    _0: impl ::std::borrow::Borrow<(
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        ::core::primitive::u32,
                    )>,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u64>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::crypto::AccountId32>,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "ReputableRegistry",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                        ],
                        [
                            175u8, 100u8, 163u8, 46u8, 20u8, 44u8, 14u8, 11u8, 115u8, 179u8, 164u8,
                            234u8, 174u8, 21u8, 150u8, 18u8, 86u8, 208u8, 219u8, 196u8, 184u8,
                            103u8, 4u8, 140u8, 90u8, 197u8, 205u8, 126u8, 173u8, 116u8, 182u8,
                            179u8,
                        ],
                    )
                }
                pub fn reputable_registry_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::crypto::AccountId32>,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "ReputableRegistry",
                        Vec::new(),
                        [
                            175u8, 100u8, 163u8, 46u8, 20u8, 44u8, 14u8, 11u8, 115u8, 179u8, 164u8,
                            234u8, 174u8, 21u8, 150u8, 18u8, 86u8, 208u8, 219u8, 196u8, 184u8,
                            103u8, 4u8, 140u8, 90u8, 197u8, 205u8, 126u8, 173u8, 116u8, 182u8,
                            179u8,
                        ],
                    )
                }
                pub fn reputable_index(
                    &self,
                    _0: impl ::std::borrow::Borrow<(
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        ::core::primitive::u32,
                    )>,
                    _1: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "ReputableIndex",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                        ],
                        [
                            162u8, 203u8, 232u8, 89u8, 63u8, 239u8, 177u8, 76u8, 36u8, 218u8,
                            126u8, 229u8, 92u8, 65u8, 59u8, 31u8, 82u8, 169u8, 161u8, 16u8, 88u8,
                            114u8, 91u8, 11u8, 86u8, 60u8, 255u8, 0u8, 225u8, 117u8, 169u8, 245u8,
                        ],
                    )
                }
                pub fn reputable_index_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "ReputableIndex",
                        Vec::new(),
                        [
                            162u8, 203u8, 232u8, 89u8, 63u8, 239u8, 177u8, 76u8, 36u8, 218u8,
                            126u8, 229u8, 92u8, 65u8, 59u8, 31u8, 82u8, 169u8, 161u8, 16u8, 88u8,
                            114u8, 91u8, 11u8, 86u8, 60u8, 255u8, 0u8, 225u8, 117u8, 169u8, 245u8,
                        ],
                    )
                }
                pub fn reputable_count(
                    &self,
                    _0: impl ::std::borrow::Borrow<
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    >,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "ReputableCount",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            &(_0.borrow(), _1.borrow()),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            15u8, 173u8, 241u8, 167u8, 219u8, 7u8, 148u8, 17u8, 186u8, 35u8, 248u8,
                            222u8, 116u8, 138u8, 244u8, 162u8, 207u8, 151u8, 37u8, 181u8, 235u8,
                            254u8, 55u8, 249u8, 250u8, 22u8, 193u8, 87u8, 84u8, 119u8, 188u8, 70u8,
                        ],
                    )
                }
                pub fn reputable_count_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "ReputableCount",
                        Vec::new(),
                        [
                            15u8, 173u8, 241u8, 167u8, 219u8, 7u8, 148u8, 17u8, 186u8, 35u8, 248u8,
                            222u8, 116u8, 138u8, 244u8, 162u8, 207u8, 151u8, 37u8, 181u8, 235u8,
                            254u8, 55u8, 249u8, 250u8, 22u8, 193u8, 87u8, 84u8, 119u8, 188u8, 70u8,
                        ],
                    )
                }
                pub fn endorsee_registry(
                    &self,
                    _0: impl ::std::borrow::Borrow<(
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        ::core::primitive::u32,
                    )>,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u64>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::crypto::AccountId32>,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "EndorseeRegistry",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                        ],
                        [
                            63u8, 235u8, 185u8, 121u8, 72u8, 238u8, 242u8, 81u8, 135u8, 194u8,
                            32u8, 247u8, 99u8, 193u8, 65u8, 104u8, 0u8, 222u8, 98u8, 208u8, 73u8,
                            62u8, 231u8, 148u8, 49u8, 4u8, 198u8, 245u8, 211u8, 16u8, 140u8, 59u8,
                        ],
                    )
                }
                pub fn endorsee_registry_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::crypto::AccountId32>,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "EndorseeRegistry",
                        Vec::new(),
                        [
                            63u8, 235u8, 185u8, 121u8, 72u8, 238u8, 242u8, 81u8, 135u8, 194u8,
                            32u8, 247u8, 99u8, 193u8, 65u8, 104u8, 0u8, 222u8, 98u8, 208u8, 73u8,
                            62u8, 231u8, 148u8, 49u8, 4u8, 198u8, 245u8, 211u8, 16u8, 140u8, 59u8,
                        ],
                    )
                }
                pub fn endorsee_index(
                    &self,
                    _0: impl ::std::borrow::Borrow<(
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        ::core::primitive::u32,
                    )>,
                    _1: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "EndorseeIndex",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                        ],
                        [
                            205u8, 246u8, 79u8, 147u8, 45u8, 34u8, 66u8, 89u8, 177u8, 232u8, 104u8,
                            37u8, 199u8, 6u8, 195u8, 188u8, 232u8, 247u8, 140u8, 253u8, 194u8,
                            20u8, 188u8, 9u8, 201u8, 249u8, 13u8, 148u8, 138u8, 210u8, 163u8, 91u8,
                        ],
                    )
                }
                pub fn endorsee_index_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "EndorseeIndex",
                        Vec::new(),
                        [
                            205u8, 246u8, 79u8, 147u8, 45u8, 34u8, 66u8, 89u8, 177u8, 232u8, 104u8,
                            37u8, 199u8, 6u8, 195u8, 188u8, 232u8, 247u8, 140u8, 253u8, 194u8,
                            20u8, 188u8, 9u8, 201u8, 249u8, 13u8, 148u8, 138u8, 210u8, 163u8, 91u8,
                        ],
                    )
                }
                pub fn endorsee_count(
                    &self,
                    _0: impl ::std::borrow::Borrow<
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    >,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "EndorseeCount",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            &(_0.borrow(), _1.borrow()),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            128u8, 237u8, 165u8, 206u8, 55u8, 18u8, 182u8, 151u8, 31u8, 169u8,
                            192u8, 99u8, 230u8, 192u8, 243u8, 216u8, 2u8, 166u8, 176u8, 100u8,
                            222u8, 18u8, 131u8, 156u8, 13u8, 232u8, 33u8, 211u8, 146u8, 204u8,
                            170u8, 37u8,
                        ],
                    )
                }
                pub fn endorsee_count_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "EndorseeCount",
                        Vec::new(),
                        [
                            128u8, 237u8, 165u8, 206u8, 55u8, 18u8, 182u8, 151u8, 31u8, 169u8,
                            192u8, 99u8, 230u8, 192u8, 243u8, 216u8, 2u8, 166u8, 176u8, 100u8,
                            222u8, 18u8, 131u8, 156u8, 13u8, 232u8, 33u8, 211u8, 146u8, 204u8,
                            170u8, 37u8,
                        ],
                    )
                }
                pub fn newbie_registry(
                    &self,
                    _0: impl ::std::borrow::Borrow<(
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        ::core::primitive::u32,
                    )>,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u64>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::crypto::AccountId32>,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "NewbieRegistry",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                        ],
                        [
                            195u8, 86u8, 70u8, 8u8, 238u8, 10u8, 69u8, 145u8, 107u8, 230u8, 93u8,
                            0u8, 95u8, 84u8, 231u8, 73u8, 193u8, 221u8, 140u8, 186u8, 10u8, 194u8,
                            35u8, 171u8, 183u8, 61u8, 244u8, 23u8, 30u8, 179u8, 74u8, 73u8,
                        ],
                    )
                }
                pub fn newbie_registry_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::crypto::AccountId32>,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "NewbieRegistry",
                        Vec::new(),
                        [
                            195u8, 86u8, 70u8, 8u8, 238u8, 10u8, 69u8, 145u8, 107u8, 230u8, 93u8,
                            0u8, 95u8, 84u8, 231u8, 73u8, 193u8, 221u8, 140u8, 186u8, 10u8, 194u8,
                            35u8, 171u8, 183u8, 61u8, 244u8, 23u8, 30u8, 179u8, 74u8, 73u8,
                        ],
                    )
                }
                pub fn newbie_index(
                    &self,
                    _0: impl ::std::borrow::Borrow<(
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        ::core::primitive::u32,
                    )>,
                    _1: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "NewbieIndex",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                        ],
                        [
                            168u8, 33u8, 47u8, 174u8, 133u8, 63u8, 157u8, 246u8, 38u8, 79u8, 152u8,
                            146u8, 117u8, 29u8, 173u8, 170u8, 190u8, 178u8, 231u8, 210u8, 224u8,
                            253u8, 152u8, 181u8, 130u8, 20u8, 7u8, 147u8, 31u8, 84u8, 86u8, 155u8,
                        ],
                    )
                }
                pub fn newbie_index_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "NewbieIndex",
                        Vec::new(),
                        [
                            168u8, 33u8, 47u8, 174u8, 133u8, 63u8, 157u8, 246u8, 38u8, 79u8, 152u8,
                            146u8, 117u8, 29u8, 173u8, 170u8, 190u8, 178u8, 231u8, 210u8, 224u8,
                            253u8, 152u8, 181u8, 130u8, 20u8, 7u8, 147u8, 31u8, 84u8, 86u8, 155u8,
                        ],
                    )
                }
                pub fn newbie_count(
                    &self,
                    _0: impl ::std::borrow::Borrow<
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    >,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "NewbieCount",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            &(_0.borrow(), _1.borrow()),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            134u8, 124u8, 111u8, 105u8, 179u8, 108u8, 5u8, 244u8, 132u8, 95u8,
                            196u8, 239u8, 27u8, 123u8, 115u8, 5u8, 32u8, 20u8, 39u8, 61u8, 82u8,
                            112u8, 112u8, 235u8, 5u8, 32u8, 112u8, 4u8, 68u8, 250u8, 134u8, 188u8,
                        ],
                    )
                }
                pub fn newbie_count_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "NewbieCount",
                        Vec::new(),
                        [
                            134u8, 124u8, 111u8, 105u8, 179u8, 108u8, 5u8, 244u8, 132u8, 95u8,
                            196u8, 239u8, 27u8, 123u8, 115u8, 5u8, 32u8, 20u8, 39u8, 61u8, 82u8,
                            112u8, 112u8, 235u8, 5u8, 32u8, 112u8, 4u8, 68u8, 250u8, 134u8, 188u8,
                        ],
                    )
                }
                pub fn assignment_counts(
                    &self,
                    _0: impl ::std::borrow::Borrow<
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    >,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::encointer_primitives::ceremonies::AssignmentCount,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "AssignmentCounts",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            &(_0.borrow(), _1.borrow()),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            25u8, 12u8, 130u8, 15u8, 69u8, 95u8, 67u8, 61u8, 201u8, 164u8, 103u8,
                            174u8, 102u8, 57u8, 88u8, 193u8, 159u8, 76u8, 104u8, 77u8, 13u8, 164u8,
                            6u8, 233u8, 7u8, 70u8, 137u8, 64u8, 155u8, 75u8, 249u8, 169u8,
                        ],
                    )
                }
                pub fn assignment_counts_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::encointer_primitives::ceremonies::AssignmentCount,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "AssignmentCounts",
                        Vec::new(),
                        [
                            25u8, 12u8, 130u8, 15u8, 69u8, 95u8, 67u8, 61u8, 201u8, 164u8, 103u8,
                            174u8, 102u8, 57u8, 88u8, 193u8, 159u8, 76u8, 104u8, 77u8, 13u8, 164u8,
                            6u8, 233u8, 7u8, 70u8, 137u8, 64u8, 155u8, 75u8, 249u8, 169u8,
                        ],
                    )
                }
                pub fn assignments(
                    &self,
                    _0: impl ::std::borrow::Borrow<
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    >,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::encointer_primitives::ceremonies::Assignment,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "Assignments",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            &(_0.borrow(), _1.borrow()),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            153u8, 24u8, 51u8, 135u8, 218u8, 241u8, 35u8, 95u8, 111u8, 162u8, 27u8,
                            193u8, 37u8, 230u8, 181u8, 234u8, 26u8, 139u8, 165u8, 127u8, 153u8,
                            223u8, 51u8, 181u8, 22u8, 221u8, 132u8, 246u8, 122u8, 213u8, 191u8,
                            219u8,
                        ],
                    )
                }
                pub fn assignments_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::encointer_primitives::ceremonies::Assignment,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "Assignments",
                        Vec::new(),
                        [
                            153u8, 24u8, 51u8, 135u8, 218u8, 241u8, 35u8, 95u8, 111u8, 162u8, 27u8,
                            193u8, 37u8, 230u8, 181u8, 234u8, 26u8, 139u8, 165u8, 127u8, 153u8,
                            223u8, 51u8, 181u8, 22u8, 221u8, 132u8, 246u8, 122u8, 213u8, 191u8,
                            219u8,
                        ],
                    )
                }
                pub fn participant_reputation(
                    &self,
                    _0: impl ::std::borrow::Borrow<(
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        ::core::primitive::u32,
                    )>,
                    _1: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::encointer_primitives::ceremonies::Reputation,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "ParticipantReputation",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                        ],
                        [
                            126u8, 148u8, 55u8, 147u8, 75u8, 159u8, 248u8, 107u8, 233u8, 93u8,
                            150u8, 252u8, 203u8, 86u8, 13u8, 149u8, 55u8, 173u8, 128u8, 248u8,
                            189u8, 39u8, 253u8, 58u8, 121u8, 190u8, 22u8, 89u8, 99u8, 14u8, 202u8,
                            232u8,
                        ],
                    )
                }
                pub fn participant_reputation_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::encointer_primitives::ceremonies::Reputation,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "ParticipantReputation",
                        Vec::new(),
                        [
                            126u8, 148u8, 55u8, 147u8, 75u8, 159u8, 248u8, 107u8, 233u8, 93u8,
                            150u8, 252u8, 203u8, 86u8, 13u8, 149u8, 55u8, 173u8, 128u8, 248u8,
                            189u8, 39u8, 253u8, 58u8, 121u8, 190u8, 22u8, 89u8, 99u8, 14u8, 202u8,
                            232u8,
                        ],
                    )
                }
                #[doc = " Accounts that have been endorsed by a reputable or a bootstrapper."]
                #[doc = ""]
                #[doc = " This is not the same as `EndorseeRegistry`, which contains the `Endorsees` who"]
                #[doc = " have registered for a meetup."]
                pub fn endorsees(
                    &self,
                    _0: impl ::std::borrow::Borrow<(
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        ::core::primitive::u32,
                    )>,
                    _1: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<()>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "Endorsees",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                        ],
                        [
                            248u8, 220u8, 240u8, 147u8, 149u8, 107u8, 198u8, 255u8, 196u8, 200u8,
                            136u8, 140u8, 113u8, 149u8, 209u8, 27u8, 217u8, 241u8, 25u8, 181u8,
                            95u8, 15u8, 91u8, 144u8, 81u8, 127u8, 245u8, 169u8, 231u8, 232u8, 24u8,
                            33u8,
                        ],
                    )
                }
                #[doc = " Accounts that have been endorsed by a reputable or a bootstrapper."]
                #[doc = ""]
                #[doc = " This is not the same as `EndorseeRegistry`, which contains the `Endorsees` who"]
                #[doc = " have registered for a meetup."]
                pub fn endorsees_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<()>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "Endorsees",
                        Vec::new(),
                        [
                            248u8, 220u8, 240u8, 147u8, 149u8, 107u8, 198u8, 255u8, 196u8, 200u8,
                            136u8, 140u8, 113u8, 149u8, 209u8, 27u8, 217u8, 241u8, 25u8, 181u8,
                            95u8, 15u8, 91u8, 144u8, 81u8, 127u8, 245u8, 169u8, 231u8, 232u8, 24u8,
                            33u8,
                        ],
                    )
                }
                pub fn endorsees_count(
                    &self,
                    _0: impl ::std::borrow::Borrow<
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    >,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "EndorseesCount",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            &(_0.borrow(), _1.borrow()),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            127u8, 233u8, 38u8, 138u8, 252u8, 83u8, 42u8, 91u8, 107u8, 31u8, 131u8,
                            133u8, 76u8, 11u8, 212u8, 28u8, 201u8, 153u8, 142u8, 83u8, 26u8, 133u8,
                            29u8, 121u8, 129u8, 167u8, 224u8, 12u8, 105u8, 153u8, 74u8, 59u8,
                        ],
                    )
                }
                pub fn endorsees_count_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "EndorseesCount",
                        Vec::new(),
                        [
                            127u8, 233u8, 38u8, 138u8, 252u8, 83u8, 42u8, 91u8, 107u8, 31u8, 131u8,
                            133u8, 76u8, 11u8, 212u8, 28u8, 201u8, 153u8, 142u8, 83u8, 26u8, 133u8,
                            29u8, 121u8, 129u8, 167u8, 224u8, 12u8, 105u8, 153u8, 74u8, 59u8,
                        ],
                    )
                }
                pub fn meetup_count(
                    &self,
                    _0: impl ::std::borrow::Borrow<
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    >,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "MeetupCount",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            &(_0.borrow(), _1.borrow()),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            164u8, 156u8, 186u8, 50u8, 91u8, 33u8, 254u8, 209u8, 9u8, 30u8, 120u8,
                            209u8, 133u8, 12u8, 201u8, 228u8, 251u8, 244u8, 12u8, 13u8, 170u8,
                            157u8, 216u8, 156u8, 62u8, 6u8, 48u8, 180u8, 85u8, 239u8, 143u8, 99u8,
                        ],
                    )
                }
                pub fn meetup_count_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "MeetupCount",
                        Vec::new(),
                        [
                            164u8, 156u8, 186u8, 50u8, 91u8, 33u8, 254u8, 209u8, 9u8, 30u8, 120u8,
                            209u8, 133u8, 12u8, 201u8, 228u8, 251u8, 244u8, 12u8, 13u8, 170u8,
                            157u8, 216u8, 156u8, 62u8, 6u8, 48u8, 180u8, 85u8, 239u8, 143u8, 99u8,
                        ],
                    )
                }
                pub fn attestation_registry(
                    &self,
                    _0: impl ::std::borrow::Borrow<(
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        ::core::primitive::u32,
                    )>,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u64>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "AttestationRegistry",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                        ],
                        [
                            184u8, 242u8, 221u8, 42u8, 40u8, 91u8, 24u8, 94u8, 50u8, 100u8, 173u8,
                            143u8, 29u8, 232u8, 199u8, 87u8, 133u8, 121u8, 63u8, 192u8, 238u8,
                            158u8, 147u8, 240u8, 118u8, 250u8, 37u8, 255u8, 173u8, 6u8, 88u8,
                            220u8,
                        ],
                    )
                }
                pub fn attestation_registry_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "AttestationRegistry",
                        Vec::new(),
                        [
                            184u8, 242u8, 221u8, 42u8, 40u8, 91u8, 24u8, 94u8, 50u8, 100u8, 173u8,
                            143u8, 29u8, 232u8, 199u8, 87u8, 133u8, 121u8, 63u8, 192u8, 238u8,
                            158u8, 147u8, 240u8, 118u8, 250u8, 37u8, 255u8, 173u8, 6u8, 88u8,
                            220u8,
                        ],
                    )
                }
                pub fn attestation_index(
                    &self,
                    _0: impl ::std::borrow::Borrow<(
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        ::core::primitive::u32,
                    )>,
                    _1: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "AttestationIndex",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                        ],
                        [
                            215u8, 244u8, 174u8, 93u8, 68u8, 110u8, 213u8, 233u8, 16u8, 33u8, 94u8,
                            245u8, 55u8, 228u8, 27u8, 39u8, 158u8, 102u8, 229u8, 120u8, 60u8,
                            102u8, 30u8, 66u8, 251u8, 95u8, 46u8, 108u8, 78u8, 10u8, 157u8, 141u8,
                        ],
                    )
                }
                pub fn attestation_index_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "AttestationIndex",
                        Vec::new(),
                        [
                            215u8, 244u8, 174u8, 93u8, 68u8, 110u8, 213u8, 233u8, 16u8, 33u8, 94u8,
                            245u8, 55u8, 228u8, 27u8, 39u8, 158u8, 102u8, 229u8, 120u8, 60u8,
                            102u8, 30u8, 66u8, 251u8, 95u8, 46u8, 108u8, 78u8, 10u8, 157u8, 141u8,
                        ],
                    )
                }
                pub fn attestation_count(
                    &self,
                    _0: impl ::std::borrow::Borrow<
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    >,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "AttestationCount",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            &(_0.borrow(), _1.borrow()),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            122u8, 174u8, 62u8, 50u8, 255u8, 135u8, 27u8, 180u8, 52u8, 35u8, 135u8,
                            59u8, 211u8, 76u8, 98u8, 120u8, 99u8, 240u8, 156u8, 235u8, 140u8,
                            181u8, 239u8, 5u8, 52u8, 155u8, 253u8, 114u8, 131u8, 179u8, 39u8,
                            214u8,
                        ],
                    )
                }
                pub fn attestation_count_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "AttestationCount",
                        Vec::new(),
                        [
                            122u8, 174u8, 62u8, 50u8, 255u8, 135u8, 27u8, 180u8, 52u8, 35u8, 135u8,
                            59u8, 211u8, 76u8, 98u8, 120u8, 99u8, 240u8, 156u8, 235u8, 140u8,
                            181u8, 239u8, 5u8, 52u8, 155u8, 253u8, 114u8, 131u8, 179u8, 39u8,
                            214u8,
                        ],
                    )
                }
                pub fn meetup_participant_count_vote(
                    &self,
                    _0: impl ::std::borrow::Borrow<(
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        ::core::primitive::u32,
                    )>,
                    _1: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "MeetupParticipantCountVote",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                        ],
                        [
                            26u8, 143u8, 162u8, 236u8, 46u8, 232u8, 212u8, 103u8, 177u8, 27u8,
                            253u8, 169u8, 55u8, 26u8, 240u8, 95u8, 164u8, 50u8, 134u8, 33u8, 2u8,
                            212u8, 108u8, 59u8, 216u8, 148u8, 50u8, 16u8, 41u8, 145u8, 146u8, 24u8,
                        ],
                    )
                }
                pub fn meetup_participant_count_vote_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "MeetupParticipantCountVote",
                        Vec::new(),
                        [
                            26u8, 143u8, 162u8, 236u8, 46u8, 232u8, 212u8, 103u8, 177u8, 27u8,
                            253u8, 169u8, 55u8, 26u8, 240u8, 95u8, 164u8, 50u8, 134u8, 33u8, 2u8,
                            212u8, 108u8, 59u8, 216u8, 148u8, 50u8, 16u8, 41u8, 145u8, 146u8, 24u8,
                        ],
                    )
                }
                #[doc = " the default UBI for a ceremony attendee if no community specific value is set."]
                pub fn ceremony_reward(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::substrate_fixed::FixedI128<
                            runtime_types::typenum::uint::UInt<
                                runtime_types::typenum::uint::UInt<
                                    runtime_types::typenum::uint::UInt<
                                        runtime_types::typenum::uint::UInt<
                                            runtime_types::typenum::uint::UInt<
                                                runtime_types::typenum::uint::UInt<
                                                    runtime_types::typenum::uint::UInt<
                                                        runtime_types::typenum::uint::UTerm,
                                                        runtime_types::typenum::bit::B1,
                                                    >,
                                                    runtime_types::typenum::bit::B0,
                                                >,
                                                runtime_types::typenum::bit::B0,
                                            >,
                                            runtime_types::typenum::bit::B0,
                                        >,
                                        runtime_types::typenum::bit::B0,
                                    >,
                                    runtime_types::typenum::bit::B0,
                                >,
                                runtime_types::typenum::bit::B0,
                            >,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "CeremonyReward",
                        vec![],
                        [
                            211u8, 181u8, 142u8, 232u8, 247u8, 106u8, 208u8, 53u8, 96u8, 251u8,
                            164u8, 54u8, 171u8, 207u8, 62u8, 156u8, 233u8, 217u8, 204u8, 80u8,
                            25u8, 98u8, 65u8, 72u8, 56u8, 221u8, 176u8, 36u8, 219u8, 254u8, 131u8,
                            216u8,
                        ],
                    )
                }
                pub fn location_tolerance(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "LocationTolerance",
                        vec![],
                        [
                            189u8, 104u8, 89u8, 0u8, 73u8, 138u8, 116u8, 63u8, 174u8, 202u8, 152u8,
                            88u8, 88u8, 143u8, 122u8, 193u8, 85u8, 107u8, 238u8, 198u8, 210u8,
                            23u8, 72u8, 135u8, 22u8, 179u8, 118u8, 4u8, 172u8, 49u8, 63u8, 225u8,
                        ],
                    )
                }
                pub fn time_tolerance(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "TimeTolerance",
                        vec![],
                        [
                            23u8, 251u8, 150u8, 54u8, 134u8, 77u8, 59u8, 254u8, 235u8, 248u8, 83u8,
                            141u8, 172u8, 113u8, 112u8, 89u8, 145u8, 175u8, 7u8, 58u8, 118u8,
                            152u8, 239u8, 52u8, 75u8, 96u8, 183u8, 218u8, 13u8, 136u8, 140u8,
                            225u8,
                        ],
                    )
                }
                pub fn issued_rewards(
                    &self,
                    _0: impl ::std::borrow::Borrow<(
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        ::core::primitive::u32,
                    )>,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u64>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::encointer_primitives::ceremonies::MeetupResult,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "IssuedRewards",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                        ],
                        [
                            166u8, 208u8, 245u8, 79u8, 131u8, 85u8, 36u8, 147u8, 230u8, 91u8,
                            255u8, 166u8, 210u8, 181u8, 46u8, 186u8, 243u8, 23u8, 152u8, 80u8,
                            66u8, 48u8, 0u8, 100u8, 224u8, 7u8, 255u8, 11u8, 8u8, 60u8, 199u8,
                            114u8,
                        ],
                    )
                }
                pub fn issued_rewards_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::encointer_primitives::ceremonies::MeetupResult,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "IssuedRewards",
                        Vec::new(),
                        [
                            166u8, 208u8, 245u8, 79u8, 131u8, 85u8, 36u8, 147u8, 230u8, 91u8,
                            255u8, 166u8, 210u8, 181u8, 46u8, 186u8, 243u8, 23u8, 152u8, 80u8,
                            66u8, 48u8, 0u8, 100u8, 224u8, 7u8, 255u8, 11u8, 8u8, 60u8, 199u8,
                            114u8,
                        ],
                    )
                }
                pub fn inactivity_counters(
                    &self,
                    _0: impl ::std::borrow::Borrow<
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    >,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "InactivityCounters",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            231u8, 254u8, 15u8, 119u8, 223u8, 195u8, 29u8, 77u8, 218u8, 114u8,
                            37u8, 117u8, 117u8, 106u8, 225u8, 198u8, 213u8, 134u8, 160u8, 100u8,
                            1u8, 144u8, 6u8, 43u8, 40u8, 65u8, 75u8, 165u8, 217u8, 238u8, 98u8,
                            117u8,
                        ],
                    )
                }
                pub fn inactivity_counters_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "InactivityCounters",
                        Vec::new(),
                        [
                            231u8, 254u8, 15u8, 119u8, 223u8, 195u8, 29u8, 77u8, 218u8, 114u8,
                            37u8, 117u8, 117u8, 106u8, 225u8, 198u8, 213u8, 134u8, 160u8, 100u8,
                            1u8, 144u8, 6u8, 43u8, 40u8, 65u8, 75u8, 165u8, 217u8, 238u8, 98u8,
                            117u8,
                        ],
                    )
                }
                #[doc = " The number of ceremony cycles a community can skip ceremonies before it gets purged"]
                pub fn inactivity_timeout(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "InactivityTimeout",
                        vec![],
                        [
                            28u8, 99u8, 117u8, 65u8, 72u8, 153u8, 47u8, 139u8, 162u8, 130u8, 109u8,
                            176u8, 127u8, 238u8, 237u8, 26u8, 217u8, 196u8, 8u8, 136u8, 155u8,
                            243u8, 155u8, 82u8, 135u8, 239u8, 245u8, 139u8, 166u8, 119u8, 127u8,
                            233u8,
                        ],
                    )
                }
                #[doc = " The number of newbies a bootstrapper can endorse to accelerate community growth"]
                pub fn endorsement_tickets_per_bootstrapper(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u8>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "EndorsementTicketsPerBootstrapper",
                        vec![],
                        [
                            137u8, 65u8, 186u8, 173u8, 224u8, 110u8, 173u8, 141u8, 125u8, 189u8,
                            167u8, 103u8, 140u8, 93u8, 79u8, 87u8, 88u8, 144u8, 254u8, 61u8, 255u8,
                            80u8, 1u8, 255u8, 98u8, 77u8, 164u8, 241u8, 146u8, 106u8, 48u8, 193u8,
                        ],
                    )
                }
                #[doc = " The number of newbies a reputable can endorse per cycle to accelerate community growth"]
                pub fn endorsement_tickets_per_reputable(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u8>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "EndorsementTicketsPerReputable",
                        vec![],
                        [
                            183u8, 0u8, 85u8, 177u8, 4u8, 70u8, 161u8, 116u8, 219u8, 33u8, 236u8,
                            40u8, 227u8, 150u8, 112u8, 65u8, 14u8, 118u8, 211u8, 249u8, 105u8,
                            159u8, 186u8, 77u8, 142u8, 181u8, 148u8, 235u8, 195u8, 248u8, 136u8,
                            225u8,
                        ],
                    )
                }
                #[doc = " The number of ceremony cycles that a participant's reputation is valid for"]
                pub fn reputation_lifetime(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "ReputationLifetime",
                        vec![],
                        [
                            189u8, 90u8, 152u8, 192u8, 94u8, 134u8, 53u8, 98u8, 162u8, 14u8, 23u8,
                            106u8, 159u8, 38u8, 150u8, 58u8, 87u8, 63u8, 71u8, 147u8, 170u8, 226u8,
                            157u8, 114u8, 37u8, 176u8, 214u8, 179u8, 172u8, 234u8, 55u8, 11u8,
                        ],
                    )
                }
                pub fn meetup_time_offset(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::i32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCeremonies",
                        "MeetupTimeOffset",
                        vec![],
                        [
                            198u8, 119u8, 240u8, 4u8, 66u8, 16u8, 25u8, 12u8, 195u8, 190u8, 224u8,
                            135u8, 113u8, 38u8, 23u8, 250u8, 145u8, 156u8, 79u8, 30u8, 31u8, 134u8,
                            240u8, 78u8, 251u8, 238u8, 65u8, 8u8, 223u8, 12u8, 158u8, 72u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn meetup_size_target(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "EncointerCeremonies",
                        "MeetupSizeTarget",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
                pub fn meetup_min_size(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "EncointerCeremonies",
                        "MeetupMinSize",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
                pub fn meetup_newbie_limit_divider(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "EncointerCeremonies",
                        "MeetupNewbieLimitDivider",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod encointer_communities {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct NewCommunity {
                pub location: runtime_types::encointer_primitives::communities::Location,
                pub bootstrappers: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
                pub community_metadata:
                    runtime_types::encointer_primitives::communities::CommunityMetadata,
                pub demurrage: ::core::option::Option<
                    runtime_types::substrate_fixed::FixedI128<
                        runtime_types::typenum::uint::UInt<
                            runtime_types::typenum::uint::UInt<
                                runtime_types::typenum::uint::UInt<
                                    runtime_types::typenum::uint::UInt<
                                        runtime_types::typenum::uint::UInt<
                                            runtime_types::typenum::uint::UInt<
                                                runtime_types::typenum::uint::UInt<
                                                    runtime_types::typenum::uint::UTerm,
                                                    runtime_types::typenum::bit::B1,
                                                >,
                                                runtime_types::typenum::bit::B0,
                                            >,
                                            runtime_types::typenum::bit::B0,
                                        >,
                                        runtime_types::typenum::bit::B0,
                                    >,
                                    runtime_types::typenum::bit::B0,
                                >,
                                runtime_types::typenum::bit::B0,
                            >,
                            runtime_types::typenum::bit::B0,
                        >,
                    >,
                >,
                pub nominal_income: ::core::option::Option<
                    runtime_types::substrate_fixed::FixedI128<
                        runtime_types::typenum::uint::UInt<
                            runtime_types::typenum::uint::UInt<
                                runtime_types::typenum::uint::UInt<
                                    runtime_types::typenum::uint::UInt<
                                        runtime_types::typenum::uint::UInt<
                                            runtime_types::typenum::uint::UInt<
                                                runtime_types::typenum::uint::UInt<
                                                    runtime_types::typenum::uint::UTerm,
                                                    runtime_types::typenum::bit::B1,
                                                >,
                                                runtime_types::typenum::bit::B0,
                                            >,
                                            runtime_types::typenum::bit::B0,
                                        >,
                                        runtime_types::typenum::bit::B0,
                                    >,
                                    runtime_types::typenum::bit::B0,
                                >,
                                runtime_types::typenum::bit::B0,
                            >,
                            runtime_types::typenum::bit::B0,
                        >,
                    >,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct AddLocation {
                pub cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                pub location: runtime_types::encointer_primitives::communities::Location,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct RemoveLocation {
                pub cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                pub location: runtime_types::encointer_primitives::communities::Location,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct UpdateCommunityMetadata {
                pub cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                pub community_metadata:
                    runtime_types::encointer_primitives::communities::CommunityMetadata,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct UpdateDemurrage {
                pub cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                pub demurrage: runtime_types::substrate_fixed::FixedI128<
                    runtime_types::typenum::uint::UInt<
                        runtime_types::typenum::uint::UInt<
                            runtime_types::typenum::uint::UInt<
                                runtime_types::typenum::uint::UInt<
                                    runtime_types::typenum::uint::UInt<
                                        runtime_types::typenum::uint::UInt<
                                            runtime_types::typenum::uint::UInt<
                                                runtime_types::typenum::uint::UTerm,
                                                runtime_types::typenum::bit::B1,
                                            >,
                                            runtime_types::typenum::bit::B0,
                                        >,
                                        runtime_types::typenum::bit::B0,
                                    >,
                                    runtime_types::typenum::bit::B0,
                                >,
                                runtime_types::typenum::bit::B0,
                            >,
                            runtime_types::typenum::bit::B0,
                        >,
                        runtime_types::typenum::bit::B0,
                    >,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct UpdateNominalIncome {
                pub cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                pub nominal_income: runtime_types::substrate_fixed::FixedI128<
                    runtime_types::typenum::uint::UInt<
                        runtime_types::typenum::uint::UInt<
                            runtime_types::typenum::uint::UInt<
                                runtime_types::typenum::uint::UInt<
                                    runtime_types::typenum::uint::UInt<
                                        runtime_types::typenum::uint::UInt<
                                            runtime_types::typenum::uint::UInt<
                                                runtime_types::typenum::uint::UTerm,
                                                runtime_types::typenum::bit::B1,
                                            >,
                                            runtime_types::typenum::bit::B0,
                                        >,
                                        runtime_types::typenum::bit::B0,
                                    >,
                                    runtime_types::typenum::bit::B0,
                                >,
                                runtime_types::typenum::bit::B0,
                            >,
                            runtime_types::typenum::bit::B0,
                        >,
                        runtime_types::typenum::bit::B0,
                    >,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct SetMinSolarTripTimeS {
                pub min_solar_trip_time_s: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct SetMaxSpeedMps {
                pub max_speed_mps: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct PurgeCommunity {
                pub cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Add a new community."]
                #[doc = ""]
                #[doc = "May only be called from `T::TrustableForNonDestructiveAction`."]
                pub fn new_community(
                    &self,
                    location: runtime_types::encointer_primitives::communities::Location,
                    bootstrappers: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
                    community_metadata : runtime_types :: encointer_primitives :: communities :: CommunityMetadata,
                    demurrage: ::core::option::Option<
                        runtime_types::substrate_fixed::FixedI128<
                            runtime_types::typenum::uint::UInt<
                                runtime_types::typenum::uint::UInt<
                                    runtime_types::typenum::uint::UInt<
                                        runtime_types::typenum::uint::UInt<
                                            runtime_types::typenum::uint::UInt<
                                                runtime_types::typenum::uint::UInt<
                                                    runtime_types::typenum::uint::UInt<
                                                        runtime_types::typenum::uint::UTerm,
                                                        runtime_types::typenum::bit::B1,
                                                    >,
                                                    runtime_types::typenum::bit::B0,
                                                >,
                                                runtime_types::typenum::bit::B0,
                                            >,
                                            runtime_types::typenum::bit::B0,
                                        >,
                                        runtime_types::typenum::bit::B0,
                                    >,
                                    runtime_types::typenum::bit::B0,
                                >,
                                runtime_types::typenum::bit::B0,
                            >,
                        >,
                    >,
                    nominal_income: ::core::option::Option<
                        runtime_types::substrate_fixed::FixedI128<
                            runtime_types::typenum::uint::UInt<
                                runtime_types::typenum::uint::UInt<
                                    runtime_types::typenum::uint::UInt<
                                        runtime_types::typenum::uint::UInt<
                                            runtime_types::typenum::uint::UInt<
                                                runtime_types::typenum::uint::UInt<
                                                    runtime_types::typenum::uint::UInt<
                                                        runtime_types::typenum::uint::UTerm,
                                                        runtime_types::typenum::bit::B1,
                                                    >,
                                                    runtime_types::typenum::bit::B0,
                                                >,
                                                runtime_types::typenum::bit::B0,
                                            >,
                                            runtime_types::typenum::bit::B0,
                                        >,
                                        runtime_types::typenum::bit::B0,
                                    >,
                                    runtime_types::typenum::bit::B0,
                                >,
                                runtime_types::typenum::bit::B0,
                            >,
                        >,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<NewCommunity> {
                    ::subxt::tx::StaticTxPayload::new(
                        "EncointerCommunities",
                        "new_community",
                        NewCommunity {
                            location,
                            bootstrappers,
                            community_metadata,
                            demurrage,
                            nominal_income,
                        },
                        [
                            36u8, 205u8, 160u8, 245u8, 224u8, 197u8, 82u8, 7u8, 71u8, 148u8, 201u8,
                            217u8, 5u8, 141u8, 31u8, 16u8, 5u8, 227u8, 112u8, 107u8, 228u8, 118u8,
                            166u8, 204u8, 215u8, 3u8, 197u8, 175u8, 166u8, 206u8, 44u8, 89u8,
                        ],
                    )
                }
                #[doc = "Add a new meetup `location` to the community with `cid`."]
                #[doc = ""]
                #[doc = "May only be called from `T::TrustableForNonDestructiveAction`."]
                #[doc = ""]
                #[doc = "Todo: Replace `T::CommunityMaster` with community governance: #137."]
                pub fn add_location(
                    &self,
                    cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    location: runtime_types::encointer_primitives::communities::Location,
                ) -> ::subxt::tx::StaticTxPayload<AddLocation> {
                    ::subxt::tx::StaticTxPayload::new(
                        "EncointerCommunities",
                        "add_location",
                        AddLocation { cid, location },
                        [
                            103u8, 165u8, 97u8, 211u8, 72u8, 231u8, 146u8, 82u8, 103u8, 143u8,
                            150u8, 198u8, 8u8, 204u8, 128u8, 176u8, 144u8, 45u8, 178u8, 232u8,
                            182u8, 105u8, 60u8, 211u8, 109u8, 151u8, 222u8, 166u8, 128u8, 63u8,
                            37u8, 36u8,
                        ],
                    )
                }
                #[doc = "Remove an existing meetup `location` from the community with `cid`."]
                #[doc = ""]
                #[doc = "May only be called from `T::CommunityMaster`."]
                #[doc = ""]
                #[doc = "Todo: Replace `T::CommunityMaster` with community governance: #137."]
                pub fn remove_location(
                    &self,
                    cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    location: runtime_types::encointer_primitives::communities::Location,
                ) -> ::subxt::tx::StaticTxPayload<RemoveLocation> {
                    ::subxt::tx::StaticTxPayload::new(
                        "EncointerCommunities",
                        "remove_location",
                        RemoveLocation { cid, location },
                        [
                            221u8, 93u8, 213u8, 72u8, 236u8, 241u8, 161u8, 141u8, 71u8, 117u8,
                            147u8, 11u8, 220u8, 240u8, 75u8, 94u8, 0u8, 73u8, 50u8, 122u8, 36u8,
                            3u8, 254u8, 159u8, 203u8, 241u8, 212u8, 25u8, 231u8, 161u8, 9u8, 133u8,
                        ],
                    )
                }
                #[doc = "Update the metadata of the community with `cid`."]
                #[doc = ""]
                #[doc = "May only be called from `T::CommunityMaster`."]
                pub fn update_community_metadata(
                    &self,
                    cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    community_metadata : runtime_types :: encointer_primitives :: communities :: CommunityMetadata,
                ) -> ::subxt::tx::StaticTxPayload<UpdateCommunityMetadata> {
                    ::subxt::tx::StaticTxPayload::new(
                        "EncointerCommunities",
                        "update_community_metadata",
                        UpdateCommunityMetadata {
                            cid,
                            community_metadata,
                        },
                        [
                            25u8, 16u8, 138u8, 183u8, 156u8, 204u8, 208u8, 127u8, 59u8, 18u8,
                            147u8, 8u8, 186u8, 75u8, 231u8, 159u8, 223u8, 231u8, 230u8, 199u8,
                            86u8, 113u8, 128u8, 0u8, 185u8, 241u8, 204u8, 4u8, 69u8, 219u8, 184u8,
                            228u8,
                        ],
                    )
                }
                pub fn update_demurrage(
                    &self,
                    cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    demurrage: runtime_types::substrate_fixed::FixedI128<
                        runtime_types::typenum::uint::UInt<
                            runtime_types::typenum::uint::UInt<
                                runtime_types::typenum::uint::UInt<
                                    runtime_types::typenum::uint::UInt<
                                        runtime_types::typenum::uint::UInt<
                                            runtime_types::typenum::uint::UInt<
                                                runtime_types::typenum::uint::UInt<
                                                    runtime_types::typenum::uint::UTerm,
                                                    runtime_types::typenum::bit::B1,
                                                >,
                                                runtime_types::typenum::bit::B0,
                                            >,
                                            runtime_types::typenum::bit::B0,
                                        >,
                                        runtime_types::typenum::bit::B0,
                                    >,
                                    runtime_types::typenum::bit::B0,
                                >,
                                runtime_types::typenum::bit::B0,
                            >,
                            runtime_types::typenum::bit::B0,
                        >,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<UpdateDemurrage> {
                    ::subxt::tx::StaticTxPayload::new(
                        "EncointerCommunities",
                        "update_demurrage",
                        UpdateDemurrage { cid, demurrage },
                        [
                            81u8, 175u8, 97u8, 148u8, 176u8, 114u8, 65u8, 147u8, 138u8, 230u8,
                            165u8, 76u8, 195u8, 134u8, 224u8, 25u8, 132u8, 32u8, 177u8, 203u8,
                            139u8, 76u8, 226u8, 28u8, 31u8, 53u8, 251u8, 241u8, 253u8, 58u8, 143u8,
                            150u8,
                        ],
                    )
                }
                pub fn update_nominal_income(
                    &self,
                    cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    nominal_income: runtime_types::substrate_fixed::FixedI128<
                        runtime_types::typenum::uint::UInt<
                            runtime_types::typenum::uint::UInt<
                                runtime_types::typenum::uint::UInt<
                                    runtime_types::typenum::uint::UInt<
                                        runtime_types::typenum::uint::UInt<
                                            runtime_types::typenum::uint::UInt<
                                                runtime_types::typenum::uint::UInt<
                                                    runtime_types::typenum::uint::UTerm,
                                                    runtime_types::typenum::bit::B1,
                                                >,
                                                runtime_types::typenum::bit::B0,
                                            >,
                                            runtime_types::typenum::bit::B0,
                                        >,
                                        runtime_types::typenum::bit::B0,
                                    >,
                                    runtime_types::typenum::bit::B0,
                                >,
                                runtime_types::typenum::bit::B0,
                            >,
                            runtime_types::typenum::bit::B0,
                        >,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<UpdateNominalIncome> {
                    ::subxt::tx::StaticTxPayload::new(
                        "EncointerCommunities",
                        "update_nominal_income",
                        UpdateNominalIncome {
                            cid,
                            nominal_income,
                        },
                        [
                            42u8, 31u8, 249u8, 94u8, 69u8, 193u8, 17u8, 225u8, 111u8, 245u8, 97u8,
                            64u8, 13u8, 172u8, 167u8, 181u8, 253u8, 122u8, 110u8, 162u8, 26u8,
                            96u8, 247u8, 192u8, 99u8, 40u8, 122u8, 132u8, 175u8, 227u8, 165u8,
                            71u8,
                        ],
                    )
                }
                pub fn set_min_solar_trip_time_s(
                    &self,
                    min_solar_trip_time_s: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<SetMinSolarTripTimeS> {
                    ::subxt::tx::StaticTxPayload::new(
                        "EncointerCommunities",
                        "set_min_solar_trip_time_s",
                        SetMinSolarTripTimeS {
                            min_solar_trip_time_s,
                        },
                        [
                            205u8, 230u8, 82u8, 201u8, 190u8, 253u8, 231u8, 167u8, 227u8, 125u8,
                            103u8, 96u8, 177u8, 224u8, 66u8, 58u8, 198u8, 95u8, 215u8, 173u8,
                            107u8, 81u8, 64u8, 24u8, 97u8, 148u8, 138u8, 101u8, 138u8, 44u8, 204u8,
                            247u8,
                        ],
                    )
                }
                pub fn set_max_speed_mps(
                    &self,
                    max_speed_mps: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<SetMaxSpeedMps> {
                    ::subxt::tx::StaticTxPayload::new(
                        "EncointerCommunities",
                        "set_max_speed_mps",
                        SetMaxSpeedMps { max_speed_mps },
                        [
                            40u8, 134u8, 87u8, 189u8, 159u8, 128u8, 233u8, 88u8, 124u8, 167u8,
                            131u8, 166u8, 251u8, 221u8, 110u8, 227u8, 25u8, 13u8, 127u8, 95u8,
                            22u8, 236u8, 167u8, 125u8, 24u8, 43u8, 241u8, 16u8, 13u8, 136u8, 116u8,
                            153u8,
                        ],
                    )
                }
                pub fn purge_community(
                    &self,
                    cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                ) -> ::subxt::tx::StaticTxPayload<PurgeCommunity> {
                    ::subxt::tx::StaticTxPayload::new(
                        "EncointerCommunities",
                        "purge_community",
                        PurgeCommunity { cid },
                        [
                            157u8, 102u8, 222u8, 241u8, 166u8, 144u8, 201u8, 201u8, 17u8, 100u8,
                            174u8, 218u8, 37u8, 59u8, 149u8, 4u8, 188u8, 229u8, 181u8, 7u8, 194u8,
                            15u8, 117u8, 111u8, 242u8, 255u8, 216u8, 78u8, 237u8, 8u8, 107u8,
                            122u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_encointer_communities::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A new community was registered [community_identifier]"]
            pub struct CommunityRegistered(
                pub runtime_types::encointer_primitives::communities::CommunityIdentifier,
            );
            impl ::subxt::events::StaticEvent for CommunityRegistered {
                const PALLET: &'static str = "EncointerCommunities";
                const EVENT: &'static str = "CommunityRegistered";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "CommunityMetadata was updated [community_identifier]"]
            pub struct MetadataUpdated(
                pub runtime_types::encointer_primitives::communities::CommunityIdentifier,
            );
            impl ::subxt::events::StaticEvent for MetadataUpdated {
                const PALLET: &'static str = "EncointerCommunities";
                const EVENT: &'static str = "MetadataUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A community's nominal income was updated [community_identifier, new_income]"]
            pub struct NominalIncomeUpdated(
                pub runtime_types::encointer_primitives::communities::CommunityIdentifier,
                pub  runtime_types::substrate_fixed::FixedI128<
                    runtime_types::typenum::uint::UInt<
                        runtime_types::typenum::uint::UInt<
                            runtime_types::typenum::uint::UInt<
                                runtime_types::typenum::uint::UInt<
                                    runtime_types::typenum::uint::UInt<
                                        runtime_types::typenum::uint::UInt<
                                            runtime_types::typenum::uint::UInt<
                                                runtime_types::typenum::uint::UTerm,
                                                runtime_types::typenum::bit::B1,
                                            >,
                                            runtime_types::typenum::bit::B0,
                                        >,
                                        runtime_types::typenum::bit::B0,
                                    >,
                                    runtime_types::typenum::bit::B0,
                                >,
                                runtime_types::typenum::bit::B0,
                            >,
                            runtime_types::typenum::bit::B0,
                        >,
                        runtime_types::typenum::bit::B0,
                    >,
                >,
            );
            impl ::subxt::events::StaticEvent for NominalIncomeUpdated {
                const PALLET: &'static str = "EncointerCommunities";
                const EVENT: &'static str = "NominalIncomeUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A community's demurrage was updated [community_identifier, new_demurrage]"]
            pub struct DemurrageUpdated(
                pub runtime_types::encointer_primitives::communities::CommunityIdentifier,
                pub  runtime_types::substrate_fixed::FixedI128<
                    runtime_types::typenum::uint::UInt<
                        runtime_types::typenum::uint::UInt<
                            runtime_types::typenum::uint::UInt<
                                runtime_types::typenum::uint::UInt<
                                    runtime_types::typenum::uint::UInt<
                                        runtime_types::typenum::uint::UInt<
                                            runtime_types::typenum::uint::UInt<
                                                runtime_types::typenum::uint::UTerm,
                                                runtime_types::typenum::bit::B1,
                                            >,
                                            runtime_types::typenum::bit::B0,
                                        >,
                                        runtime_types::typenum::bit::B0,
                                    >,
                                    runtime_types::typenum::bit::B0,
                                >,
                                runtime_types::typenum::bit::B0,
                            >,
                            runtime_types::typenum::bit::B0,
                        >,
                        runtime_types::typenum::bit::B0,
                    >,
                >,
            );
            impl ::subxt::events::StaticEvent for DemurrageUpdated {
                const PALLET: &'static str = "EncointerCommunities";
                const EVENT: &'static str = "DemurrageUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A location has been added"]
            pub struct LocationAdded(
                pub runtime_types::encointer_primitives::communities::CommunityIdentifier,
                pub runtime_types::encointer_primitives::communities::Location,
            );
            impl ::subxt::events::StaticEvent for LocationAdded {
                const PALLET: &'static str = "EncointerCommunities";
                const EVENT: &'static str = "LocationAdded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A location has been removed"]
            pub struct LocationRemoved(
                pub runtime_types::encointer_primitives::communities::CommunityIdentifier,
                pub runtime_types::encointer_primitives::communities::Location,
            );
            impl ::subxt::events::StaticEvent for LocationRemoved {
                const PALLET: &'static str = "EncointerCommunities";
                const EVENT: &'static str = "LocationRemoved";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            #[doc = "A security parameter for minimum meetup location distance has changed"]
            pub struct MinSolarTripTimeSUpdated(pub ::core::primitive::u32);
            impl ::subxt::events::StaticEvent for MinSolarTripTimeSUpdated {
                const PALLET: &'static str = "EncointerCommunities";
                const EVENT: &'static str = "MinSolarTripTimeSUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            #[doc = "A security parameter for minimum meetup location distance has changed"]
            pub struct MaxSpeedMpsUpdated(pub ::core::primitive::u32);
            impl ::subxt::events::StaticEvent for MaxSpeedMpsUpdated {
                const PALLET: &'static str = "EncointerCommunities";
                const EVENT: &'static str = "MaxSpeedMpsUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "a community has been purged"]
            pub struct CommunityPurged(
                pub runtime_types::encointer_primitives::communities::CommunityIdentifier,
            );
            impl ::subxt::events::StaticEvent for CommunityPurged {
                const PALLET: &'static str = "EncointerCommunities";
                const EVENT: &'static str = "CommunityPurged";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                pub fn community_identifiers_by_geohash(
                    &self,
                    _0: impl ::std::borrow::Borrow<runtime_types::geohash::GeoHash>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<
                            runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCommunities",
                        "CommunityIdentifiersByGeohash",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Identity,
                        )],
                        [
                            230u8, 213u8, 136u8, 198u8, 151u8, 92u8, 27u8, 117u8, 22u8, 134u8,
                            226u8, 172u8, 244u8, 55u8, 27u8, 91u8, 220u8, 90u8, 83u8, 245u8, 91u8,
                            30u8, 159u8, 129u8, 21u8, 3u8, 202u8, 115u8, 224u8, 19u8, 152u8, 178u8,
                        ],
                    )
                }
                pub fn community_identifiers_by_geohash_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<
                            runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        >,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCommunities",
                        "CommunityIdentifiersByGeohash",
                        Vec::new(),
                        [
                            230u8, 213u8, 136u8, 198u8, 151u8, 92u8, 27u8, 117u8, 22u8, 134u8,
                            226u8, 172u8, 244u8, 55u8, 27u8, 91u8, 220u8, 90u8, 83u8, 245u8, 91u8,
                            30u8, 159u8, 129u8, 21u8, 3u8, 202u8, 115u8, 224u8, 19u8, 152u8, 178u8,
                        ],
                    )
                }
                pub fn locations(
                    &self,
                    _0: impl ::std::borrow::Borrow<
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    >,
                    _1: impl ::std::borrow::Borrow<runtime_types::geohash::GeoHash>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<runtime_types::encointer_primitives::communities::Location>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCommunities",
                        "Locations",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Identity,
                            ),
                        ],
                        [
                            39u8, 189u8, 4u8, 53u8, 170u8, 216u8, 142u8, 217u8, 219u8, 112u8, 84u8,
                            189u8, 11u8, 228u8, 69u8, 40u8, 60u8, 198u8, 86u8, 48u8, 64u8, 223u8,
                            102u8, 125u8, 8u8, 182u8, 109u8, 159u8, 236u8, 58u8, 162u8, 192u8,
                        ],
                    )
                }
                pub fn locations_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<runtime_types::encointer_primitives::communities::Location>,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCommunities",
                        "Locations",
                        Vec::new(),
                        [
                            39u8, 189u8, 4u8, 53u8, 170u8, 216u8, 142u8, 217u8, 219u8, 112u8, 84u8,
                            189u8, 11u8, 228u8, 69u8, 40u8, 60u8, 198u8, 86u8, 48u8, 64u8, 223u8,
                            102u8, 125u8, 8u8, 182u8, 109u8, 159u8, 236u8, 58u8, 162u8, 192u8,
                        ],
                    )
                }
                pub fn bootstrappers(
                    &self,
                    _0: impl ::std::borrow::Borrow<
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    >,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCommunities",
                        "Bootstrappers",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            152u8, 157u8, 141u8, 188u8, 169u8, 164u8, 70u8, 247u8, 21u8, 221u8,
                            172u8, 254u8, 197u8, 30u8, 14u8, 190u8, 237u8, 180u8, 59u8, 208u8,
                            42u8, 133u8, 49u8, 114u8, 49u8, 26u8, 187u8, 222u8, 197u8, 27u8, 53u8,
                            195u8,
                        ],
                    )
                }
                pub fn bootstrappers_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCommunities",
                        "Bootstrappers",
                        Vec::new(),
                        [
                            152u8, 157u8, 141u8, 188u8, 169u8, 164u8, 70u8, 247u8, 21u8, 221u8,
                            172u8, 254u8, 197u8, 30u8, 14u8, 190u8, 237u8, 180u8, 59u8, 208u8,
                            42u8, 133u8, 49u8, 114u8, 49u8, 26u8, 187u8, 222u8, 197u8, 27u8, 53u8,
                            195u8,
                        ],
                    )
                }
                pub fn community_identifiers(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<
                            runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCommunities",
                        "CommunityIdentifiers",
                        vec![],
                        [
                            68u8, 177u8, 45u8, 25u8, 83u8, 2u8, 10u8, 66u8, 67u8, 249u8, 52u8,
                            188u8, 96u8, 49u8, 90u8, 46u8, 177u8, 79u8, 137u8, 113u8, 103u8, 81u8,
                            62u8, 71u8, 205u8, 120u8, 100u8, 163u8, 110u8, 72u8, 163u8, 69u8,
                        ],
                    )
                }
                pub fn community_metadata(
                    &self,
                    _0: impl ::std::borrow::Borrow<
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    >,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::encointer_primitives::communities::CommunityMetadata,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCommunities",
                        "CommunityMetadata",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            237u8, 147u8, 221u8, 204u8, 155u8, 114u8, 250u8, 71u8, 90u8, 96u8,
                            25u8, 28u8, 216u8, 115u8, 170u8, 214u8, 85u8, 233u8, 105u8, 151u8,
                            201u8, 230u8, 53u8, 12u8, 15u8, 108u8, 141u8, 37u8, 55u8, 33u8, 9u8,
                            234u8,
                        ],
                    )
                }
                pub fn community_metadata_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::encointer_primitives::communities::CommunityMetadata,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCommunities",
                        "CommunityMetadata",
                        Vec::new(),
                        [
                            237u8, 147u8, 221u8, 204u8, 155u8, 114u8, 250u8, 71u8, 90u8, 96u8,
                            25u8, 28u8, 216u8, 115u8, 170u8, 214u8, 85u8, 233u8, 105u8, 151u8,
                            201u8, 230u8, 53u8, 12u8, 15u8, 108u8, 141u8, 37u8, 55u8, 33u8, 9u8,
                            234u8,
                        ],
                    )
                }
                #[doc = " Amount of UBI to be paid for every attended ceremony."]
                pub fn nominal_income(
                    &self,
                    _0: impl ::std::borrow::Borrow<
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    >,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::substrate_fixed::FixedI128<
                            runtime_types::typenum::uint::UInt<
                                runtime_types::typenum::uint::UInt<
                                    runtime_types::typenum::uint::UInt<
                                        runtime_types::typenum::uint::UInt<
                                            runtime_types::typenum::uint::UInt<
                                                runtime_types::typenum::uint::UInt<
                                                    runtime_types::typenum::uint::UInt<
                                                        runtime_types::typenum::uint::UTerm,
                                                        runtime_types::typenum::bit::B1,
                                                    >,
                                                    runtime_types::typenum::bit::B0,
                                                >,
                                                runtime_types::typenum::bit::B0,
                                            >,
                                            runtime_types::typenum::bit::B0,
                                        >,
                                        runtime_types::typenum::bit::B0,
                                    >,
                                    runtime_types::typenum::bit::B0,
                                >,
                                runtime_types::typenum::bit::B0,
                            >,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCommunities",
                        "NominalIncome",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            137u8, 91u8, 122u8, 128u8, 238u8, 140u8, 207u8, 162u8, 196u8, 221u8,
                            111u8, 76u8, 126u8, 61u8, 250u8, 168u8, 75u8, 113u8, 174u8, 107u8,
                            164u8, 174u8, 166u8, 55u8, 19u8, 243u8, 54u8, 225u8, 32u8, 57u8, 167u8,
                            246u8,
                        ],
                    )
                }
                #[doc = " Amount of UBI to be paid for every attended ceremony."]
                pub fn nominal_income_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::substrate_fixed::FixedI128<
                            runtime_types::typenum::uint::UInt<
                                runtime_types::typenum::uint::UInt<
                                    runtime_types::typenum::uint::UInt<
                                        runtime_types::typenum::uint::UInt<
                                            runtime_types::typenum::uint::UInt<
                                                runtime_types::typenum::uint::UInt<
                                                    runtime_types::typenum::uint::UInt<
                                                        runtime_types::typenum::uint::UTerm,
                                                        runtime_types::typenum::bit::B1,
                                                    >,
                                                    runtime_types::typenum::bit::B0,
                                                >,
                                                runtime_types::typenum::bit::B0,
                                            >,
                                            runtime_types::typenum::bit::B0,
                                        >,
                                        runtime_types::typenum::bit::B0,
                                    >,
                                    runtime_types::typenum::bit::B0,
                                >,
                                runtime_types::typenum::bit::B0,
                            >,
                        >,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCommunities",
                        "NominalIncome",
                        Vec::new(),
                        [
                            137u8, 91u8, 122u8, 128u8, 238u8, 140u8, 207u8, 162u8, 196u8, 221u8,
                            111u8, 76u8, 126u8, 61u8, 250u8, 168u8, 75u8, 113u8, 174u8, 107u8,
                            164u8, 174u8, 166u8, 55u8, 19u8, 243u8, 54u8, 225u8, 32u8, 57u8, 167u8,
                            246u8,
                        ],
                    )
                }
                pub fn min_solar_trip_time_s(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCommunities",
                        "MinSolarTripTimeS",
                        vec![],
                        [
                            83u8, 190u8, 214u8, 30u8, 51u8, 212u8, 213u8, 98u8, 139u8, 75u8, 171u8,
                            162u8, 201u8, 13u8, 119u8, 153u8, 109u8, 241u8, 129u8, 252u8, 32u8,
                            135u8, 164u8, 219u8, 253u8, 97u8, 180u8, 214u8, 103u8, 148u8, 75u8,
                            109u8,
                        ],
                    )
                }
                pub fn max_speed_mps(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerCommunities",
                        "MaxSpeedMps",
                        vec![],
                        [
                            117u8, 225u8, 17u8, 221u8, 220u8, 29u8, 125u8, 18u8, 104u8, 58u8, 37u8,
                            198u8, 61u8, 105u8, 89u8, 211u8, 235u8, 132u8, 218u8, 153u8, 85u8,
                            229u8, 187u8, 158u8, 240u8, 240u8, 195u8, 161u8, 211u8, 241u8, 47u8,
                            96u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod encointer_balances {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Transfer {
                pub dest: ::subxt::ext::sp_core::crypto::AccountId32,
                pub community_id:
                    runtime_types::encointer_primitives::communities::CommunityIdentifier,
                pub amount: runtime_types::substrate_fixed::FixedI128<
                    runtime_types::typenum::uint::UInt<
                        runtime_types::typenum::uint::UInt<
                            runtime_types::typenum::uint::UInt<
                                runtime_types::typenum::uint::UInt<
                                    runtime_types::typenum::uint::UInt<
                                        runtime_types::typenum::uint::UInt<
                                            runtime_types::typenum::uint::UInt<
                                                runtime_types::typenum::uint::UTerm,
                                                runtime_types::typenum::bit::B1,
                                            >,
                                            runtime_types::typenum::bit::B0,
                                        >,
                                        runtime_types::typenum::bit::B0,
                                    >,
                                    runtime_types::typenum::bit::B0,
                                >,
                                runtime_types::typenum::bit::B0,
                            >,
                            runtime_types::typenum::bit::B0,
                        >,
                        runtime_types::typenum::bit::B0,
                    >,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct SetFeeConversionFactor {
                pub fee_conversion_factor: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct TransferAll {
                pub dest: ::subxt::ext::sp_core::crypto::AccountId32,
                pub cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Transfer some balance to another account."]
                pub fn transfer(
                    &self,
                    dest: ::subxt::ext::sp_core::crypto::AccountId32,
                    community_id : runtime_types :: encointer_primitives :: communities :: CommunityIdentifier,
                    amount: runtime_types::substrate_fixed::FixedI128<
                        runtime_types::typenum::uint::UInt<
                            runtime_types::typenum::uint::UInt<
                                runtime_types::typenum::uint::UInt<
                                    runtime_types::typenum::uint::UInt<
                                        runtime_types::typenum::uint::UInt<
                                            runtime_types::typenum::uint::UInt<
                                                runtime_types::typenum::uint::UInt<
                                                    runtime_types::typenum::uint::UTerm,
                                                    runtime_types::typenum::bit::B1,
                                                >,
                                                runtime_types::typenum::bit::B0,
                                            >,
                                            runtime_types::typenum::bit::B0,
                                        >,
                                        runtime_types::typenum::bit::B0,
                                    >,
                                    runtime_types::typenum::bit::B0,
                                >,
                                runtime_types::typenum::bit::B0,
                            >,
                            runtime_types::typenum::bit::B0,
                        >,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<Transfer> {
                    ::subxt::tx::StaticTxPayload::new(
                        "EncointerBalances",
                        "transfer",
                        Transfer {
                            dest,
                            community_id,
                            amount,
                        },
                        [
                            22u8, 198u8, 160u8, 177u8, 249u8, 148u8, 182u8, 36u8, 66u8, 188u8,
                            246u8, 75u8, 31u8, 31u8, 165u8, 247u8, 19u8, 224u8, 242u8, 62u8, 167u8,
                            152u8, 76u8, 149u8, 51u8, 61u8, 202u8, 68u8, 217u8, 181u8, 48u8, 31u8,
                        ],
                    )
                }
                pub fn set_fee_conversion_factor(
                    &self,
                    fee_conversion_factor: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<SetFeeConversionFactor> {
                    ::subxt::tx::StaticTxPayload::new(
                        "EncointerBalances",
                        "set_fee_conversion_factor",
                        SetFeeConversionFactor {
                            fee_conversion_factor,
                        },
                        [
                            231u8, 241u8, 55u8, 130u8, 56u8, 103u8, 99u8, 108u8, 109u8, 164u8,
                            200u8, 162u8, 64u8, 188u8, 15u8, 220u8, 76u8, 198u8, 86u8, 191u8, 33u8,
                            168u8, 146u8, 161u8, 19u8, 173u8, 56u8, 4u8, 187u8, 93u8, 75u8, 158u8,
                        ],
                    )
                }
                pub fn transfer_all(
                    &self,
                    dest: ::subxt::ext::sp_core::crypto::AccountId32,
                    cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                ) -> ::subxt::tx::StaticTxPayload<TransferAll> {
                    ::subxt::tx::StaticTxPayload::new(
                        "EncointerBalances",
                        "transfer_all",
                        TransferAll { dest, cid },
                        [
                            121u8, 211u8, 86u8, 78u8, 157u8, 90u8, 215u8, 10u8, 233u8, 116u8,
                            128u8, 64u8, 66u8, 211u8, 163u8, 228u8, 141u8, 166u8, 50u8, 193u8,
                            195u8, 53u8, 21u8, 18u8, 115u8, 217u8, 138u8, 21u8, 5u8, 139u8, 246u8,
                            246u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_encointer_balances::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Endowed a new account with a respective currency `[community_id, who, balance]`"]
            pub struct Endowed {
                pub cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
                pub balance: runtime_types::substrate_fixed::FixedI128<
                    runtime_types::typenum::uint::UInt<
                        runtime_types::typenum::uint::UInt<
                            runtime_types::typenum::uint::UInt<
                                runtime_types::typenum::uint::UInt<
                                    runtime_types::typenum::uint::UInt<
                                        runtime_types::typenum::uint::UInt<
                                            runtime_types::typenum::uint::UInt<
                                                runtime_types::typenum::uint::UTerm,
                                                runtime_types::typenum::bit::B1,
                                            >,
                                            runtime_types::typenum::bit::B0,
                                        >,
                                        runtime_types::typenum::bit::B0,
                                    >,
                                    runtime_types::typenum::bit::B0,
                                >,
                                runtime_types::typenum::bit::B0,
                            >,
                            runtime_types::typenum::bit::B0,
                        >,
                        runtime_types::typenum::bit::B0,
                    >,
                >,
            }
            impl ::subxt::events::StaticEvent for Endowed {
                const PALLET: &'static str = "EncointerBalances";
                const EVENT: &'static str = "Endowed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Token transfer success `[community_id, from, to, amount]`"]
            pub struct Transferred(
                pub runtime_types::encointer_primitives::communities::CommunityIdentifier,
                pub ::subxt::ext::sp_core::crypto::AccountId32,
                pub ::subxt::ext::sp_core::crypto::AccountId32,
                pub  runtime_types::substrate_fixed::FixedI128<
                    runtime_types::typenum::uint::UInt<
                        runtime_types::typenum::uint::UInt<
                            runtime_types::typenum::uint::UInt<
                                runtime_types::typenum::uint::UInt<
                                    runtime_types::typenum::uint::UInt<
                                        runtime_types::typenum::uint::UInt<
                                            runtime_types::typenum::uint::UInt<
                                                runtime_types::typenum::uint::UTerm,
                                                runtime_types::typenum::bit::B1,
                                            >,
                                            runtime_types::typenum::bit::B0,
                                        >,
                                        runtime_types::typenum::bit::B0,
                                    >,
                                    runtime_types::typenum::bit::B0,
                                >,
                                runtime_types::typenum::bit::B0,
                            >,
                            runtime_types::typenum::bit::B0,
                        >,
                        runtime_types::typenum::bit::B0,
                    >,
                >,
            );
            impl ::subxt::events::StaticEvent for Transferred {
                const PALLET: &'static str = "EncointerBalances";
                const EVENT: &'static str = "Transferred";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Token issuance success `[community_id, beneficiary, amount]`"]
            pub struct Issued(
                pub runtime_types::encointer_primitives::communities::CommunityIdentifier,
                pub ::subxt::ext::sp_core::crypto::AccountId32,
                pub  runtime_types::substrate_fixed::FixedI128<
                    runtime_types::typenum::uint::UInt<
                        runtime_types::typenum::uint::UInt<
                            runtime_types::typenum::uint::UInt<
                                runtime_types::typenum::uint::UInt<
                                    runtime_types::typenum::uint::UInt<
                                        runtime_types::typenum::uint::UInt<
                                            runtime_types::typenum::uint::UInt<
                                                runtime_types::typenum::uint::UTerm,
                                                runtime_types::typenum::bit::B1,
                                            >,
                                            runtime_types::typenum::bit::B0,
                                        >,
                                        runtime_types::typenum::bit::B0,
                                    >,
                                    runtime_types::typenum::bit::B0,
                                >,
                                runtime_types::typenum::bit::B0,
                            >,
                            runtime_types::typenum::bit::B0,
                        >,
                        runtime_types::typenum::bit::B0,
                    >,
                >,
            );
            impl ::subxt::events::StaticEvent for Issued {
                const PALLET: &'static str = "EncointerBalances";
                const EVENT: &'static str = "Issued";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            #[doc = "fee conversion factor updated successfully"]
            pub struct FeeConversionFactorUpdated(pub ::core::primitive::u128);
            impl ::subxt::events::StaticEvent for FeeConversionFactorUpdated {
                const PALLET: &'static str = "EncointerBalances";
                const EVENT: &'static str = "FeeConversionFactorUpdated";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                pub fn total_issuance(
                    &self,
                    _0: impl ::std::borrow::Borrow<
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    >,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::encointer_primitives::balances::BalanceEntry<
                            ::core::primitive::u32,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerBalances",
                        "TotalIssuance",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            18u8, 77u8, 57u8, 137u8, 118u8, 46u8, 118u8, 112u8, 212u8, 89u8, 87u8,
                            74u8, 16u8, 239u8, 16u8, 183u8, 74u8, 190u8, 220u8, 160u8, 10u8, 19u8,
                            123u8, 35u8, 175u8, 187u8, 16u8, 56u8, 182u8, 66u8, 96u8, 77u8,
                        ],
                    )
                }
                pub fn total_issuance_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::encointer_primitives::balances::BalanceEntry<
                            ::core::primitive::u32,
                        >,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerBalances",
                        "TotalIssuance",
                        Vec::new(),
                        [
                            18u8, 77u8, 57u8, 137u8, 118u8, 46u8, 118u8, 112u8, 212u8, 89u8, 87u8,
                            74u8, 16u8, 239u8, 16u8, 183u8, 74u8, 190u8, 220u8, 160u8, 10u8, 19u8,
                            123u8, 35u8, 175u8, 187u8, 16u8, 56u8, 182u8, 66u8, 96u8, 77u8,
                        ],
                    )
                }
                pub fn balance(
                    &self,
                    _0: impl ::std::borrow::Borrow<
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    >,
                    _1: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::encointer_primitives::balances::BalanceEntry<
                            ::core::primitive::u32,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerBalances",
                        "Balance",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                        ],
                        [
                            92u8, 47u8, 142u8, 132u8, 251u8, 45u8, 38u8, 164u8, 207u8, 224u8,
                            192u8, 50u8, 203u8, 151u8, 27u8, 99u8, 175u8, 253u8, 255u8, 78u8, 89u8,
                            56u8, 180u8, 118u8, 65u8, 211u8, 92u8, 189u8, 142u8, 67u8, 125u8,
                            122u8,
                        ],
                    )
                }
                pub fn balance_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::encointer_primitives::balances::BalanceEntry<
                            ::core::primitive::u32,
                        >,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerBalances",
                        "Balance",
                        Vec::new(),
                        [
                            92u8, 47u8, 142u8, 132u8, 251u8, 45u8, 38u8, 164u8, 207u8, 224u8,
                            192u8, 50u8, 203u8, 151u8, 27u8, 99u8, 175u8, 253u8, 255u8, 78u8, 89u8,
                            56u8, 180u8, 118u8, 65u8, 211u8, 92u8, 189u8, 142u8, 67u8, 125u8,
                            122u8,
                        ],
                    )
                }
                pub fn demurrage_per_block(
                    &self,
                    _0: impl ::std::borrow::Borrow<
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    >,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::substrate_fixed::FixedI128<
                            runtime_types::typenum::uint::UInt<
                                runtime_types::typenum::uint::UInt<
                                    runtime_types::typenum::uint::UInt<
                                        runtime_types::typenum::uint::UInt<
                                            runtime_types::typenum::uint::UInt<
                                                runtime_types::typenum::uint::UInt<
                                                    runtime_types::typenum::uint::UInt<
                                                        runtime_types::typenum::uint::UTerm,
                                                        runtime_types::typenum::bit::B1,
                                                    >,
                                                    runtime_types::typenum::bit::B0,
                                                >,
                                                runtime_types::typenum::bit::B0,
                                            >,
                                            runtime_types::typenum::bit::B0,
                                        >,
                                        runtime_types::typenum::bit::B0,
                                    >,
                                    runtime_types::typenum::bit::B0,
                                >,
                                runtime_types::typenum::bit::B0,
                            >,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerBalances",
                        "DemurragePerBlock",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            118u8, 161u8, 208u8, 159u8, 123u8, 250u8, 146u8, 161u8, 211u8, 57u8,
                            137u8, 97u8, 65u8, 159u8, 161u8, 66u8, 76u8, 134u8, 150u8, 155u8,
                            201u8, 109u8, 66u8, 51u8, 30u8, 43u8, 139u8, 26u8, 5u8, 220u8, 45u8,
                            69u8,
                        ],
                    )
                }
                pub fn demurrage_per_block_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::substrate_fixed::FixedI128<
                            runtime_types::typenum::uint::UInt<
                                runtime_types::typenum::uint::UInt<
                                    runtime_types::typenum::uint::UInt<
                                        runtime_types::typenum::uint::UInt<
                                            runtime_types::typenum::uint::UInt<
                                                runtime_types::typenum::uint::UInt<
                                                    runtime_types::typenum::uint::UInt<
                                                        runtime_types::typenum::uint::UTerm,
                                                        runtime_types::typenum::bit::B1,
                                                    >,
                                                    runtime_types::typenum::bit::B0,
                                                >,
                                                runtime_types::typenum::bit::B0,
                                            >,
                                            runtime_types::typenum::bit::B0,
                                        >,
                                        runtime_types::typenum::bit::B0,
                                    >,
                                    runtime_types::typenum::bit::B0,
                                >,
                                runtime_types::typenum::bit::B0,
                            >,
                        >,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerBalances",
                        "DemurragePerBlock",
                        Vec::new(),
                        [
                            118u8, 161u8, 208u8, 159u8, 123u8, 250u8, 146u8, 161u8, 211u8, 57u8,
                            137u8, 97u8, 65u8, 159u8, 161u8, 66u8, 76u8, 134u8, 150u8, 155u8,
                            201u8, 109u8, 66u8, 51u8, 30u8, 43u8, 139u8, 26u8, 5u8, 220u8, 45u8,
                            69u8,
                        ],
                    )
                }
                pub fn fee_conversion_factor(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerBalances",
                        "FeeConversionFactor",
                        vec![],
                        [
                            22u8, 224u8, 159u8, 183u8, 170u8, 206u8, 63u8, 120u8, 112u8, 107u8,
                            27u8, 180u8, 180u8, 245u8, 125u8, 236u8, 9u8, 202u8, 208u8, 31u8,
                            102u8, 189u8, 99u8, 130u8, 164u8, 232u8, 219u8, 223u8, 57u8, 154u8,
                            125u8, 190u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " the default demurrage rate applied to community balances"]
                pub fn default_demurrage(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::substrate_fixed::FixedI128<
                            runtime_types::typenum::uint::UInt<
                                runtime_types::typenum::uint::UInt<
                                    runtime_types::typenum::uint::UInt<
                                        runtime_types::typenum::uint::UInt<
                                            runtime_types::typenum::uint::UInt<
                                                runtime_types::typenum::uint::UInt<
                                                    runtime_types::typenum::uint::UInt<
                                                        runtime_types::typenum::uint::UTerm,
                                                        runtime_types::typenum::bit::B1,
                                                    >,
                                                    runtime_types::typenum::bit::B0,
                                                >,
                                                runtime_types::typenum::bit::B0,
                                            >,
                                            runtime_types::typenum::bit::B0,
                                        >,
                                        runtime_types::typenum::bit::B0,
                                    >,
                                    runtime_types::typenum::bit::B0,
                                >,
                                runtime_types::typenum::bit::B0,
                            >,
                        >,
                    >,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "EncointerBalances",
                        "DefaultDemurrage",
                        [
                            228u8, 9u8, 169u8, 7u8, 163u8, 48u8, 183u8, 56u8, 86u8, 3u8, 234u8,
                            235u8, 86u8, 134u8, 105u8, 247u8, 239u8, 175u8, 121u8, 213u8, 104u8,
                            186u8, 207u8, 73u8, 128u8, 244u8, 83u8, 132u8, 207u8, 7u8, 54u8, 131u8,
                        ],
                    )
                }
                #[doc = " Existential deposit needed to have an account in the respective community currency"]
                #[doc = ""]
                #[doc = " This does currently not prevent dust-accounts, but it prevents account creation"]
                #[doc = " by transferring tiny amounts of funds."]
                pub fn existential_deposit(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::substrate_fixed::FixedI128<
                            runtime_types::typenum::uint::UInt<
                                runtime_types::typenum::uint::UInt<
                                    runtime_types::typenum::uint::UInt<
                                        runtime_types::typenum::uint::UInt<
                                            runtime_types::typenum::uint::UInt<
                                                runtime_types::typenum::uint::UInt<
                                                    runtime_types::typenum::uint::UInt<
                                                        runtime_types::typenum::uint::UTerm,
                                                        runtime_types::typenum::bit::B1,
                                                    >,
                                                    runtime_types::typenum::bit::B0,
                                                >,
                                                runtime_types::typenum::bit::B0,
                                            >,
                                            runtime_types::typenum::bit::B0,
                                        >,
                                        runtime_types::typenum::bit::B0,
                                    >,
                                    runtime_types::typenum::bit::B0,
                                >,
                                runtime_types::typenum::bit::B0,
                            >,
                        >,
                    >,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "EncointerBalances",
                        "ExistentialDeposit",
                        [
                            228u8, 9u8, 169u8, 7u8, 163u8, 48u8, 183u8, 56u8, 86u8, 3u8, 234u8,
                            235u8, 86u8, 134u8, 105u8, 247u8, 239u8, 175u8, 121u8, 213u8, 104u8,
                            186u8, 207u8, 73u8, 128u8, 244u8, 83u8, 132u8, 207u8, 7u8, 54u8, 131u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod encointer_bazaar {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct CreateBusiness {
                pub cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                pub url: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct UpdateBusiness {
                pub cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                pub url: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct DeleteBusiness {
                pub cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct CreateOffering {
                pub cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                pub url: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct UpdateOffering {
                pub cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                pub oid: ::core::primitive::u32,
                pub url: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct DeleteOffering {
                pub cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                pub oid: ::core::primitive::u32,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                pub fn create_business(
                    &self,
                    cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    url: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<CreateBusiness> {
                    ::subxt::tx::StaticTxPayload::new(
                        "EncointerBazaar",
                        "create_business",
                        CreateBusiness { cid, url },
                        [
                            122u8, 202u8, 80u8, 248u8, 209u8, 10u8, 129u8, 240u8, 193u8, 65u8,
                            113u8, 89u8, 7u8, 114u8, 11u8, 120u8, 44u8, 69u8, 52u8, 224u8, 51u8,
                            183u8, 10u8, 142u8, 112u8, 130u8, 73u8, 0u8, 162u8, 28u8, 1u8, 46u8,
                        ],
                    )
                }
                pub fn update_business(
                    &self,
                    cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    url: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<UpdateBusiness> {
                    ::subxt::tx::StaticTxPayload::new(
                        "EncointerBazaar",
                        "update_business",
                        UpdateBusiness { cid, url },
                        [
                            238u8, 201u8, 162u8, 173u8, 38u8, 16u8, 227u8, 175u8, 168u8, 188u8,
                            240u8, 65u8, 132u8, 153u8, 170u8, 137u8, 125u8, 239u8, 100u8, 221u8,
                            53u8, 128u8, 245u8, 132u8, 76u8, 58u8, 89u8, 77u8, 178u8, 231u8, 189u8,
                            40u8,
                        ],
                    )
                }
                pub fn delete_business(
                    &self,
                    cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                ) -> ::subxt::tx::StaticTxPayload<DeleteBusiness> {
                    ::subxt::tx::StaticTxPayload::new(
                        "EncointerBazaar",
                        "delete_business",
                        DeleteBusiness { cid },
                        [
                            18u8, 181u8, 159u8, 236u8, 121u8, 34u8, 151u8, 31u8, 173u8, 103u8,
                            217u8, 44u8, 100u8, 112u8, 250u8, 79u8, 201u8, 106u8, 172u8, 160u8,
                            154u8, 179u8, 219u8, 210u8, 227u8, 8u8, 245u8, 14u8, 186u8, 37u8,
                            126u8, 82u8,
                        ],
                    )
                }
                pub fn create_offering(
                    &self,
                    cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    url: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<CreateOffering> {
                    ::subxt::tx::StaticTxPayload::new(
                        "EncointerBazaar",
                        "create_offering",
                        CreateOffering { cid, url },
                        [
                            16u8, 4u8, 229u8, 241u8, 12u8, 145u8, 216u8, 197u8, 68u8, 64u8, 203u8,
                            245u8, 212u8, 123u8, 202u8, 120u8, 114u8, 206u8, 105u8, 36u8, 173u8,
                            112u8, 191u8, 142u8, 23u8, 49u8, 230u8, 188u8, 205u8, 0u8, 197u8,
                            228u8,
                        ],
                    )
                }
                pub fn update_offering(
                    &self,
                    cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    oid: ::core::primitive::u32,
                    url: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<UpdateOffering> {
                    ::subxt::tx::StaticTxPayload::new(
                        "EncointerBazaar",
                        "update_offering",
                        UpdateOffering { cid, oid, url },
                        [
                            119u8, 111u8, 227u8, 84u8, 23u8, 65u8, 117u8, 106u8, 235u8, 15u8,
                            226u8, 159u8, 241u8, 135u8, 201u8, 179u8, 191u8, 244u8, 143u8, 245u8,
                            35u8, 225u8, 93u8, 87u8, 80u8, 115u8, 243u8, 113u8, 100u8, 218u8, 53u8,
                            136u8,
                        ],
                    )
                }
                pub fn delete_offering(
                    &self,
                    cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    oid: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<DeleteOffering> {
                    ::subxt::tx::StaticTxPayload::new(
                        "EncointerBazaar",
                        "delete_offering",
                        DeleteOffering { cid, oid },
                        [
                            71u8, 53u8, 58u8, 56u8, 212u8, 52u8, 217u8, 47u8, 230u8, 66u8, 1u8,
                            237u8, 65u8, 252u8, 235u8, 198u8, 125u8, 2u8, 185u8, 158u8, 195u8,
                            24u8, 1u8, 92u8, 17u8, 141u8, 117u8, 150u8, 55u8, 66u8, 220u8, 19u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_encointer_bazaar::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Event emitted when a business is created. [community, who]"]
            pub struct BusinessCreated(
                pub runtime_types::encointer_primitives::communities::CommunityIdentifier,
                pub ::subxt::ext::sp_core::crypto::AccountId32,
            );
            impl ::subxt::events::StaticEvent for BusinessCreated {
                const PALLET: &'static str = "EncointerBazaar";
                const EVENT: &'static str = "BusinessCreated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Event emitted when a business is updated. [community, who]"]
            pub struct BusinessUpdated(
                pub runtime_types::encointer_primitives::communities::CommunityIdentifier,
                pub ::subxt::ext::sp_core::crypto::AccountId32,
            );
            impl ::subxt::events::StaticEvent for BusinessUpdated {
                const PALLET: &'static str = "EncointerBazaar";
                const EVENT: &'static str = "BusinessUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Event emitted when a business is deleted. [community, who]"]
            pub struct BusinessDeleted(
                pub runtime_types::encointer_primitives::communities::CommunityIdentifier,
                pub ::subxt::ext::sp_core::crypto::AccountId32,
            );
            impl ::subxt::events::StaticEvent for BusinessDeleted {
                const PALLET: &'static str = "EncointerBazaar";
                const EVENT: &'static str = "BusinessDeleted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Event emitted when an offering is created. [community, who, oid]"]
            pub struct OfferingCreated(
                pub runtime_types::encointer_primitives::communities::CommunityIdentifier,
                pub ::subxt::ext::sp_core::crypto::AccountId32,
                pub ::core::primitive::u32,
            );
            impl ::subxt::events::StaticEvent for OfferingCreated {
                const PALLET: &'static str = "EncointerBazaar";
                const EVENT: &'static str = "OfferingCreated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Event emitted when an offering is updated. [community, who, oid]"]
            pub struct OfferingUpdated(
                pub runtime_types::encointer_primitives::communities::CommunityIdentifier,
                pub ::subxt::ext::sp_core::crypto::AccountId32,
                pub ::core::primitive::u32,
            );
            impl ::subxt::events::StaticEvent for OfferingUpdated {
                const PALLET: &'static str = "EncointerBazaar";
                const EVENT: &'static str = "OfferingUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Event emitted when an offering is deleted. [community, who, oid]"]
            pub struct OfferingDeleted(
                pub runtime_types::encointer_primitives::communities::CommunityIdentifier,
                pub ::subxt::ext::sp_core::crypto::AccountId32,
                pub ::core::primitive::u32,
            );
            impl ::subxt::events::StaticEvent for OfferingDeleted {
                const PALLET: &'static str = "EncointerBazaar";
                const EVENT: &'static str = "OfferingDeleted";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                pub fn business_registry(
                    &self,
                    _0: impl ::std::borrow::Borrow<
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    >,
                    _1: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::encointer_primitives::bazaar::BusinessData,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerBazaar",
                        "BusinessRegistry",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                        ],
                        [
                            198u8, 60u8, 76u8, 242u8, 184u8, 193u8, 99u8, 90u8, 144u8, 15u8, 147u8,
                            115u8, 68u8, 183u8, 34u8, 76u8, 149u8, 27u8, 130u8, 163u8, 249u8,
                            253u8, 45u8, 129u8, 245u8, 138u8, 208u8, 179u8, 22u8, 84u8, 63u8, 36u8,
                        ],
                    )
                }
                pub fn business_registry_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::encointer_primitives::bazaar::BusinessData,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerBazaar",
                        "BusinessRegistry",
                        Vec::new(),
                        [
                            198u8, 60u8, 76u8, 242u8, 184u8, 193u8, 99u8, 90u8, 144u8, 15u8, 147u8,
                            115u8, 68u8, 183u8, 34u8, 76u8, 149u8, 27u8, 130u8, 163u8, 249u8,
                            253u8, 45u8, 129u8, 245u8, 138u8, 208u8, 179u8, 22u8, 84u8, 63u8, 36u8,
                        ],
                    )
                }
                pub fn offering_registry(
                    &self,
                    _0: impl ::std::borrow::Borrow<
                        runtime_types::encointer_primitives::bazaar::BusinessIdentifier<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                        >,
                    >,
                    _1: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::encointer_primitives::bazaar::OfferingData,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerBazaar",
                        "OfferingRegistry",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                        ],
                        [
                            10u8, 86u8, 222u8, 248u8, 165u8, 231u8, 247u8, 112u8, 134u8, 211u8,
                            175u8, 151u8, 187u8, 114u8, 201u8, 141u8, 251u8, 218u8, 67u8, 49u8,
                            186u8, 41u8, 173u8, 59u8, 35u8, 188u8, 8u8, 80u8, 120u8, 235u8, 209u8,
                            232u8,
                        ],
                    )
                }
                pub fn offering_registry_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::encointer_primitives::bazaar::OfferingData,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "EncointerBazaar",
                        "OfferingRegistry",
                        Vec::new(),
                        [
                            10u8, 86u8, 222u8, 248u8, 165u8, 231u8, 247u8, 112u8, 134u8, 211u8,
                            175u8, 151u8, 187u8, 114u8, 201u8, 141u8, 251u8, 218u8, 67u8, 49u8,
                            186u8, 41u8, 173u8, 59u8, 35u8, 188u8, 8u8, 80u8, 120u8, 235u8, 209u8,
                            232u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod runtime_types {
        use super::runtime_types;
        pub mod cumulus_pallet_dmp_queue {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Service a single overweight message."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must pass `ExecuteOverweightOrigin`."]
                    #[doc = "- `index`: The index of the overweight message to service."]
                    #[doc = "- `weight_limit`: The amount of weight that message execution may take."]
                    #[doc = ""]
                    #[doc = "Errors:"]
                    #[doc = "- `Unknown`: Message of `index` is unknown."]
                    #[doc = "- `OverLimit`: Message execution may use greater than `weight_limit`."]
                    #[doc = ""]
                    #[doc = "Events:"]
                    #[doc = "- `OverweightServiced`: On success."]
                    service_overweight {
                        index: ::core::primitive::u64,
                        weight_limit: ::core::primitive::u64,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The message index given is unknown."]
                    Unknown,
                    #[codec(index = 1)]
                    #[doc = "The amount of weight given is possibly not enough for executing the message."]
                    OverLimit,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Downward message is invalid XCM."]
                    InvalidFormat {
                        message_id: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 1)]
                    #[doc = "Downward message is unsupported version of XCM."]
                    UnsupportedVersion {
                        message_id: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 2)]
                    #[doc = "Downward message executed with the given outcome."]
                    ExecutedDownward {
                        message_id: [::core::primitive::u8; 32usize],
                        outcome: runtime_types::xcm::v2::traits::Outcome,
                    },
                    #[codec(index = 3)]
                    #[doc = "The weight limit for handling downward messages was reached."]
                    WeightExhausted {
                        message_id: [::core::primitive::u8; 32usize],
                        remaining_weight: ::core::primitive::u64,
                        required_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 4)]
                    #[doc = "Downward message is overweight and was placed in the overweight queue."]
                    OverweightEnqueued {
                        message_id: [::core::primitive::u8; 32usize],
                        overweight_index: ::core::primitive::u64,
                        required_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 5)]
                    #[doc = "Downward message from the overweight queue was executed."]
                    OverweightServiced {
                        overweight_index: ::core::primitive::u64,
                        weight_used: ::core::primitive::u64,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct ConfigData {
                pub max_individual: ::core::primitive::u64,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct PageIndexData {
                pub begin_used: ::core::primitive::u32,
                pub end_used: ::core::primitive::u32,
                pub overweight_count: ::core::primitive::u64,
            }
        }
        pub mod cumulus_pallet_parachain_system {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    # [codec (index = 0)] # [doc = "Set the current validation data."] # [doc = ""] # [doc = "This should be invoked exactly once per block. It will panic at the finalization"] # [doc = "phase if the call was not invoked."] # [doc = ""] # [doc = "The dispatch origin for this call must be `Inherent`"] # [doc = ""] # [doc = "As a side effect, this function upgrades the current validation function"] # [doc = "if the appropriate time has come."] set_validation_data { data : runtime_types :: cumulus_primitives_parachain_inherent :: ParachainInherentData , } , # [codec (index = 1)] sudo_send_upward_message { message : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , # [codec (index = 2)] authorize_upgrade { code_hash : :: subxt :: ext :: sp_core :: H256 , } , # [codec (index = 3)] enact_authorized_upgrade { code : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Attempt to upgrade validation function while existing upgrade pending"]
                    OverlappingUpgrades,
                    #[codec(index = 1)]
                    #[doc = "Polkadot currently prohibits this parachain from upgrading its validation function"]
                    ProhibitedByPolkadot,
                    #[codec(index = 2)]
                    #[doc = "The supplied validation function has compiled into a blob larger than Polkadot is"]
                    #[doc = "willing to run"]
                    TooBig,
                    #[codec(index = 3)]
                    #[doc = "The inherent which supplies the validation data did not run this block"]
                    ValidationDataNotAvailable,
                    #[codec(index = 4)]
                    #[doc = "The inherent which supplies the host configuration did not run this block"]
                    HostConfigurationNotAvailable,
                    #[codec(index = 5)]
                    #[doc = "No validation function upgrade is currently scheduled."]
                    NotScheduled,
                    #[codec(index = 6)]
                    #[doc = "No code upgrade has been authorized."]
                    NothingAuthorized,
                    #[codec(index = 7)]
                    #[doc = "The given code upgrade has not been authorized."]
                    Unauthorized,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "The validation function has been scheduled to apply."]
                    ValidationFunctionStored,
                    #[codec(index = 1)]
                    #[doc = "The validation function was applied as of the contained relay chain block number."]
                    ValidationFunctionApplied {
                        relay_chain_block_num: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    #[doc = "The relay-chain aborted the upgrade process."]
                    ValidationFunctionDiscarded,
                    #[codec(index = 3)]
                    #[doc = "An upgrade has been authorized."]
                    UpgradeAuthorized {
                        code_hash: ::subxt::ext::sp_core::H256,
                    },
                    #[codec(index = 4)]
                    #[doc = "Some downward messages have been received and will be processed."]
                    DownwardMessagesReceived { count: ::core::primitive::u32 },
                    #[codec(index = 5)]
                    #[doc = "Downward messages were processed using the given weight."]
                    DownwardMessagesProcessed {
                        weight_used: ::core::primitive::u64,
                        dmq_head: ::subxt::ext::sp_core::H256,
                    },
                }
            }
            pub mod relay_state_snapshot {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct MessagingStateSnapshot {
                    pub dmq_mqc_head: ::subxt::ext::sp_core::H256,
                    pub relay_dispatch_queue_size: (::core::primitive::u32, ::core::primitive::u32),
                    pub ingress_channels: ::std::vec::Vec<(
                        runtime_types::polkadot_parachain::primitives::Id,
                        runtime_types::polkadot_primitives::v2::AbridgedHrmpChannel,
                    )>,
                    pub egress_channels: ::std::vec::Vec<(
                        runtime_types::polkadot_parachain::primitives::Id,
                        runtime_types::polkadot_primitives::v2::AbridgedHrmpChannel,
                    )>,
                }
            }
        }
        pub mod cumulus_pallet_xcm {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {}
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Downward message is invalid XCM."]
                    #[doc = "\\[ id \\]"]
                    InvalidFormat([::core::primitive::u8; 8usize]),
                    #[codec(index = 1)]
                    #[doc = "Downward message is unsupported version of XCM."]
                    #[doc = "\\[ id \\]"]
                    UnsupportedVersion([::core::primitive::u8; 8usize]),
                    #[codec(index = 2)]
                    #[doc = "Downward message executed with the given outcome."]
                    #[doc = "\\[ id, outcome \\]"]
                    ExecutedDownward(
                        [::core::primitive::u8; 8usize],
                        runtime_types::xcm::v2::traits::Outcome,
                    ),
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub enum Origin {
                    #[codec(index = 0)]
                    Relay,
                    #[codec(index = 1)]
                    SiblingParachain(runtime_types::polkadot_parachain::primitives::Id),
                }
            }
        }
        pub mod cumulus_pallet_xcmp_queue {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Services a single overweight XCM."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must pass `ExecuteOverweightOrigin`."]
                    #[doc = "- `index`: The index of the overweight XCM to service"]
                    #[doc = "- `weight_limit`: The amount of weight that XCM execution may take."]
                    #[doc = ""]
                    #[doc = "Errors:"]
                    #[doc = "- `BadOverweightIndex`: XCM under `index` is not found in the `Overweight` storage map."]
                    #[doc = "- `BadXcm`: XCM under `index` cannot be properly decoded into a valid XCM format."]
                    #[doc = "- `WeightOverLimit`: XCM execution may use greater `weight_limit`."]
                    #[doc = ""]
                    #[doc = "Events:"]
                    #[doc = "- `OverweightServiced`: On success."]
                    service_overweight {
                        index: ::core::primitive::u64,
                        weight_limit: ::core::primitive::u64,
                    },
                    #[codec(index = 1)]
                    #[doc = "Suspends all XCM executions for the XCMP queue, regardless of the sender's origin."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must pass `ControllerOrigin`."]
                    suspend_xcm_execution,
                    #[codec(index = 2)]
                    #[doc = "Resumes all XCM executions for the XCMP queue."]
                    #[doc = ""]
                    #[doc = "Note that this function doesn't change the status of the in/out bound channels."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must pass `ControllerOrigin`."]
                    resume_xcm_execution,
                    #[codec(index = 3)]
                    #[doc = "Overwrites the number of pages of messages which must be in the queue for the other side to be told to"]
                    #[doc = "suspend their sending."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must pass `Root`."]
                    #[doc = "- `new`: Desired value for `QueueConfigData.suspend_value`"]
                    update_suspend_threshold { new: ::core::primitive::u32 },
                    #[codec(index = 4)]
                    #[doc = "Overwrites the number of pages of messages which must be in the queue after which we drop any further"]
                    #[doc = "messages from the channel."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must pass `Root`."]
                    #[doc = "- `new`: Desired value for `QueueConfigData.drop_threshold`"]
                    update_drop_threshold { new: ::core::primitive::u32 },
                    #[codec(index = 5)]
                    #[doc = "Overwrites the number of pages of messages which the queue must be reduced to before it signals that"]
                    #[doc = "message sending may recommence after it has been suspended."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must pass `Root`."]
                    #[doc = "- `new`: Desired value for `QueueConfigData.resume_threshold`"]
                    update_resume_threshold { new: ::core::primitive::u32 },
                    #[codec(index = 6)]
                    #[doc = "Overwrites the amount of remaining weight under which we stop processing messages."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must pass `Root`."]
                    #[doc = "- `new`: Desired value for `QueueConfigData.threshold_weight`"]
                    update_threshold_weight { new: ::core::primitive::u64 },
                    #[codec(index = 7)]
                    #[doc = "Overwrites the speed to which the available weight approaches the maximum weight."]
                    #[doc = "A lower number results in a faster progression. A value of 1 makes the entire weight available initially."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must pass `Root`."]
                    #[doc = "- `new`: Desired value for `QueueConfigData.weight_restrict_decay`."]
                    update_weight_restrict_decay { new: ::core::primitive::u64 },
                    #[codec(index = 8)]
                    #[doc = "Overwrite the maximum amount of weight any individual message may consume."]
                    #[doc = "Messages above this weight go into the overweight queue and may only be serviced explicitly."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must pass `Root`."]
                    #[doc = "- `new`: Desired value for `QueueConfigData.xcmp_max_individual_weight`."]
                    update_xcmp_max_individual_weight { new: ::core::primitive::u64 },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Failed to send XCM message."]
                    FailedToSend,
                    #[codec(index = 1)]
                    #[doc = "Bad XCM origin."]
                    BadXcmOrigin,
                    #[codec(index = 2)]
                    #[doc = "Bad XCM data."]
                    BadXcm,
                    #[codec(index = 3)]
                    #[doc = "Bad overweight index."]
                    BadOverweightIndex,
                    #[codec(index = 4)]
                    #[doc = "Provided weight is possibly not enough to execute the message."]
                    WeightOverLimit,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Some XCM was executed ok."]
                    Success {
                        message_hash: ::core::option::Option<::subxt::ext::sp_core::H256>,
                        weight: ::core::primitive::u64,
                    },
                    #[codec(index = 1)]
                    #[doc = "Some XCM failed."]
                    Fail {
                        message_hash: ::core::option::Option<::subxt::ext::sp_core::H256>,
                        error: runtime_types::xcm::v2::traits::Error,
                        weight: ::core::primitive::u64,
                    },
                    #[codec(index = 2)]
                    #[doc = "Bad XCM version used."]
                    BadVersion {
                        message_hash: ::core::option::Option<::subxt::ext::sp_core::H256>,
                    },
                    #[codec(index = 3)]
                    #[doc = "Bad XCM format used."]
                    BadFormat {
                        message_hash: ::core::option::Option<::subxt::ext::sp_core::H256>,
                    },
                    #[codec(index = 4)]
                    #[doc = "An upward message was sent to the relay chain."]
                    UpwardMessageSent {
                        message_hash: ::core::option::Option<::subxt::ext::sp_core::H256>,
                    },
                    #[codec(index = 5)]
                    #[doc = "An HRMP message was sent to a sibling parachain."]
                    XcmpMessageSent {
                        message_hash: ::core::option::Option<::subxt::ext::sp_core::H256>,
                    },
                    #[codec(index = 6)]
                    #[doc = "An XCM exceeded the individual message weight budget."]
                    OverweightEnqueued {
                        sender: runtime_types::polkadot_parachain::primitives::Id,
                        sent_at: ::core::primitive::u32,
                        index: ::core::primitive::u64,
                        required: ::core::primitive::u64,
                    },
                    #[codec(index = 7)]
                    #[doc = "An XCM from the overweight queue was executed with the given actual weight used."]
                    OverweightServiced {
                        index: ::core::primitive::u64,
                        used: ::core::primitive::u64,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct InboundChannelDetails {
                pub sender: runtime_types::polkadot_parachain::primitives::Id,
                pub state: runtime_types::cumulus_pallet_xcmp_queue::InboundState,
                pub message_metadata: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    runtime_types::polkadot_parachain::primitives::XcmpMessageFormat,
                )>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum InboundState {
                #[codec(index = 0)]
                Ok,
                #[codec(index = 1)]
                Suspended,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct OutboundChannelDetails {
                pub recipient: runtime_types::polkadot_parachain::primitives::Id,
                pub state: runtime_types::cumulus_pallet_xcmp_queue::OutboundState,
                pub signals_exist: ::core::primitive::bool,
                pub first_index: ::core::primitive::u16,
                pub last_index: ::core::primitive::u16,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum OutboundState {
                #[codec(index = 0)]
                Ok,
                #[codec(index = 1)]
                Suspended,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct QueueConfigData {
                pub suspend_threshold: ::core::primitive::u32,
                pub drop_threshold: ::core::primitive::u32,
                pub resume_threshold: ::core::primitive::u32,
                pub threshold_weight: ::core::primitive::u64,
                pub weight_restrict_decay: ::core::primitive::u64,
                pub xcmp_max_individual_weight: ::core::primitive::u64,
            }
        }
        pub mod cumulus_primitives_parachain_inherent {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct MessageQueueChain(pub ::subxt::ext::sp_core::H256);
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ParachainInherentData {
                pub validation_data:
                    runtime_types::polkadot_primitives::v2::PersistedValidationData<
                        ::subxt::ext::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                pub relay_chain_state: runtime_types::sp_trie::storage_proof::StorageProof,
                pub downward_messages: ::std::vec::Vec<
                    runtime_types::polkadot_core_primitives::InboundDownwardMessage<
                        ::core::primitive::u32,
                    >,
                >,
                pub horizontal_messages: ::subxt::utils::KeyedVec<
                    runtime_types::polkadot_parachain::primitives::Id,
                    ::std::vec::Vec<
                        runtime_types::polkadot_core_primitives::InboundHrmpMessage<
                            ::core::primitive::u32,
                        >,
                    >,
                >,
            }
        }
        pub mod encointer_meetup_validation {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum ExclusionReason {
                #[codec(index = 0)]
                NoVote,
                #[codec(index = 1)]
                WrongVote,
                #[codec(index = 2)]
                TooFewIncomingAttestations,
                #[codec(index = 3)]
                TooFewOutgoingAttestations,
            }
        }
        pub mod encointer_primitives {
            use super::runtime_types;
            pub mod balances {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct BalanceEntry<_0> {
                    pub principal: runtime_types::substrate_fixed::FixedI128<
                        runtime_types::typenum::uint::UInt<
                            runtime_types::typenum::uint::UInt<
                                runtime_types::typenum::uint::UInt<
                                    runtime_types::typenum::uint::UInt<
                                        runtime_types::typenum::uint::UInt<
                                            runtime_types::typenum::uint::UInt<
                                                runtime_types::typenum::uint::UInt<
                                                    runtime_types::typenum::uint::UTerm,
                                                    runtime_types::typenum::bit::B1,
                                                >,
                                                runtime_types::typenum::bit::B0,
                                            >,
                                            runtime_types::typenum::bit::B0,
                                        >,
                                        runtime_types::typenum::bit::B0,
                                    >,
                                    runtime_types::typenum::bit::B0,
                                >,
                                runtime_types::typenum::bit::B0,
                            >,
                            runtime_types::typenum::bit::B0,
                        >,
                    >,
                    pub last_update: _0,
                }
            }
            pub mod bazaar {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct BusinessData {
                    pub url: ::std::vec::Vec<::core::primitive::u8>,
                    pub last_oid: ::core::primitive::u32,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct BusinessIdentifier<_0> {
                    pub community_identifier:
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    pub controller: _0,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct OfferingData {
                    pub url: ::std::vec::Vec<::core::primitive::u8>,
                }
            }
            pub mod ceremonies {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct Assignment {
                    pub bootstrappers_reputables:
                        runtime_types::encointer_primitives::ceremonies::AssignmentParams,
                    pub endorsees:
                        runtime_types::encointer_primitives::ceremonies::AssignmentParams,
                    pub newbies: runtime_types::encointer_primitives::ceremonies::AssignmentParams,
                    pub locations:
                        runtime_types::encointer_primitives::ceremonies::AssignmentParams,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct AssignmentCount {
                    pub bootstrappers: ::core::primitive::u64,
                    pub reputables: ::core::primitive::u64,
                    pub endorsees: ::core::primitive::u64,
                    pub newbies: ::core::primitive::u64,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct AssignmentParams {
                    pub m: ::core::primitive::u64,
                    pub s1: ::core::primitive::u64,
                    pub s2: ::core::primitive::u64,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub enum MeetupResult {
                    #[codec(index = 0)]
                    Ok,
                    #[codec(index = 1)]
                    VotesNotDependable,
                    #[codec(index = 2)]
                    MeetupValidationIndexOutOfBounds,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub enum ParticipantType {
                    #[codec(index = 0)]
                    Bootstrapper,
                    #[codec(index = 1)]
                    Reputable,
                    #[codec(index = 2)]
                    Endorsee,
                    #[codec(index = 3)]
                    Newbie,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct ProofOfAttendance<_0, _1> {
                    pub prover_public: _1,
                    pub ceremony_index: ::core::primitive::u32,
                    pub community_identifier:
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    pub attendee_public: _1,
                    pub attendee_signature: _0,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub enum Reputation {
                    #[codec(index = 0)]
                    Unverified,
                    #[codec(index = 1)]
                    UnverifiedReputable,
                    #[codec(index = 2)]
                    VerifiedUnlinked,
                    #[codec(index = 3)]
                    VerifiedLinked,
                }
            }
            pub mod communities {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct CommunityIdentifier {
                    pub geohash: [::core::primitive::u8; 5usize],
                    pub digest: [::core::primitive::u8; 4usize],
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct CommunityMetadata {
                    pub name: ::std::vec::Vec<::core::primitive::u8>,
                    pub symbol: ::std::vec::Vec<::core::primitive::u8>,
                    pub assets: ::std::vec::Vec<::core::primitive::u8>,
                    pub theme: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                    pub url: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct Location {
                    pub lat: runtime_types::substrate_fixed::FixedI128<
                        runtime_types::typenum::uint::UInt<
                            runtime_types::typenum::uint::UInt<
                                runtime_types::typenum::uint::UInt<
                                    runtime_types::typenum::uint::UInt<
                                        runtime_types::typenum::uint::UInt<
                                            runtime_types::typenum::uint::UInt<
                                                runtime_types::typenum::uint::UInt<
                                                    runtime_types::typenum::uint::UTerm,
                                                    runtime_types::typenum::bit::B1,
                                                >,
                                                runtime_types::typenum::bit::B0,
                                            >,
                                            runtime_types::typenum::bit::B0,
                                        >,
                                        runtime_types::typenum::bit::B0,
                                    >,
                                    runtime_types::typenum::bit::B0,
                                >,
                                runtime_types::typenum::bit::B0,
                            >,
                            runtime_types::typenum::bit::B0,
                        >,
                    >,
                    pub lon: runtime_types::substrate_fixed::FixedI128<
                        runtime_types::typenum::uint::UInt<
                            runtime_types::typenum::uint::UInt<
                                runtime_types::typenum::uint::UInt<
                                    runtime_types::typenum::uint::UInt<
                                        runtime_types::typenum::uint::UInt<
                                            runtime_types::typenum::uint::UInt<
                                                runtime_types::typenum::uint::UInt<
                                                    runtime_types::typenum::uint::UTerm,
                                                    runtime_types::typenum::bit::B1,
                                                >,
                                                runtime_types::typenum::bit::B0,
                                            >,
                                            runtime_types::typenum::bit::B0,
                                        >,
                                        runtime_types::typenum::bit::B0,
                                    >,
                                    runtime_types::typenum::bit::B0,
                                >,
                                runtime_types::typenum::bit::B0,
                            >,
                            runtime_types::typenum::bit::B0,
                        >,
                    >,
                }
            }
            pub mod scheduler {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub enum CeremonyPhaseType {
                    #[codec(index = 0)]
                    Registering,
                    #[codec(index = 1)]
                    Assigning,
                    #[codec(index = 2)]
                    Attesting,
                }
            }
        }
        pub mod encointer_runtime {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum Call {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Call),
                #[codec(index = 1)]
                ParachainSystem(runtime_types::cumulus_pallet_parachain_system::pallet::Call),
                #[codec(index = 3)]
                Timestamp(runtime_types::pallet_timestamp::pallet::Call),
                #[codec(index = 10)]
                Balances(runtime_types::pallet_balances::pallet::Call),
                #[codec(index = 30)]
                XcmpQueue(runtime_types::cumulus_pallet_xcmp_queue::pallet::Call),
                #[codec(index = 31)]
                PolkadotXcm(runtime_types::pallet_xcm::pallet::Call),
                #[codec(index = 33)]
                DmpQueue(runtime_types::cumulus_pallet_dmp_queue::pallet::Call),
                #[codec(index = 40)]
                Utility(runtime_types::pallet_utility::pallet::Call),
                #[codec(index = 43)]
                Treasury(runtime_types::pallet_treasury::pallet::Call),
                #[codec(index = 44)]
                Proxy(runtime_types::pallet_proxy::pallet::Call),
                #[codec(index = 48)]
                Scheduler(runtime_types::pallet_scheduler::pallet::Call),
                #[codec(index = 50)]
                Collective(runtime_types::pallet_collective::pallet::Call),
                #[codec(index = 51)]
                Membership(runtime_types::pallet_membership::pallet::Call),
                #[codec(index = 60)]
                EncointerScheduler(runtime_types::pallet_encointer_scheduler::pallet::Call),
                #[codec(index = 61)]
                EncointerCeremonies(runtime_types::pallet_encointer_ceremonies::pallet::Call),
                #[codec(index = 62)]
                EncointerCommunities(runtime_types::pallet_encointer_communities::pallet::Call),
                #[codec(index = 63)]
                EncointerBalances(runtime_types::pallet_encointer_balances::pallet::Call),
                #[codec(index = 64)]
                EncointerBazaar(runtime_types::pallet_encointer_bazaar::pallet::Call),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum Event {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Event),
                #[codec(index = 1)]
                ParachainSystem(runtime_types::cumulus_pallet_parachain_system::pallet::Event),
                #[codec(index = 10)]
                Balances(runtime_types::pallet_balances::pallet::Event),
                #[codec(index = 11)]
                TransactionPayment(runtime_types::pallet_transaction_payment::pallet::Event),
                #[codec(index = 12)]
                AssetTxPayment(runtime_types::pallet_asset_tx_payment::pallet::Event),
                #[codec(index = 30)]
                XcmpQueue(runtime_types::cumulus_pallet_xcmp_queue::pallet::Event),
                #[codec(index = 31)]
                PolkadotXcm(runtime_types::pallet_xcm::pallet::Event),
                #[codec(index = 32)]
                CumulusXcm(runtime_types::cumulus_pallet_xcm::pallet::Event),
                #[codec(index = 33)]
                DmpQueue(runtime_types::cumulus_pallet_dmp_queue::pallet::Event),
                #[codec(index = 40)]
                Utility(runtime_types::pallet_utility::pallet::Event),
                #[codec(index = 43)]
                Treasury(runtime_types::pallet_treasury::pallet::Event),
                #[codec(index = 44)]
                Proxy(runtime_types::pallet_proxy::pallet::Event),
                #[codec(index = 48)]
                Scheduler(runtime_types::pallet_scheduler::pallet::Event),
                #[codec(index = 50)]
                Collective(runtime_types::pallet_collective::pallet::Event),
                #[codec(index = 51)]
                Membership(runtime_types::pallet_membership::pallet::Event),
                #[codec(index = 60)]
                EncointerScheduler(runtime_types::pallet_encointer_scheduler::pallet::Event),
                #[codec(index = 61)]
                EncointerCeremonies(runtime_types::pallet_encointer_ceremonies::pallet::Event),
                #[codec(index = 62)]
                EncointerCommunities(runtime_types::pallet_encointer_communities::pallet::Event),
                #[codec(index = 63)]
                EncointerBalances(runtime_types::pallet_encointer_balances::pallet::Event),
                #[codec(index = 64)]
                EncointerBazaar(runtime_types::pallet_encointer_bazaar::pallet::Event),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum OriginCaller {
                #[codec(index = 0)]
                system(
                    runtime_types::frame_support::dispatch::RawOrigin<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                    >,
                ),
                #[codec(index = 31)]
                PolkadotXcm(runtime_types::pallet_xcm::pallet::Origin),
                #[codec(index = 32)]
                CumulusXcm(runtime_types::cumulus_pallet_xcm::pallet::Origin),
                #[codec(index = 50)]
                Collective(
                    runtime_types::pallet_collective::RawOrigin<
                        ::subxt::ext::sp_core::crypto::AccountId32,
                    >,
                ),
                #[codec(index = 4)]
                Void(runtime_types::sp_core::Void),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum ProxyType {
                #[codec(index = 0)]
                Any,
                #[codec(index = 1)]
                NonTransfer,
                #[codec(index = 2)]
                BazaarEdit,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Runtime;
        }
        pub mod frame_support {
            use super::runtime_types;
            pub mod dispatch {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub enum RawOrigin<_0> {
                    #[codec(index = 0)]
                    Root,
                    #[codec(index = 1)]
                    Signed(_0),
                    #[codec(index = 2)]
                    None,
                }
            }
            pub mod traits {
                use super::runtime_types;
                pub mod schedule {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub enum LookupError {
                        #[codec(index = 0)]
                        Unknown,
                        #[codec(index = 1)]
                        BadFormat,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub enum MaybeHashed<_0, _1> {
                        #[codec(index = 0)]
                        Value(_0),
                        #[codec(index = 1)]
                        Hash(_1),
                    }
                }
                pub mod tokens {
                    use super::runtime_types;
                    pub mod misc {
                        use super::runtime_types;
                        #[derive(
                            :: subxt :: ext :: codec :: Decode,
                            :: subxt :: ext :: codec :: Encode,
                            Debug,
                        )]
                        pub enum BalanceStatus {
                            #[codec(index = 0)]
                            Free,
                            #[codec(index = 1)]
                            Reserved,
                        }
                    }
                }
            }
            pub mod weights {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub enum DispatchClass {
                    #[codec(index = 0)]
                    Normal,
                    #[codec(index = 1)]
                    Operational,
                    #[codec(index = 2)]
                    Mandatory,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct DispatchInfo {
                    pub weight: ::core::primitive::u64,
                    pub class: runtime_types::frame_support::weights::DispatchClass,
                    pub pays_fee: runtime_types::frame_support::weights::Pays,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub enum Pays {
                    #[codec(index = 0)]
                    Yes,
                    #[codec(index = 1)]
                    No,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct PerDispatchClass<_0> {
                    pub normal: _0,
                    pub operational: _0,
                    pub mandatory: _0,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct RuntimeDbWeight {
                    pub read: ::core::primitive::u64,
                    pub write: ::core::primitive::u64,
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct PalletId(pub [::core::primitive::u8; 8usize]);
        }
        pub mod frame_system {
            use super::runtime_types;
            pub mod extensions {
                use super::runtime_types;
                pub mod check_genesis {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct CheckGenesis;
                }
                pub mod check_mortality {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
                }
                pub mod check_non_zero_sender {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct CheckNonZeroSender;
                }
                pub mod check_nonce {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
                }
                pub mod check_spec_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct CheckSpecVersion;
                }
                pub mod check_tx_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct CheckTxVersion;
                }
                pub mod check_weight {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct CheckWeight;
                }
            }
            pub mod limits {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct BlockLength {
                    pub max: runtime_types::frame_support::weights::PerDispatchClass<
                        ::core::primitive::u32,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct BlockWeights {
                    pub base_block: ::core::primitive::u64,
                    pub max_block: ::core::primitive::u64,
                    pub per_class: runtime_types::frame_support::weights::PerDispatchClass<
                        runtime_types::frame_system::limits::WeightsPerClass,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct WeightsPerClass {
                    pub base_extrinsic: ::core::primitive::u64,
                    pub max_extrinsic: ::core::option::Option<::core::primitive::u64>,
                    pub max_total: ::core::option::Option<::core::primitive::u64>,
                    pub reserved: ::core::option::Option<::core::primitive::u64>,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "A dispatch that will fill the block weight up to the given ratio."]
                    fill_block {
                        ratio: runtime_types::sp_arithmetic::per_things::Perbill,
                    },
                    #[codec(index = 1)]
                    #[doc = "Make some on-chain remark."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- `O(1)`"]
                    #[doc = "# </weight>"]
                    remark {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 2)]
                    #[doc = "Set the number of pages in the WebAssembly environment's heap."]
                    set_heap_pages { pages: ::core::primitive::u64 },
                    #[codec(index = 3)]
                    #[doc = "Set the new runtime code."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- `O(C + S)` where `C` length of `code` and `S` complexity of `can_set_code`"]
                    #[doc = "- 1 call to `can_set_code`: `O(S)` (calls `sp_io::misc::runtime_version` which is"]
                    #[doc = "  expensive)."]
                    #[doc = "- 1 storage write (codec `O(C)`)."]
                    #[doc = "- 1 digest item."]
                    #[doc = "- 1 event."]
                    #[doc = "The weight of this function is dependent on the runtime, but generally this is very"]
                    #[doc = "expensive. We will treat this as a full block."]
                    #[doc = "# </weight>"]
                    set_code {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    #[doc = "Set the new runtime code without doing any checks of the given `code`."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- `O(C)` where `C` length of `code`"]
                    #[doc = "- 1 storage write (codec `O(C)`)."]
                    #[doc = "- 1 digest item."]
                    #[doc = "- 1 event."]
                    #[doc = "The weight of this function is dependent on the runtime. We will treat this as a full"]
                    #[doc = "block. # </weight>"]
                    set_code_without_checks {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 5)]
                    #[doc = "Set some items of storage."]
                    set_storage {
                        items: ::std::vec::Vec<(
                            ::std::vec::Vec<::core::primitive::u8>,
                            ::std::vec::Vec<::core::primitive::u8>,
                        )>,
                    },
                    #[codec(index = 6)]
                    #[doc = "Kill some items from storage."]
                    kill_storage {
                        keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    },
                    #[codec(index = 7)]
                    #[doc = "Kill all storage items with a key that starts with the given prefix."]
                    #[doc = ""]
                    #[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
                    #[doc = "the prefix we are removing to accurately calculate the weight of this function."]
                    kill_prefix {
                        prefix: ::std::vec::Vec<::core::primitive::u8>,
                        subkeys: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
                    #[doc = "Make some on-chain remark and emit event."]
                    remark_with_event {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Error for the System pallet"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The name of specification does not match between the current runtime"]
                    #[doc = "and the new runtime."]
                    InvalidSpecName,
                    #[codec(index = 1)]
                    #[doc = "The specification version is not allowed to decrease between the current runtime"]
                    #[doc = "and the new runtime."]
                    SpecVersionNeedsToIncrease,
                    #[codec(index = 2)]
                    #[doc = "Failed to extract the runtime version from the new runtime."]
                    #[doc = ""]
                    #[doc = "Either calling `Core_version` or decoding `RuntimeVersion` failed."]
                    FailedToExtractRuntimeVersion,
                    #[codec(index = 3)]
                    #[doc = "Suicide called when the account has non-default composite data."]
                    NonDefaultComposite,
                    #[codec(index = 4)]
                    #[doc = "There is a non-zero reference count preventing the account from being purged."]
                    NonZeroRefCount,
                    #[codec(index = 5)]
                    #[doc = "The origin filter prevent the call to be dispatched."]
                    CallFiltered,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Event for the System pallet."]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "An extrinsic completed successfully."]
                    ExtrinsicSuccess {
                        dispatch_info: runtime_types::frame_support::weights::DispatchInfo,
                    },
                    #[codec(index = 1)]
                    #[doc = "An extrinsic failed."]
                    ExtrinsicFailed {
                        dispatch_error: runtime_types::sp_runtime::DispatchError,
                        dispatch_info: runtime_types::frame_support::weights::DispatchInfo,
                    },
                    #[codec(index = 2)]
                    #[doc = "`:code` was updated."]
                    CodeUpdated,
                    #[codec(index = 3)]
                    #[doc = "A new account was created."]
                    NewAccount {
                        account: ::subxt::ext::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 4)]
                    #[doc = "An account was reaped."]
                    KilledAccount {
                        account: ::subxt::ext::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 5)]
                    #[doc = "On on-chain remark happened."]
                    Remarked {
                        sender: ::subxt::ext::sp_core::crypto::AccountId32,
                        hash: ::subxt::ext::sp_core::H256,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct AccountInfo<_0, _1> {
                pub nonce: _0,
                pub consumers: _0,
                pub providers: _0,
                pub sufficients: _0,
                pub data: _1,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct EventRecord<_0, _1> {
                pub phase: runtime_types::frame_system::Phase,
                pub event: _0,
                pub topics: ::std::vec::Vec<_1>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct LastRuntimeUpgradeInfo {
                #[codec(compact)]
                pub spec_version: ::core::primitive::u32,
                pub spec_name: ::std::string::String,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum Phase {
                #[codec(index = 0)]
                ApplyExtrinsic(::core::primitive::u32),
                #[codec(index = 1)]
                Finalization,
                #[codec(index = 2)]
                Initialization,
            }
        }
        pub mod geohash {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct GeoHash(pub [::core::primitive::u8; 5usize]);
        }
        pub mod pallet_asset_tx_payment {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,"]
                    #[doc = "has been paid by `who` in an asset `asset_id`."]
                    AssetTxFeePaid {
                        who: ::subxt::ext::sp_core::crypto::AccountId32,
                        actual_fee: ::core::primitive::u128,
                        tip: ::core::primitive::u128,
                        asset_id: ::core::option::Option<
                            runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        >,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ChargeAssetTxPayment {
                #[codec(compact)]
                pub tip: ::core::primitive::u128,
                pub asset_id: ::core::option::Option<
                    runtime_types::encointer_primitives::communities::CommunityIdentifier,
                >,
            }
        }
        pub mod pallet_balances {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Transfer some liquid free balance to another account."]
                    #[doc = ""]
                    #[doc = "`transfer` will set the `FreeBalance` of the sender and receiver."]
                    #[doc = "If the sender's account is below the existential deposit as a result"]
                    #[doc = "of the transfer, the account will be reaped."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- Dependent on arguments but not critical, given proper implementations for input config"]
                    #[doc = "  types. See related functions below."]
                    #[doc = "- It contains a limited number of reads and writes internally and no complex"]
                    #[doc = "  computation."]
                    #[doc = ""]
                    #[doc = "Related functions:"]
                    #[doc = ""]
                    #[doc = "  - `ensure_can_withdraw` is always called internally but has a bounded complexity."]
                    #[doc = "  - Transferring balances to accounts that did not exist before will cause"]
                    #[doc = "    `T::OnNewAccount::on_new_account` to be called."]
                    #[doc = "  - Removing enough funds from an account will trigger `T::DustRemoval::on_unbalanced`."]
                    #[doc = "  - `transfer_keep_alive` works the same way as `transfer`, but has an additional check"]
                    #[doc = "    that the transfer will not kill the origin account."]
                    #[doc = "---------------------------------"]
                    #[doc = "- Origin account is already in memory, so no DB operations for them."]
                    #[doc = "# </weight>"]
                    transfer {
                        dest: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    #[doc = "Set the balances of a given account."]
                    #[doc = ""]
                    #[doc = "This will alter `FreeBalance` and `ReservedBalance` in storage. it will"]
                    #[doc = "also alter the total issuance of the system (`TotalIssuance`) appropriately."]
                    #[doc = "If the new free or reserved balance is below the existential deposit,"]
                    #[doc = "it will reset the account nonce (`frame_system::AccountNonce`)."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call is `root`."]
                    set_balance {
                        who: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        new_free: ::core::primitive::u128,
                        #[codec(compact)]
                        new_reserved: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "Exactly as `transfer`, except the origin must be root and the source account may be"]
                    #[doc = "specified."]
                    #[doc = "# <weight>"]
                    #[doc = "- Same as transfer, but additional read and write because the source account is not"]
                    #[doc = "  assumed to be in the overlay."]
                    #[doc = "# </weight>"]
                    force_transfer {
                        source: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            (),
                        >,
                        dest: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "Same as the [`transfer`] call, but with a check that the transfer will not kill the"]
                    #[doc = "origin account."]
                    #[doc = ""]
                    #[doc = "99% of the time you want [`transfer`] instead."]
                    #[doc = ""]
                    #[doc = "[`transfer`]: struct.Pallet.html#method.transfer"]
                    transfer_keep_alive {
                        dest: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    #[doc = "Transfer the entire transferable balance from the caller account."]
                    #[doc = ""]
                    #[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
                    #[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
                    #[doc = "transferred by this function. To ensure that this function results in a killed account,"]
                    #[doc = "you might need to prepare the account by removing any reference counters, storage"]
                    #[doc = "deposits, etc..."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be Signed."]
                    #[doc = ""]
                    #[doc = "- `dest`: The recipient of the transfer."]
                    #[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
                    #[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
                    #[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
                    #[doc = "  keep the sender account alive (true). # <weight>"]
                    #[doc = "- O(1). Just like transfer, but reading the user's transferable balance first."]
                    #[doc = "  #</weight>"]
                    transfer_all {
                        dest: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            (),
                        >,
                        keep_alive: ::core::primitive::bool,
                    },
                    #[codec(index = 5)]
                    #[doc = "Unreserve some balance from a user by force."]
                    #[doc = ""]
                    #[doc = "Can only be called by ROOT."]
                    force_unreserve {
                        who: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            (),
                        >,
                        amount: ::core::primitive::u128,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Vesting balance too high to send value"]
                    VestingBalance,
                    #[codec(index = 1)]
                    #[doc = "Account liquidity restrictions prevent withdrawal"]
                    LiquidityRestrictions,
                    #[codec(index = 2)]
                    #[doc = "Balance too low to send value"]
                    InsufficientBalance,
                    #[codec(index = 3)]
                    #[doc = "Value too low to create account due to existential deposit"]
                    ExistentialDeposit,
                    #[codec(index = 4)]
                    #[doc = "Transfer/payment would kill account"]
                    KeepAlive,
                    #[codec(index = 5)]
                    #[doc = "A vesting schedule already exists for this account"]
                    ExistingVestingSchedule,
                    #[codec(index = 6)]
                    #[doc = "Beneficiary account must pre-exist"]
                    DeadAccount,
                    #[codec(index = 7)]
                    #[doc = "Number of named reserves exceed MaxReserves"]
                    TooManyReserves,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "An account was created with some free balance."]
                    Endowed {
                        account: ::subxt::ext::sp_core::crypto::AccountId32,
                        free_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    #[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
                    #[doc = "resulting in an outright loss."]
                    DustLost {
                        account: ::subxt::ext::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "Transfer succeeded."]
                    Transfer {
                        from: ::subxt::ext::sp_core::crypto::AccountId32,
                        to: ::subxt::ext::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "A balance was set by root."]
                    BalanceSet {
                        who: ::subxt::ext::sp_core::crypto::AccountId32,
                        free: ::core::primitive::u128,
                        reserved: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    #[doc = "Some balance was reserved (moved from free to reserved)."]
                    Reserved {
                        who: ::subxt::ext::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    #[doc = "Some balance was unreserved (moved from reserved to free)."]
                    Unreserved {
                        who: ::subxt::ext::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    #[doc = "Some balance was moved from the reserve of the first account to the second account."]
                    #[doc = "Final argument indicates the destination balance type."]
                    ReserveRepatriated {
                        from: ::subxt::ext::sp_core::crypto::AccountId32,
                        to: ::subxt::ext::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                        destination_status:
                            runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
                    },
                    #[codec(index = 7)]
                    #[doc = "Some amount was deposited (e.g. for transaction fees)."]
                    Deposit {
                        who: ::subxt::ext::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    #[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
                    Withdraw {
                        who: ::subxt::ext::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    #[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
                    Slashed {
                        who: ::subxt::ext::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct AccountData<_0> {
                pub free: _0,
                pub reserved: _0,
                pub misc_frozen: _0,
                pub fee_frozen: _0,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct BalanceLock<_0> {
                pub id: [::core::primitive::u8; 8usize],
                pub amount: _0,
                pub reasons: runtime_types::pallet_balances::Reasons,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum Reasons {
                #[codec(index = 0)]
                Fee,
                #[codec(index = 1)]
                Misc,
                #[codec(index = 2)]
                All,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum Releases {
                #[codec(index = 0)]
                V1_0_0,
                #[codec(index = 1)]
                V2_0_0,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ReserveData<_0, _1> {
                pub id: _0,
                pub amount: _1,
            }
        }
        pub mod pallet_collective {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Set the collective's membership."]
                    #[doc = ""]
                    #[doc = "- `new_members`: The new member list. Be nice to the chain and provide it sorted."]
                    #[doc = "- `prime`: The prime member whose vote sets the default."]
                    #[doc = "- `old_count`: The upper bound for the previous number of members in storage. Used for"]
                    #[doc = "  weight estimation."]
                    #[doc = ""]
                    #[doc = "Requires root origin."]
                    #[doc = ""]
                    #[doc = "NOTE: Does not enforce the expected `MaxMembers` limit on the amount of members, but"]
                    #[doc = "      the weight estimations rely on it to estimate dispatchable weight."]
                    #[doc = ""]
                    #[doc = "# WARNING:"]
                    #[doc = ""]
                    #[doc = "The `pallet-collective` can also be managed by logic outside of the pallet through the"]
                    #[doc = "implementation of the trait [`ChangeMembers`]."]
                    #[doc = "Any call to `set_members` must be careful that the member set doesn't get out of sync"]
                    #[doc = "with other logic managing the member set."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "## Weight"]
                    #[doc = "- `O(MP + N)` where:"]
                    #[doc = "  - `M` old-members-count (code- and governance-bounded)"]
                    #[doc = "  - `N` new-members-count (code- and governance-bounded)"]
                    #[doc = "  - `P` proposals-count (code-bounded)"]
                    #[doc = "- DB:"]
                    #[doc = "  - 1 storage mutation (codec `O(M)` read, `O(N)` write) for reading and writing the"]
                    #[doc = "    members"]
                    #[doc = "  - 1 storage read (codec `O(P)`) for reading the proposals"]
                    #[doc = "  - `P` storage mutations (codec `O(M)`) for updating the votes for each proposal"]
                    #[doc = "  - 1 storage write (codec `O(1)`) for deleting the old `prime` and setting the new one"]
                    #[doc = "# </weight>"]
                    set_members {
                        new_members: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
                        prime: ::core::option::Option<::subxt::ext::sp_core::crypto::AccountId32>,
                        old_count: ::core::primitive::u32,
                    },
                    #[codec(index = 1)]
                    #[doc = "Dispatch a proposal from a member using the `Member` origin."]
                    #[doc = ""]
                    #[doc = "Origin must be a member of the collective."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "## Weight"]
                    #[doc = "- `O(M + P)` where `M` members-count (code-bounded) and `P` complexity of dispatching"]
                    #[doc = "  `proposal`"]
                    #[doc = "- DB: 1 read (codec `O(M)`) + DB access of `proposal`"]
                    #[doc = "- 1 event"]
                    #[doc = "# </weight>"]
                    execute {
                        proposal: ::std::boxed::Box<runtime_types::encointer_runtime::Call>,
                        #[codec(compact)]
                        length_bound: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    #[doc = "Add a new proposal to either be voted on or executed directly."]
                    #[doc = ""]
                    #[doc = "Requires the sender to be member."]
                    #[doc = ""]
                    #[doc = "`threshold` determines whether `proposal` is executed directly (`threshold < 2`)"]
                    #[doc = "or put up for voting."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "## Weight"]
                    #[doc = "- `O(B + M + P1)` or `O(B + M + P2)` where:"]
                    #[doc = "  - `B` is `proposal` size in bytes (length-fee-bounded)"]
                    #[doc = "  - `M` is members-count (code- and governance-bounded)"]
                    #[doc = "  - branching is influenced by `threshold` where:"]
                    #[doc = "    - `P1` is proposal execution complexity (`threshold < 2`)"]
                    #[doc = "    - `P2` is proposals-count (code-bounded) (`threshold >= 2`)"]
                    #[doc = "- DB:"]
                    #[doc = "  - 1 storage read `is_member` (codec `O(M)`)"]
                    #[doc = "  - 1 storage read `ProposalOf::contains_key` (codec `O(1)`)"]
                    #[doc = "  - DB accesses influenced by `threshold`:"]
                    #[doc = "    - EITHER storage accesses done by `proposal` (`threshold < 2`)"]
                    #[doc = "    - OR proposal insertion (`threshold <= 2`)"]
                    #[doc = "      - 1 storage mutation `Proposals` (codec `O(P2)`)"]
                    #[doc = "      - 1 storage mutation `ProposalCount` (codec `O(1)`)"]
                    #[doc = "      - 1 storage write `ProposalOf` (codec `O(B)`)"]
                    #[doc = "      - 1 storage write `Voting` (codec `O(M)`)"]
                    #[doc = "  - 1 event"]
                    #[doc = "# </weight>"]
                    propose {
                        #[codec(compact)]
                        threshold: ::core::primitive::u32,
                        proposal: ::std::boxed::Box<runtime_types::encointer_runtime::Call>,
                        #[codec(compact)]
                        length_bound: ::core::primitive::u32,
                    },
                    #[codec(index = 3)]
                    #[doc = "Add an aye or nay vote for the sender to the given proposal."]
                    #[doc = ""]
                    #[doc = "Requires the sender to be a member."]
                    #[doc = ""]
                    #[doc = "Transaction fees will be waived if the member is voting on any particular proposal"]
                    #[doc = "for the first time and the call is successful. Subsequent vote changes will charge a"]
                    #[doc = "fee."]
                    #[doc = "# <weight>"]
                    #[doc = "## Weight"]
                    #[doc = "- `O(M)` where `M` is members-count (code- and governance-bounded)"]
                    #[doc = "- DB:"]
                    #[doc = "  - 1 storage read `Members` (codec `O(M)`)"]
                    #[doc = "  - 1 storage mutation `Voting` (codec `O(M)`)"]
                    #[doc = "- 1 event"]
                    #[doc = "# </weight>"]
                    vote {
                        proposal: ::subxt::ext::sp_core::H256,
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                        approve: ::core::primitive::bool,
                    },
                    #[codec(index = 4)]
                    #[doc = "Close a vote that is either approved, disapproved or whose voting period has ended."]
                    #[doc = ""]
                    #[doc = "May be called by any signed account in order to finish voting and close the proposal."]
                    #[doc = ""]
                    #[doc = "If called before the end of the voting period it will only close the vote if it is"]
                    #[doc = "has enough votes to be approved or disapproved."]
                    #[doc = ""]
                    #[doc = "If called after the end of the voting period abstentions are counted as rejections"]
                    #[doc = "unless there is a prime member set and the prime member cast an approval."]
                    #[doc = ""]
                    #[doc = "If the close operation completes successfully with disapproval, the transaction fee will"]
                    #[doc = "be waived. Otherwise execution of the approved operation will be charged to the caller."]
                    #[doc = ""]
                    #[doc = "+ `proposal_weight_bound`: The maximum amount of weight consumed by executing the closed"]
                    #[doc = "proposal."]
                    #[doc = "+ `length_bound`: The upper bound for the length of the proposal in storage. Checked via"]
                    #[doc = "`storage::read` so it is `size_of::<u32>() == 4` larger than the pure length."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "## Weight"]
                    #[doc = "- `O(B + M + P1 + P2)` where:"]
                    #[doc = "  - `B` is `proposal` size in bytes (length-fee-bounded)"]
                    #[doc = "  - `M` is members-count (code- and governance-bounded)"]
                    #[doc = "  - `P1` is the complexity of `proposal` preimage."]
                    #[doc = "  - `P2` is proposal-count (code-bounded)"]
                    #[doc = "- DB:"]
                    #[doc = " - 2 storage reads (`Members`: codec `O(M)`, `Prime`: codec `O(1)`)"]
                    #[doc = " - 3 mutations (`Voting`: codec `O(M)`, `ProposalOf`: codec `O(B)`, `Proposals`: codec"]
                    #[doc = "   `O(P2)`)"]
                    #[doc = " - any mutations done while executing `proposal` (`P1`)"]
                    #[doc = "- up to 3 events"]
                    #[doc = "# </weight>"]
                    close {
                        proposal_hash: ::subxt::ext::sp_core::H256,
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                        #[codec(compact)]
                        proposal_weight_bound: ::core::primitive::u64,
                        #[codec(compact)]
                        length_bound: ::core::primitive::u32,
                    },
                    #[codec(index = 5)]
                    #[doc = "Disapprove a proposal, close, and remove it from the system, regardless of its current"]
                    #[doc = "state."]
                    #[doc = ""]
                    #[doc = "Must be called by the Root origin."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "* `proposal_hash`: The hash of the proposal that should be disapproved."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "Complexity: O(P) where P is the number of max proposals"]
                    #[doc = "DB Weight:"]
                    #[doc = "* Reads: Proposals"]
                    #[doc = "* Writes: Voting, Proposals, ProposalOf"]
                    #[doc = "# </weight>"]
                    disapprove_proposal {
                        proposal_hash: ::subxt::ext::sp_core::H256,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Account is not a member"]
                    NotMember,
                    #[codec(index = 1)]
                    #[doc = "Duplicate proposals not allowed"]
                    DuplicateProposal,
                    #[codec(index = 2)]
                    #[doc = "Proposal must exist"]
                    ProposalMissing,
                    #[codec(index = 3)]
                    #[doc = "Mismatched index"]
                    WrongIndex,
                    #[codec(index = 4)]
                    #[doc = "Duplicate vote ignored"]
                    DuplicateVote,
                    #[codec(index = 5)]
                    #[doc = "Members are already initialized!"]
                    AlreadyInitialized,
                    #[codec(index = 6)]
                    #[doc = "The close call was made too early, before the end of the voting."]
                    TooEarly,
                    #[codec(index = 7)]
                    #[doc = "There can only be a maximum of `MaxProposals` active proposals."]
                    TooManyProposals,
                    #[codec(index = 8)]
                    #[doc = "The given weight bound for the proposal was too low."]
                    WrongProposalWeight,
                    #[codec(index = 9)]
                    #[doc = "The given length bound for the proposal was too low."]
                    WrongProposalLength,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A motion (given hash) has been proposed (by given account) with a threshold (given"]
                    #[doc = "`MemberCount`)."]
                    Proposed {
                        account: ::subxt::ext::sp_core::crypto::AccountId32,
                        proposal_index: ::core::primitive::u32,
                        proposal_hash: ::subxt::ext::sp_core::H256,
                        threshold: ::core::primitive::u32,
                    },
                    #[codec(index = 1)]
                    #[doc = "A motion (given hash) has been voted on by given account, leaving"]
                    #[doc = "a tally (yes votes and no votes given respectively as `MemberCount`)."]
                    Voted {
                        account: ::subxt::ext::sp_core::crypto::AccountId32,
                        proposal_hash: ::subxt::ext::sp_core::H256,
                        voted: ::core::primitive::bool,
                        yes: ::core::primitive::u32,
                        no: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    #[doc = "A motion was approved by the required threshold."]
                    Approved {
                        proposal_hash: ::subxt::ext::sp_core::H256,
                    },
                    #[codec(index = 3)]
                    #[doc = "A motion was not approved by the required threshold."]
                    Disapproved {
                        proposal_hash: ::subxt::ext::sp_core::H256,
                    },
                    #[codec(index = 4)]
                    #[doc = "A motion was executed; result will be `Ok` if it returned without error."]
                    Executed {
                        proposal_hash: ::subxt::ext::sp_core::H256,
                        result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 5)]
                    #[doc = "A single member did some action; result will be `Ok` if it returned without error."]
                    MemberExecuted {
                        proposal_hash: ::subxt::ext::sp_core::H256,
                        result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 6)]
                    #[doc = "A proposal was closed because its threshold was reached or after its duration was up."]
                    Closed {
                        proposal_hash: ::subxt::ext::sp_core::H256,
                        yes: ::core::primitive::u32,
                        no: ::core::primitive::u32,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum RawOrigin<_0> {
                #[codec(index = 0)]
                Members(::core::primitive::u32, ::core::primitive::u32),
                #[codec(index = 1)]
                Member(_0),
                #[codec(index = 2)]
                _Phantom,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Votes<_0, _1> {
                pub index: _1,
                pub threshold: _1,
                pub ayes: ::std::vec::Vec<_0>,
                pub nays: ::std::vec::Vec<_0>,
                pub end: _1,
            }
        }
        pub mod pallet_encointer_balances {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Transfer some balance to another account."]
                    transfer {
                        dest: ::subxt::ext::sp_core::crypto::AccountId32,
                        community_id:
                            runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        amount: runtime_types::substrate_fixed::FixedI128<
                            runtime_types::typenum::uint::UInt<
                                runtime_types::typenum::uint::UInt<
                                    runtime_types::typenum::uint::UInt<
                                        runtime_types::typenum::uint::UInt<
                                            runtime_types::typenum::uint::UInt<
                                                runtime_types::typenum::uint::UInt<
                                                    runtime_types::typenum::uint::UInt<
                                                        runtime_types::typenum::uint::UTerm,
                                                        runtime_types::typenum::bit::B1,
                                                    >,
                                                    runtime_types::typenum::bit::B0,
                                                >,
                                                runtime_types::typenum::bit::B0,
                                            >,
                                            runtime_types::typenum::bit::B0,
                                        >,
                                        runtime_types::typenum::bit::B0,
                                    >,
                                    runtime_types::typenum::bit::B0,
                                >,
                                runtime_types::typenum::bit::B0,
                            >,
                        >,
                    },
                    #[codec(index = 1)]
                    set_fee_conversion_factor {
                        fee_conversion_factor: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    transfer_all {
                        dest: ::subxt::ext::sp_core::crypto::AccountId32,
                        cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "the balance is too low to perform this action"]
                    BalanceTooLow,
                    #[codec(index = 1)]
                    #[doc = "the total issuance would overflow"]
                    TotalIssuanceOverflow,
                    #[codec(index = 2)]
                    #[doc = "Account to alter does not exist in community"]
                    NoAccount,
                    #[codec(index = 3)]
                    #[doc = "Balance too low to create an account"]
                    ExistentialDeposit,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Endowed a new account with a respective currency `[community_id, who, balance]`"]
                    Endowed {
                        cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        who: ::subxt::ext::sp_core::crypto::AccountId32,
                        balance: runtime_types::substrate_fixed::FixedI128<
                            runtime_types::typenum::uint::UInt<
                                runtime_types::typenum::uint::UInt<
                                    runtime_types::typenum::uint::UInt<
                                        runtime_types::typenum::uint::UInt<
                                            runtime_types::typenum::uint::UInt<
                                                runtime_types::typenum::uint::UInt<
                                                    runtime_types::typenum::uint::UInt<
                                                        runtime_types::typenum::uint::UTerm,
                                                        runtime_types::typenum::bit::B1,
                                                    >,
                                                    runtime_types::typenum::bit::B0,
                                                >,
                                                runtime_types::typenum::bit::B0,
                                            >,
                                            runtime_types::typenum::bit::B0,
                                        >,
                                        runtime_types::typenum::bit::B0,
                                    >,
                                    runtime_types::typenum::bit::B0,
                                >,
                                runtime_types::typenum::bit::B0,
                            >,
                        >,
                    },
                    #[codec(index = 1)]
                    #[doc = "Token transfer success `[community_id, from, to, amount]`"]
                    Transferred(
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        runtime_types::substrate_fixed::FixedI128<
                            runtime_types::typenum::uint::UInt<
                                runtime_types::typenum::uint::UInt<
                                    runtime_types::typenum::uint::UInt<
                                        runtime_types::typenum::uint::UInt<
                                            runtime_types::typenum::uint::UInt<
                                                runtime_types::typenum::uint::UInt<
                                                    runtime_types::typenum::uint::UInt<
                                                        runtime_types::typenum::uint::UTerm,
                                                        runtime_types::typenum::bit::B1,
                                                    >,
                                                    runtime_types::typenum::bit::B0,
                                                >,
                                                runtime_types::typenum::bit::B0,
                                            >,
                                            runtime_types::typenum::bit::B0,
                                        >,
                                        runtime_types::typenum::bit::B0,
                                    >,
                                    runtime_types::typenum::bit::B0,
                                >,
                                runtime_types::typenum::bit::B0,
                            >,
                        >,
                    ),
                    #[codec(index = 2)]
                    #[doc = "Token issuance success `[community_id, beneficiary, amount]`"]
                    Issued(
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        runtime_types::substrate_fixed::FixedI128<
                            runtime_types::typenum::uint::UInt<
                                runtime_types::typenum::uint::UInt<
                                    runtime_types::typenum::uint::UInt<
                                        runtime_types::typenum::uint::UInt<
                                            runtime_types::typenum::uint::UInt<
                                                runtime_types::typenum::uint::UInt<
                                                    runtime_types::typenum::uint::UInt<
                                                        runtime_types::typenum::uint::UTerm,
                                                        runtime_types::typenum::bit::B1,
                                                    >,
                                                    runtime_types::typenum::bit::B0,
                                                >,
                                                runtime_types::typenum::bit::B0,
                                            >,
                                            runtime_types::typenum::bit::B0,
                                        >,
                                        runtime_types::typenum::bit::B0,
                                    >,
                                    runtime_types::typenum::bit::B0,
                                >,
                                runtime_types::typenum::bit::B0,
                            >,
                        >,
                    ),
                    #[codec(index = 3)]
                    #[doc = "fee conversion factor updated successfully"]
                    FeeConversionFactorUpdated(::core::primitive::u128),
                }
            }
        }
        pub mod pallet_encointer_bazaar {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    create_business {
                        cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        url: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    update_business {
                        cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        url: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 2)]
                    delete_business {
                        cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    },
                    #[codec(index = 3)]
                    create_offering {
                        cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        url: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    update_offering {
                        cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        oid: ::core::primitive::u32,
                        url: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 5)]
                    delete_offering {
                        cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        oid: ::core::primitive::u32,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "community identifier not found"]
                    NonexistentCommunity,
                    #[codec(index = 1)]
                    #[doc = "business already registered for this cid"]
                    ExistingBusiness,
                    #[codec(index = 2)]
                    #[doc = "business does not exist"]
                    NonexistentBusiness,
                    #[codec(index = 3)]
                    #[doc = "offering does not exist"]
                    NonexistentOffering,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Event emitted when a business is created. [community, who]"]
                    BusinessCreated(
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        ::subxt::ext::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 1)]
                    #[doc = "Event emitted when a business is updated. [community, who]"]
                    BusinessUpdated(
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        ::subxt::ext::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 2)]
                    #[doc = "Event emitted when a business is deleted. [community, who]"]
                    BusinessDeleted(
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        ::subxt::ext::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 3)]
                    #[doc = "Event emitted when an offering is created. [community, who, oid]"]
                    OfferingCreated(
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    ),
                    #[codec(index = 4)]
                    #[doc = "Event emitted when an offering is updated. [community, who, oid]"]
                    OfferingUpdated(
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    ),
                    #[codec(index = 5)]
                    #[doc = "Event emitted when an offering is deleted. [community, who, oid]"]
                    OfferingDeleted(
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    ),
                }
            }
        }
        pub mod pallet_encointer_ceremonies {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    register_participant {
                        cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        proof: ::core::option::Option<
                            runtime_types::encointer_primitives::ceremonies::ProofOfAttendance<
                                runtime_types::sp_runtime::MultiSignature,
                                ::subxt::ext::sp_core::crypto::AccountId32,
                            >,
                        >,
                    },
                    #[codec(index = 1)]
                    upgrade_registration {
                        cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        proof: runtime_types::encointer_primitives::ceremonies::ProofOfAttendance<
                            runtime_types::sp_runtime::MultiSignature,
                            ::subxt::ext::sp_core::crypto::AccountId32,
                        >,
                    },
                    #[codec(index = 2)]
                    unregister_participant {
                        cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        maybe_reputation_community_ceremony: ::core::option::Option<(
                            runtime_types::encointer_primitives::communities::CommunityIdentifier,
                            ::core::primitive::u32,
                        )>,
                    },
                    #[codec(index = 3)]
                    attest_attendees {
                        cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        number_of_participants_vote: ::core::primitive::u32,
                        attestations: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
                    },
                    #[codec(index = 4)]
                    endorse_newcomer {
                        cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        newbie: ::subxt::ext::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 5)]
                    claim_rewards {
                        cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        maybe_meetup_index: ::core::option::Option<::core::primitive::u64>,
                    },
                    #[codec(index = 6)]
                    set_inactivity_timeout {
                        inactivity_timeout: ::core::primitive::u32,
                    },
                    #[codec(index = 7)]
                    set_endorsement_tickets_per_bootstrapper {
                        endorsement_tickets_per_bootstrapper: ::core::primitive::u8,
                    },
                    #[codec(index = 8)]
                    set_endorsement_tickets_per_reputable {
                        endorsement_tickets_per_reputable: ::core::primitive::u8,
                    },
                    #[codec(index = 9)]
                    set_reputation_lifetime {
                        reputation_lifetime: ::core::primitive::u32,
                    },
                    #[codec(index = 10)]
                    set_meetup_time_offset {
                        meetup_time_offset: ::core::primitive::i32,
                    },
                    #[codec(index = 11)]
                    set_time_tolerance {
                        time_tolerance: ::core::primitive::u64,
                    },
                    #[codec(index = 12)]
                    set_location_tolerance {
                        location_tolerance: ::core::primitive::u32,
                    },
                    #[codec(index = 13)]
                    purge_community_ceremony {
                        community_ceremony: (
                            runtime_types::encointer_primitives::communities::CommunityIdentifier,
                            ::core::primitive::u32,
                        ),
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "the participant is already registered"]
                    ParticipantAlreadyRegistered,
                    #[codec(index = 1)]
                    #[doc = "verification of signature of attendee failed"]
                    BadProofOfAttendanceSignature,
                    #[codec(index = 2)]
                    #[doc = "verification of signature of attendee failed"]
                    BadAttendeeSignature,
                    #[codec(index = 3)]
                    #[doc = "meetup location was not found"]
                    MeetupLocationNotFound,
                    #[codec(index = 4)]
                    #[doc = "meetup time calculation failed"]
                    MeetupTimeCalculationError,
                    #[codec(index = 5)]
                    #[doc = "no valid claims were supplied"]
                    NoValidClaims,
                    #[codec(index = 6)]
                    #[doc = "the action can only be performed during REGISTERING phase"]
                    RegisteringPhaseRequired,
                    #[codec(index = 7)]
                    #[doc = "the action can only be performed during ATTESTING phase"]
                    AttestationPhaseRequired,
                    #[codec(index = 8)]
                    #[doc = "the action can only be performed during REGISTERING or ATTESTING phase"]
                    RegisteringOrAttestationPhaseRequired,
                    #[codec(index = 9)]
                    #[doc = "CommunityIdentifier not found"]
                    InexistentCommunity,
                    #[codec(index = 10)]
                    #[doc = "proof is outdated"]
                    ProofOutdated,
                    #[codec(index = 11)]
                    #[doc = "proof is acausal"]
                    ProofAcausal,
                    #[codec(index = 12)]
                    #[doc = "supplied proof is not proving sender"]
                    WrongProofSubject,
                    #[codec(index = 13)]
                    #[doc = "former attendance has not been verified or has already been linked to other account"]
                    AttendanceUnverifiedOrAlreadyUsed,
                    #[codec(index = 14)]
                    #[doc = "origin not part of this meetup"]
                    OriginNotParticipant,
                    #[codec(index = 15)]
                    #[doc = "can't have more attestations than other meetup participants"]
                    TooManyAttestations,
                    #[codec(index = 16)]
                    #[doc = "can't have more claims than other meetup participants"]
                    TooManyClaims,
                    #[codec(index = 17)]
                    #[doc = "sender has run out of newbie tickets"]
                    NoMoreNewbieTickets,
                    #[codec(index = 18)]
                    #[doc = "newbie is already endorsed"]
                    AlreadyEndorsed,
                    #[codec(index = 19)]
                    #[doc = "Participant is not registered"]
                    ParticipantIsNotRegistered,
                    #[codec(index = 20)]
                    #[doc = "No locations are available for assigning participants"]
                    NoLocationsAvailable,
                    #[codec(index = 21)]
                    #[doc = "Trying to issue rewards in a phase that is not REGISTERING"]
                    WrongPhaseForClaimingRewards,
                    #[codec(index = 22)]
                    #[doc = "Trying to issue rewards for a meetup for which UBI was already issued"]
                    RewardsAlreadyIssued,
                    #[codec(index = 23)]
                    #[doc = "Trying to claim UBI for a meetup where votes are not dependable"]
                    VotesNotDependable,
                    #[codec(index = 24)]
                    #[doc = "Overflow adding user to registry"]
                    RegistryOverflow,
                    #[codec(index = 25)]
                    #[doc = "CheckedMath operation error"]
                    CheckedMath,
                    #[codec(index = 26)]
                    #[doc = "Only Bootstrappers are allowed to be registered at this time"]
                    OnlyBootstrappers,
                    #[codec(index = 27)]
                    #[doc = "MeetupTimeOffset can only be changed during registering"]
                    WrongPhaseForChangingMeetupTimeOffset,
                    #[codec(index = 28)]
                    #[doc = "MeetupTimeOffset needs to be in [-8h, 8h]"]
                    InvalidMeetupTimeOffset,
                    #[codec(index = 29)]
                    #[doc = "the history for given ceremony index and community has been purged"]
                    CommunityCeremonyHistoryPurged,
                    #[codec(index = 30)]
                    #[doc = "Unregistering can only be performed during the registering phase"]
                    WrongPhaseForUnregistering,
                    #[codec(index = 31)]
                    #[doc = "Error while finding meetup participants"]
                    GetMeetupParticipantsError,
                    #[codec(index = 32)]
                    #[doc = "index out of bounds while validating the meetup"]
                    MeetupValidationIndexOutOfBounds,
                    #[codec(index = 33)]
                    #[doc = "Attestations beyond time tolerance"]
                    AttestationsBeyondTimeTolerance,
                    #[codec(index = 34)]
                    #[doc = "Not possible to pay rewards in attestations phase"]
                    EarlyRewardsNotPossible,
                    #[codec(index = 35)]
                    #[doc = "Only newbies can upgrade their registration"]
                    MustBeNewbieToUpgradeRegistration,
                    #[codec(index = 36)]
                    #[doc = "To unregister as a reputable you need to provide a provide a community ceremony where you have a linked reputation"]
                    ReputationCommunityCeremonyRequired,
                    #[codec(index = 37)]
                    #[doc = "In order to unregister a reputable, the provided reputation must be linked"]
                    ReputationMustBeLinked,
                    #[codec(index = 38)]
                    #[doc = "Meetup Index > Meetup Count or < 1"]
                    InvalidMeetupIndex,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Participant registered for next ceremony [community, participant type, who]"]
                    ParticipantRegistered(
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        runtime_types::encointer_primitives::ceremonies::ParticipantType,
                        ::subxt::ext::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 1)]
                    #[doc = "A bootstrapper (first accountid) has endorsed a participant (second accountid) who can now register as endorsee for this ceremony"]
                    EndorsedParticipant(
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        ::subxt::ext::sp_core::crypto::AccountId32,
                        ::subxt::ext::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 2)]
                    #[doc = "A participant has registered N attestations for fellow meetup participants"]
                    AttestationsRegistered(
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        ::core::primitive::u64,
                        ::core::primitive::u32,
                        ::subxt::ext::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 3)]
                    #[doc = "rewards have been claimed and issued successfully for N participants for their meetup at the previous ceremony"]
                    RewardsIssued(
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        ::core::primitive::u64,
                        ::core::primitive::u8,
                    ),
                    #[codec(index = 4)]
                    #[doc = "inactivity timeout has changed. affects how many ceremony cycles a community can be idle before getting purged"]
                    InactivityTimeoutUpdated(::core::primitive::u32),
                    #[codec(index = 5)]
                    #[doc = "The number of endorsement tickets which bootstrappers can give out has changed"]
                    EndorsementTicketsPerBootstrapperUpdated(::core::primitive::u8),
                    #[codec(index = 6)]
                    #[doc = "The number of endorsement tickets which bootstrappers can give out has changed"]
                    EndorsementTicketsPerReputableUpdated(::core::primitive::u8),
                    #[codec(index = 7)]
                    #[doc = "reputation lifetime has changed. After this many ceremony cycles, reputations is outdated"]
                    ReputationLifetimeUpdated(::core::primitive::u32),
                    #[codec(index = 8)]
                    #[doc = "meetup time offset has changed. affects the exact time the upcoming ceremony meetups will take place"]
                    MeetupTimeOffsetUpdated(::core::primitive::i32),
                    #[codec(index = 9)]
                    #[doc = "meetup time tolerance has changed"]
                    TimeToleranceUpdated(::core::primitive::u64),
                    #[codec(index = 10)]
                    #[doc = "meetup location tolerance changed [m]"]
                    LocationToleranceUpdated(::core::primitive::u32),
                    #[codec(index = 11)]
                    #[doc = "the registry for given ceremony index and community has been purged"]
                    CommunityCeremonyHistoryPurged(
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        ::core::primitive::u32,
                    ),
                    #[codec(index = 12)]
                    NoReward {
                        cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        cindex: ::core::primitive::u32,
                        meetup_index: ::core::primitive::u64,
                        account: ::subxt::ext::sp_core::crypto::AccountId32,
                        reason: runtime_types::encointer_meetup_validation::ExclusionReason,
                    },
                    #[codec(index = 13)]
                    #[doc = "The inactivity counter of a community has been increased"]
                    InactivityCounterUpdated(
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        ::core::primitive::u32,
                    ),
                    #[codec(index = 14)]
                    #[doc = "Result of the meetup at the previous ceremony"]
                    MeetupEvaluated(
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        ::core::primitive::u64,
                        runtime_types::encointer_primitives::ceremonies::MeetupResult,
                    ),
                }
            }
        }
        pub mod pallet_encointer_communities {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Add a new community."]
                    #[doc = ""]
                    #[doc = "May only be called from `T::TrustableForNonDestructiveAction`."]
                    new_community {
                        location: runtime_types::encointer_primitives::communities::Location,
                        bootstrappers: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
                        community_metadata:
                            runtime_types::encointer_primitives::communities::CommunityMetadata,
                        demurrage: ::core::option::Option<
                            runtime_types::substrate_fixed::FixedI128<
                                runtime_types::typenum::uint::UInt<
                                    runtime_types::typenum::uint::UInt<
                                        runtime_types::typenum::uint::UInt<
                                            runtime_types::typenum::uint::UInt<
                                                runtime_types::typenum::uint::UInt<
                                                    runtime_types::typenum::uint::UInt<
                                                        runtime_types::typenum::uint::UInt<
                                                            runtime_types::typenum::uint::UTerm,
                                                            runtime_types::typenum::bit::B1,
                                                        >,
                                                        runtime_types::typenum::bit::B0,
                                                    >,
                                                    runtime_types::typenum::bit::B0,
                                                >,
                                                runtime_types::typenum::bit::B0,
                                            >,
                                            runtime_types::typenum::bit::B0,
                                        >,
                                        runtime_types::typenum::bit::B0,
                                    >,
                                    runtime_types::typenum::bit::B0,
                                >,
                            >,
                        >,
                        nominal_income: ::core::option::Option<
                            runtime_types::substrate_fixed::FixedI128<
                                runtime_types::typenum::uint::UInt<
                                    runtime_types::typenum::uint::UInt<
                                        runtime_types::typenum::uint::UInt<
                                            runtime_types::typenum::uint::UInt<
                                                runtime_types::typenum::uint::UInt<
                                                    runtime_types::typenum::uint::UInt<
                                                        runtime_types::typenum::uint::UInt<
                                                            runtime_types::typenum::uint::UTerm,
                                                            runtime_types::typenum::bit::B1,
                                                        >,
                                                        runtime_types::typenum::bit::B0,
                                                    >,
                                                    runtime_types::typenum::bit::B0,
                                                >,
                                                runtime_types::typenum::bit::B0,
                                            >,
                                            runtime_types::typenum::bit::B0,
                                        >,
                                        runtime_types::typenum::bit::B0,
                                    >,
                                    runtime_types::typenum::bit::B0,
                                >,
                            >,
                        >,
                    },
                    #[codec(index = 1)]
                    #[doc = "Add a new meetup `location` to the community with `cid`."]
                    #[doc = ""]
                    #[doc = "May only be called from `T::TrustableForNonDestructiveAction`."]
                    #[doc = ""]
                    #[doc = "Todo: Replace `T::CommunityMaster` with community governance: #137."]
                    add_location {
                        cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        location: runtime_types::encointer_primitives::communities::Location,
                    },
                    #[codec(index = 2)]
                    #[doc = "Remove an existing meetup `location` from the community with `cid`."]
                    #[doc = ""]
                    #[doc = "May only be called from `T::CommunityMaster`."]
                    #[doc = ""]
                    #[doc = "Todo: Replace `T::CommunityMaster` with community governance: #137."]
                    remove_location {
                        cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        location: runtime_types::encointer_primitives::communities::Location,
                    },
                    #[codec(index = 3)]
                    #[doc = "Update the metadata of the community with `cid`."]
                    #[doc = ""]
                    #[doc = "May only be called from `T::CommunityMaster`."]
                    update_community_metadata {
                        cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        community_metadata:
                            runtime_types::encointer_primitives::communities::CommunityMetadata,
                    },
                    #[codec(index = 4)]
                    update_demurrage {
                        cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        demurrage: runtime_types::substrate_fixed::FixedI128<
                            runtime_types::typenum::uint::UInt<
                                runtime_types::typenum::uint::UInt<
                                    runtime_types::typenum::uint::UInt<
                                        runtime_types::typenum::uint::UInt<
                                            runtime_types::typenum::uint::UInt<
                                                runtime_types::typenum::uint::UInt<
                                                    runtime_types::typenum::uint::UInt<
                                                        runtime_types::typenum::uint::UTerm,
                                                        runtime_types::typenum::bit::B1,
                                                    >,
                                                    runtime_types::typenum::bit::B0,
                                                >,
                                                runtime_types::typenum::bit::B0,
                                            >,
                                            runtime_types::typenum::bit::B0,
                                        >,
                                        runtime_types::typenum::bit::B0,
                                    >,
                                    runtime_types::typenum::bit::B0,
                                >,
                                runtime_types::typenum::bit::B0,
                            >,
                        >,
                    },
                    #[codec(index = 5)]
                    update_nominal_income {
                        cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        nominal_income: runtime_types::substrate_fixed::FixedI128<
                            runtime_types::typenum::uint::UInt<
                                runtime_types::typenum::uint::UInt<
                                    runtime_types::typenum::uint::UInt<
                                        runtime_types::typenum::uint::UInt<
                                            runtime_types::typenum::uint::UInt<
                                                runtime_types::typenum::uint::UInt<
                                                    runtime_types::typenum::uint::UInt<
                                                        runtime_types::typenum::uint::UTerm,
                                                        runtime_types::typenum::bit::B1,
                                                    >,
                                                    runtime_types::typenum::bit::B0,
                                                >,
                                                runtime_types::typenum::bit::B0,
                                            >,
                                            runtime_types::typenum::bit::B0,
                                        >,
                                        runtime_types::typenum::bit::B0,
                                    >,
                                    runtime_types::typenum::bit::B0,
                                >,
                                runtime_types::typenum::bit::B0,
                            >,
                        >,
                    },
                    #[codec(index = 6)]
                    set_min_solar_trip_time_s {
                        min_solar_trip_time_s: ::core::primitive::u32,
                    },
                    #[codec(index = 7)]
                    set_max_speed_mps {
                        max_speed_mps: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
                    purge_community {
                        cid: runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Location is not a valid geolocation"]
                    InvalidLocation,
                    #[codec(index = 1)]
                    #[doc = "Invalid amount of bootstrappers supplied. Needs to be \\[3, 12\\]"]
                    InvalidAmountBootstrappers,
                    #[codec(index = 2)]
                    #[doc = "minimum distance violation to other location"]
                    MinimumDistanceViolationToOtherLocation,
                    #[codec(index = 3)]
                    #[doc = "minimum distance violated towards dateline"]
                    MinimumDistanceViolationToDateLine,
                    #[codec(index = 4)]
                    #[doc = "Can't register community that already exists"]
                    CommunityAlreadyRegistered,
                    #[codec(index = 5)]
                    #[doc = "Community does not exist yet"]
                    CommunityInexistent,
                    #[codec(index = 6)]
                    #[doc = "Invalid Metadata supplied"]
                    InvalidCommunityMetadata,
                    #[codec(index = 7)]
                    #[doc = "Invalid demurrage supplied"]
                    InvalidDemurrage,
                    #[codec(index = 8)]
                    #[doc = "Invalid demurrage supplied"]
                    InvalidNominalIncome,
                    #[codec(index = 9)]
                    #[doc = "Invalid location provided when computing geohash"]
                    InvalidLocationForGeohash,
                    #[codec(index = 10)]
                    #[doc = "Invalid Geohash provided"]
                    InvalidGeohash,
                    #[codec(index = 11)]
                    #[doc = "sender is not authorized"]
                    BadOrigin,
                    #[codec(index = 12)]
                    #[doc = "Locations can only be added in Registration Phase"]
                    RegistrationPhaseRequired,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A new community was registered [community_identifier]"]
                    CommunityRegistered(
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    ),
                    #[codec(index = 1)]
                    #[doc = "CommunityMetadata was updated [community_identifier]"]
                    MetadataUpdated(
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    ),
                    #[codec(index = 2)]
                    #[doc = "A community's nominal income was updated [community_identifier, new_income]"]
                    NominalIncomeUpdated(
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        runtime_types::substrate_fixed::FixedI128<
                            runtime_types::typenum::uint::UInt<
                                runtime_types::typenum::uint::UInt<
                                    runtime_types::typenum::uint::UInt<
                                        runtime_types::typenum::uint::UInt<
                                            runtime_types::typenum::uint::UInt<
                                                runtime_types::typenum::uint::UInt<
                                                    runtime_types::typenum::uint::UInt<
                                                        runtime_types::typenum::uint::UTerm,
                                                        runtime_types::typenum::bit::B1,
                                                    >,
                                                    runtime_types::typenum::bit::B0,
                                                >,
                                                runtime_types::typenum::bit::B0,
                                            >,
                                            runtime_types::typenum::bit::B0,
                                        >,
                                        runtime_types::typenum::bit::B0,
                                    >,
                                    runtime_types::typenum::bit::B0,
                                >,
                                runtime_types::typenum::bit::B0,
                            >,
                        >,
                    ),
                    #[codec(index = 3)]
                    #[doc = "A community's demurrage was updated [community_identifier, new_demurrage]"]
                    DemurrageUpdated(
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        runtime_types::substrate_fixed::FixedI128<
                            runtime_types::typenum::uint::UInt<
                                runtime_types::typenum::uint::UInt<
                                    runtime_types::typenum::uint::UInt<
                                        runtime_types::typenum::uint::UInt<
                                            runtime_types::typenum::uint::UInt<
                                                runtime_types::typenum::uint::UInt<
                                                    runtime_types::typenum::uint::UInt<
                                                        runtime_types::typenum::uint::UTerm,
                                                        runtime_types::typenum::bit::B1,
                                                    >,
                                                    runtime_types::typenum::bit::B0,
                                                >,
                                                runtime_types::typenum::bit::B0,
                                            >,
                                            runtime_types::typenum::bit::B0,
                                        >,
                                        runtime_types::typenum::bit::B0,
                                    >,
                                    runtime_types::typenum::bit::B0,
                                >,
                                runtime_types::typenum::bit::B0,
                            >,
                        >,
                    ),
                    #[codec(index = 4)]
                    #[doc = "A location has been added"]
                    LocationAdded(
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        runtime_types::encointer_primitives::communities::Location,
                    ),
                    #[codec(index = 5)]
                    #[doc = "A location has been removed"]
                    LocationRemoved(
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                        runtime_types::encointer_primitives::communities::Location,
                    ),
                    #[codec(index = 6)]
                    #[doc = "A security parameter for minimum meetup location distance has changed"]
                    MinSolarTripTimeSUpdated(::core::primitive::u32),
                    #[codec(index = 7)]
                    #[doc = "A security parameter for minimum meetup location distance has changed"]
                    MaxSpeedMpsUpdated(::core::primitive::u32),
                    #[codec(index = 8)]
                    #[doc = "a community has been purged"]
                    CommunityPurged(
                        runtime_types::encointer_primitives::communities::CommunityIdentifier,
                    ),
                }
            }
        }
        pub mod pallet_encointer_scheduler {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Manually transition to next phase without affecting the ceremony rhythm"]
                    #[doc = ""]
                    #[doc = "May only be called from `T::CeremonyMaster`."]
                    next_phase,
                    #[codec(index = 1)]
                    #[doc = "Push next phase change by one entire day"]
                    #[doc = ""]
                    #[doc = "May only be called from `T::CeremonyMaster`."]
                    push_by_one_day,
                    #[codec(index = 2)]
                    set_phase_duration {
                        ceremony_phase:
                            runtime_types::encointer_primitives::scheduler::CeremonyPhaseType,
                        duration: ::core::primitive::u64,
                    },
                    #[codec(index = 3)]
                    set_next_phase_timestamp { timestamp: ::core::primitive::u64 },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "a division by zero occurred"]
                    DivisionByZero,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Phase changed to `[new phase]`"]
                    PhaseChangedTo(
                        runtime_types::encointer_primitives::scheduler::CeremonyPhaseType,
                    ),
                    #[codec(index = 1)]
                    CeremonySchedulePushedByOneDay,
                }
            }
        }
        pub mod pallet_membership {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Add a member `who` to the set."]
                    #[doc = ""]
                    #[doc = "May only be called from `T::AddOrigin`."]
                    add_member {
                        who: ::subxt::ext::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 1)]
                    #[doc = "Remove a member `who` from the set."]
                    #[doc = ""]
                    #[doc = "May only be called from `T::RemoveOrigin`."]
                    remove_member {
                        who: ::subxt::ext::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 2)]
                    #[doc = "Swap out one member `remove` for another `add`."]
                    #[doc = ""]
                    #[doc = "May only be called from `T::SwapOrigin`."]
                    #[doc = ""]
                    #[doc = "Prime membership is *not* passed from `remove` to `add`, if extant."]
                    swap_member {
                        remove: ::subxt::ext::sp_core::crypto::AccountId32,
                        add: ::subxt::ext::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 3)]
                    #[doc = "Change the membership to a new set, disregarding the existing membership. Be nice and"]
                    #[doc = "pass `members` pre-sorted."]
                    #[doc = ""]
                    #[doc = "May only be called from `T::ResetOrigin`."]
                    reset_members {
                        members: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
                    },
                    #[codec(index = 4)]
                    #[doc = "Swap out the sending member for some other key `new`."]
                    #[doc = ""]
                    #[doc = "May only be called from `Signed` origin of a current member."]
                    #[doc = ""]
                    #[doc = "Prime membership is passed from the origin account to `new`, if extant."]
                    change_key {
                        new: ::subxt::ext::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 5)]
                    #[doc = "Set the prime member. Must be a current member."]
                    #[doc = ""]
                    #[doc = "May only be called from `T::PrimeOrigin`."]
                    set_prime {
                        who: ::subxt::ext::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 6)]
                    #[doc = "Remove the prime member if it exists."]
                    #[doc = ""]
                    #[doc = "May only be called from `T::PrimeOrigin`."]
                    clear_prime,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Already a member."]
                    AlreadyMember,
                    #[codec(index = 1)]
                    #[doc = "Not a member."]
                    NotMember,
                    #[codec(index = 2)]
                    #[doc = "Too many members."]
                    TooManyMembers,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "The given member was added; see the transaction for who."]
                    MemberAdded,
                    #[codec(index = 1)]
                    #[doc = "The given member was removed; see the transaction for who."]
                    MemberRemoved,
                    #[codec(index = 2)]
                    #[doc = "Two members were swapped; see the transaction for who."]
                    MembersSwapped,
                    #[codec(index = 3)]
                    #[doc = "The membership was reset; see the transaction for who the new set is."]
                    MembersReset,
                    #[codec(index = 4)]
                    #[doc = "One of the members' keys changed."]
                    KeyChanged,
                    #[codec(index = 5)]
                    #[doc = "Phantom member, never used."]
                    Dummy,
                }
            }
        }
        pub mod pallet_proxy {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Dispatch the given `call` from an account that the sender is authorised for through"]
                    #[doc = "`add_proxy`."]
                    #[doc = ""]
                    #[doc = "Removes any corresponding announcement(s)."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `real`: The account that the proxy will make a call on behalf of."]
                    #[doc = "- `force_proxy_type`: Specify the exact proxy type to be used and checked for this call."]
                    #[doc = "- `call`: The call to be made by the `real` account."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "Weight is a function of the number of proxies the user has (P)."]
                    #[doc = "# </weight>"]
                    proxy {
                        real: ::subxt::ext::sp_core::crypto::AccountId32,
                        force_proxy_type:
                            ::core::option::Option<runtime_types::encointer_runtime::ProxyType>,
                        call: ::std::boxed::Box<runtime_types::encointer_runtime::Call>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Register a proxy account for the sender that is able to make calls on its behalf."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `proxy`: The account that the `caller` would like to make a proxy."]
                    #[doc = "- `proxy_type`: The permissions allowed for this proxy account."]
                    #[doc = "- `delay`: The announcement period required of the initial proxy. Will generally be"]
                    #[doc = "zero."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "Weight is a function of the number of proxies the user has (P)."]
                    #[doc = "# </weight>"]
                    add_proxy {
                        delegate: ::subxt::ext::sp_core::crypto::AccountId32,
                        proxy_type: runtime_types::encointer_runtime::ProxyType,
                        delay: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    #[doc = "Unregister a proxy account for the sender."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `proxy`: The account that the `caller` would like to remove as a proxy."]
                    #[doc = "- `proxy_type`: The permissions currently enabled for the removed proxy account."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "Weight is a function of the number of proxies the user has (P)."]
                    #[doc = "# </weight>"]
                    remove_proxy {
                        delegate: ::subxt::ext::sp_core::crypto::AccountId32,
                        proxy_type: runtime_types::encointer_runtime::ProxyType,
                        delay: ::core::primitive::u32,
                    },
                    #[codec(index = 3)]
                    #[doc = "Unregister all proxy accounts for the sender."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "WARNING: This may be called on accounts created by `anonymous`, however if done, then"]
                    #[doc = "the unreserved fees will be inaccessible. **All access to this account will be lost.**"]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "Weight is a function of the number of proxies the user has (P)."]
                    #[doc = "# </weight>"]
                    remove_proxies,
                    #[codec(index = 4)]
                    #[doc = "Spawn a fresh new account that is guaranteed to be otherwise inaccessible, and"]
                    #[doc = "initialize it with a proxy of `proxy_type` for `origin` sender."]
                    #[doc = ""]
                    #[doc = "Requires a `Signed` origin."]
                    #[doc = ""]
                    #[doc = "- `proxy_type`: The type of the proxy that the sender will be registered as over the"]
                    #[doc = "new account. This will almost always be the most permissive `ProxyType` possible to"]
                    #[doc = "allow for maximum flexibility."]
                    #[doc = "- `index`: A disambiguation index, in case this is called multiple times in the same"]
                    #[doc = "transaction (e.g. with `utility::batch`). Unless you're using `batch` you probably just"]
                    #[doc = "want to use `0`."]
                    #[doc = "- `delay`: The announcement period required of the initial proxy. Will generally be"]
                    #[doc = "zero."]
                    #[doc = ""]
                    #[doc = "Fails with `Duplicate` if this has already been called in this transaction, from the"]
                    #[doc = "same sender, with the same parameters."]
                    #[doc = ""]
                    #[doc = "Fails if there are insufficient funds to pay for deposit."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "Weight is a function of the number of proxies the user has (P)."]
                    #[doc = "# </weight>"]
                    #[doc = "TODO: Might be over counting 1 read"]
                    anonymous {
                        proxy_type: runtime_types::encointer_runtime::ProxyType,
                        delay: ::core::primitive::u32,
                        index: ::core::primitive::u16,
                    },
                    #[codec(index = 5)]
                    #[doc = "Removes a previously spawned anonymous proxy."]
                    #[doc = ""]
                    #[doc = "WARNING: **All access to this account will be lost.** Any funds held in it will be"]
                    #[doc = "inaccessible."]
                    #[doc = ""]
                    #[doc = "Requires a `Signed` origin, and the sender account must have been created by a call to"]
                    #[doc = "`anonymous` with corresponding parameters."]
                    #[doc = ""]
                    #[doc = "- `spawner`: The account that originally called `anonymous` to create this account."]
                    #[doc = "- `index`: The disambiguation index originally passed to `anonymous`. Probably `0`."]
                    #[doc = "- `proxy_type`: The proxy type originally passed to `anonymous`."]
                    #[doc = "- `height`: The height of the chain when the call to `anonymous` was processed."]
                    #[doc = "- `ext_index`: The extrinsic index in which the call to `anonymous` was processed."]
                    #[doc = ""]
                    #[doc = "Fails with `NoPermission` in case the caller is not a previously created anonymous"]
                    #[doc = "account whose `anonymous` call has corresponding parameters."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "Weight is a function of the number of proxies the user has (P)."]
                    #[doc = "# </weight>"]
                    kill_anonymous {
                        spawner: ::subxt::ext::sp_core::crypto::AccountId32,
                        proxy_type: runtime_types::encointer_runtime::ProxyType,
                        index: ::core::primitive::u16,
                        #[codec(compact)]
                        height: ::core::primitive::u32,
                        #[codec(compact)]
                        ext_index: ::core::primitive::u32,
                    },
                    #[codec(index = 6)]
                    #[doc = "Publish the hash of a proxy-call that will be made in the future."]
                    #[doc = ""]
                    #[doc = "This must be called some number of blocks before the corresponding `proxy` is attempted"]
                    #[doc = "if the delay associated with the proxy relationship is greater than zero."]
                    #[doc = ""]
                    #[doc = "No more than `MaxPending` announcements may be made at any one time."]
                    #[doc = ""]
                    #[doc = "This will take a deposit of `AnnouncementDepositFactor` as well as"]
                    #[doc = "`AnnouncementDepositBase` if there are no other pending announcements."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and a proxy of `real`."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `real`: The account that the proxy will make a call on behalf of."]
                    #[doc = "- `call_hash`: The hash of the call to be made by the `real` account."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "Weight is a function of:"]
                    #[doc = "- A: the number of announcements made."]
                    #[doc = "- P: the number of proxies the user has."]
                    #[doc = "# </weight>"]
                    announce {
                        real: ::subxt::ext::sp_core::crypto::AccountId32,
                        call_hash: ::subxt::ext::sp_core::H256,
                    },
                    #[codec(index = 7)]
                    #[doc = "Remove a given announcement."]
                    #[doc = ""]
                    #[doc = "May be called by a proxy account to remove a call they previously announced and return"]
                    #[doc = "the deposit."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `real`: The account that the proxy will make a call on behalf of."]
                    #[doc = "- `call_hash`: The hash of the call to be made by the `real` account."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "Weight is a function of:"]
                    #[doc = "- A: the number of announcements made."]
                    #[doc = "- P: the number of proxies the user has."]
                    #[doc = "# </weight>"]
                    remove_announcement {
                        real: ::subxt::ext::sp_core::crypto::AccountId32,
                        call_hash: ::subxt::ext::sp_core::H256,
                    },
                    #[codec(index = 8)]
                    #[doc = "Remove the given announcement of a delegate."]
                    #[doc = ""]
                    #[doc = "May be called by a target (proxied) account to remove a call that one of their delegates"]
                    #[doc = "(`delegate`) has announced they want to execute. The deposit is returned."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `delegate`: The account that previously announced the call."]
                    #[doc = "- `call_hash`: The hash of the call to be made."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "Weight is a function of:"]
                    #[doc = "- A: the number of announcements made."]
                    #[doc = "- P: the number of proxies the user has."]
                    #[doc = "# </weight>"]
                    reject_announcement {
                        delegate: ::subxt::ext::sp_core::crypto::AccountId32,
                        call_hash: ::subxt::ext::sp_core::H256,
                    },
                    #[codec(index = 9)]
                    #[doc = "Dispatch the given `call` from an account that the sender is authorized for through"]
                    #[doc = "`add_proxy`."]
                    #[doc = ""]
                    #[doc = "Removes any corresponding announcement(s)."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `real`: The account that the proxy will make a call on behalf of."]
                    #[doc = "- `force_proxy_type`: Specify the exact proxy type to be used and checked for this call."]
                    #[doc = "- `call`: The call to be made by the `real` account."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "Weight is a function of:"]
                    #[doc = "- A: the number of announcements made."]
                    #[doc = "- P: the number of proxies the user has."]
                    #[doc = "# </weight>"]
                    proxy_announced {
                        delegate: ::subxt::ext::sp_core::crypto::AccountId32,
                        real: ::subxt::ext::sp_core::crypto::AccountId32,
                        force_proxy_type:
                            ::core::option::Option<runtime_types::encointer_runtime::ProxyType>,
                        call: ::std::boxed::Box<runtime_types::encointer_runtime::Call>,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "There are too many proxies registered or too many announcements pending."]
                    TooMany,
                    #[codec(index = 1)]
                    #[doc = "Proxy registration not found."]
                    NotFound,
                    #[codec(index = 2)]
                    #[doc = "Sender is not a proxy of the account to be proxied."]
                    NotProxy,
                    #[codec(index = 3)]
                    #[doc = "A call which is incompatible with the proxy type's filter was attempted."]
                    Unproxyable,
                    #[codec(index = 4)]
                    #[doc = "Account is already a proxy."]
                    Duplicate,
                    #[codec(index = 5)]
                    #[doc = "Call may not be made by proxy because it may escalate its privileges."]
                    NoPermission,
                    #[codec(index = 6)]
                    #[doc = "Announcement, if made at all, was made too recently."]
                    Unannounced,
                    #[codec(index = 7)]
                    #[doc = "Cannot add self as proxy."]
                    NoSelfProxy,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A proxy was executed correctly, with the given."]
                    ProxyExecuted {
                        result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Anonymous account has been created by new proxy with given"]
                    #[doc = "disambiguation index and proxy type."]
                    AnonymousCreated {
                        anonymous: ::subxt::ext::sp_core::crypto::AccountId32,
                        who: ::subxt::ext::sp_core::crypto::AccountId32,
                        proxy_type: runtime_types::encointer_runtime::ProxyType,
                        disambiguation_index: ::core::primitive::u16,
                    },
                    #[codec(index = 2)]
                    #[doc = "An announcement was placed to make a call in the future."]
                    Announced {
                        real: ::subxt::ext::sp_core::crypto::AccountId32,
                        proxy: ::subxt::ext::sp_core::crypto::AccountId32,
                        call_hash: ::subxt::ext::sp_core::H256,
                    },
                    #[codec(index = 3)]
                    #[doc = "A proxy was added."]
                    ProxyAdded {
                        delegator: ::subxt::ext::sp_core::crypto::AccountId32,
                        delegatee: ::subxt::ext::sp_core::crypto::AccountId32,
                        proxy_type: runtime_types::encointer_runtime::ProxyType,
                        delay: ::core::primitive::u32,
                    },
                    #[codec(index = 4)]
                    #[doc = "A proxy was removed."]
                    ProxyRemoved {
                        delegator: ::subxt::ext::sp_core::crypto::AccountId32,
                        delegatee: ::subxt::ext::sp_core::crypto::AccountId32,
                        proxy_type: runtime_types::encointer_runtime::ProxyType,
                        delay: ::core::primitive::u32,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Announcement<_0, _1, _2> {
                pub real: _0,
                pub call_hash: _1,
                pub height: _2,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ProxyDefinition<_0, _1, _2> {
                pub delegate: _0,
                pub proxy_type: _1,
                pub delay: _2,
            }
        }
        pub mod pallet_scheduler {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Anonymously schedule a task."]
                    schedule {
                        when: ::core::primitive::u32,
                        maybe_periodic: ::core::option::Option<(
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        )>,
                        priority: ::core::primitive::u8,
                        call: ::std::boxed::Box<
                            runtime_types::frame_support::traits::schedule::MaybeHashed<
                                runtime_types::encointer_runtime::Call,
                                ::subxt::ext::sp_core::H256,
                            >,
                        >,
                    },
                    #[codec(index = 1)]
                    #[doc = "Cancel an anonymously scheduled task."]
                    cancel {
                        when: ::core::primitive::u32,
                        index: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    #[doc = "Schedule a named task."]
                    schedule_named {
                        id: ::std::vec::Vec<::core::primitive::u8>,
                        when: ::core::primitive::u32,
                        maybe_periodic: ::core::option::Option<(
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        )>,
                        priority: ::core::primitive::u8,
                        call: ::std::boxed::Box<
                            runtime_types::frame_support::traits::schedule::MaybeHashed<
                                runtime_types::encointer_runtime::Call,
                                ::subxt::ext::sp_core::H256,
                            >,
                        >,
                    },
                    #[codec(index = 3)]
                    #[doc = "Cancel a named scheduled task."]
                    cancel_named {
                        id: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    #[doc = "Anonymously schedule a task after a delay."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "Same as [`schedule`]."]
                    #[doc = "# </weight>"]
                    schedule_after {
                        after: ::core::primitive::u32,
                        maybe_periodic: ::core::option::Option<(
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        )>,
                        priority: ::core::primitive::u8,
                        call: ::std::boxed::Box<
                            runtime_types::frame_support::traits::schedule::MaybeHashed<
                                runtime_types::encointer_runtime::Call,
                                ::subxt::ext::sp_core::H256,
                            >,
                        >,
                    },
                    #[codec(index = 5)]
                    #[doc = "Schedule a named task after a delay."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "Same as [`schedule_named`](Self::schedule_named)."]
                    #[doc = "# </weight>"]
                    schedule_named_after {
                        id: ::std::vec::Vec<::core::primitive::u8>,
                        after: ::core::primitive::u32,
                        maybe_periodic: ::core::option::Option<(
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        )>,
                        priority: ::core::primitive::u8,
                        call: ::std::boxed::Box<
                            runtime_types::frame_support::traits::schedule::MaybeHashed<
                                runtime_types::encointer_runtime::Call,
                                ::subxt::ext::sp_core::H256,
                            >,
                        >,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Failed to schedule a call"]
                    FailedToSchedule,
                    #[codec(index = 1)]
                    #[doc = "Cannot find the scheduled call."]
                    NotFound,
                    #[codec(index = 2)]
                    #[doc = "Given target block number is in the past."]
                    TargetBlockNumberInPast,
                    #[codec(index = 3)]
                    #[doc = "Reschedule failed because it does not change scheduled time."]
                    RescheduleNoChange,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Events type."]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Scheduled some task."]
                    Scheduled {
                        when: ::core::primitive::u32,
                        index: ::core::primitive::u32,
                    },
                    #[codec(index = 1)]
                    #[doc = "Canceled some task."]
                    Canceled {
                        when: ::core::primitive::u32,
                        index: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    #[doc = "Dispatched some task."]
                    Dispatched {
                        task: (::core::primitive::u32, ::core::primitive::u32),
                        id: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                        result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 3)]
                    #[doc = "The call for the provided hash was not found so the task has been aborted."]
                    CallLookupFailed {
                        task: (::core::primitive::u32, ::core::primitive::u32),
                        id: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                        error: runtime_types::frame_support::traits::schedule::LookupError,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ScheduledV3<_0, _1, _2, _3> {
                pub maybe_id: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                pub priority: ::core::primitive::u8,
                pub call: _0,
                pub maybe_periodic: ::core::option::Option<(_1, _1)>,
                pub origin: _2,
                #[codec(skip)]
                pub __subxt_unused_type_params: ::core::marker::PhantomData<_3>,
            }
        }
        pub mod pallet_timestamp {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Set the current time."]
                    #[doc = ""]
                    #[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
                    #[doc = "phase, if this call hasn't been invoked by that time."]
                    #[doc = ""]
                    #[doc = "The timestamp should be greater than the previous one by the amount specified by"]
                    #[doc = "`MinimumPeriod`."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be `Inherent`."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
                    #[doc = "- 1 storage read and 1 storage mutation (codec `O(1)`). (because of `DidUpdate::take` in"]
                    #[doc = "  `on_finalize`)"]
                    #[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
                    #[doc = "# </weight>"]
                    set {
                        #[codec(compact)]
                        now: ::core::primitive::u64,
                    },
                }
            }
        }
        pub mod pallet_transaction_payment {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,"]
                    #[doc = "has been paid by `who`."]
                    TransactionFeePaid {
                        who: ::subxt::ext::sp_core::crypto::AccountId32,
                        actual_fee: ::core::primitive::u128,
                        tip: ::core::primitive::u128,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum Releases {
                #[codec(index = 0)]
                V1Ancient,
                #[codec(index = 1)]
                V2,
            }
        }
        pub mod pallet_treasury {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Put forward a suggestion for spending. A deposit proportional to the value"]
                    #[doc = "is reserved and slashed if the proposal is rejected. It is returned once the"]
                    #[doc = "proposal is awarded."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- Complexity: O(1)"]
                    #[doc = "- DbReads: `ProposalCount`, `origin account`"]
                    #[doc = "- DbWrites: `ProposalCount`, `Proposals`, `origin account`"]
                    #[doc = "# </weight>"]
                    propose_spend {
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        beneficiary: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 1)]
                    #[doc = "Reject a proposed spend. The original deposit will be slashed."]
                    #[doc = ""]
                    #[doc = "May only be called from `T::RejectOrigin`."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- Complexity: O(1)"]
                    #[doc = "- DbReads: `Proposals`, `rejected proposer account`"]
                    #[doc = "- DbWrites: `Proposals`, `rejected proposer account`"]
                    #[doc = "# </weight>"]
                    reject_proposal {
                        #[codec(compact)]
                        proposal_id: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    #[doc = "Approve a proposal. At a later time, the proposal will be allocated to the beneficiary"]
                    #[doc = "and the original deposit will be returned."]
                    #[doc = ""]
                    #[doc = "May only be called from `T::ApproveOrigin`."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- Complexity: O(1)."]
                    #[doc = "- DbReads: `Proposals`, `Approvals`"]
                    #[doc = "- DbWrite: `Approvals`"]
                    #[doc = "# </weight>"]
                    approve_proposal {
                        #[codec(compact)]
                        proposal_id: ::core::primitive::u32,
                    },
                    #[codec(index = 3)]
                    #[doc = "Propose and approve a spend of treasury funds."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be `SpendOrigin` with the `Success` value being at least `amount`."]
                    #[doc = "- `amount`: The amount to be transferred from the treasury to the `beneficiary`."]
                    #[doc = "- `beneficiary`: The destination account for the transfer."]
                    #[doc = ""]
                    #[doc = "NOTE: For record-keeping purposes, the proposer is deemed to be equivalent to the"]
                    #[doc = "beneficiary."]
                    spend {
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                        beneficiary: ::subxt::ext::sp_runtime::MultiAddress<
                            ::subxt::ext::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 4)]
                    #[doc = "Force a previously approved proposal to be removed from the approval queue."]
                    #[doc = "The original deposit will no longer be returned."]
                    #[doc = ""]
                    #[doc = "May only be called from `T::RejectOrigin`."]
                    #[doc = "- `proposal_id`: The index of a proposal"]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- Complexity: O(A) where `A` is the number of approvals"]
                    #[doc = "- Db reads and writes: `Approvals`"]
                    #[doc = "# </weight>"]
                    #[doc = ""]
                    #[doc = "Errors:"]
                    #[doc = "- `ProposalNotApproved`: The `proposal_id` supplied was not found in the approval queue,"]
                    #[doc = "i.e., the proposal has not been approved. This could also mean the proposal does not"]
                    #[doc = "exist altogether, thus there is no way it would have been approved in the first place."]
                    remove_approval {
                        #[codec(compact)]
                        proposal_id: ::core::primitive::u32,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Error for the treasury pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Proposer's balance is too low."]
                    InsufficientProposersBalance,
                    #[codec(index = 1)]
                    #[doc = "No proposal or bounty at that index."]
                    InvalidIndex,
                    #[codec(index = 2)]
                    #[doc = "Too many approvals in the queue."]
                    TooManyApprovals,
                    #[codec(index = 3)]
                    #[doc = "The spend origin is valid but the amount it is allowed to spend is lower than the"]
                    #[doc = "amount to be spent."]
                    InsufficientPermission,
                    #[codec(index = 4)]
                    #[doc = "Proposal has not been approved."]
                    ProposalNotApproved,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "New proposal."]
                    Proposed {
                        proposal_index: ::core::primitive::u32,
                    },
                    #[codec(index = 1)]
                    #[doc = "We have ended a spend period and will now allocate funds."]
                    Spending {
                        budget_remaining: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "Some funds have been allocated."]
                    Awarded {
                        proposal_index: ::core::primitive::u32,
                        award: ::core::primitive::u128,
                        account: ::subxt::ext::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 3)]
                    #[doc = "A proposal was rejected; funds were slashed."]
                    Rejected {
                        proposal_index: ::core::primitive::u32,
                        slashed: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    #[doc = "Some of our funds have been burnt."]
                    Burnt {
                        burnt_funds: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    #[doc = "Spending has finished; this is the amount that rolls over until next spend."]
                    Rollover {
                        rollover_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    #[doc = "Some funds have been deposited."]
                    Deposit { value: ::core::primitive::u128 },
                    #[codec(index = 7)]
                    #[doc = "A new spend proposal has been approved."]
                    SpendApproved {
                        proposal_index: ::core::primitive::u32,
                        amount: ::core::primitive::u128,
                        beneficiary: ::subxt::ext::sp_core::crypto::AccountId32,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Proposal<_0, _1> {
                pub proposer: _0,
                pub value: _1,
                pub beneficiary: _0,
                pub bond: _1,
            }
        }
        pub mod pallet_utility {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Send a batch of dispatch calls."]
                    #[doc = ""]
                    #[doc = "May be called from any origin."]
                    #[doc = ""]
                    #[doc = "- `calls`: The calls to be dispatched from the same origin. The number of call must not"]
                    #[doc = "  exceed the constant: `batched_calls_limit` (available in constant metadata)."]
                    #[doc = ""]
                    #[doc = "If origin is root then call are dispatch without checking origin filter. (This includes"]
                    #[doc = "bypassing `frame_system::Config::BaseCallFilter`)."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- Complexity: O(C) where C is the number of calls to be batched."]
                    #[doc = "# </weight>"]
                    #[doc = ""]
                    #[doc = "This will return `Ok` in all circumstances. To determine the success of the batch, an"]
                    #[doc = "event is deposited. If a call failed and the batch was interrupted, then the"]
                    #[doc = "`BatchInterrupted` event is deposited, along with the number of successful calls made"]
                    #[doc = "and the error of the failed call. If all were successful, then the `BatchCompleted`"]
                    #[doc = "event is deposited."]
                    batch {
                        calls: ::std::vec::Vec<runtime_types::encointer_runtime::Call>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Send a call through an indexed pseudonym of the sender."]
                    #[doc = ""]
                    #[doc = "Filter from origin are passed along. The call will be dispatched with an origin which"]
                    #[doc = "use the same filter as the origin of this call."]
                    #[doc = ""]
                    #[doc = "NOTE: If you need to ensure that any account-based filtering is not honored (i.e."]
                    #[doc = "because you expect `proxy` to have been used prior in the call stack and you do not want"]
                    #[doc = "the call restrictions to apply to any sub-accounts), then use `as_multi_threshold_1`"]
                    #[doc = "in the Multisig pallet instead."]
                    #[doc = ""]
                    #[doc = "NOTE: Prior to version *12, this was called `as_limited_sub`."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    as_derivative {
                        index: ::core::primitive::u16,
                        call: ::std::boxed::Box<runtime_types::encointer_runtime::Call>,
                    },
                    #[codec(index = 2)]
                    #[doc = "Send a batch of dispatch calls and atomically execute them."]
                    #[doc = "The whole transaction will rollback and fail if any of the calls failed."]
                    #[doc = ""]
                    #[doc = "May be called from any origin."]
                    #[doc = ""]
                    #[doc = "- `calls`: The calls to be dispatched from the same origin. The number of call must not"]
                    #[doc = "  exceed the constant: `batched_calls_limit` (available in constant metadata)."]
                    #[doc = ""]
                    #[doc = "If origin is root then call are dispatch without checking origin filter. (This includes"]
                    #[doc = "bypassing `frame_system::Config::BaseCallFilter`)."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- Complexity: O(C) where C is the number of calls to be batched."]
                    #[doc = "# </weight>"]
                    batch_all {
                        calls: ::std::vec::Vec<runtime_types::encointer_runtime::Call>,
                    },
                    #[codec(index = 3)]
                    #[doc = "Dispatches a function call with a provided origin."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Root_."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- O(1)."]
                    #[doc = "- Limited storage reads."]
                    #[doc = "- One DB write (event)."]
                    #[doc = "- Weight of derivative `call` execution + T::WeightInfo::dispatch_as()."]
                    #[doc = "# </weight>"]
                    dispatch_as {
                        as_origin:
                            ::std::boxed::Box<runtime_types::encointer_runtime::OriginCaller>,
                        call: ::std::boxed::Box<runtime_types::encointer_runtime::Call>,
                    },
                    #[codec(index = 4)]
                    #[doc = "Send a batch of dispatch calls."]
                    #[doc = "Unlike `batch`, it allows errors and won't interrupt."]
                    #[doc = ""]
                    #[doc = "May be called from any origin."]
                    #[doc = ""]
                    #[doc = "- `calls`: The calls to be dispatched from the same origin. The number of call must not"]
                    #[doc = "  exceed the constant: `batched_calls_limit` (available in constant metadata)."]
                    #[doc = ""]
                    #[doc = "If origin is root then call are dispatch without checking origin filter. (This includes"]
                    #[doc = "bypassing `frame_system::Config::BaseCallFilter`)."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- Complexity: O(C) where C is the number of calls to be batched."]
                    #[doc = "# </weight>"]
                    force_batch {
                        calls: ::std::vec::Vec<runtime_types::encointer_runtime::Call>,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Too many calls batched."]
                    TooManyCalls,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Batch of dispatches did not complete fully. Index of first failing dispatch given, as"]
                    #[doc = "well as the error."]
                    BatchInterrupted {
                        index: ::core::primitive::u32,
                        error: runtime_types::sp_runtime::DispatchError,
                    },
                    #[codec(index = 1)]
                    #[doc = "Batch of dispatches completed fully with no error."]
                    BatchCompleted,
                    #[codec(index = 2)]
                    #[doc = "Batch of dispatches completed but has errors."]
                    BatchCompletedWithErrors,
                    #[codec(index = 3)]
                    #[doc = "A single item within a Batch of dispatches has completed with no error."]
                    ItemCompleted,
                    #[codec(index = 4)]
                    #[doc = "A single item within a Batch of dispatches has completed with error."]
                    ItemFailed {
                        error: runtime_types::sp_runtime::DispatchError,
                    },
                    #[codec(index = 5)]
                    #[doc = "A call was dispatched."]
                    DispatchedAs {
                        result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                }
            }
        }
        pub mod pallet_xcm {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    send {
                        dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                        message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Teleport some assets from the local chain to some destination chain."]
                    #[doc = ""]
                    #[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
                    #[doc = "index `fee_asset_item`. The weight limit for fees is not provided and thus is unlimited,"]
                    #[doc = "with all fees taken as needed from the asset."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
                    #[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
                    #[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
                    #[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
                    #[doc = "  an `AccountId32` value."]
                    #[doc = "- `assets`: The assets to be withdrawn. The first item should be the currency used to to pay the fee on the"]
                    #[doc = "  `dest` side. May not be empty."]
                    #[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
                    #[doc = "  fees."]
                    teleport_assets {
                        dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                        beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                        assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
                        fee_asset_item: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    #[doc = "Transfer some assets from the local chain to the sovereign account of a destination"]
                    #[doc = "chain and forward a notification XCM."]
                    #[doc = ""]
                    #[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
                    #[doc = "index `fee_asset_item`. The weight limit for fees is not provided and thus is unlimited,"]
                    #[doc = "with all fees taken as needed from the asset."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
                    #[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
                    #[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
                    #[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
                    #[doc = "  an `AccountId32` value."]
                    #[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the fee on the"]
                    #[doc = "  `dest` side."]
                    #[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
                    #[doc = "  fees."]
                    reserve_transfer_assets {
                        dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                        beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                        assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
                        fee_asset_item: ::core::primitive::u32,
                    },
                    #[codec(index = 3)]
                    #[doc = "Execute an XCM message from a local, signed, origin."]
                    #[doc = ""]
                    #[doc = "An event is deposited indicating whether `msg` could be executed completely or only"]
                    #[doc = "partially."]
                    #[doc = ""]
                    #[doc = "No more than `max_weight` will be used in its attempted execution. If this is less than the"]
                    #[doc = "maximum amount of weight that the message could take to be executed, then no execution"]
                    #[doc = "attempt will be made."]
                    #[doc = ""]
                    #[doc = "NOTE: A successful return to this does *not* imply that the `msg` was executed successfully"]
                    #[doc = "to completion; only that *some* of it was executed."]
                    execute {
                        message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
                        max_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 4)]
                    #[doc = "Extoll that a particular destination can be communicated with through a particular"]
                    #[doc = "version of XCM."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be Root."]
                    #[doc = "- `location`: The destination that is being described."]
                    #[doc = "- `xcm_version`: The latest version of XCM that `location` supports."]
                    force_xcm_version {
                        location:
                            ::std::boxed::Box<runtime_types::xcm::v1::multilocation::MultiLocation>,
                        xcm_version: ::core::primitive::u32,
                    },
                    #[codec(index = 5)]
                    #[doc = "Set a safe XCM version (the version that XCM should be encoded with if the most recent"]
                    #[doc = "version a destination can accept is unknown)."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be Root."]
                    #[doc = "- `maybe_xcm_version`: The default XCM encoding version, or `None` to disable."]
                    force_default_xcm_version {
                        maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
                    },
                    #[codec(index = 6)]
                    #[doc = "Ask a location to notify us regarding their XCM version and any changes to it."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be Root."]
                    #[doc = "- `location`: The location to which we should subscribe for XCM version notifications."]
                    force_subscribe_version_notify {
                        location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                    },
                    #[codec(index = 7)]
                    #[doc = "Require that a particular destination should no longer notify us regarding any XCM"]
                    #[doc = "version changes."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be Root."]
                    #[doc = "- `location`: The location to which we are currently subscribed for XCM version"]
                    #[doc = "  notifications which we no longer desire."]
                    force_unsubscribe_version_notify {
                        location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                    },
                    #[codec(index = 8)]
                    #[doc = "Transfer some assets from the local chain to the sovereign account of a destination"]
                    #[doc = "chain and forward a notification XCM."]
                    #[doc = ""]
                    #[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
                    #[doc = "index `fee_asset_item`, up to enough to pay for `weight_limit` of weight. If more weight"]
                    #[doc = "is needed than `weight_limit`, then the operation will fail and the assets send may be"]
                    #[doc = "at risk."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
                    #[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
                    #[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
                    #[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
                    #[doc = "  an `AccountId32` value."]
                    #[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the fee on the"]
                    #[doc = "  `dest` side."]
                    #[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
                    #[doc = "  fees."]
                    #[doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."]
                    limited_reserve_transfer_assets {
                        dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                        beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                        assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
                        fee_asset_item: ::core::primitive::u32,
                        weight_limit: runtime_types::xcm::v2::WeightLimit,
                    },
                    #[codec(index = 9)]
                    #[doc = "Teleport some assets from the local chain to some destination chain."]
                    #[doc = ""]
                    #[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
                    #[doc = "index `fee_asset_item`, up to enough to pay for `weight_limit` of weight. If more weight"]
                    #[doc = "is needed than `weight_limit`, then the operation will fail and the assets send may be"]
                    #[doc = "at risk."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
                    #[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
                    #[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
                    #[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
                    #[doc = "  an `AccountId32` value."]
                    #[doc = "- `assets`: The assets to be withdrawn. The first item should be the currency used to to pay the fee on the"]
                    #[doc = "  `dest` side. May not be empty."]
                    #[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
                    #[doc = "  fees."]
                    #[doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."]
                    limited_teleport_assets {
                        dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                        beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                        assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
                        fee_asset_item: ::core::primitive::u32,
                        weight_limit: runtime_types::xcm::v2::WeightLimit,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The desired destination was unreachable, generally because there is a no way of routing"]
                    #[doc = "to it."]
                    Unreachable,
                    #[codec(index = 1)]
                    #[doc = "There was some other issue (i.e. not to do with routing) in sending the message. Perhaps"]
                    #[doc = "a lack of space for buffering the message."]
                    SendFailure,
                    #[codec(index = 2)]
                    #[doc = "The message execution fails the filter."]
                    Filtered,
                    #[codec(index = 3)]
                    #[doc = "The message's weight could not be determined."]
                    UnweighableMessage,
                    #[codec(index = 4)]
                    #[doc = "The destination `MultiLocation` provided cannot be inverted."]
                    DestinationNotInvertible,
                    #[codec(index = 5)]
                    #[doc = "The assets to be sent are empty."]
                    Empty,
                    #[codec(index = 6)]
                    #[doc = "Could not re-anchor the assets to declare the fees for the destination chain."]
                    CannotReanchor,
                    #[codec(index = 7)]
                    #[doc = "Too many assets have been attempted for transfer."]
                    TooManyAssets,
                    #[codec(index = 8)]
                    #[doc = "Origin is invalid for sending."]
                    InvalidOrigin,
                    #[codec(index = 9)]
                    #[doc = "The version of the `Versioned` value used is not able to be interpreted."]
                    BadVersion,
                    #[codec(index = 10)]
                    #[doc = "The given location could not be used (e.g. because it cannot be expressed in the"]
                    #[doc = "desired version of XCM)."]
                    BadLocation,
                    #[codec(index = 11)]
                    #[doc = "The referenced subscription could not be found."]
                    NoSubscription,
                    #[codec(index = 12)]
                    #[doc = "The location is invalid since it already has a subscription from us."]
                    AlreadySubscribed,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Execution of an XCM message was attempted."]
                    #[doc = ""]
                    #[doc = "\\[ outcome \\]"]
                    Attempted(runtime_types::xcm::v2::traits::Outcome),
                    #[codec(index = 1)]
                    #[doc = "A XCM message was sent."]
                    #[doc = ""]
                    #[doc = "\\[ origin, destination, message \\]"]
                    Sent(
                        runtime_types::xcm::v1::multilocation::MultiLocation,
                        runtime_types::xcm::v1::multilocation::MultiLocation,
                        runtime_types::xcm::v2::Xcm,
                    ),
                    #[codec(index = 2)]
                    #[doc = "Query response received which does not match a registered query. This may be because a"]
                    #[doc = "matching query was never registered, it may be because it is a duplicate response, or"]
                    #[doc = "because the query timed out."]
                    #[doc = ""]
                    #[doc = "\\[ origin location, id \\]"]
                    UnexpectedResponse(
                        runtime_types::xcm::v1::multilocation::MultiLocation,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 3)]
                    #[doc = "Query response has been received and is ready for taking with `take_response`. There is"]
                    #[doc = "no registered notification call."]
                    #[doc = ""]
                    #[doc = "\\[ id, response \\]"]
                    ResponseReady(::core::primitive::u64, runtime_types::xcm::v2::Response),
                    #[codec(index = 4)]
                    #[doc = "Query response has been received and query is removed. The registered notification has"]
                    #[doc = "been dispatched and executed successfully."]
                    #[doc = ""]
                    #[doc = "\\[ id, pallet index, call index \\]"]
                    Notified(
                        ::core::primitive::u64,
                        ::core::primitive::u8,
                        ::core::primitive::u8,
                    ),
                    #[codec(index = 5)]
                    #[doc = "Query response has been received and query is removed. The registered notification could"]
                    #[doc = "not be dispatched because the dispatch weight is greater than the maximum weight"]
                    #[doc = "originally budgeted by this runtime for the query result."]
                    #[doc = ""]
                    #[doc = "\\[ id, pallet index, call index, actual weight, max budgeted weight \\]"]
                    NotifyOverweight(
                        ::core::primitive::u64,
                        ::core::primitive::u8,
                        ::core::primitive::u8,
                        ::core::primitive::u64,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 6)]
                    #[doc = "Query response has been received and query is removed. There was a general error with"]
                    #[doc = "dispatching the notification call."]
                    #[doc = ""]
                    #[doc = "\\[ id, pallet index, call index \\]"]
                    NotifyDispatchError(
                        ::core::primitive::u64,
                        ::core::primitive::u8,
                        ::core::primitive::u8,
                    ),
                    #[codec(index = 7)]
                    #[doc = "Query response has been received and query is removed. The dispatch was unable to be"]
                    #[doc = "decoded into a `Call`; this might be due to dispatch function having a signature which"]
                    #[doc = "is not `(origin, QueryId, Response)`."]
                    #[doc = ""]
                    #[doc = "\\[ id, pallet index, call index \\]"]
                    NotifyDecodeFailed(
                        ::core::primitive::u64,
                        ::core::primitive::u8,
                        ::core::primitive::u8,
                    ),
                    #[codec(index = 8)]
                    #[doc = "Expected query response has been received but the origin location of the response does"]
                    #[doc = "not match that expected. The query remains registered for a later, valid, response to"]
                    #[doc = "be received and acted upon."]
                    #[doc = ""]
                    #[doc = "\\[ origin location, id, expected location \\]"]
                    InvalidResponder(
                        runtime_types::xcm::v1::multilocation::MultiLocation,
                        ::core::primitive::u64,
                        ::core::option::Option<
                            runtime_types::xcm::v1::multilocation::MultiLocation,
                        >,
                    ),
                    #[codec(index = 9)]
                    #[doc = "Expected query response has been received but the expected origin location placed in"]
                    #[doc = "storage by this runtime previously cannot be decoded. The query remains registered."]
                    #[doc = ""]
                    #[doc = "This is unexpected (since a location placed in storage in a previously executing"]
                    #[doc = "runtime should be readable prior to query timeout) and dangerous since the possibly"]
                    #[doc = "valid response will be dropped. Manual governance intervention is probably going to be"]
                    #[doc = "needed."]
                    #[doc = ""]
                    #[doc = "\\[ origin location, id \\]"]
                    InvalidResponderVersion(
                        runtime_types::xcm::v1::multilocation::MultiLocation,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 10)]
                    #[doc = "Received query response has been read and removed."]
                    #[doc = ""]
                    #[doc = "\\[ id \\]"]
                    ResponseTaken(::core::primitive::u64),
                    #[codec(index = 11)]
                    #[doc = "Some assets have been placed in an asset trap."]
                    #[doc = ""]
                    #[doc = "\\[ hash, origin, assets \\]"]
                    AssetsTrapped(
                        ::subxt::ext::sp_core::H256,
                        runtime_types::xcm::v1::multilocation::MultiLocation,
                        runtime_types::xcm::VersionedMultiAssets,
                    ),
                    #[codec(index = 12)]
                    #[doc = "An XCM version change notification message has been attempted to be sent."]
                    #[doc = ""]
                    #[doc = "\\[ destination, result \\]"]
                    VersionChangeNotified(
                        runtime_types::xcm::v1::multilocation::MultiLocation,
                        ::core::primitive::u32,
                    ),
                    #[codec(index = 13)]
                    #[doc = "The supported version of a location has been changed. This might be through an"]
                    #[doc = "automatic notification or a manual intervention."]
                    #[doc = ""]
                    #[doc = "\\[ location, XCM version \\]"]
                    SupportedVersionChanged(
                        runtime_types::xcm::v1::multilocation::MultiLocation,
                        ::core::primitive::u32,
                    ),
                    #[codec(index = 14)]
                    #[doc = "A given location which had a version change subscription was dropped owing to an error"]
                    #[doc = "sending the notification to it."]
                    #[doc = ""]
                    #[doc = "\\[ location, query ID, error \\]"]
                    NotifyTargetSendFail(
                        runtime_types::xcm::v1::multilocation::MultiLocation,
                        ::core::primitive::u64,
                        runtime_types::xcm::v2::traits::Error,
                    ),
                    #[codec(index = 15)]
                    #[doc = "A given location which had a version change subscription was dropped owing to an error"]
                    #[doc = "migrating the location to our new XCM format."]
                    #[doc = ""]
                    #[doc = "\\[ location, query ID \\]"]
                    NotifyTargetMigrationFail(
                        runtime_types::xcm::VersionedMultiLocation,
                        ::core::primitive::u64,
                    ),
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub enum Origin {
                    #[codec(index = 0)]
                    Xcm(runtime_types::xcm::v1::multilocation::MultiLocation),
                    #[codec(index = 1)]
                    Response(runtime_types::xcm::v1::multilocation::MultiLocation),
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub enum QueryStatus<_0> {
                    #[codec(index = 0)]
                    Pending {
                        responder: runtime_types::xcm::VersionedMultiLocation,
                        maybe_notify:
                            ::core::option::Option<(::core::primitive::u8, ::core::primitive::u8)>,
                        timeout: _0,
                    },
                    #[codec(index = 1)]
                    VersionNotifier {
                        origin: runtime_types::xcm::VersionedMultiLocation,
                        is_active: ::core::primitive::bool,
                    },
                    #[codec(index = 2)]
                    Ready {
                        response: runtime_types::xcm::VersionedResponse,
                        at: _0,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub enum VersionMigrationStage {
                    #[codec(index = 0)]
                    MigrateSupportedVersion,
                    #[codec(index = 1)]
                    MigrateVersionNotifiers,
                    #[codec(index = 2)]
                    NotifyCurrentTargets(
                        ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                    ),
                    #[codec(index = 3)]
                    MigrateAndNotifyOldTargets,
                }
            }
        }
        pub mod polkadot_core_primitives {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct InboundDownwardMessage<_0> {
                pub sent_at: _0,
                pub msg: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct InboundHrmpMessage<_0> {
                pub sent_at: _0,
                pub data: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct OutboundHrmpMessage<_0> {
                pub recipient: _0,
                pub data: ::std::vec::Vec<::core::primitive::u8>,
            }
        }
        pub mod polkadot_parachain {
            use super::runtime_types;
            pub mod primitives {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct HeadData(pub ::std::vec::Vec<::core::primitive::u8>);
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct Id(pub ::core::primitive::u32);
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub enum XcmpMessageFormat {
                    #[codec(index = 0)]
                    ConcatenatedVersionedXcm,
                    #[codec(index = 1)]
                    ConcatenatedEncodedBlob,
                    #[codec(index = 2)]
                    Signals,
                }
            }
        }
        pub mod polkadot_primitives {
            use super::runtime_types;
            pub mod v2 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct AbridgedHostConfiguration {
                    pub max_code_size: ::core::primitive::u32,
                    pub max_head_data_size: ::core::primitive::u32,
                    pub max_upward_queue_count: ::core::primitive::u32,
                    pub max_upward_queue_size: ::core::primitive::u32,
                    pub max_upward_message_size: ::core::primitive::u32,
                    pub max_upward_message_num_per_candidate: ::core::primitive::u32,
                    pub hrmp_max_message_num_per_candidate: ::core::primitive::u32,
                    pub validation_upgrade_cooldown: ::core::primitive::u32,
                    pub validation_upgrade_delay: ::core::primitive::u32,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct AbridgedHrmpChannel {
                    pub max_capacity: ::core::primitive::u32,
                    pub max_total_size: ::core::primitive::u32,
                    pub max_message_size: ::core::primitive::u32,
                    pub msg_count: ::core::primitive::u32,
                    pub total_size: ::core::primitive::u32,
                    pub mqc_head: ::core::option::Option<::subxt::ext::sp_core::H256>,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct PersistedValidationData<_0, _1> {
                    pub parent_head: runtime_types::polkadot_parachain::primitives::HeadData,
                    pub relay_parent_number: _1,
                    pub relay_parent_storage_root: _0,
                    pub max_pov_size: _1,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub enum UpgradeRestriction {
                    #[codec(index = 0)]
                    Present,
                }
            }
        }
        pub mod primitive_types {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct H256(pub [::core::primitive::u8; 32usize]);
        }
        pub mod sp_arithmetic {
            use super::runtime_types;
            pub mod fixed_point {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct FixedU128(pub ::core::primitive::u128);
            }
            pub mod per_things {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct Perbill(pub ::core::primitive::u32);
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct Permill(pub ::core::primitive::u32);
            }
        }
        pub mod sp_consensus_aura {
            use super::runtime_types;
            pub mod sr25519 {
                use super::runtime_types;
                pub mod app_sr25519 {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct Public(pub runtime_types::sp_core::sr25519::Public);
                }
            }
        }
        pub mod sp_consensus_slots {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct Slot(pub ::core::primitive::u64);
        }
        pub mod sp_core {
            use super::runtime_types;
            pub mod crypto {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct AccountId32(pub [::core::primitive::u8; 32usize]);
            }
            pub mod ecdsa {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct Signature(pub [::core::primitive::u8; 65usize]);
            }
            pub mod ed25519 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            pub mod sr25519 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum Void {}
        }
        pub mod sp_runtime {
            use super::runtime_types;
            pub mod bounded {
                use super::runtime_types;
                pub mod bounded_vec {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct BoundedVec<_0>(pub ::std::vec::Vec<_0>);
                }
                pub mod weak_bounded_vec {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct WeakBoundedVec<_0>(pub ::std::vec::Vec<_0>);
                }
            }
            pub mod generic {
                use super::runtime_types;
                pub mod digest {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct Digest {
                        pub logs:
                            ::std::vec::Vec<runtime_types::sp_runtime::generic::digest::DigestItem>,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub enum DigestItem {
                        #[codec(index = 6)]
                        PreRuntime(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 4)]
                        Consensus(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 5)]
                        Seal(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 0)]
                        Other(::std::vec::Vec<::core::primitive::u8>),
                        #[codec(index = 8)]
                        RuntimeEnvironmentUpdated,
                    }
                }
                pub mod era {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub enum Era {
                        #[codec(index = 0)]
                        Immortal,
                        #[codec(index = 1)]
                        Mortal1(::core::primitive::u8),
                        #[codec(index = 2)]
                        Mortal2(::core::primitive::u8),
                        #[codec(index = 3)]
                        Mortal3(::core::primitive::u8),
                        #[codec(index = 4)]
                        Mortal4(::core::primitive::u8),
                        #[codec(index = 5)]
                        Mortal5(::core::primitive::u8),
                        #[codec(index = 6)]
                        Mortal6(::core::primitive::u8),
                        #[codec(index = 7)]
                        Mortal7(::core::primitive::u8),
                        #[codec(index = 8)]
                        Mortal8(::core::primitive::u8),
                        #[codec(index = 9)]
                        Mortal9(::core::primitive::u8),
                        #[codec(index = 10)]
                        Mortal10(::core::primitive::u8),
                        #[codec(index = 11)]
                        Mortal11(::core::primitive::u8),
                        #[codec(index = 12)]
                        Mortal12(::core::primitive::u8),
                        #[codec(index = 13)]
                        Mortal13(::core::primitive::u8),
                        #[codec(index = 14)]
                        Mortal14(::core::primitive::u8),
                        #[codec(index = 15)]
                        Mortal15(::core::primitive::u8),
                        #[codec(index = 16)]
                        Mortal16(::core::primitive::u8),
                        #[codec(index = 17)]
                        Mortal17(::core::primitive::u8),
                        #[codec(index = 18)]
                        Mortal18(::core::primitive::u8),
                        #[codec(index = 19)]
                        Mortal19(::core::primitive::u8),
                        #[codec(index = 20)]
                        Mortal20(::core::primitive::u8),
                        #[codec(index = 21)]
                        Mortal21(::core::primitive::u8),
                        #[codec(index = 22)]
                        Mortal22(::core::primitive::u8),
                        #[codec(index = 23)]
                        Mortal23(::core::primitive::u8),
                        #[codec(index = 24)]
                        Mortal24(::core::primitive::u8),
                        #[codec(index = 25)]
                        Mortal25(::core::primitive::u8),
                        #[codec(index = 26)]
                        Mortal26(::core::primitive::u8),
                        #[codec(index = 27)]
                        Mortal27(::core::primitive::u8),
                        #[codec(index = 28)]
                        Mortal28(::core::primitive::u8),
                        #[codec(index = 29)]
                        Mortal29(::core::primitive::u8),
                        #[codec(index = 30)]
                        Mortal30(::core::primitive::u8),
                        #[codec(index = 31)]
                        Mortal31(::core::primitive::u8),
                        #[codec(index = 32)]
                        Mortal32(::core::primitive::u8),
                        #[codec(index = 33)]
                        Mortal33(::core::primitive::u8),
                        #[codec(index = 34)]
                        Mortal34(::core::primitive::u8),
                        #[codec(index = 35)]
                        Mortal35(::core::primitive::u8),
                        #[codec(index = 36)]
                        Mortal36(::core::primitive::u8),
                        #[codec(index = 37)]
                        Mortal37(::core::primitive::u8),
                        #[codec(index = 38)]
                        Mortal38(::core::primitive::u8),
                        #[codec(index = 39)]
                        Mortal39(::core::primitive::u8),
                        #[codec(index = 40)]
                        Mortal40(::core::primitive::u8),
                        #[codec(index = 41)]
                        Mortal41(::core::primitive::u8),
                        #[codec(index = 42)]
                        Mortal42(::core::primitive::u8),
                        #[codec(index = 43)]
                        Mortal43(::core::primitive::u8),
                        #[codec(index = 44)]
                        Mortal44(::core::primitive::u8),
                        #[codec(index = 45)]
                        Mortal45(::core::primitive::u8),
                        #[codec(index = 46)]
                        Mortal46(::core::primitive::u8),
                        #[codec(index = 47)]
                        Mortal47(::core::primitive::u8),
                        #[codec(index = 48)]
                        Mortal48(::core::primitive::u8),
                        #[codec(index = 49)]
                        Mortal49(::core::primitive::u8),
                        #[codec(index = 50)]
                        Mortal50(::core::primitive::u8),
                        #[codec(index = 51)]
                        Mortal51(::core::primitive::u8),
                        #[codec(index = 52)]
                        Mortal52(::core::primitive::u8),
                        #[codec(index = 53)]
                        Mortal53(::core::primitive::u8),
                        #[codec(index = 54)]
                        Mortal54(::core::primitive::u8),
                        #[codec(index = 55)]
                        Mortal55(::core::primitive::u8),
                        #[codec(index = 56)]
                        Mortal56(::core::primitive::u8),
                        #[codec(index = 57)]
                        Mortal57(::core::primitive::u8),
                        #[codec(index = 58)]
                        Mortal58(::core::primitive::u8),
                        #[codec(index = 59)]
                        Mortal59(::core::primitive::u8),
                        #[codec(index = 60)]
                        Mortal60(::core::primitive::u8),
                        #[codec(index = 61)]
                        Mortal61(::core::primitive::u8),
                        #[codec(index = 62)]
                        Mortal62(::core::primitive::u8),
                        #[codec(index = 63)]
                        Mortal63(::core::primitive::u8),
                        #[codec(index = 64)]
                        Mortal64(::core::primitive::u8),
                        #[codec(index = 65)]
                        Mortal65(::core::primitive::u8),
                        #[codec(index = 66)]
                        Mortal66(::core::primitive::u8),
                        #[codec(index = 67)]
                        Mortal67(::core::primitive::u8),
                        #[codec(index = 68)]
                        Mortal68(::core::primitive::u8),
                        #[codec(index = 69)]
                        Mortal69(::core::primitive::u8),
                        #[codec(index = 70)]
                        Mortal70(::core::primitive::u8),
                        #[codec(index = 71)]
                        Mortal71(::core::primitive::u8),
                        #[codec(index = 72)]
                        Mortal72(::core::primitive::u8),
                        #[codec(index = 73)]
                        Mortal73(::core::primitive::u8),
                        #[codec(index = 74)]
                        Mortal74(::core::primitive::u8),
                        #[codec(index = 75)]
                        Mortal75(::core::primitive::u8),
                        #[codec(index = 76)]
                        Mortal76(::core::primitive::u8),
                        #[codec(index = 77)]
                        Mortal77(::core::primitive::u8),
                        #[codec(index = 78)]
                        Mortal78(::core::primitive::u8),
                        #[codec(index = 79)]
                        Mortal79(::core::primitive::u8),
                        #[codec(index = 80)]
                        Mortal80(::core::primitive::u8),
                        #[codec(index = 81)]
                        Mortal81(::core::primitive::u8),
                        #[codec(index = 82)]
                        Mortal82(::core::primitive::u8),
                        #[codec(index = 83)]
                        Mortal83(::core::primitive::u8),
                        #[codec(index = 84)]
                        Mortal84(::core::primitive::u8),
                        #[codec(index = 85)]
                        Mortal85(::core::primitive::u8),
                        #[codec(index = 86)]
                        Mortal86(::core::primitive::u8),
                        #[codec(index = 87)]
                        Mortal87(::core::primitive::u8),
                        #[codec(index = 88)]
                        Mortal88(::core::primitive::u8),
                        #[codec(index = 89)]
                        Mortal89(::core::primitive::u8),
                        #[codec(index = 90)]
                        Mortal90(::core::primitive::u8),
                        #[codec(index = 91)]
                        Mortal91(::core::primitive::u8),
                        #[codec(index = 92)]
                        Mortal92(::core::primitive::u8),
                        #[codec(index = 93)]
                        Mortal93(::core::primitive::u8),
                        #[codec(index = 94)]
                        Mortal94(::core::primitive::u8),
                        #[codec(index = 95)]
                        Mortal95(::core::primitive::u8),
                        #[codec(index = 96)]
                        Mortal96(::core::primitive::u8),
                        #[codec(index = 97)]
                        Mortal97(::core::primitive::u8),
                        #[codec(index = 98)]
                        Mortal98(::core::primitive::u8),
                        #[codec(index = 99)]
                        Mortal99(::core::primitive::u8),
                        #[codec(index = 100)]
                        Mortal100(::core::primitive::u8),
                        #[codec(index = 101)]
                        Mortal101(::core::primitive::u8),
                        #[codec(index = 102)]
                        Mortal102(::core::primitive::u8),
                        #[codec(index = 103)]
                        Mortal103(::core::primitive::u8),
                        #[codec(index = 104)]
                        Mortal104(::core::primitive::u8),
                        #[codec(index = 105)]
                        Mortal105(::core::primitive::u8),
                        #[codec(index = 106)]
                        Mortal106(::core::primitive::u8),
                        #[codec(index = 107)]
                        Mortal107(::core::primitive::u8),
                        #[codec(index = 108)]
                        Mortal108(::core::primitive::u8),
                        #[codec(index = 109)]
                        Mortal109(::core::primitive::u8),
                        #[codec(index = 110)]
                        Mortal110(::core::primitive::u8),
                        #[codec(index = 111)]
                        Mortal111(::core::primitive::u8),
                        #[codec(index = 112)]
                        Mortal112(::core::primitive::u8),
                        #[codec(index = 113)]
                        Mortal113(::core::primitive::u8),
                        #[codec(index = 114)]
                        Mortal114(::core::primitive::u8),
                        #[codec(index = 115)]
                        Mortal115(::core::primitive::u8),
                        #[codec(index = 116)]
                        Mortal116(::core::primitive::u8),
                        #[codec(index = 117)]
                        Mortal117(::core::primitive::u8),
                        #[codec(index = 118)]
                        Mortal118(::core::primitive::u8),
                        #[codec(index = 119)]
                        Mortal119(::core::primitive::u8),
                        #[codec(index = 120)]
                        Mortal120(::core::primitive::u8),
                        #[codec(index = 121)]
                        Mortal121(::core::primitive::u8),
                        #[codec(index = 122)]
                        Mortal122(::core::primitive::u8),
                        #[codec(index = 123)]
                        Mortal123(::core::primitive::u8),
                        #[codec(index = 124)]
                        Mortal124(::core::primitive::u8),
                        #[codec(index = 125)]
                        Mortal125(::core::primitive::u8),
                        #[codec(index = 126)]
                        Mortal126(::core::primitive::u8),
                        #[codec(index = 127)]
                        Mortal127(::core::primitive::u8),
                        #[codec(index = 128)]
                        Mortal128(::core::primitive::u8),
                        #[codec(index = 129)]
                        Mortal129(::core::primitive::u8),
                        #[codec(index = 130)]
                        Mortal130(::core::primitive::u8),
                        #[codec(index = 131)]
                        Mortal131(::core::primitive::u8),
                        #[codec(index = 132)]
                        Mortal132(::core::primitive::u8),
                        #[codec(index = 133)]
                        Mortal133(::core::primitive::u8),
                        #[codec(index = 134)]
                        Mortal134(::core::primitive::u8),
                        #[codec(index = 135)]
                        Mortal135(::core::primitive::u8),
                        #[codec(index = 136)]
                        Mortal136(::core::primitive::u8),
                        #[codec(index = 137)]
                        Mortal137(::core::primitive::u8),
                        #[codec(index = 138)]
                        Mortal138(::core::primitive::u8),
                        #[codec(index = 139)]
                        Mortal139(::core::primitive::u8),
                        #[codec(index = 140)]
                        Mortal140(::core::primitive::u8),
                        #[codec(index = 141)]
                        Mortal141(::core::primitive::u8),
                        #[codec(index = 142)]
                        Mortal142(::core::primitive::u8),
                        #[codec(index = 143)]
                        Mortal143(::core::primitive::u8),
                        #[codec(index = 144)]
                        Mortal144(::core::primitive::u8),
                        #[codec(index = 145)]
                        Mortal145(::core::primitive::u8),
                        #[codec(index = 146)]
                        Mortal146(::core::primitive::u8),
                        #[codec(index = 147)]
                        Mortal147(::core::primitive::u8),
                        #[codec(index = 148)]
                        Mortal148(::core::primitive::u8),
                        #[codec(index = 149)]
                        Mortal149(::core::primitive::u8),
                        #[codec(index = 150)]
                        Mortal150(::core::primitive::u8),
                        #[codec(index = 151)]
                        Mortal151(::core::primitive::u8),
                        #[codec(index = 152)]
                        Mortal152(::core::primitive::u8),
                        #[codec(index = 153)]
                        Mortal153(::core::primitive::u8),
                        #[codec(index = 154)]
                        Mortal154(::core::primitive::u8),
                        #[codec(index = 155)]
                        Mortal155(::core::primitive::u8),
                        #[codec(index = 156)]
                        Mortal156(::core::primitive::u8),
                        #[codec(index = 157)]
                        Mortal157(::core::primitive::u8),
                        #[codec(index = 158)]
                        Mortal158(::core::primitive::u8),
                        #[codec(index = 159)]
                        Mortal159(::core::primitive::u8),
                        #[codec(index = 160)]
                        Mortal160(::core::primitive::u8),
                        #[codec(index = 161)]
                        Mortal161(::core::primitive::u8),
                        #[codec(index = 162)]
                        Mortal162(::core::primitive::u8),
                        #[codec(index = 163)]
                        Mortal163(::core::primitive::u8),
                        #[codec(index = 164)]
                        Mortal164(::core::primitive::u8),
                        #[codec(index = 165)]
                        Mortal165(::core::primitive::u8),
                        #[codec(index = 166)]
                        Mortal166(::core::primitive::u8),
                        #[codec(index = 167)]
                        Mortal167(::core::primitive::u8),
                        #[codec(index = 168)]
                        Mortal168(::core::primitive::u8),
                        #[codec(index = 169)]
                        Mortal169(::core::primitive::u8),
                        #[codec(index = 170)]
                        Mortal170(::core::primitive::u8),
                        #[codec(index = 171)]
                        Mortal171(::core::primitive::u8),
                        #[codec(index = 172)]
                        Mortal172(::core::primitive::u8),
                        #[codec(index = 173)]
                        Mortal173(::core::primitive::u8),
                        #[codec(index = 174)]
                        Mortal174(::core::primitive::u8),
                        #[codec(index = 175)]
                        Mortal175(::core::primitive::u8),
                        #[codec(index = 176)]
                        Mortal176(::core::primitive::u8),
                        #[codec(index = 177)]
                        Mortal177(::core::primitive::u8),
                        #[codec(index = 178)]
                        Mortal178(::core::primitive::u8),
                        #[codec(index = 179)]
                        Mortal179(::core::primitive::u8),
                        #[codec(index = 180)]
                        Mortal180(::core::primitive::u8),
                        #[codec(index = 181)]
                        Mortal181(::core::primitive::u8),
                        #[codec(index = 182)]
                        Mortal182(::core::primitive::u8),
                        #[codec(index = 183)]
                        Mortal183(::core::primitive::u8),
                        #[codec(index = 184)]
                        Mortal184(::core::primitive::u8),
                        #[codec(index = 185)]
                        Mortal185(::core::primitive::u8),
                        #[codec(index = 186)]
                        Mortal186(::core::primitive::u8),
                        #[codec(index = 187)]
                        Mortal187(::core::primitive::u8),
                        #[codec(index = 188)]
                        Mortal188(::core::primitive::u8),
                        #[codec(index = 189)]
                        Mortal189(::core::primitive::u8),
                        #[codec(index = 190)]
                        Mortal190(::core::primitive::u8),
                        #[codec(index = 191)]
                        Mortal191(::core::primitive::u8),
                        #[codec(index = 192)]
                        Mortal192(::core::primitive::u8),
                        #[codec(index = 193)]
                        Mortal193(::core::primitive::u8),
                        #[codec(index = 194)]
                        Mortal194(::core::primitive::u8),
                        #[codec(index = 195)]
                        Mortal195(::core::primitive::u8),
                        #[codec(index = 196)]
                        Mortal196(::core::primitive::u8),
                        #[codec(index = 197)]
                        Mortal197(::core::primitive::u8),
                        #[codec(index = 198)]
                        Mortal198(::core::primitive::u8),
                        #[codec(index = 199)]
                        Mortal199(::core::primitive::u8),
                        #[codec(index = 200)]
                        Mortal200(::core::primitive::u8),
                        #[codec(index = 201)]
                        Mortal201(::core::primitive::u8),
                        #[codec(index = 202)]
                        Mortal202(::core::primitive::u8),
                        #[codec(index = 203)]
                        Mortal203(::core::primitive::u8),
                        #[codec(index = 204)]
                        Mortal204(::core::primitive::u8),
                        #[codec(index = 205)]
                        Mortal205(::core::primitive::u8),
                        #[codec(index = 206)]
                        Mortal206(::core::primitive::u8),
                        #[codec(index = 207)]
                        Mortal207(::core::primitive::u8),
                        #[codec(index = 208)]
                        Mortal208(::core::primitive::u8),
                        #[codec(index = 209)]
                        Mortal209(::core::primitive::u8),
                        #[codec(index = 210)]
                        Mortal210(::core::primitive::u8),
                        #[codec(index = 211)]
                        Mortal211(::core::primitive::u8),
                        #[codec(index = 212)]
                        Mortal212(::core::primitive::u8),
                        #[codec(index = 213)]
                        Mortal213(::core::primitive::u8),
                        #[codec(index = 214)]
                        Mortal214(::core::primitive::u8),
                        #[codec(index = 215)]
                        Mortal215(::core::primitive::u8),
                        #[codec(index = 216)]
                        Mortal216(::core::primitive::u8),
                        #[codec(index = 217)]
                        Mortal217(::core::primitive::u8),
                        #[codec(index = 218)]
                        Mortal218(::core::primitive::u8),
                        #[codec(index = 219)]
                        Mortal219(::core::primitive::u8),
                        #[codec(index = 220)]
                        Mortal220(::core::primitive::u8),
                        #[codec(index = 221)]
                        Mortal221(::core::primitive::u8),
                        #[codec(index = 222)]
                        Mortal222(::core::primitive::u8),
                        #[codec(index = 223)]
                        Mortal223(::core::primitive::u8),
                        #[codec(index = 224)]
                        Mortal224(::core::primitive::u8),
                        #[codec(index = 225)]
                        Mortal225(::core::primitive::u8),
                        #[codec(index = 226)]
                        Mortal226(::core::primitive::u8),
                        #[codec(index = 227)]
                        Mortal227(::core::primitive::u8),
                        #[codec(index = 228)]
                        Mortal228(::core::primitive::u8),
                        #[codec(index = 229)]
                        Mortal229(::core::primitive::u8),
                        #[codec(index = 230)]
                        Mortal230(::core::primitive::u8),
                        #[codec(index = 231)]
                        Mortal231(::core::primitive::u8),
                        #[codec(index = 232)]
                        Mortal232(::core::primitive::u8),
                        #[codec(index = 233)]
                        Mortal233(::core::primitive::u8),
                        #[codec(index = 234)]
                        Mortal234(::core::primitive::u8),
                        #[codec(index = 235)]
                        Mortal235(::core::primitive::u8),
                        #[codec(index = 236)]
                        Mortal236(::core::primitive::u8),
                        #[codec(index = 237)]
                        Mortal237(::core::primitive::u8),
                        #[codec(index = 238)]
                        Mortal238(::core::primitive::u8),
                        #[codec(index = 239)]
                        Mortal239(::core::primitive::u8),
                        #[codec(index = 240)]
                        Mortal240(::core::primitive::u8),
                        #[codec(index = 241)]
                        Mortal241(::core::primitive::u8),
                        #[codec(index = 242)]
                        Mortal242(::core::primitive::u8),
                        #[codec(index = 243)]
                        Mortal243(::core::primitive::u8),
                        #[codec(index = 244)]
                        Mortal244(::core::primitive::u8),
                        #[codec(index = 245)]
                        Mortal245(::core::primitive::u8),
                        #[codec(index = 246)]
                        Mortal246(::core::primitive::u8),
                        #[codec(index = 247)]
                        Mortal247(::core::primitive::u8),
                        #[codec(index = 248)]
                        Mortal248(::core::primitive::u8),
                        #[codec(index = 249)]
                        Mortal249(::core::primitive::u8),
                        #[codec(index = 250)]
                        Mortal250(::core::primitive::u8),
                        #[codec(index = 251)]
                        Mortal251(::core::primitive::u8),
                        #[codec(index = 252)]
                        Mortal252(::core::primitive::u8),
                        #[codec(index = 253)]
                        Mortal253(::core::primitive::u8),
                        #[codec(index = 254)]
                        Mortal254(::core::primitive::u8),
                        #[codec(index = 255)]
                        Mortal255(::core::primitive::u8),
                    }
                }
                pub mod unchecked_extrinsic {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct UncheckedExtrinsic<_0, _1, _2, _3>(
                        pub ::std::vec::Vec<::core::primitive::u8>,
                        #[codec(skip)] pub ::core::marker::PhantomData<(_0, _1, _2, _3)>,
                    );
                }
            }
            pub mod multiaddress {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub enum MultiAddress<_0, _1> {
                    #[codec(index = 0)]
                    Id(_0),
                    #[codec(index = 1)]
                    Index(#[codec(compact)] _1),
                    #[codec(index = 2)]
                    Raw(::std::vec::Vec<::core::primitive::u8>),
                    #[codec(index = 3)]
                    Address32([::core::primitive::u8; 32usize]),
                    #[codec(index = 4)]
                    Address20([::core::primitive::u8; 20usize]),
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum ArithmeticError {
                #[codec(index = 0)]
                Underflow,
                #[codec(index = 1)]
                Overflow,
                #[codec(index = 2)]
                DivisionByZero,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum DispatchError {
                #[codec(index = 0)]
                Other,
                #[codec(index = 1)]
                CannotLookup,
                #[codec(index = 2)]
                BadOrigin,
                #[codec(index = 3)]
                Module(runtime_types::sp_runtime::ModuleError),
                #[codec(index = 4)]
                ConsumerRemaining,
                #[codec(index = 5)]
                NoProviders,
                #[codec(index = 6)]
                TooManyConsumers,
                #[codec(index = 7)]
                Token(runtime_types::sp_runtime::TokenError),
                #[codec(index = 8)]
                Arithmetic(runtime_types::sp_runtime::ArithmeticError),
                #[codec(index = 9)]
                Transactional(runtime_types::sp_runtime::TransactionalError),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ModuleError {
                pub index: ::core::primitive::u8,
                pub error: [::core::primitive::u8; 4usize],
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum MultiSignature {
                #[codec(index = 0)]
                Ed25519(runtime_types::sp_core::ed25519::Signature),
                #[codec(index = 1)]
                Sr25519(runtime_types::sp_core::sr25519::Signature),
                #[codec(index = 2)]
                Ecdsa(runtime_types::sp_core::ecdsa::Signature),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum TokenError {
                #[codec(index = 0)]
                NoFunds,
                #[codec(index = 1)]
                WouldDie,
                #[codec(index = 2)]
                BelowMinimum,
                #[codec(index = 3)]
                CannotCreate,
                #[codec(index = 4)]
                UnknownAsset,
                #[codec(index = 5)]
                Frozen,
                #[codec(index = 6)]
                Unsupported,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum TransactionalError {
                #[codec(index = 0)]
                LimitReached,
                #[codec(index = 1)]
                NoLayer,
            }
        }
        pub mod sp_trie {
            use super::runtime_types;
            pub mod storage_proof {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct StorageProof {
                    pub trie_nodes: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                }
            }
        }
        pub mod sp_version {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct RuntimeVersion {
                pub spec_name: ::std::string::String,
                pub impl_name: ::std::string::String,
                pub authoring_version: ::core::primitive::u32,
                pub spec_version: ::core::primitive::u32,
                pub impl_version: ::core::primitive::u32,
                pub apis:
                    ::std::vec::Vec<([::core::primitive::u8; 8usize], ::core::primitive::u32)>,
                pub transaction_version: ::core::primitive::u32,
                pub state_version: ::core::primitive::u8,
            }
        }
        pub mod substrate_fixed {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct FixedI128<_0> {
                pub bits: ::core::primitive::i128,
                #[codec(skip)]
                pub __subxt_unused_type_params: ::core::marker::PhantomData<_0>,
            }
        }
        pub mod typenum {
            use super::runtime_types;
            pub mod bit {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct B0;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct B1;
            }
            pub mod uint {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct UInt<_0, _1> {
                    pub msb: _0,
                    pub lsb: _1,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct UTerm;
            }
        }
        pub mod xcm {
            use super::runtime_types;
            pub mod double_encoded {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct DoubleEncoded {
                    pub encoded: ::std::vec::Vec<::core::primitive::u8>,
                }
            }
            pub mod v0 {
                use super::runtime_types;
                pub mod junction {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub enum BodyId {
                        #[codec(index = 0)]
                        Unit,
                        #[codec(index = 1)]
                        Named(
                            runtime_types::sp_runtime::bounded::weak_bounded_vec::WeakBoundedVec<
                                ::core::primitive::u8,
                            >,
                        ),
                        #[codec(index = 2)]
                        Index(#[codec(compact)] ::core::primitive::u32),
                        #[codec(index = 3)]
                        Executive,
                        #[codec(index = 4)]
                        Technical,
                        #[codec(index = 5)]
                        Legislative,
                        #[codec(index = 6)]
                        Judicial,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub enum BodyPart {
                        #[codec(index = 0)]
                        Voice,
                        #[codec(index = 1)]
                        Members {
                            #[codec(compact)]
                            count: ::core::primitive::u32,
                        },
                        #[codec(index = 2)]
                        Fraction {
                            #[codec(compact)]
                            nom: ::core::primitive::u32,
                            #[codec(compact)]
                            denom: ::core::primitive::u32,
                        },
                        #[codec(index = 3)]
                        AtLeastProportion {
                            #[codec(compact)]
                            nom: ::core::primitive::u32,
                            #[codec(compact)]
                            denom: ::core::primitive::u32,
                        },
                        #[codec(index = 4)]
                        MoreThanProportion {
                            #[codec(compact)]
                            nom: ::core::primitive::u32,
                            #[codec(compact)]
                            denom: ::core::primitive::u32,
                        },
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub enum Junction {
                        #[codec(index = 0)]
                        Parent,
                        #[codec(index = 1)]
                        Parachain(#[codec(compact)] ::core::primitive::u32),
                        #[codec(index = 2)]
                        AccountId32 {
                            network: runtime_types::xcm::v0::junction::NetworkId,
                            id: [::core::primitive::u8; 32usize],
                        },
                        #[codec(index = 3)]
                        AccountIndex64 {
                            network: runtime_types::xcm::v0::junction::NetworkId,
                            #[codec(compact)]
                            index: ::core::primitive::u64,
                        },
                        #[codec(index = 4)]
                        AccountKey20 {
                            network: runtime_types::xcm::v0::junction::NetworkId,
                            key: [::core::primitive::u8; 20usize],
                        },
                        #[codec(index = 5)]
                        PalletInstance(::core::primitive::u8),
                        #[codec(index = 6)]
                        GeneralIndex(#[codec(compact)] ::core::primitive::u128),
                        #[codec(index = 7)]
                        GeneralKey(
                            runtime_types::sp_runtime::bounded::weak_bounded_vec::WeakBoundedVec<
                                ::core::primitive::u8,
                            >,
                        ),
                        #[codec(index = 8)]
                        OnlyChild,
                        #[codec(index = 9)]
                        Plurality {
                            id: runtime_types::xcm::v0::junction::BodyId,
                            part: runtime_types::xcm::v0::junction::BodyPart,
                        },
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub enum NetworkId {
                        #[codec(index = 0)]
                        Any,
                        #[codec(index = 1)]
                        Named(
                            runtime_types::sp_runtime::bounded::weak_bounded_vec::WeakBoundedVec<
                                ::core::primitive::u8,
                            >,
                        ),
                        #[codec(index = 2)]
                        Polkadot,
                        #[codec(index = 3)]
                        Kusama,
                    }
                }
                pub mod multi_asset {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub enum MultiAsset {
                        #[codec(index = 0)]
                        None,
                        #[codec(index = 1)]
                        All,
                        #[codec(index = 2)]
                        AllFungible,
                        #[codec(index = 3)]
                        AllNonFungible,
                        #[codec(index = 4)]
                        AllAbstractFungible {
                            id: ::std::vec::Vec<::core::primitive::u8>,
                        },
                        #[codec(index = 5)]
                        AllAbstractNonFungible {
                            class: ::std::vec::Vec<::core::primitive::u8>,
                        },
                        #[codec(index = 6)]
                        AllConcreteFungible {
                            id: runtime_types::xcm::v0::multi_location::MultiLocation,
                        },
                        #[codec(index = 7)]
                        AllConcreteNonFungible {
                            class: runtime_types::xcm::v0::multi_location::MultiLocation,
                        },
                        #[codec(index = 8)]
                        AbstractFungible {
                            id: ::std::vec::Vec<::core::primitive::u8>,
                            #[codec(compact)]
                            amount: ::core::primitive::u128,
                        },
                        #[codec(index = 9)]
                        AbstractNonFungible {
                            class: ::std::vec::Vec<::core::primitive::u8>,
                            instance: runtime_types::xcm::v1::multiasset::AssetInstance,
                        },
                        #[codec(index = 10)]
                        ConcreteFungible {
                            id: runtime_types::xcm::v0::multi_location::MultiLocation,
                            #[codec(compact)]
                            amount: ::core::primitive::u128,
                        },
                        #[codec(index = 11)]
                        ConcreteNonFungible {
                            class: runtime_types::xcm::v0::multi_location::MultiLocation,
                            instance: runtime_types::xcm::v1::multiasset::AssetInstance,
                        },
                    }
                }
                pub mod multi_location {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub enum MultiLocation {
                        #[codec(index = 0)]
                        Null,
                        #[codec(index = 1)]
                        X1(runtime_types::xcm::v0::junction::Junction),
                        #[codec(index = 2)]
                        X2(
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                        ),
                        #[codec(index = 3)]
                        X3(
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                        ),
                        #[codec(index = 4)]
                        X4(
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                        ),
                        #[codec(index = 5)]
                        X5(
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                        ),
                        #[codec(index = 6)]
                        X6(
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                        ),
                        #[codec(index = 7)]
                        X7(
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                        ),
                        #[codec(index = 8)]
                        X8(
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                        ),
                    }
                }
                pub mod order {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub enum Order {
                        #[codec(index = 0)]
                        Null,
                        #[codec(index = 1)]
                        DepositAsset {
                            assets:
                                ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                            dest: runtime_types::xcm::v0::multi_location::MultiLocation,
                        },
                        #[codec(index = 2)]
                        DepositReserveAsset {
                            assets:
                                ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                            dest: runtime_types::xcm::v0::multi_location::MultiLocation,
                            effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
                        },
                        #[codec(index = 3)]
                        ExchangeAsset {
                            give: ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                            receive:
                                ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                        },
                        #[codec(index = 4)]
                        InitiateReserveWithdraw {
                            assets:
                                ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                            reserve: runtime_types::xcm::v0::multi_location::MultiLocation,
                            effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
                        },
                        #[codec(index = 5)]
                        InitiateTeleport {
                            assets:
                                ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                            dest: runtime_types::xcm::v0::multi_location::MultiLocation,
                            effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
                        },
                        #[codec(index = 6)]
                        QueryHolding {
                            #[codec(compact)]
                            query_id: ::core::primitive::u64,
                            dest: runtime_types::xcm::v0::multi_location::MultiLocation,
                            assets:
                                ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                        },
                        #[codec(index = 7)]
                        BuyExecution {
                            fees: runtime_types::xcm::v0::multi_asset::MultiAsset,
                            weight: ::core::primitive::u64,
                            debt: ::core::primitive::u64,
                            halt_on_error: ::core::primitive::bool,
                            xcm: ::std::vec::Vec<runtime_types::xcm::v0::Xcm>,
                        },
                    }
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub enum OriginKind {
                    #[codec(index = 0)]
                    Native,
                    #[codec(index = 1)]
                    SovereignAccount,
                    #[codec(index = 2)]
                    Superuser,
                    #[codec(index = 3)]
                    Xcm,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub enum Response {
                    #[codec(index = 0)]
                    Assets(::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>),
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub enum Xcm {
                    #[codec(index = 0)]
                    WithdrawAsset {
                        assets: ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                        effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
                    },
                    #[codec(index = 1)]
                    ReserveAssetDeposit {
                        assets: ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                        effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
                    },
                    #[codec(index = 2)]
                    TeleportAsset {
                        assets: ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                        effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
                    },
                    #[codec(index = 3)]
                    QueryResponse {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        response: runtime_types::xcm::v0::Response,
                    },
                    #[codec(index = 4)]
                    TransferAsset {
                        assets: ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                        dest: runtime_types::xcm::v0::multi_location::MultiLocation,
                    },
                    #[codec(index = 5)]
                    TransferReserveAsset {
                        assets: ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                        dest: runtime_types::xcm::v0::multi_location::MultiLocation,
                        effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
                    },
                    #[codec(index = 6)]
                    Transact {
                        origin_type: runtime_types::xcm::v0::OriginKind,
                        require_weight_at_most: ::core::primitive::u64,
                        call: runtime_types::xcm::double_encoded::DoubleEncoded,
                    },
                    #[codec(index = 7)]
                    HrmpNewChannelOpenRequest {
                        #[codec(compact)]
                        sender: ::core::primitive::u32,
                        #[codec(compact)]
                        max_message_size: ::core::primitive::u32,
                        #[codec(compact)]
                        max_capacity: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
                    HrmpChannelAccepted {
                        #[codec(compact)]
                        recipient: ::core::primitive::u32,
                    },
                    #[codec(index = 9)]
                    HrmpChannelClosing {
                        #[codec(compact)]
                        initiator: ::core::primitive::u32,
                        #[codec(compact)]
                        sender: ::core::primitive::u32,
                        #[codec(compact)]
                        recipient: ::core::primitive::u32,
                    },
                    #[codec(index = 10)]
                    RelayedFrom {
                        who: runtime_types::xcm::v0::multi_location::MultiLocation,
                        message: ::std::boxed::Box<runtime_types::xcm::v0::Xcm>,
                    },
                }
            }
            pub mod v1 {
                use super::runtime_types;
                pub mod junction {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub enum Junction {
                        #[codec(index = 0)]
                        Parachain(#[codec(compact)] ::core::primitive::u32),
                        #[codec(index = 1)]
                        AccountId32 {
                            network: runtime_types::xcm::v0::junction::NetworkId,
                            id: [::core::primitive::u8; 32usize],
                        },
                        #[codec(index = 2)]
                        AccountIndex64 {
                            network: runtime_types::xcm::v0::junction::NetworkId,
                            #[codec(compact)]
                            index: ::core::primitive::u64,
                        },
                        #[codec(index = 3)]
                        AccountKey20 {
                            network: runtime_types::xcm::v0::junction::NetworkId,
                            key: [::core::primitive::u8; 20usize],
                        },
                        #[codec(index = 4)]
                        PalletInstance(::core::primitive::u8),
                        #[codec(index = 5)]
                        GeneralIndex(#[codec(compact)] ::core::primitive::u128),
                        #[codec(index = 6)]
                        GeneralKey(
                            runtime_types::sp_runtime::bounded::weak_bounded_vec::WeakBoundedVec<
                                ::core::primitive::u8,
                            >,
                        ),
                        #[codec(index = 7)]
                        OnlyChild,
                        #[codec(index = 8)]
                        Plurality {
                            id: runtime_types::xcm::v0::junction::BodyId,
                            part: runtime_types::xcm::v0::junction::BodyPart,
                        },
                    }
                }
                pub mod multiasset {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub enum AssetId {
                        #[codec(index = 0)]
                        Concrete(runtime_types::xcm::v1::multilocation::MultiLocation),
                        #[codec(index = 1)]
                        Abstract(::std::vec::Vec<::core::primitive::u8>),
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub enum AssetInstance {
                        #[codec(index = 0)]
                        Undefined,
                        #[codec(index = 1)]
                        Index(#[codec(compact)] ::core::primitive::u128),
                        #[codec(index = 2)]
                        Array4([::core::primitive::u8; 4usize]),
                        #[codec(index = 3)]
                        Array8([::core::primitive::u8; 8usize]),
                        #[codec(index = 4)]
                        Array16([::core::primitive::u8; 16usize]),
                        #[codec(index = 5)]
                        Array32([::core::primitive::u8; 32usize]),
                        #[codec(index = 6)]
                        Blob(::std::vec::Vec<::core::primitive::u8>),
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub enum Fungibility {
                        #[codec(index = 0)]
                        Fungible(#[codec(compact)] ::core::primitive::u128),
                        #[codec(index = 1)]
                        NonFungible(runtime_types::xcm::v1::multiasset::AssetInstance),
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct MultiAsset {
                        pub id: runtime_types::xcm::v1::multiasset::AssetId,
                        pub fun: runtime_types::xcm::v1::multiasset::Fungibility,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub enum MultiAssetFilter {
                        #[codec(index = 0)]
                        Definite(runtime_types::xcm::v1::multiasset::MultiAssets),
                        #[codec(index = 1)]
                        Wild(runtime_types::xcm::v1::multiasset::WildMultiAsset),
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct MultiAssets(
                        pub ::std::vec::Vec<runtime_types::xcm::v1::multiasset::MultiAsset>,
                    );
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub enum WildFungibility {
                        #[codec(index = 0)]
                        Fungible,
                        #[codec(index = 1)]
                        NonFungible,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub enum WildMultiAsset {
                        #[codec(index = 0)]
                        All,
                        #[codec(index = 1)]
                        AllOf {
                            id: runtime_types::xcm::v1::multiasset::AssetId,
                            fun: runtime_types::xcm::v1::multiasset::WildFungibility,
                        },
                    }
                }
                pub mod multilocation {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub enum Junctions {
                        #[codec(index = 0)]
                        Here,
                        #[codec(index = 1)]
                        X1(runtime_types::xcm::v1::junction::Junction),
                        #[codec(index = 2)]
                        X2(
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                        ),
                        #[codec(index = 3)]
                        X3(
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                        ),
                        #[codec(index = 4)]
                        X4(
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                        ),
                        #[codec(index = 5)]
                        X5(
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                        ),
                        #[codec(index = 6)]
                        X6(
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                        ),
                        #[codec(index = 7)]
                        X7(
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                        ),
                        #[codec(index = 8)]
                        X8(
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                        ),
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct MultiLocation {
                        pub parents: ::core::primitive::u8,
                        pub interior: runtime_types::xcm::v1::multilocation::Junctions,
                    }
                }
                pub mod order {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub enum Order {
                        #[codec(index = 0)]
                        Noop,
                        #[codec(index = 1)]
                        DepositAsset {
                            assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                            max_assets: ::core::primitive::u32,
                            beneficiary: runtime_types::xcm::v1::multilocation::MultiLocation,
                        },
                        #[codec(index = 2)]
                        DepositReserveAsset {
                            assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                            max_assets: ::core::primitive::u32,
                            dest: runtime_types::xcm::v1::multilocation::MultiLocation,
                            effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
                        },
                        #[codec(index = 3)]
                        ExchangeAsset {
                            give: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                            receive: runtime_types::xcm::v1::multiasset::MultiAssets,
                        },
                        #[codec(index = 4)]
                        InitiateReserveWithdraw {
                            assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                            reserve: runtime_types::xcm::v1::multilocation::MultiLocation,
                            effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
                        },
                        #[codec(index = 5)]
                        InitiateTeleport {
                            assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                            dest: runtime_types::xcm::v1::multilocation::MultiLocation,
                            effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
                        },
                        #[codec(index = 6)]
                        QueryHolding {
                            #[codec(compact)]
                            query_id: ::core::primitive::u64,
                            dest: runtime_types::xcm::v1::multilocation::MultiLocation,
                            assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                        },
                        #[codec(index = 7)]
                        BuyExecution {
                            fees: runtime_types::xcm::v1::multiasset::MultiAsset,
                            weight: ::core::primitive::u64,
                            debt: ::core::primitive::u64,
                            halt_on_error: ::core::primitive::bool,
                            instructions: ::std::vec::Vec<runtime_types::xcm::v1::Xcm>,
                        },
                    }
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub enum Response {
                    #[codec(index = 0)]
                    Assets(runtime_types::xcm::v1::multiasset::MultiAssets),
                    #[codec(index = 1)]
                    Version(::core::primitive::u32),
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub enum Xcm {
                    #[codec(index = 0)]
                    WithdrawAsset {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssets,
                        effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
                    },
                    #[codec(index = 1)]
                    ReserveAssetDeposited {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssets,
                        effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
                    },
                    #[codec(index = 2)]
                    ReceiveTeleportedAsset {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssets,
                        effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
                    },
                    #[codec(index = 3)]
                    QueryResponse {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        response: runtime_types::xcm::v1::Response,
                    },
                    #[codec(index = 4)]
                    TransferAsset {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssets,
                        beneficiary: runtime_types::xcm::v1::multilocation::MultiLocation,
                    },
                    #[codec(index = 5)]
                    TransferReserveAsset {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssets,
                        dest: runtime_types::xcm::v1::multilocation::MultiLocation,
                        effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
                    },
                    #[codec(index = 6)]
                    Transact {
                        origin_type: runtime_types::xcm::v0::OriginKind,
                        require_weight_at_most: ::core::primitive::u64,
                        call: runtime_types::xcm::double_encoded::DoubleEncoded,
                    },
                    #[codec(index = 7)]
                    HrmpNewChannelOpenRequest {
                        #[codec(compact)]
                        sender: ::core::primitive::u32,
                        #[codec(compact)]
                        max_message_size: ::core::primitive::u32,
                        #[codec(compact)]
                        max_capacity: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
                    HrmpChannelAccepted {
                        #[codec(compact)]
                        recipient: ::core::primitive::u32,
                    },
                    #[codec(index = 9)]
                    HrmpChannelClosing {
                        #[codec(compact)]
                        initiator: ::core::primitive::u32,
                        #[codec(compact)]
                        sender: ::core::primitive::u32,
                        #[codec(compact)]
                        recipient: ::core::primitive::u32,
                    },
                    #[codec(index = 10)]
                    RelayedFrom {
                        who: runtime_types::xcm::v1::multilocation::Junctions,
                        message: ::std::boxed::Box<runtime_types::xcm::v1::Xcm>,
                    },
                    #[codec(index = 11)]
                    SubscribeVersion {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        #[codec(compact)]
                        max_response_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 12)]
                    UnsubscribeVersion,
                }
            }
            pub mod v2 {
                use super::runtime_types;
                pub mod traits {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub enum Error {
                        #[codec(index = 0)]
                        Overflow,
                        #[codec(index = 1)]
                        Unimplemented,
                        #[codec(index = 2)]
                        UntrustedReserveLocation,
                        #[codec(index = 3)]
                        UntrustedTeleportLocation,
                        #[codec(index = 4)]
                        MultiLocationFull,
                        #[codec(index = 5)]
                        MultiLocationNotInvertible,
                        #[codec(index = 6)]
                        BadOrigin,
                        #[codec(index = 7)]
                        InvalidLocation,
                        #[codec(index = 8)]
                        AssetNotFound,
                        #[codec(index = 9)]
                        FailedToTransactAsset,
                        #[codec(index = 10)]
                        NotWithdrawable,
                        #[codec(index = 11)]
                        LocationCannotHold,
                        #[codec(index = 12)]
                        ExceedsMaxMessageSize,
                        #[codec(index = 13)]
                        DestinationUnsupported,
                        #[codec(index = 14)]
                        Transport,
                        #[codec(index = 15)]
                        Unroutable,
                        #[codec(index = 16)]
                        UnknownClaim,
                        #[codec(index = 17)]
                        FailedToDecode,
                        #[codec(index = 18)]
                        MaxWeightInvalid,
                        #[codec(index = 19)]
                        NotHoldingFees,
                        #[codec(index = 20)]
                        TooExpensive,
                        #[codec(index = 21)]
                        Trap(::core::primitive::u64),
                        #[codec(index = 22)]
                        UnhandledXcmVersion,
                        #[codec(index = 23)]
                        WeightLimitReached(::core::primitive::u64),
                        #[codec(index = 24)]
                        Barrier,
                        #[codec(index = 25)]
                        WeightNotComputable,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub enum Outcome {
                        #[codec(index = 0)]
                        Complete(::core::primitive::u64),
                        #[codec(index = 1)]
                        Incomplete(
                            ::core::primitive::u64,
                            runtime_types::xcm::v2::traits::Error,
                        ),
                        #[codec(index = 2)]
                        Error(runtime_types::xcm::v2::traits::Error),
                    }
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub enum Instruction {
                    #[codec(index = 0)]
                    WithdrawAsset(runtime_types::xcm::v1::multiasset::MultiAssets),
                    #[codec(index = 1)]
                    ReserveAssetDeposited(runtime_types::xcm::v1::multiasset::MultiAssets),
                    #[codec(index = 2)]
                    ReceiveTeleportedAsset(runtime_types::xcm::v1::multiasset::MultiAssets),
                    #[codec(index = 3)]
                    QueryResponse {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        response: runtime_types::xcm::v2::Response,
                        #[codec(compact)]
                        max_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 4)]
                    TransferAsset {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssets,
                        beneficiary: runtime_types::xcm::v1::multilocation::MultiLocation,
                    },
                    #[codec(index = 5)]
                    TransferReserveAsset {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssets,
                        dest: runtime_types::xcm::v1::multilocation::MultiLocation,
                        xcm: runtime_types::xcm::v2::Xcm,
                    },
                    #[codec(index = 6)]
                    Transact {
                        origin_type: runtime_types::xcm::v0::OriginKind,
                        #[codec(compact)]
                        require_weight_at_most: ::core::primitive::u64,
                        call: runtime_types::xcm::double_encoded::DoubleEncoded,
                    },
                    #[codec(index = 7)]
                    HrmpNewChannelOpenRequest {
                        #[codec(compact)]
                        sender: ::core::primitive::u32,
                        #[codec(compact)]
                        max_message_size: ::core::primitive::u32,
                        #[codec(compact)]
                        max_capacity: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
                    HrmpChannelAccepted {
                        #[codec(compact)]
                        recipient: ::core::primitive::u32,
                    },
                    #[codec(index = 9)]
                    HrmpChannelClosing {
                        #[codec(compact)]
                        initiator: ::core::primitive::u32,
                        #[codec(compact)]
                        sender: ::core::primitive::u32,
                        #[codec(compact)]
                        recipient: ::core::primitive::u32,
                    },
                    #[codec(index = 10)]
                    ClearOrigin,
                    #[codec(index = 11)]
                    DescendOrigin(runtime_types::xcm::v1::multilocation::Junctions),
                    #[codec(index = 12)]
                    ReportError {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        dest: runtime_types::xcm::v1::multilocation::MultiLocation,
                        #[codec(compact)]
                        max_response_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 13)]
                    DepositAsset {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                        #[codec(compact)]
                        max_assets: ::core::primitive::u32,
                        beneficiary: runtime_types::xcm::v1::multilocation::MultiLocation,
                    },
                    #[codec(index = 14)]
                    DepositReserveAsset {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                        #[codec(compact)]
                        max_assets: ::core::primitive::u32,
                        dest: runtime_types::xcm::v1::multilocation::MultiLocation,
                        xcm: runtime_types::xcm::v2::Xcm,
                    },
                    #[codec(index = 15)]
                    ExchangeAsset {
                        give: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                        receive: runtime_types::xcm::v1::multiasset::MultiAssets,
                    },
                    #[codec(index = 16)]
                    InitiateReserveWithdraw {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                        reserve: runtime_types::xcm::v1::multilocation::MultiLocation,
                        xcm: runtime_types::xcm::v2::Xcm,
                    },
                    #[codec(index = 17)]
                    InitiateTeleport {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                        dest: runtime_types::xcm::v1::multilocation::MultiLocation,
                        xcm: runtime_types::xcm::v2::Xcm,
                    },
                    #[codec(index = 18)]
                    QueryHolding {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        dest: runtime_types::xcm::v1::multilocation::MultiLocation,
                        assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                        #[codec(compact)]
                        max_response_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 19)]
                    BuyExecution {
                        fees: runtime_types::xcm::v1::multiasset::MultiAsset,
                        weight_limit: runtime_types::xcm::v2::WeightLimit,
                    },
                    #[codec(index = 20)]
                    RefundSurplus,
                    #[codec(index = 21)]
                    SetErrorHandler(runtime_types::xcm::v2::Xcm),
                    #[codec(index = 22)]
                    SetAppendix(runtime_types::xcm::v2::Xcm),
                    #[codec(index = 23)]
                    ClearError,
                    #[codec(index = 24)]
                    ClaimAsset {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssets,
                        ticket: runtime_types::xcm::v1::multilocation::MultiLocation,
                    },
                    #[codec(index = 25)]
                    Trap(#[codec(compact)] ::core::primitive::u64),
                    #[codec(index = 26)]
                    SubscribeVersion {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        #[codec(compact)]
                        max_response_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 27)]
                    UnsubscribeVersion,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub enum Response {
                    #[codec(index = 0)]
                    Null,
                    #[codec(index = 1)]
                    Assets(runtime_types::xcm::v1::multiasset::MultiAssets),
                    #[codec(index = 2)]
                    ExecutionResult(
                        ::core::option::Option<(
                            ::core::primitive::u32,
                            runtime_types::xcm::v2::traits::Error,
                        )>,
                    ),
                    #[codec(index = 3)]
                    Version(::core::primitive::u32),
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub enum WeightLimit {
                    #[codec(index = 0)]
                    Unlimited,
                    #[codec(index = 1)]
                    Limited(#[codec(compact)] ::core::primitive::u64),
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct Xcm(pub ::std::vec::Vec<runtime_types::xcm::v2::Instruction>);
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum VersionedMultiAssets {
                #[codec(index = 0)]
                V0(::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>),
                #[codec(index = 1)]
                V1(runtime_types::xcm::v1::multiasset::MultiAssets),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum VersionedMultiLocation {
                #[codec(index = 0)]
                V0(runtime_types::xcm::v0::multi_location::MultiLocation),
                #[codec(index = 1)]
                V1(runtime_types::xcm::v1::multilocation::MultiLocation),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum VersionedResponse {
                #[codec(index = 0)]
                V0(runtime_types::xcm::v0::Response),
                #[codec(index = 1)]
                V1(runtime_types::xcm::v1::Response),
                #[codec(index = 2)]
                V2(runtime_types::xcm::v2::Response),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum VersionedXcm {
                #[codec(index = 0)]
                V0(runtime_types::xcm::v0::Xcm),
                #[codec(index = 1)]
                V1(runtime_types::xcm::v1::Xcm),
                #[codec(index = 2)]
                V2(runtime_types::xcm::v2::Xcm),
            }
        }
    }
    #[doc = r" The default error type returned when there is a runtime issue,"]
    #[doc = r" exposed here for ease of use."]
    pub type DispatchError = runtime_types::sp_runtime::DispatchError;
    pub fn constants() -> ConstantsApi {
        ConstantsApi
    }
    pub fn storage() -> StorageApi {
        StorageApi
    }
    pub fn tx() -> TransactionApi {
        TransactionApi
    }
    pub struct ConstantsApi;
    impl ConstantsApi {
        pub fn system(&self) -> system::constants::ConstantsApi {
            system::constants::ConstantsApi
        }
        pub fn timestamp(&self) -> timestamp::constants::ConstantsApi {
            timestamp::constants::ConstantsApi
        }
        pub fn balances(&self) -> balances::constants::ConstantsApi {
            balances::constants::ConstantsApi
        }
        pub fn transaction_payment(&self) -> transaction_payment::constants::ConstantsApi {
            transaction_payment::constants::ConstantsApi
        }
        pub fn utility(&self) -> utility::constants::ConstantsApi {
            utility::constants::ConstantsApi
        }
        pub fn treasury(&self) -> treasury::constants::ConstantsApi {
            treasury::constants::ConstantsApi
        }
        pub fn proxy(&self) -> proxy::constants::ConstantsApi {
            proxy::constants::ConstantsApi
        }
        pub fn scheduler(&self) -> scheduler::constants::ConstantsApi {
            scheduler::constants::ConstantsApi
        }
        pub fn encointer_scheduler(&self) -> encointer_scheduler::constants::ConstantsApi {
            encointer_scheduler::constants::ConstantsApi
        }
        pub fn encointer_ceremonies(&self) -> encointer_ceremonies::constants::ConstantsApi {
            encointer_ceremonies::constants::ConstantsApi
        }
        pub fn encointer_balances(&self) -> encointer_balances::constants::ConstantsApi {
            encointer_balances::constants::ConstantsApi
        }
    }
    pub struct StorageApi;
    impl StorageApi {
        pub fn system(&self) -> system::storage::StorageApi {
            system::storage::StorageApi
        }
        pub fn parachain_system(&self) -> parachain_system::storage::StorageApi {
            parachain_system::storage::StorageApi
        }
        pub fn randomness_collective_flip(
            &self,
        ) -> randomness_collective_flip::storage::StorageApi {
            randomness_collective_flip::storage::StorageApi
        }
        pub fn timestamp(&self) -> timestamp::storage::StorageApi {
            timestamp::storage::StorageApi
        }
        pub fn parachain_info(&self) -> parachain_info::storage::StorageApi {
            parachain_info::storage::StorageApi
        }
        pub fn balances(&self) -> balances::storage::StorageApi {
            balances::storage::StorageApi
        }
        pub fn transaction_payment(&self) -> transaction_payment::storage::StorageApi {
            transaction_payment::storage::StorageApi
        }
        pub fn asset_tx_payment(&self) -> asset_tx_payment::storage::StorageApi {
            asset_tx_payment::storage::StorageApi
        }
        pub fn aura(&self) -> aura::storage::StorageApi {
            aura::storage::StorageApi
        }
        pub fn aura_ext(&self) -> aura_ext::storage::StorageApi {
            aura_ext::storage::StorageApi
        }
        pub fn xcmp_queue(&self) -> xcmp_queue::storage::StorageApi {
            xcmp_queue::storage::StorageApi
        }
        pub fn polkadot_xcm(&self) -> polkadot_xcm::storage::StorageApi {
            polkadot_xcm::storage::StorageApi
        }
        pub fn dmp_queue(&self) -> dmp_queue::storage::StorageApi {
            dmp_queue::storage::StorageApi
        }
        pub fn treasury(&self) -> treasury::storage::StorageApi {
            treasury::storage::StorageApi
        }
        pub fn proxy(&self) -> proxy::storage::StorageApi {
            proxy::storage::StorageApi
        }
        pub fn scheduler(&self) -> scheduler::storage::StorageApi {
            scheduler::storage::StorageApi
        }
        pub fn collective(&self) -> collective::storage::StorageApi {
            collective::storage::StorageApi
        }
        pub fn membership(&self) -> membership::storage::StorageApi {
            membership::storage::StorageApi
        }
        pub fn encointer_scheduler(&self) -> encointer_scheduler::storage::StorageApi {
            encointer_scheduler::storage::StorageApi
        }
        pub fn encointer_ceremonies(&self) -> encointer_ceremonies::storage::StorageApi {
            encointer_ceremonies::storage::StorageApi
        }
        pub fn encointer_communities(&self) -> encointer_communities::storage::StorageApi {
            encointer_communities::storage::StorageApi
        }
        pub fn encointer_balances(&self) -> encointer_balances::storage::StorageApi {
            encointer_balances::storage::StorageApi
        }
        pub fn encointer_bazaar(&self) -> encointer_bazaar::storage::StorageApi {
            encointer_bazaar::storage::StorageApi
        }
    }
    pub struct TransactionApi;
    impl TransactionApi {
        pub fn system(&self) -> system::calls::TransactionApi {
            system::calls::TransactionApi
        }
        pub fn parachain_system(&self) -> parachain_system::calls::TransactionApi {
            parachain_system::calls::TransactionApi
        }
        pub fn timestamp(&self) -> timestamp::calls::TransactionApi {
            timestamp::calls::TransactionApi
        }
        pub fn balances(&self) -> balances::calls::TransactionApi {
            balances::calls::TransactionApi
        }
        pub fn xcmp_queue(&self) -> xcmp_queue::calls::TransactionApi {
            xcmp_queue::calls::TransactionApi
        }
        pub fn polkadot_xcm(&self) -> polkadot_xcm::calls::TransactionApi {
            polkadot_xcm::calls::TransactionApi
        }
        pub fn dmp_queue(&self) -> dmp_queue::calls::TransactionApi {
            dmp_queue::calls::TransactionApi
        }
        pub fn utility(&self) -> utility::calls::TransactionApi {
            utility::calls::TransactionApi
        }
        pub fn treasury(&self) -> treasury::calls::TransactionApi {
            treasury::calls::TransactionApi
        }
        pub fn proxy(&self) -> proxy::calls::TransactionApi {
            proxy::calls::TransactionApi
        }
        pub fn scheduler(&self) -> scheduler::calls::TransactionApi {
            scheduler::calls::TransactionApi
        }
        pub fn collective(&self) -> collective::calls::TransactionApi {
            collective::calls::TransactionApi
        }
        pub fn membership(&self) -> membership::calls::TransactionApi {
            membership::calls::TransactionApi
        }
        pub fn encointer_scheduler(&self) -> encointer_scheduler::calls::TransactionApi {
            encointer_scheduler::calls::TransactionApi
        }
        pub fn encointer_ceremonies(&self) -> encointer_ceremonies::calls::TransactionApi {
            encointer_ceremonies::calls::TransactionApi
        }
        pub fn encointer_communities(&self) -> encointer_communities::calls::TransactionApi {
            encointer_communities::calls::TransactionApi
        }
        pub fn encointer_balances(&self) -> encointer_balances::calls::TransactionApi {
            encointer_balances::calls::TransactionApi
        }
        pub fn encointer_bazaar(&self) -> encointer_bazaar::calls::TransactionApi {
            encointer_bazaar::calls::TransactionApi
        }
    }
    #[doc = r" check whether the Client you are using is aligned with the statically generated codegen."]
    pub fn validate_codegen<T: ::subxt::Config, C: ::subxt::client::OfflineClientT<T>>(
        client: &C,
    ) -> Result<(), ::subxt::error::MetadataError> {
        let runtime_metadata_hash = client.metadata().metadata_hash(&PALLETS);
        if runtime_metadata_hash
            != [
                218u8, 170u8, 110u8, 77u8, 147u8, 190u8, 4u8, 81u8, 54u8, 114u8, 21u8, 19u8, 242u8,
                75u8, 12u8, 9u8, 198u8, 89u8, 82u8, 6u8, 138u8, 240u8, 65u8, 99u8, 236u8, 80u8,
                204u8, 199u8, 255u8, 240u8, 183u8, 249u8,
            ]
        {
            Err(::subxt::error::MetadataError::IncompatibleMetadata)
        } else {
            Ok(())
        }
    }
}
