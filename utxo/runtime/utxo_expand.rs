/// utxo
pub mod utxo {
    use super::Aura;
    use codec::{Decode, Encode};
    use primitives::{H256, H512};
    use rstd::collections::btree_map::BTreeMap;
    use runtime_io::sr25519_verify;
    #[cfg(feature = "std")]
    use serde_derive::{Deserialize, Serialize};
    use sr_primitives::traits::{BlakeTwo256, Hash};
    use support::{
        decl_event, decl_module, decl_storage,
        dispatch::{Result, Vec},
        ensure, StorageMap, StorageValue,
    };
    use system::ensure_signed;
    pub trait Trait: system::Trait {
        type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    }
    /// Representation of UTXO value
    pub type Value = u128;
    /// Representation of UTXO value
    type Signature = H512;
    /// Single transaction to be dispatched
    pub struct Transaction<BlockNumber> {
        /// UTXOs to be used as inputs for current transaction
        pub inputs: Vec<TransactionInput>,
        /// UTXOs to be created as a result of current transaction dispatch
        pub outputs: Vec<TransactionOutput<BlockNumber>>,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_Transaction: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<BlockNumber> _serde::Serialize for Transaction<BlockNumber>
        where
            BlockNumber: _serde::Serialize,
        {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Transaction",
                    false as usize + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "inputs",
                    &self.inputs,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "outputs",
                    &self.outputs,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_Transaction: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de, BlockNumber> _serde::Deserialize<'de> for Transaction<BlockNumber>
        where
            BlockNumber: _serde::Deserialize<'de>,
        {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 2",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "inputs" => _serde::export::Ok(__Field::__field0),
                            "outputs" => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"inputs" => _serde::export::Ok(__Field::__field0),
                            b"outputs" => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de, BlockNumber>
                where
                    BlockNumber: _serde::Deserialize<'de>,
                {
                    marker: _serde::export::PhantomData<Transaction<BlockNumber>>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de, BlockNumber> _serde::de::Visitor<'de> for __Visitor<'de, BlockNumber>
                where
                    BlockNumber: _serde::Deserialize<'de>,
                {
                    type Value = Transaction<BlockNumber>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct Transaction")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            Vec<TransactionInput>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct Transaction with 2 elements",
                                ));
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            Vec<TransactionOutput<BlockNumber>>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct Transaction with 2 elements",
                                ));
                            }
                        };
                        _serde::export::Ok(Transaction {
                            inputs: __field0,
                            outputs: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<Vec<TransactionInput>> =
                            _serde::export::None;
                        let mut __field1: _serde::export::Option<
                            Vec<TransactionOutput<BlockNumber>>,
                        > = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "inputs",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Vec<TransactionInput>,
                                        >(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "outputs",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Vec<TransactionOutput<BlockNumber>>,
                                        >(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("inputs") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("outputs") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(Transaction {
                            inputs: __field0,
                            outputs: __field1,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["inputs", "outputs"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Transaction",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<Transaction<BlockNumber>>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<BlockNumber: ::core::fmt::Debug> ::core::fmt::Debug for Transaction<BlockNumber> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Transaction {
                    inputs: ref __self_0_0,
                    outputs: ref __self_0_1,
                } => {
                    let mut debug_trait_builder = f.debug_struct("Transaction");
                    let _ = debug_trait_builder.field("inputs", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("outputs", &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<BlockNumber: ::core::cmp::PartialEq> ::core::cmp::PartialEq for Transaction<BlockNumber> {
        #[inline]
        fn eq(&self, other: &Transaction<BlockNumber>) -> bool {
            match *other {
                Transaction {
                    inputs: ref __self_1_0,
                    outputs: ref __self_1_1,
                } => match *self {
                    Transaction {
                        inputs: ref __self_0_0,
                        outputs: ref __self_0_1,
                    } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &Transaction<BlockNumber>) -> bool {
            match *other {
                Transaction {
                    inputs: ref __self_1_0,
                    outputs: ref __self_1_1,
                } => match *self {
                    Transaction {
                        inputs: ref __self_0_0,
                        outputs: ref __self_0_1,
                    } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<BlockNumber: ::core::cmp::Eq> ::core::cmp::Eq for Transaction<BlockNumber> {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<Vec<TransactionInput>>;
                let _: ::core::cmp::AssertParamIsEq<Vec<TransactionOutput<BlockNumber>>>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<BlockNumber: ::core::cmp::PartialOrd> ::core::cmp::PartialOrd for Transaction<BlockNumber> {
        #[inline]
        fn partial_cmp(
            &self,
            other: &Transaction<BlockNumber>,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match *other {
                Transaction {
                    inputs: ref __self_1_0,
                    outputs: ref __self_1_1,
                } => match *self {
                    Transaction {
                        inputs: ref __self_0_0,
                        outputs: ref __self_0_1,
                    } => match ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0))
                    {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                            match ::core::cmp::PartialOrd::partial_cmp(
                                &(*__self_0_1),
                                &(*__self_1_1),
                            ) {
                                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                    ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                                }
                                cmp => cmp,
                            }
                        }
                        cmp => cmp,
                    },
                },
            }
        }
        #[inline]
        fn lt(&self, other: &Transaction<BlockNumber>) -> bool {
            match *other {
                Transaction {
                    inputs: ref __self_1_0,
                    outputs: ref __self_1_1,
                } => match *self {
                    Transaction {
                        inputs: ref __self_0_0,
                        outputs: ref __self_0_1,
                    } => {
                        ::core::cmp::Ordering::then_with(
                            ::core::option::Option::unwrap_or(
                                ::core::cmp::PartialOrd::partial_cmp(
                                    &(*__self_0_0),
                                    &(*__self_1_0),
                                ),
                                ::core::cmp::Ordering::Equal,
                            ),
                            || {
                                ::core::option::Option::unwrap_or(
                                    ::core::cmp::PartialOrd::partial_cmp(
                                        &(*__self_0_1),
                                        &(*__self_1_1),
                                    ),
                                    ::core::cmp::Ordering::Greater,
                                )
                            },
                        ) == ::core::cmp::Ordering::Less
                    }
                },
            }
        }
        #[inline]
        fn le(&self, other: &Transaction<BlockNumber>) -> bool {
            match *other {
                Transaction {
                    inputs: ref __self_1_0,
                    outputs: ref __self_1_1,
                } => match *self {
                    Transaction {
                        inputs: ref __self_0_0,
                        outputs: ref __self_0_1,
                    } => {
                        ::core::cmp::Ordering::then_with(
                            ::core::option::Option::unwrap_or(
                                ::core::cmp::PartialOrd::partial_cmp(
                                    &(*__self_0_0),
                                    &(*__self_1_0),
                                ),
                                ::core::cmp::Ordering::Equal,
                            ),
                            || {
                                ::core::option::Option::unwrap_or(
                                    ::core::cmp::PartialOrd::partial_cmp(
                                        &(*__self_0_1),
                                        &(*__self_1_1),
                                    ),
                                    ::core::cmp::Ordering::Greater,
                                )
                            },
                        ) != ::core::cmp::Ordering::Greater
                    }
                },
            }
        }
        #[inline]
        fn gt(&self, other: &Transaction<BlockNumber>) -> bool {
            match *other {
                Transaction {
                    inputs: ref __self_1_0,
                    outputs: ref __self_1_1,
                } => match *self {
                    Transaction {
                        inputs: ref __self_0_0,
                        outputs: ref __self_0_1,
                    } => {
                        ::core::cmp::Ordering::then_with(
                            ::core::option::Option::unwrap_or(
                                ::core::cmp::PartialOrd::partial_cmp(
                                    &(*__self_0_0),
                                    &(*__self_1_0),
                                ),
                                ::core::cmp::Ordering::Equal,
                            ),
                            || {
                                ::core::option::Option::unwrap_or(
                                    ::core::cmp::PartialOrd::partial_cmp(
                                        &(*__self_0_1),
                                        &(*__self_1_1),
                                    ),
                                    ::core::cmp::Ordering::Less,
                                )
                            },
                        ) == ::core::cmp::Ordering::Greater
                    }
                },
            }
        }
        #[inline]
        fn ge(&self, other: &Transaction<BlockNumber>) -> bool {
            match *other {
                Transaction {
                    inputs: ref __self_1_0,
                    outputs: ref __self_1_1,
                } => match *self {
                    Transaction {
                        inputs: ref __self_0_0,
                        outputs: ref __self_0_1,
                    } => {
                        ::core::cmp::Ordering::then_with(
                            ::core::option::Option::unwrap_or(
                                ::core::cmp::PartialOrd::partial_cmp(
                                    &(*__self_0_0),
                                    &(*__self_1_0),
                                ),
                                ::core::cmp::Ordering::Equal,
                            ),
                            || {
                                ::core::option::Option::unwrap_or(
                                    ::core::cmp::PartialOrd::partial_cmp(
                                        &(*__self_0_1),
                                        &(*__self_1_1),
                                    ),
                                    ::core::cmp::Ordering::Less,
                                )
                            },
                        ) != ::core::cmp::Ordering::Less
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<BlockNumber: ::core::cmp::Ord> ::core::cmp::Ord for Transaction<BlockNumber> {
        #[inline]
        fn cmp(&self, other: &Transaction<BlockNumber>) -> ::core::cmp::Ordering {
            match *other {
                Transaction {
                    inputs: ref __self_1_0,
                    outputs: ref __self_1_1,
                } => match *self {
                    Transaction {
                        inputs: ref __self_0_0,
                        outputs: ref __self_0_1,
                    } => match ::core::cmp::Ord::cmp(&(*__self_0_0), &(*__self_1_0)) {
                        ::core::cmp::Ordering::Equal => {
                            match ::core::cmp::Ord::cmp(&(*__self_0_1), &(*__self_1_1)) {
                                ::core::cmp::Ordering::Equal => ::core::cmp::Ordering::Equal,
                                cmp => cmp,
                            }
                        }
                        cmp => cmp,
                    },
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<BlockNumber: ::core::default::Default> ::core::default::Default for Transaction<BlockNumber> {
        #[inline]
        fn default() -> Transaction<BlockNumber> {
            Transaction {
                inputs: ::core::default::Default::default(),
                outputs: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<BlockNumber: ::core::clone::Clone> ::core::clone::Clone for Transaction<BlockNumber> {
        #[inline]
        fn clone(&self) -> Transaction<BlockNumber> {
            match *self {
                Transaction {
                    inputs: ref __self_0_0,
                    outputs: ref __self_0_1,
                } => Transaction {
                    inputs: ::core::clone::Clone::clone(&(*__self_0_0)),
                    outputs: ::core::clone::Clone::clone(&(*__self_0_1)),
                },
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_Transaction: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl<BlockNumber> _parity_scale_codec::Encode for Transaction<BlockNumber>
        where
            Vec<TransactionOutput<BlockNumber>>: _parity_scale_codec::Encode,
            Vec<TransactionOutput<BlockNumber>>: _parity_scale_codec::Encode,
        {
            fn encode_to<EncOut: _parity_scale_codec::Output>(&self, dest: &mut EncOut) {
                dest.push(&self.inputs);
                dest.push(&self.outputs);
            }
        }
        impl<BlockNumber> _parity_scale_codec::EncodeLike for Transaction<BlockNumber>
        where
            Vec<TransactionOutput<BlockNumber>>: _parity_scale_codec::Encode,
            Vec<TransactionOutput<BlockNumber>>: _parity_scale_codec::Encode,
        {
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_Transaction: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl<BlockNumber> _parity_scale_codec::Decode for Transaction<BlockNumber>
        where
            Vec<TransactionOutput<BlockNumber>>: _parity_scale_codec::Decode,
            Vec<TransactionOutput<BlockNumber>>: _parity_scale_codec::Decode,
        {
            fn decode<DecIn: _parity_scale_codec::Input>(
                input: &mut DecIn,
            ) -> core::result::Result<Self, _parity_scale_codec::Error> {
                Ok(Transaction {
                    inputs: {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => return Err("Error decoding field Transaction.inputs".into()),
                            Ok(a) => a,
                        }
                    },
                    outputs: {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => return Err("Error decoding field Transaction.outputs".into()),
                            Ok(a) => a,
                        }
                    },
                })
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<BlockNumber: ::core::hash::Hash> ::core::hash::Hash for Transaction<BlockNumber> {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                Transaction {
                    inputs: ref __self_0_0,
                    outputs: ref __self_0_1,
                } => {
                    ::core::hash::Hash::hash(&(*__self_0_0), state);
                    ::core::hash::Hash::hash(&(*__self_0_1), state)
                }
            }
        }
    }
    /// Single transaction input that refers to one UTXO
    pub struct TransactionInput {
        /// Reference to an UTXO to be spent
        pub parent_output: H256,
        /// Proof that transaction owner is authorized to spend referred UTXO
        pub signature: Signature,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_TransactionInput: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for TransactionInput {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "TransactionInput",
                    false as usize + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "parent_output",
                    &self.parent_output,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "signature",
                    &self.signature,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_TransactionInput: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for TransactionInput {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 2",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "parent_output" => _serde::export::Ok(__Field::__field0),
                            "signature" => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"parent_output" => _serde::export::Ok(__Field::__field0),
                            b"signature" => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<TransactionInput>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = TransactionInput;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct TransactionInput")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<H256>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct TransactionInput with 2 elements",
                                    ));
                                }
                            };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<Signature>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct TransactionInput with 2 elements",
                                ));
                            }
                        };
                        _serde::export::Ok(TransactionInput {
                            parent_output: __field0,
                            signature: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<H256> = _serde::export::None;
                        let mut __field1: _serde::export::Option<Signature> = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "parent_output",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<H256>(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "signature",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Signature>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("parent_output") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("signature") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(TransactionInput {
                            parent_output: __field0,
                            signature: __field1,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["parent_output", "signature"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "TransactionInput",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<TransactionInput>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for TransactionInput {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                TransactionInput {
                    parent_output: ref __self_0_0,
                    signature: ref __self_0_1,
                } => {
                    let mut debug_trait_builder = f.debug_struct("TransactionInput");
                    let _ = debug_trait_builder.field("parent_output", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("signature", &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for TransactionInput {
        #[inline]
        fn eq(&self, other: &TransactionInput) -> bool {
            match *other {
                TransactionInput {
                    parent_output: ref __self_1_0,
                    signature: ref __self_1_1,
                } => match *self {
                    TransactionInput {
                        parent_output: ref __self_0_0,
                        signature: ref __self_0_1,
                    } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &TransactionInput) -> bool {
            match *other {
                TransactionInput {
                    parent_output: ref __self_1_0,
                    signature: ref __self_1_1,
                } => match *self {
                    TransactionInput {
                        parent_output: ref __self_0_0,
                        signature: ref __self_0_1,
                    } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for TransactionInput {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<H256>;
                let _: ::core::cmp::AssertParamIsEq<Signature>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialOrd for TransactionInput {
        #[inline]
        fn partial_cmp(
            &self,
            other: &TransactionInput,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match *other {
                TransactionInput {
                    parent_output: ref __self_1_0,
                    signature: ref __self_1_1,
                } => match *self {
                    TransactionInput {
                        parent_output: ref __self_0_0,
                        signature: ref __self_0_1,
                    } => match ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0))
                    {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                            match ::core::cmp::PartialOrd::partial_cmp(
                                &(*__self_0_1),
                                &(*__self_1_1),
                            ) {
                                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                    ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                                }
                                cmp => cmp,
                            }
                        }
                        cmp => cmp,
                    },
                },
            }
        }
        #[inline]
        fn lt(&self, other: &TransactionInput) -> bool {
            match *other {
                TransactionInput {
                    parent_output: ref __self_1_0,
                    signature: ref __self_1_1,
                } => match *self {
                    TransactionInput {
                        parent_output: ref __self_0_0,
                        signature: ref __self_0_1,
                    } => {
                        ::core::cmp::Ordering::then_with(
                            ::core::option::Option::unwrap_or(
                                ::core::cmp::PartialOrd::partial_cmp(
                                    &(*__self_0_0),
                                    &(*__self_1_0),
                                ),
                                ::core::cmp::Ordering::Equal,
                            ),
                            || {
                                ::core::option::Option::unwrap_or(
                                    ::core::cmp::PartialOrd::partial_cmp(
                                        &(*__self_0_1),
                                        &(*__self_1_1),
                                    ),
                                    ::core::cmp::Ordering::Greater,
                                )
                            },
                        ) == ::core::cmp::Ordering::Less
                    }
                },
            }
        }
        #[inline]
        fn le(&self, other: &TransactionInput) -> bool {
            match *other {
                TransactionInput {
                    parent_output: ref __self_1_0,
                    signature: ref __self_1_1,
                } => match *self {
                    TransactionInput {
                        parent_output: ref __self_0_0,
                        signature: ref __self_0_1,
                    } => {
                        ::core::cmp::Ordering::then_with(
                            ::core::option::Option::unwrap_or(
                                ::core::cmp::PartialOrd::partial_cmp(
                                    &(*__self_0_0),
                                    &(*__self_1_0),
                                ),
                                ::core::cmp::Ordering::Equal,
                            ),
                            || {
                                ::core::option::Option::unwrap_or(
                                    ::core::cmp::PartialOrd::partial_cmp(
                                        &(*__self_0_1),
                                        &(*__self_1_1),
                                    ),
                                    ::core::cmp::Ordering::Greater,
                                )
                            },
                        ) != ::core::cmp::Ordering::Greater
                    }
                },
            }
        }
        #[inline]
        fn gt(&self, other: &TransactionInput) -> bool {
            match *other {
                TransactionInput {
                    parent_output: ref __self_1_0,
                    signature: ref __self_1_1,
                } => match *self {
                    TransactionInput {
                        parent_output: ref __self_0_0,
                        signature: ref __self_0_1,
                    } => {
                        ::core::cmp::Ordering::then_with(
                            ::core::option::Option::unwrap_or(
                                ::core::cmp::PartialOrd::partial_cmp(
                                    &(*__self_0_0),
                                    &(*__self_1_0),
                                ),
                                ::core::cmp::Ordering::Equal,
                            ),
                            || {
                                ::core::option::Option::unwrap_or(
                                    ::core::cmp::PartialOrd::partial_cmp(
                                        &(*__self_0_1),
                                        &(*__self_1_1),
                                    ),
                                    ::core::cmp::Ordering::Less,
                                )
                            },
                        ) == ::core::cmp::Ordering::Greater
                    }
                },
            }
        }
        #[inline]
        fn ge(&self, other: &TransactionInput) -> bool {
            match *other {
                TransactionInput {
                    parent_output: ref __self_1_0,
                    signature: ref __self_1_1,
                } => match *self {
                    TransactionInput {
                        parent_output: ref __self_0_0,
                        signature: ref __self_0_1,
                    } => {
                        ::core::cmp::Ordering::then_with(
                            ::core::option::Option::unwrap_or(
                                ::core::cmp::PartialOrd::partial_cmp(
                                    &(*__self_0_0),
                                    &(*__self_1_0),
                                ),
                                ::core::cmp::Ordering::Equal,
                            ),
                            || {
                                ::core::option::Option::unwrap_or(
                                    ::core::cmp::PartialOrd::partial_cmp(
                                        &(*__self_0_1),
                                        &(*__self_1_1),
                                    ),
                                    ::core::cmp::Ordering::Less,
                                )
                            },
                        ) != ::core::cmp::Ordering::Less
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Ord for TransactionInput {
        #[inline]
        fn cmp(&self, other: &TransactionInput) -> ::core::cmp::Ordering {
            match *other {
                TransactionInput {
                    parent_output: ref __self_1_0,
                    signature: ref __self_1_1,
                } => match *self {
                    TransactionInput {
                        parent_output: ref __self_0_0,
                        signature: ref __self_0_1,
                    } => match ::core::cmp::Ord::cmp(&(*__self_0_0), &(*__self_1_0)) {
                        ::core::cmp::Ordering::Equal => {
                            match ::core::cmp::Ord::cmp(&(*__self_0_1), &(*__self_1_1)) {
                                ::core::cmp::Ordering::Equal => ::core::cmp::Ordering::Equal,
                                cmp => cmp,
                            }
                        }
                        cmp => cmp,
                    },
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for TransactionInput {
        #[inline]
        fn default() -> TransactionInput {
            TransactionInput {
                parent_output: ::core::default::Default::default(),
                signature: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for TransactionInput {
        #[inline]
        fn clone(&self) -> TransactionInput {
            match *self {
                TransactionInput {
                    parent_output: ref __self_0_0,
                    signature: ref __self_0_1,
                } => TransactionInput {
                    parent_output: ::core::clone::Clone::clone(&(*__self_0_0)),
                    signature: ::core::clone::Clone::clone(&(*__self_0_1)),
                },
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_TransactionInput: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl _parity_scale_codec::Encode for TransactionInput {
            fn encode_to<EncOut: _parity_scale_codec::Output>(&self, dest: &mut EncOut) {
                dest.push(&self.parent_output);
                dest.push(&self.signature);
            }
        }
        impl _parity_scale_codec::EncodeLike for TransactionInput {}
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_TransactionInput: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl _parity_scale_codec::Decode for TransactionInput {
            fn decode<DecIn: _parity_scale_codec::Input>(
                input: &mut DecIn,
            ) -> core::result::Result<Self, _parity_scale_codec::Error> {
                Ok(TransactionInput {
                    parent_output: {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err(
                                    "Error decoding field TransactionInput.parent_output".into()
                                )
                            }
                            Ok(a) => a,
                        }
                    },
                    signature: {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err("Error decoding field TransactionInput.signature".into())
                            }
                            Ok(a) => a,
                        }
                    },
                })
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::hash::Hash for TransactionInput {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                TransactionInput {
                    parent_output: ref __self_0_0,
                    signature: ref __self_0_1,
                } => {
                    ::core::hash::Hash::hash(&(*__self_0_0), state);
                    ::core::hash::Hash::hash(&(*__self_0_1), state)
                }
            }
        }
    }
    /// Single transaction output to create upon transaction dispatch
    pub struct TransactionOutput<BlockNumber> {
        /// Value associated with this output
        pub value: Value,
        /// Public key associated with this output. In order to spend this output
        /// owner must provide a proof by hashing whole `TransactionOutput` and
        /// signing it with a corresponding private key.
        pub pubkey: H256,
        /// Unique (potentially random) value used to distinguish this
        /// particular output from others addressed to the same public
        /// key with the same value. Prevents potential replay attacks.
        pub salt: BlockNumber,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_TransactionOutput: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<BlockNumber> _serde::Serialize for TransactionOutput<BlockNumber>
        where
            BlockNumber: _serde::Serialize,
        {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "TransactionOutput",
                    false as usize + 1 + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "value",
                    &self.value,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "pubkey",
                    &self.pubkey,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "salt",
                    &self.salt,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_TransactionOutput: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de, BlockNumber> _serde::Deserialize<'de> for TransactionOutput<BlockNumber>
        where
            BlockNumber: _serde::Deserialize<'de>,
        {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            2u64 => _serde::export::Ok(__Field::__field2),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 3",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "value" => _serde::export::Ok(__Field::__field0),
                            "pubkey" => _serde::export::Ok(__Field::__field1),
                            "salt" => _serde::export::Ok(__Field::__field2),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"value" => _serde::export::Ok(__Field::__field0),
                            b"pubkey" => _serde::export::Ok(__Field::__field1),
                            b"salt" => _serde::export::Ok(__Field::__field2),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de, BlockNumber>
                where
                    BlockNumber: _serde::Deserialize<'de>,
                {
                    marker: _serde::export::PhantomData<TransactionOutput<BlockNumber>>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de, BlockNumber> _serde::de::Visitor<'de> for __Visitor<'de, BlockNumber>
                where
                    BlockNumber: _serde::Deserialize<'de>,
                {
                    type Value = TransactionOutput<BlockNumber>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(
                            __formatter,
                            "struct TransactionOutput",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<Value>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct TransactionOutput with 3 elements",
                                    ));
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<H256>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct TransactionOutput with 3 elements",
                                    ));
                                }
                            };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<BlockNumber>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct TransactionOutput with 3 elements",
                                ));
                            }
                        };
                        _serde::export::Ok(TransactionOutput {
                            value: __field0,
                            pubkey: __field1,
                            salt: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<Value> = _serde::export::None;
                        let mut __field1: _serde::export::Option<H256> = _serde::export::None;
                        let mut __field2: _serde::export::Option<BlockNumber> =
                            _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "value",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Value>(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "pubkey",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<H256>(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "salt",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<BlockNumber>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("value") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("pubkey") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::export::Some(__field2) => __field2,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("salt") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(TransactionOutput {
                            value: __field0,
                            pubkey: __field1,
                            salt: __field2,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["value", "pubkey", "salt"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "TransactionOutput",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<TransactionOutput<BlockNumber>>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<BlockNumber: ::core::fmt::Debug> ::core::fmt::Debug for TransactionOutput<BlockNumber> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                TransactionOutput {
                    value: ref __self_0_0,
                    pubkey: ref __self_0_1,
                    salt: ref __self_0_2,
                } => {
                    let mut debug_trait_builder = f.debug_struct("TransactionOutput");
                    let _ = debug_trait_builder.field("value", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("pubkey", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("salt", &&(*__self_0_2));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<BlockNumber: ::core::cmp::PartialEq> ::core::cmp::PartialEq
        for TransactionOutput<BlockNumber>
    {
        #[inline]
        fn eq(&self, other: &TransactionOutput<BlockNumber>) -> bool {
            match *other {
                TransactionOutput {
                    value: ref __self_1_0,
                    pubkey: ref __self_1_1,
                    salt: ref __self_1_2,
                } => match *self {
                    TransactionOutput {
                        value: ref __self_0_0,
                        pubkey: ref __self_0_1,
                        salt: ref __self_0_2,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &TransactionOutput<BlockNumber>) -> bool {
            match *other {
                TransactionOutput {
                    value: ref __self_1_0,
                    pubkey: ref __self_1_1,
                    salt: ref __self_1_2,
                } => match *self {
                    TransactionOutput {
                        value: ref __self_0_0,
                        pubkey: ref __self_0_1,
                        salt: ref __self_0_2,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<BlockNumber: ::core::cmp::Eq> ::core::cmp::Eq for TransactionOutput<BlockNumber> {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<Value>;
                let _: ::core::cmp::AssertParamIsEq<H256>;
                let _: ::core::cmp::AssertParamIsEq<BlockNumber>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<BlockNumber: ::core::cmp::PartialOrd> ::core::cmp::PartialOrd
        for TransactionOutput<BlockNumber>
    {
        #[inline]
        fn partial_cmp(
            &self,
            other: &TransactionOutput<BlockNumber>,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match *other {
                TransactionOutput {
                    value: ref __self_1_0,
                    pubkey: ref __self_1_1,
                    salt: ref __self_1_2,
                } => match *self {
                    TransactionOutput {
                        value: ref __self_0_0,
                        pubkey: ref __self_0_1,
                        salt: ref __self_0_2,
                    } => match ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0))
                    {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                            match ::core::cmp::PartialOrd::partial_cmp(
                                &(*__self_0_1),
                                &(*__self_1_1),
                            ) {
                                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                    match ::core::cmp::PartialOrd::partial_cmp(
                                        &(*__self_0_2),
                                        &(*__self_1_2),
                                    ) {
                                        ::core::option::Option::Some(
                                            ::core::cmp::Ordering::Equal,
                                        ) => ::core::option::Option::Some(
                                            ::core::cmp::Ordering::Equal,
                                        ),
                                        cmp => cmp,
                                    }
                                }
                                cmp => cmp,
                            }
                        }
                        cmp => cmp,
                    },
                },
            }
        }
        #[inline]
        fn lt(&self, other: &TransactionOutput<BlockNumber>) -> bool {
            match *other {
                TransactionOutput {
                    value: ref __self_1_0,
                    pubkey: ref __self_1_1,
                    salt: ref __self_1_2,
                } => match *self {
                    TransactionOutput {
                        value: ref __self_0_0,
                        pubkey: ref __self_0_1,
                        salt: ref __self_0_2,
                    } => {
                        ::core::cmp::Ordering::then_with(
                            ::core::option::Option::unwrap_or(
                                ::core::cmp::PartialOrd::partial_cmp(
                                    &(*__self_0_0),
                                    &(*__self_1_0),
                                ),
                                ::core::cmp::Ordering::Equal,
                            ),
                            || {
                                ::core::cmp::Ordering::then_with(
                                    ::core::option::Option::unwrap_or(
                                        ::core::cmp::PartialOrd::partial_cmp(
                                            &(*__self_0_1),
                                            &(*__self_1_1),
                                        ),
                                        ::core::cmp::Ordering::Equal,
                                    ),
                                    || {
                                        ::core::option::Option::unwrap_or(
                                            ::core::cmp::PartialOrd::partial_cmp(
                                                &(*__self_0_2),
                                                &(*__self_1_2),
                                            ),
                                            ::core::cmp::Ordering::Greater,
                                        )
                                    },
                                )
                            },
                        ) == ::core::cmp::Ordering::Less
                    }
                },
            }
        }
        #[inline]
        fn le(&self, other: &TransactionOutput<BlockNumber>) -> bool {
            match *other {
                TransactionOutput {
                    value: ref __self_1_0,
                    pubkey: ref __self_1_1,
                    salt: ref __self_1_2,
                } => match *self {
                    TransactionOutput {
                        value: ref __self_0_0,
                        pubkey: ref __self_0_1,
                        salt: ref __self_0_2,
                    } => {
                        ::core::cmp::Ordering::then_with(
                            ::core::option::Option::unwrap_or(
                                ::core::cmp::PartialOrd::partial_cmp(
                                    &(*__self_0_0),
                                    &(*__self_1_0),
                                ),
                                ::core::cmp::Ordering::Equal,
                            ),
                            || {
                                ::core::cmp::Ordering::then_with(
                                    ::core::option::Option::unwrap_or(
                                        ::core::cmp::PartialOrd::partial_cmp(
                                            &(*__self_0_1),
                                            &(*__self_1_1),
                                        ),
                                        ::core::cmp::Ordering::Equal,
                                    ),
                                    || {
                                        ::core::option::Option::unwrap_or(
                                            ::core::cmp::PartialOrd::partial_cmp(
                                                &(*__self_0_2),
                                                &(*__self_1_2),
                                            ),
                                            ::core::cmp::Ordering::Greater,
                                        )
                                    },
                                )
                            },
                        ) != ::core::cmp::Ordering::Greater
                    }
                },
            }
        }
        #[inline]
        fn gt(&self, other: &TransactionOutput<BlockNumber>) -> bool {
            match *other {
                TransactionOutput {
                    value: ref __self_1_0,
                    pubkey: ref __self_1_1,
                    salt: ref __self_1_2,
                } => match *self {
                    TransactionOutput {
                        value: ref __self_0_0,
                        pubkey: ref __self_0_1,
                        salt: ref __self_0_2,
                    } => {
                        ::core::cmp::Ordering::then_with(
                            ::core::option::Option::unwrap_or(
                                ::core::cmp::PartialOrd::partial_cmp(
                                    &(*__self_0_0),
                                    &(*__self_1_0),
                                ),
                                ::core::cmp::Ordering::Equal,
                            ),
                            || {
                                ::core::cmp::Ordering::then_with(
                                    ::core::option::Option::unwrap_or(
                                        ::core::cmp::PartialOrd::partial_cmp(
                                            &(*__self_0_1),
                                            &(*__self_1_1),
                                        ),
                                        ::core::cmp::Ordering::Equal,
                                    ),
                                    || {
                                        ::core::option::Option::unwrap_or(
                                            ::core::cmp::PartialOrd::partial_cmp(
                                                &(*__self_0_2),
                                                &(*__self_1_2),
                                            ),
                                            ::core::cmp::Ordering::Less,
                                        )
                                    },
                                )
                            },
                        ) == ::core::cmp::Ordering::Greater
                    }
                },
            }
        }
        #[inline]
        fn ge(&self, other: &TransactionOutput<BlockNumber>) -> bool {
            match *other {
                TransactionOutput {
                    value: ref __self_1_0,
                    pubkey: ref __self_1_1,
                    salt: ref __self_1_2,
                } => match *self {
                    TransactionOutput {
                        value: ref __self_0_0,
                        pubkey: ref __self_0_1,
                        salt: ref __self_0_2,
                    } => {
                        ::core::cmp::Ordering::then_with(
                            ::core::option::Option::unwrap_or(
                                ::core::cmp::PartialOrd::partial_cmp(
                                    &(*__self_0_0),
                                    &(*__self_1_0),
                                ),
                                ::core::cmp::Ordering::Equal,
                            ),
                            || {
                                ::core::cmp::Ordering::then_with(
                                    ::core::option::Option::unwrap_or(
                                        ::core::cmp::PartialOrd::partial_cmp(
                                            &(*__self_0_1),
                                            &(*__self_1_1),
                                        ),
                                        ::core::cmp::Ordering::Equal,
                                    ),
                                    || {
                                        ::core::option::Option::unwrap_or(
                                            ::core::cmp::PartialOrd::partial_cmp(
                                                &(*__self_0_2),
                                                &(*__self_1_2),
                                            ),
                                            ::core::cmp::Ordering::Less,
                                        )
                                    },
                                )
                            },
                        ) != ::core::cmp::Ordering::Less
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<BlockNumber: ::core::cmp::Ord> ::core::cmp::Ord for TransactionOutput<BlockNumber> {
        #[inline]
        fn cmp(&self, other: &TransactionOutput<BlockNumber>) -> ::core::cmp::Ordering {
            match *other {
                TransactionOutput {
                    value: ref __self_1_0,
                    pubkey: ref __self_1_1,
                    salt: ref __self_1_2,
                } => match *self {
                    TransactionOutput {
                        value: ref __self_0_0,
                        pubkey: ref __self_0_1,
                        salt: ref __self_0_2,
                    } => match ::core::cmp::Ord::cmp(&(*__self_0_0), &(*__self_1_0)) {
                        ::core::cmp::Ordering::Equal => {
                            match ::core::cmp::Ord::cmp(&(*__self_0_1), &(*__self_1_1)) {
                                ::core::cmp::Ordering::Equal => {
                                    match ::core::cmp::Ord::cmp(&(*__self_0_2), &(*__self_1_2)) {
                                        ::core::cmp::Ordering::Equal => {
                                            ::core::cmp::Ordering::Equal
                                        }
                                        cmp => cmp,
                                    }
                                }
                                cmp => cmp,
                            }
                        }
                        cmp => cmp,
                    },
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<BlockNumber: ::core::default::Default> ::core::default::Default
        for TransactionOutput<BlockNumber>
    {
        #[inline]
        fn default() -> TransactionOutput<BlockNumber> {
            TransactionOutput {
                value: ::core::default::Default::default(),
                pubkey: ::core::default::Default::default(),
                salt: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<BlockNumber: ::core::clone::Clone> ::core::clone::Clone for TransactionOutput<BlockNumber> {
        #[inline]
        fn clone(&self) -> TransactionOutput<BlockNumber> {
            match *self {
                TransactionOutput {
                    value: ref __self_0_0,
                    pubkey: ref __self_0_1,
                    salt: ref __self_0_2,
                } => TransactionOutput {
                    value: ::core::clone::Clone::clone(&(*__self_0_0)),
                    pubkey: ::core::clone::Clone::clone(&(*__self_0_1)),
                    salt: ::core::clone::Clone::clone(&(*__self_0_2)),
                },
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_TransactionOutput: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl<BlockNumber> _parity_scale_codec::Encode for TransactionOutput<BlockNumber>
        where
            BlockNumber: _parity_scale_codec::Encode,
            BlockNumber: _parity_scale_codec::Encode,
        {
            fn encode_to<EncOut: _parity_scale_codec::Output>(&self, dest: &mut EncOut) {
                dest.push(&self.value);
                dest.push(&self.pubkey);
                dest.push(&self.salt);
            }
        }
        impl<BlockNumber> _parity_scale_codec::EncodeLike for TransactionOutput<BlockNumber>
        where
            BlockNumber: _parity_scale_codec::Encode,
            BlockNumber: _parity_scale_codec::Encode,
        {
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_TransactionOutput: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl<BlockNumber> _parity_scale_codec::Decode for TransactionOutput<BlockNumber>
        where
            BlockNumber: _parity_scale_codec::Decode,
            BlockNumber: _parity_scale_codec::Decode,
        {
            fn decode<DecIn: _parity_scale_codec::Input>(
                input: &mut DecIn,
            ) -> core::result::Result<Self, _parity_scale_codec::Error> {
                Ok(TransactionOutput {
                    value: {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err("Error decoding field TransactionOutput.value".into())
                            }
                            Ok(a) => a,
                        }
                    },
                    pubkey: {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err("Error decoding field TransactionOutput.pubkey".into())
                            }
                            Ok(a) => a,
                        }
                    },
                    salt: {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err("Error decoding field TransactionOutput.salt".into())
                            }
                            Ok(a) => a,
                        }
                    },
                })
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<BlockNumber: ::core::hash::Hash> ::core::hash::Hash for TransactionOutput<BlockNumber> {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                TransactionOutput {
                    value: ref __self_0_0,
                    pubkey: ref __self_0_1,
                    salt: ref __self_0_2,
                } => {
                    ::core::hash::Hash::hash(&(*__self_0_0), state);
                    ::core::hash::Hash::hash(&(*__self_0_1), state);
                    ::core::hash::Hash::hash(&(*__self_0_2), state)
                }
            }
        }
    }
    /// A UTXO can be locked indefinitely or until a certain block height
    pub enum LockStatus<BlockNumber> {
        Locked,
        LockedUntil(BlockNumber),
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_LockStatus: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<BlockNumber> _serde::Serialize for LockStatus<BlockNumber>
        where
            BlockNumber: _serde::Serialize,
        {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    LockStatus::Locked => _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "LockStatus",
                        0u32,
                        "Locked",
                    ),
                    LockStatus::LockedUntil(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "LockStatus",
                            1u32,
                            "LockedUntil",
                            __field0,
                        )
                    }
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_LockStatus: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de, BlockNumber> _serde::Deserialize<'de> for LockStatus<BlockNumber>
        where
            BlockNumber: _serde::Deserialize<'de>,
        {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "variant identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"variant index 0 <= i < 2",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "Locked" => _serde::export::Ok(__Field::__field0),
                            "LockedUntil" => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Err(_serde::de::Error::unknown_variant(
                                __value, VARIANTS,
                            )),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"Locked" => _serde::export::Ok(__Field::__field0),
                            b"LockedUntil" => _serde::export::Ok(__Field::__field1),
                            _ => {
                                let __value = &_serde::export::from_utf8_lossy(__value);
                                _serde::export::Err(_serde::de::Error::unknown_variant(
                                    __value, VARIANTS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de, BlockNumber>
                where
                    BlockNumber: _serde::Deserialize<'de>,
                {
                    marker: _serde::export::PhantomData<LockStatus<BlockNumber>>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de, BlockNumber> _serde::de::Visitor<'de> for __Visitor<'de, BlockNumber>
                where
                    BlockNumber: _serde::Deserialize<'de>,
                {
                    type Value = LockStatus<BlockNumber>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "enum LockStatus")
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match match _serde::de::EnumAccess::variant(__data) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            (__Field::__field0, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                _serde::export::Ok(LockStatus::Locked)
                            }
                            (__Field::__field1, __variant) => _serde::export::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<BlockNumber>(
                                    __variant,
                                ),
                                LockStatus::LockedUntil,
                            ),
                        }
                    }
                }
                const VARIANTS: &'static [&'static str] = &["Locked", "LockedUntil"];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "LockStatus",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<LockStatus<BlockNumber>>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<BlockNumber: ::core::fmt::Debug> ::core::fmt::Debug for LockStatus<BlockNumber> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&LockStatus::Locked,) => {
                    let mut debug_trait_builder = f.debug_tuple("Locked");
                    debug_trait_builder.finish()
                }
                (&LockStatus::LockedUntil(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("LockedUntil");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<BlockNumber: ::core::cmp::PartialEq> ::core::cmp::PartialEq for LockStatus<BlockNumber> {
        #[inline]
        fn eq(&self, other: &LockStatus<BlockNumber>) -> bool {
            {
                let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi =
                    unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (
                            &LockStatus::LockedUntil(ref __self_0),
                            &LockStatus::LockedUntil(ref __arg_1_0),
                        ) => (*__self_0) == (*__arg_1_0),
                        _ => true,
                    }
                } else {
                    false
                }
            }
        }
        #[inline]
        fn ne(&self, other: &LockStatus<BlockNumber>) -> bool {
            {
                let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi =
                    unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (
                            &LockStatus::LockedUntil(ref __self_0),
                            &LockStatus::LockedUntil(ref __arg_1_0),
                        ) => (*__self_0) != (*__arg_1_0),
                        _ => false,
                    }
                } else {
                    true
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<BlockNumber: ::core::cmp::Eq> ::core::cmp::Eq for LockStatus<BlockNumber> {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<BlockNumber>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<BlockNumber: ::core::cmp::PartialOrd> ::core::cmp::PartialOrd for LockStatus<BlockNumber> {
        #[inline]
        fn partial_cmp(
            &self,
            other: &LockStatus<BlockNumber>,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            {
                let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi =
                    unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (
                            &LockStatus::LockedUntil(ref __self_0),
                            &LockStatus::LockedUntil(ref __arg_1_0),
                        ) => {
                            match ::core::cmp::PartialOrd::partial_cmp(&(*__self_0), &(*__arg_1_0))
                            {
                                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                    ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                                }
                                cmp => cmp,
                            }
                        }
                        _ => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
                    }
                } else {
                    __self_vi.partial_cmp(&__arg_1_vi)
                }
            }
        }
        #[inline]
        fn lt(&self, other: &LockStatus<BlockNumber>) -> bool {
            {
                let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi =
                    unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (
                            &LockStatus::LockedUntil(ref __self_0),
                            &LockStatus::LockedUntil(ref __arg_1_0),
                        ) => {
                            ::core::option::Option::unwrap_or(
                                ::core::cmp::PartialOrd::partial_cmp(&(*__self_0), &(*__arg_1_0)),
                                ::core::cmp::Ordering::Greater,
                            ) == ::core::cmp::Ordering::Less
                        }
                        _ => false,
                    }
                } else {
                    __self_vi.lt(&__arg_1_vi)
                }
            }
        }
        #[inline]
        fn le(&self, other: &LockStatus<BlockNumber>) -> bool {
            {
                let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi =
                    unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (
                            &LockStatus::LockedUntil(ref __self_0),
                            &LockStatus::LockedUntil(ref __arg_1_0),
                        ) => {
                            ::core::option::Option::unwrap_or(
                                ::core::cmp::PartialOrd::partial_cmp(&(*__self_0), &(*__arg_1_0)),
                                ::core::cmp::Ordering::Greater,
                            ) != ::core::cmp::Ordering::Greater
                        }
                        _ => true,
                    }
                } else {
                    __self_vi.le(&__arg_1_vi)
                }
            }
        }
        #[inline]
        fn gt(&self, other: &LockStatus<BlockNumber>) -> bool {
            {
                let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi =
                    unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (
                            &LockStatus::LockedUntil(ref __self_0),
                            &LockStatus::LockedUntil(ref __arg_1_0),
                        ) => {
                            ::core::option::Option::unwrap_or(
                                ::core::cmp::PartialOrd::partial_cmp(&(*__self_0), &(*__arg_1_0)),
                                ::core::cmp::Ordering::Less,
                            ) == ::core::cmp::Ordering::Greater
                        }
                        _ => false,
                    }
                } else {
                    __self_vi.gt(&__arg_1_vi)
                }
            }
        }
        #[inline]
        fn ge(&self, other: &LockStatus<BlockNumber>) -> bool {
            {
                let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi =
                    unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (
                            &LockStatus::LockedUntil(ref __self_0),
                            &LockStatus::LockedUntil(ref __arg_1_0),
                        ) => {
                            ::core::option::Option::unwrap_or(
                                ::core::cmp::PartialOrd::partial_cmp(&(*__self_0), &(*__arg_1_0)),
                                ::core::cmp::Ordering::Less,
                            ) != ::core::cmp::Ordering::Less
                        }
                        _ => true,
                    }
                } else {
                    __self_vi.ge(&__arg_1_vi)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<BlockNumber: ::core::cmp::Ord> ::core::cmp::Ord for LockStatus<BlockNumber> {
        #[inline]
        fn cmp(&self, other: &LockStatus<BlockNumber>) -> ::core::cmp::Ordering {
            {
                let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi =
                    unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (
                            &LockStatus::LockedUntil(ref __self_0),
                            &LockStatus::LockedUntil(ref __arg_1_0),
                        ) => match ::core::cmp::Ord::cmp(&(*__self_0), &(*__arg_1_0)) {
                            ::core::cmp::Ordering::Equal => ::core::cmp::Ordering::Equal,
                            cmp => cmp,
                        },
                        _ => ::core::cmp::Ordering::Equal,
                    }
                } else {
                    __self_vi.cmp(&__arg_1_vi)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<BlockNumber: ::core::clone::Clone> ::core::clone::Clone for LockStatus<BlockNumber> {
        #[inline]
        fn clone(&self) -> LockStatus<BlockNumber> {
            match (&*self,) {
                (&LockStatus::Locked,) => LockStatus::Locked,
                (&LockStatus::LockedUntil(ref __self_0),) => {
                    LockStatus::LockedUntil(::core::clone::Clone::clone(&(*__self_0)))
                }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_LockStatus: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl<BlockNumber> _parity_scale_codec::Encode for LockStatus<BlockNumber>
        where
            BlockNumber: _parity_scale_codec::Encode,
            BlockNumber: _parity_scale_codec::Encode,
        {
            fn encode_to<EncOut: _parity_scale_codec::Output>(&self, dest: &mut EncOut) {
                match *self {
                    LockStatus::Locked => {
                        dest.push_byte(0usize as u8);
                    }
                    LockStatus::LockedUntil(ref aa) => {
                        dest.push_byte(1usize as u8);
                        dest.push(aa);
                    }
                    _ => (),
                }
            }
        }
        impl<BlockNumber> _parity_scale_codec::EncodeLike for LockStatus<BlockNumber>
        where
            BlockNumber: _parity_scale_codec::Encode,
            BlockNumber: _parity_scale_codec::Encode,
        {
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_LockStatus: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl<BlockNumber> _parity_scale_codec::Decode for LockStatus<BlockNumber>
        where
            BlockNumber: _parity_scale_codec::Decode,
            BlockNumber: _parity_scale_codec::Decode,
        {
            fn decode<DecIn: _parity_scale_codec::Input>(
                input: &mut DecIn,
            ) -> core::result::Result<Self, _parity_scale_codec::Error> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => Ok(LockStatus::Locked),
                    x if x == 1usize as u8 => Ok(LockStatus::LockedUntil({
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err(
                                    "Error decoding field LockStatus :: LockedUntil.0".into()
                                )
                            }
                            Ok(a) => a,
                        }
                    })),
                    x => Err("No such variant in enum LockStatus".into()),
                }
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<BlockNumber: ::core::hash::Hash> ::core::hash::Hash for LockStatus<BlockNumber> {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match (&*self,) {
                (&LockStatus::LockedUntil(ref __self_0),) => {
                    ::core::hash::Hash::hash(
                        &unsafe { ::core::intrinsics::discriminant_value(self) },
                        state,
                    );
                    ::core::hash::Hash::hash(&(*__self_0), state)
                }
                _ => ::core::hash::Hash::hash(
                    &unsafe { ::core::intrinsics::discriminant_value(self) },
                    state,
                ),
            }
        }
    }
    #[doc(hidden)]
    mod sr_api_hidden_includes_decl_storage {
        pub extern crate support as hidden_include;
    }
    /// Tag a type as an instance of a module.
    ///
    /// Defines storage prefixes, they must be unique.
    #[doc(hidden)]
    pub trait __GeneratedInstantiable: 'static {
        /// The prefix used by any storage entry of an instance.
        const PREFIX: &'static str;
        const PREFIX_FOR_UnspentOutputs: &'static str;
        const PREFIX_FOR_LeftoverTotal: &'static str;
        const PREFIX_FOR_LockedOutputs: &'static str;
    }
    #[doc(hidden)]
    pub struct __InherentHiddenInstance;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for __InherentHiddenInstance {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                __InherentHiddenInstance => {
                    let mut debug_trait_builder = f.debug_tuple("__InherentHiddenInstance");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for __InherentHiddenInstance {
        #[inline]
        fn clone(&self) -> __InherentHiddenInstance {
            match *self {
                __InherentHiddenInstance => __InherentHiddenInstance,
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for __InherentHiddenInstance {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for __InherentHiddenInstance {
        #[inline]
        fn eq(&self, other: &__InherentHiddenInstance) -> bool {
            match *other {
                __InherentHiddenInstance => match *self {
                    __InherentHiddenInstance => true,
                },
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR___InherentHiddenInstance: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl _parity_scale_codec::Encode for __InherentHiddenInstance {
            fn encode_to<EncOut: _parity_scale_codec::Output>(&self, dest: &mut EncOut) {
                drop(dest);
            }
        }
        impl _parity_scale_codec::EncodeLike for __InherentHiddenInstance {}
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR___InherentHiddenInstance: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl _parity_scale_codec::Decode for __InherentHiddenInstance {
            fn decode<DecIn: _parity_scale_codec::Input>(
                input: &mut DecIn,
            ) -> core::result::Result<Self, _parity_scale_codec::Error> {
                drop(input);
                Ok(__InherentHiddenInstance)
            }
        }
    };
    impl __GeneratedInstantiable for __InherentHiddenInstance {
        const PREFIX: &'static str = "Utxo";
        const PREFIX_FOR_UnspentOutputs: &'static str = "Utxo UnspentOutputs";
        const PREFIX_FOR_LeftoverTotal: &'static str = "Utxo LeftoverTotal";
        const PREFIX_FOR_LockedOutputs: &'static str = "Utxo LockedOutputs";
    }
    /// All valid unspent transaction outputs are stored in this map.
    /// Initial set of UTXO is populated from the list stored in genesis.
    struct UnspentOutputs<T: Trait>(
        self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T,)>,
    );
    impl < T : Trait > self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < H256 , TransactionOutput < T :: BlockNumber > > for UnspentOutputs < T > { type Query = Option < TransactionOutput < T :: BlockNumber > > ; type Hasher = self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 ; # [ doc = r" Get the prefix key in storage." ] fn prefix ( ) -> & 'static [ u8 ] { "Utxo UnspentOutputs" . as_bytes ( ) } # [ doc = r" Get the storage key used to fetch a value corresponding to a specific key." ] fn key_for ( x : & H256 ) -> self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: vec :: Vec < u8 > { let mut key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < H256 , TransactionOutput < T :: BlockNumber > > > :: prefix ( ) . to_vec ( ) ; self :: sr_api_hidden_includes_decl_storage :: hidden_include :: codec :: Encode :: encode_to ( x , & mut key ) ; key } # [ doc = r" Load the value associated with the given key from the map." ] fn get < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & H256 , storage : & S ) -> Self :: Query { let key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < H256 , TransactionOutput < T :: BlockNumber > > > :: key_for ( key ) ; storage . get ( & key [ .. ] ) . or_else ( | | Default :: default ( ) ) } # [ doc = r" Take the value, reading and removing it." ] fn take < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & H256 , storage : & mut S ) -> Self :: Query { let key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < H256 , TransactionOutput < T :: BlockNumber > > > :: key_for ( key ) ; storage . take ( & key [ .. ] ) . or_else ( | | Default :: default ( ) ) } # [ doc = r" Mutate the value under a key" ] fn mutate < R , F , S > ( key : & H256 , f : F , storage : & mut S ) -> R where F : FnOnce ( & mut Self :: Query ) -> R , S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > { let mut val = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < H256 , TransactionOutput < T :: BlockNumber > > > :: get ( key , storage ) ; let ret = f ( & mut val ) ; match val { Some ( ref val ) => < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < H256 , TransactionOutput < T :: BlockNumber > > > :: insert ( key , & val , storage ) , None => < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < H256 , TransactionOutput < T :: BlockNumber > > > :: remove ( key , storage ) , } ; ret } }
    impl < T : Trait > self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: AppendableStorageMap < H256 , TransactionOutput < T :: BlockNumber > > for UnspentOutputs < T > { }
    /// Total leftover value to be redistributed among authorities.
    /// It is accumulated during block execution and then drained
    /// on block finalization.
    pub struct LeftoverTotal(
        self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<()>,
    );
    impl self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageValue < Value > for LeftoverTotal < > { type Query = Value ; # [ doc = r" Get the storage key." ] fn key ( ) -> & 'static [ u8 ] { "Utxo LeftoverTotal" . as_bytes ( ) } # [ doc = r" Load the value from the provided storage instance." ] fn get < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Twox128 > > ( storage : & S ) -> Self :: Query { storage . get ( < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageValue < Value > > :: key ( ) ) . unwrap_or_else ( | | Default :: default ( ) ) } # [ doc = r" Take a value from storage, removing it afterwards." ] fn take < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Twox128 > > ( storage : & mut S ) -> Self :: Query { storage . take ( < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageValue < Value > > :: key ( ) ) . unwrap_or_else ( | | Default :: default ( ) ) } # [ doc = r" Mutate the value under a key." ] fn mutate < R , F , S > ( f : F , storage : & mut S ) -> R where F : FnOnce ( & mut Self :: Query ) -> R , S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Twox128 > { let mut val = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageValue < Value > > :: get ( storage ) ; let ret = f ( & mut val ) ; < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageValue < Value > > :: put ( & val , storage ) ; ret } }
    /// All UTXO that are locked
    struct LockedOutputs<T: Trait>(
        self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T,)>,
    );
    impl < T : Trait > self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < H256 , LockStatus < T :: BlockNumber > > for LockedOutputs < T > { type Query = Option < LockStatus < T :: BlockNumber > > ; type Hasher = self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 ; # [ doc = r" Get the prefix key in storage." ] fn prefix ( ) -> & 'static [ u8 ] { "Utxo LockedOutputs" . as_bytes ( ) } # [ doc = r" Get the storage key used to fetch a value corresponding to a specific key." ] fn key_for ( x : & H256 ) -> self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: vec :: Vec < u8 > { let mut key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < H256 , LockStatus < T :: BlockNumber > > > :: prefix ( ) . to_vec ( ) ; self :: sr_api_hidden_includes_decl_storage :: hidden_include :: codec :: Encode :: encode_to ( x , & mut key ) ; key } # [ doc = r" Load the value associated with the given key from the map." ] fn get < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & H256 , storage : & S ) -> Self :: Query { let key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < H256 , LockStatus < T :: BlockNumber > > > :: key_for ( key ) ; storage . get ( & key [ .. ] ) . or_else ( | | Default :: default ( ) ) } # [ doc = r" Take the value, reading and removing it." ] fn take < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & H256 , storage : & mut S ) -> Self :: Query { let key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < H256 , LockStatus < T :: BlockNumber > > > :: key_for ( key ) ; storage . take ( & key [ .. ] ) . or_else ( | | Default :: default ( ) ) } # [ doc = r" Mutate the value under a key" ] fn mutate < R , F , S > ( key : & H256 , f : F , storage : & mut S ) -> R where F : FnOnce ( & mut Self :: Query ) -> R , S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > { let mut val = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < H256 , LockStatus < T :: BlockNumber > > > :: get ( key , storage ) ; let ret = f ( & mut val ) ; match val { Some ( ref val ) => < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < H256 , LockStatus < T :: BlockNumber > > > :: insert ( key , & val , storage ) , None => < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < H256 , LockStatus < T :: BlockNumber > > > :: remove ( key , storage ) , } ; ret } }
    impl < T : Trait > self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: AppendableStorageMap < H256 , LockStatus < T :: BlockNumber > > for LockedOutputs < T > { }
    trait Store {
        type UnspentOutputs;
        type LeftoverTotal;
        type LockedOutputs;
    }
    #[doc(hidden)]
    pub struct __GetByteStructUnspentOutputs<T>(
        pub  self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<
            (T),
        >,
    );
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_UnspentOutputs:
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
            self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>,
        > =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl<T: Trait> self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
        for __GetByteStructUnspentOutputs<T>
    {
        fn default_byte(
            &self,
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_UnspentOutputs
                .get_or_init(|| {
                    let def_val: Option<TransactionOutput<T::BlockNumber>> = Default::default();
                    <Option<TransactionOutput<T::BlockNumber>> as Encode>::encode(&def_val)
                })
                .clone()
        }
    }
    unsafe impl<T: Trait> Send for __GetByteStructUnspentOutputs<T> {}
    unsafe impl<T: Trait> Sync for __GetByteStructUnspentOutputs<T> {}
    #[doc(hidden)]
    pub struct __GetByteStructLeftoverTotal<T>(
        pub  self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<
            (T),
        >,
    );
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_LeftoverTotal:
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
            self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>,
        > =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl<T: Trait> self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
        for __GetByteStructLeftoverTotal<T>
    {
        fn default_byte(
            &self,
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_LeftoverTotal
                .get_or_init(|| {
                    let def_val: Value = Default::default();
                    <Value as Encode>::encode(&def_val)
                })
                .clone()
        }
    }
    unsafe impl<T: Trait> Send for __GetByteStructLeftoverTotal<T> {}
    unsafe impl<T: Trait> Sync for __GetByteStructLeftoverTotal<T> {}
    #[doc(hidden)]
    pub struct __GetByteStructLockedOutputs<T>(
        pub  self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<
            (T),
        >,
    );
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_LockedOutputs:
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
            self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>,
        > =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl<T: Trait> self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
        for __GetByteStructLockedOutputs<T>
    {
        fn default_byte(
            &self,
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_LockedOutputs
                .get_or_init(|| {
                    let def_val: Option<LockStatus<T::BlockNumber>> = Default::default();
                    <Option<LockStatus<T::BlockNumber>> as Encode>::encode(&def_val)
                })
                .clone()
        }
    }
    unsafe impl<T: Trait> Send for __GetByteStructLockedOutputs<T> {}
    unsafe impl<T: Trait> Sync for __GetByteStructLockedOutputs<T> {}
    impl<T: Trait> Store for Module<T> {
        type UnspentOutputs = UnspentOutputs<T>;
        type LeftoverTotal = LeftoverTotal;
        type LockedOutputs = LockedOutputs<T>;
    }
    impl<T: 'static + Trait> Module<T> {
        /// Total leftover value to be redistributed among authorities.
        /// It is accumulated during block execution and then drained
        /// on block finalization.
        pub fn leftover_total() -> Value {
            < LeftoverTotal < > as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageValue < Value > > :: get ( & self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: RuntimeStorage )
        }
        #[doc(hidden)]
        pub fn storage_metadata(
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata
        {
            self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageMetadata { prefix : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "Utxo" ) , entries : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryMetadata { name : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "UnspentOutputs" ) , modifier : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryModifier :: Optional , ty : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryType :: Map { hasher : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_256 , key : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "H256" ) , value : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "TransactionOutput<T::BlockNumber>" ) , is_linked : false , } , default : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructUnspentOutputs :: < T > ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: marker :: PhantomData ) ) ) , documentation : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ " All valid unspent transaction outputs are stored in this map." , " Initial set of UTXO is populated from the list stored in genesis." ] ) , } , self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryMetadata { name : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "LeftoverTotal" ) , modifier : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryModifier :: Default , ty : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryType :: Plain ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "Value" ) ) , default : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructLeftoverTotal :: < T > ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: marker :: PhantomData ) ) ) , documentation : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ " Total leftover value to be redistributed among authorities." , " It is accumulated during block execution and then drained" , " on block finalization." ] ) , } , self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryMetadata { name : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "LockedOutputs" ) , modifier : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryModifier :: Optional , ty : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryType :: Map { hasher : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_256 , key : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "H256" ) , value : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "LockStatus<T::BlockNumber>" ) , is_linked : false , } , default : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructLockedOutputs :: < T > ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: marker :: PhantomData ) ) ) , documentation : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ " All UTXO that are locked" ] ) , } ] [ .. ] ) , }
        }
    }
    #[cfg(feature = "std")]
    #[serde(rename_all = "camelCase")]
    #[serde(deny_unknown_fields)]
    #[serde(bound(
        serialize = "Vec < TransactionOutput < T :: BlockNumber > > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, "
    ))]
    #[serde(bound(
        deserialize = "Vec < TransactionOutput < T :: BlockNumber > > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, "
    ))]
    pub struct GenesisConfig<T: Trait> {
        pub initial_utxo: Vec<TransactionOutput<T::BlockNumber>>,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_GenesisConfig: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<T: Trait> _serde::Serialize for GenesisConfig<T>
        where
            Vec<TransactionOutput<T::BlockNumber>>:
                self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
        {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "GenesisConfig",
                    false as usize + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "initialUtxo",
                    &self.initial_utxo,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_GenesisConfig: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl < 'de , T : Trait > _serde :: Deserialize < 'de > for GenesisConfig < T > where Vec < TransactionOutput < T :: BlockNumber > > : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: serde :: de :: DeserializeOwned { fn deserialize < __D > ( __deserializer : __D ) -> _serde :: export :: Result < Self , __D :: Error > where __D : _serde :: Deserializer < 'de > { # [ allow ( non_camel_case_types ) ] enum __Field { __field0 , } struct __FieldVisitor ; impl < 'de > _serde :: de :: Visitor < 'de > for __FieldVisitor { type Value = __Field ; fn expecting ( & self , __formatter : & mut _serde :: export :: Formatter ) -> _serde :: export :: fmt :: Result { _serde :: export :: Formatter :: write_str ( __formatter , "field identifier" ) } fn visit_u64 < __E > ( self , __value : u64 ) -> _serde :: export :: Result < Self :: Value , __E > where __E : _serde :: de :: Error { match __value { 0u64 => _serde :: export :: Ok ( __Field :: __field0 ) , _ => _serde :: export :: Err ( _serde :: de :: Error :: invalid_value ( _serde :: de :: Unexpected :: Unsigned ( __value ) , & "field index 0 <= i < 1" ) ) , } } fn visit_str < __E > ( self , __value : & str ) -> _serde :: export :: Result < Self :: Value , __E > where __E : _serde :: de :: Error { match __value { "initialUtxo" => _serde :: export :: Ok ( __Field :: __field0 ) , _ => { _serde :: export :: Err ( _serde :: de :: Error :: unknown_field ( __value , FIELDS ) ) } } } fn visit_bytes < __E > ( self , __value : & [ u8 ] ) -> _serde :: export :: Result < Self :: Value , __E > where __E : _serde :: de :: Error { match __value { b"initialUtxo" => _serde :: export :: Ok ( __Field :: __field0 ) , _ => { let __value = & _serde :: export :: from_utf8_lossy ( __value ) ; _serde :: export :: Err ( _serde :: de :: Error :: unknown_field ( __value , FIELDS ) ) } } } } impl < 'de > _serde :: Deserialize < 'de > for __Field { # [ inline ] fn deserialize < __D > ( __deserializer : __D ) -> _serde :: export :: Result < Self , __D :: Error > where __D : _serde :: Deserializer < 'de > { _serde :: Deserializer :: deserialize_identifier ( __deserializer , __FieldVisitor ) } } struct __Visitor < 'de , T : Trait > where Vec < TransactionOutput < T :: BlockNumber > > : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: serde :: de :: DeserializeOwned { marker : _serde :: export :: PhantomData < GenesisConfig < T > > , lifetime : _serde :: export :: PhantomData < & 'de ( ) > , } impl < 'de , T : Trait > _serde :: de :: Visitor < 'de > for __Visitor < 'de , T > where Vec < TransactionOutput < T :: BlockNumber > > : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: serde :: de :: DeserializeOwned { type Value = GenesisConfig < T > ; fn expecting ( & self , __formatter : & mut _serde :: export :: Formatter ) -> _serde :: export :: fmt :: Result { _serde :: export :: Formatter :: write_str ( __formatter , "struct GenesisConfig" ) } # [ inline ] fn visit_seq < __A > ( self , mut __seq : __A ) -> _serde :: export :: Result < Self :: Value , __A :: Error > where __A : _serde :: de :: SeqAccess < 'de > { let __field0 = match match _serde :: de :: SeqAccess :: next_element :: < Vec < TransactionOutput < T :: BlockNumber > > > ( & mut __seq ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } { _serde :: export :: Some ( __value ) => __value , _serde :: export :: None => { return _serde :: export :: Err ( _serde :: de :: Error :: invalid_length ( 0usize , & "struct GenesisConfig with 1 element" ) ) ; } } ; _serde :: export :: Ok ( GenesisConfig { initial_utxo : __field0 , } ) } # [ inline ] fn visit_map < __A > ( self , mut __map : __A ) -> _serde :: export :: Result < Self :: Value , __A :: Error > where __A : _serde :: de :: MapAccess < 'de > { let mut __field0 : _serde :: export :: Option < Vec < TransactionOutput < T :: BlockNumber > > > = _serde :: export :: None ; while let _serde :: export :: Some ( __key ) = match _serde :: de :: MapAccess :: next_key :: < __Field > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } { match __key { __Field :: __field0 => { if _serde :: export :: Option :: is_some ( & __field0 ) { return _serde :: export :: Err ( < __A :: Error as _serde :: de :: Error > :: duplicate_field ( "initialUtxo" ) ) ; } __field0 = _serde :: export :: Some ( match _serde :: de :: MapAccess :: next_value :: < Vec < TransactionOutput < T :: BlockNumber > > > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ) ; } } } let __field0 = match __field0 { _serde :: export :: Some ( __field0 ) => __field0 , _serde :: export :: None => match _serde :: private :: de :: missing_field ( "initialUtxo" ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } , } ; _serde :: export :: Ok ( GenesisConfig { initial_utxo : __field0 , } ) } } const FIELDS : & 'static [ & 'static str ] = & [ "initialUtxo" ] ; _serde :: Deserializer :: deserialize_struct ( __deserializer , "GenesisConfig" , FIELDS , __Visitor { marker : _serde :: export :: PhantomData :: < GenesisConfig < T > > , lifetime : _serde :: export :: PhantomData , } ) } }
    };
    #[cfg(feature = "std")]
    impl<T: Trait> Default for GenesisConfig<T> {
        fn default() -> Self {
            GenesisConfig {
                initial_utxo: Default::default(),
            }
        }
    }
    #[cfg(feature = "std")]
    impl<T: Trait> GenesisConfig<T> {
        pub fn build_storage ( self ) -> std :: result :: Result < ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: sr_primitives :: StorageOverlay , self :: sr_api_hidden_includes_decl_storage :: hidden_include :: sr_primitives :: ChildrenStorageOverlay ) , String >{
            let mut storage = (Default::default(), Default::default());
            self.assimilate_storage(&mut storage)?;
            Ok(storage)
        }
        /// Assimilate the storage for this module into pre-existing overlays.
        pub fn assimilate_storage(
            self,
            tuple_storage : & mut ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: sr_primitives :: StorageOverlay , self :: sr_api_hidden_includes_decl_storage :: hidden_include :: sr_primitives :: ChildrenStorageOverlay ),
        ) -> std::result::Result<(), String> {
            let storage = &mut tuple_storage.0;
            {
                let data = (|config: &GenesisConfig<T>| {
                    config
                        .initial_utxo
                        .iter()
                        .cloned()
                        .map(|u| (BlakeTwo256::hash_of(&u), u))
                        .collect::<Vec<_>>()
                })(&self);
                data . into_iter ( ) . for_each ( | ( k , v ) | { < UnspentOutputs < T > as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < H256 , TransactionOutput < T :: BlockNumber > > > :: insert ( & k , & v , storage ) ; } ) ;
            }
            (|_, _| {})(tuple_storage, &self);
            Ok(())
        }
    }
    #[cfg(feature = "std")]
    impl < T : Trait , __GeneratedInstance : __GeneratedInstantiable > self :: sr_api_hidden_includes_decl_storage :: hidden_include :: sr_primitives :: BuildModuleGenesisStorage < T , __GeneratedInstance > for GenesisConfig < T > { fn build_module_genesis_storage ( self , storage : & mut ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: sr_primitives :: StorageOverlay , self :: sr_api_hidden_includes_decl_storage :: hidden_include :: sr_primitives :: ChildrenStorageOverlay ) ) -> std :: result :: Result < ( ) , String > { self . assimilate_storage :: < > ( storage ) } }
    pub struct Module<T: Trait>(::srml_support::rstd::marker::PhantomData<(T)>);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::clone::Clone + Trait> ::core::clone::Clone for Module<T> {
        #[inline]
        fn clone(&self) -> Module<T> {
            match *self {
                Module(ref __self_0_0) => Module(::core::clone::Clone::clone(&(*__self_0_0))),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::marker::Copy + Trait> ::core::marker::Copy for Module<T> {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::cmp::PartialEq + Trait> ::core::cmp::PartialEq for Module<T> {
        #[inline]
        fn eq(&self, other: &Module<T>) -> bool {
            match *other {
                Module(ref __self_1_0) => match *self {
                    Module(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &Module<T>) -> bool {
            match *other {
                Module(ref __self_1_0) => match *self {
                    Module(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::cmp::Eq + Trait> ::core::cmp::Eq for Module<T> {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<
                    ::srml_support::rstd::marker::PhantomData<(T)>,
                >;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::fmt::Debug + Trait> ::core::fmt::Debug for Module<T> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Module(ref __self_0_0) => {
                    let mut debug_trait_builder = f.debug_tuple("Module");
                    let _ = debug_trait_builder.field(&&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl<T: Trait> ::srml_support::sr_primitives::traits::OnInitialize<T::BlockNumber> for Module<T> {}
    impl<T: Trait> ::srml_support::sr_primitives::traits::OnFinalize<T::BlockNumber> for Module<T> {
        fn on_finalize(_block_number_not_used: T::BlockNumber) {}
    }
    impl<T: Trait> ::srml_support::sr_primitives::traits::OffchainWorker<T::BlockNumber> for Module<T> {}
    impl<T: Trait> Module<T> {
        fn deposit_event(event: Event<T>) {
            <system::Module<T>>::deposit_event(<T as Trait>::from(event).into());
        }
    }
    /// Can also be called using [`Call`].
    ///
    /// [`Call`]: enum.Call.html
    impl<T: Trait> Module<T> {
        /// Dispatch a single transaction and update UTXO set accordingly
        pub fn execute(origin: T::Origin, transaction: Transaction<T::BlockNumber>) -> Result {
            ensure_signed(origin)?;
            let leftover = match Self::check_transaction(&transaction)? {
                CheckInfo::Totals { input, output } => input - output,
                CheckInfo::MissingInputs(_) => return Err("Invalid transaction inputs"),
            };
            Self::update_storage(&transaction, leftover)?;
            Self::deposit_event(Event::<T>::TransactionExecuted(transaction));
            Ok(())
        }
        /// DANGEROUS! Adds specified output to the storage potentially overwriting existing one.
        /// Does not perform enough checks. Must only be used for testing purposes.
        pub fn mint(origin: T::Origin, value: Value, pubkey: H256) -> Result {
            ensure_signed(origin)?;
            let salt = <system::Module<T>>::block_number();
            let utxo = TransactionOutput {
                value,
                pubkey,
                salt,
            };
            let hash = BlakeTwo256::hash_of(&utxo);
            if !<UnspentOutputs<T>>::exists(hash) {
                <UnspentOutputs<T>>::insert(hash, utxo);
            } else {
                runtime_io::print("cannot mint due to hash collision");
            }
            Ok(())
        }
    }
    pub enum Call<T: Trait> {
        #[doc(hidden)]
        #[codec(skip)]
        __PhantomItem(
            ::srml_support::rstd::marker::PhantomData<(T)>,
            ::srml_support::dispatch::Never,
        ),
        #[allow(non_camel_case_types)]
        /// Dispatch a single transaction and update UTXO set accordingly
        execute(Transaction<T::BlockNumber>),
        #[allow(non_camel_case_types)]
        /// DANGEROUS! Adds specified output to the storage potentially overwriting existing one.
        /// Does not perform enough checks. Must only be used for testing purposes.
        mint(Value, H256),
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_Call: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl<T: Trait> _parity_scale_codec::Encode for Call<T>
        where
            Transaction<T::BlockNumber>: _parity_scale_codec::Encode,
            Transaction<T::BlockNumber>: _parity_scale_codec::Encode,
        {
            fn encode_to<EncOut: _parity_scale_codec::Output>(&self, dest: &mut EncOut) {
                match *self {
                    Call::execute(ref aa) => {
                        dest.push_byte(0usize as u8);
                        dest.push(aa);
                    }
                    Call::mint(ref aa, ref ba) => {
                        dest.push_byte(1usize as u8);
                        dest.push(aa);
                        dest.push(ba);
                    }
                    _ => (),
                }
            }
        }
        impl<T: Trait> _parity_scale_codec::EncodeLike for Call<T>
        where
            Transaction<T::BlockNumber>: _parity_scale_codec::Encode,
            Transaction<T::BlockNumber>: _parity_scale_codec::Encode,
        {
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_Call: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl<T: Trait> _parity_scale_codec::Decode for Call<T>
        where
            Transaction<T::BlockNumber>: _parity_scale_codec::Decode,
            Transaction<T::BlockNumber>: _parity_scale_codec::Decode,
        {
            fn decode<DecIn: _parity_scale_codec::Input>(
                input: &mut DecIn,
            ) -> core::result::Result<Self, _parity_scale_codec::Error> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => Ok(Call::execute({
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => return Err("Error decoding field Call :: execute.0".into()),
                            Ok(a) => a,
                        }
                    })),
                    x if x == 1usize as u8 => Ok(Call::mint(
                        {
                            let res = _parity_scale_codec::Decode::decode(input);
                            match res {
                                Err(_) => return Err("Error decoding field Call :: mint.0".into()),
                                Ok(a) => a,
                            }
                        },
                        {
                            let res = _parity_scale_codec::Decode::decode(input);
                            match res {
                                Err(_) => return Err("Error decoding field Call :: mint.1".into()),
                                Ok(a) => a,
                            }
                        },
                    )),
                    x => Err("No such variant in enum Call".into()),
                }
            }
        }
    };
    impl<T: Trait> ::srml_support::dispatch::GetDispatchInfo for Call<T> {
        fn get_dispatch_info(&self) -> ::srml_support::dispatch::DispatchInfo {
            if let Call::execute(ref transaction) = self {
                let weight = <dyn::srml_support::dispatch::WeighData<
                    (&Transaction<T::BlockNumber>,),
                >>::weigh_data(
                    &::srml_support::dispatch::SimpleDispatchInfo::default(),
                    (transaction,),
                );
                let class = <dyn::srml_support::dispatch::ClassifyDispatch<(
                    &Transaction<T::BlockNumber>,
                )>>::classify_dispatch(
                    &::srml_support::dispatch::SimpleDispatchInfo::default(),
                    (transaction,),
                );
                return ::srml_support::dispatch::DispatchInfo { weight, class };
            }
            if let Call::__PhantomItem(_, _) = self {
                {
                    {
                        {
                            ::std::rt::begin_panic_fmt(
                                &::core::fmt::Arguments::new_v1(
                                    &["internal error: entered unreachable code: "],
                                    &match (&"__PhantomItem should never be used.",) {
                                        (arg0,) => [::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        )],
                                    },
                                ),
                                &("utxo/runtime/src/utxo.rs", 104u32, 1u32),
                            )
                        }
                    }
                }
            }
            if let Call::mint(ref value, ref pubkey) = self {
                let weight = <dyn::srml_support::dispatch::WeighData<(&Value, &H256)>>::weigh_data(
                    &::srml_support::dispatch::SimpleDispatchInfo::default(),
                    (value, pubkey),
                );
                let class = < dyn :: srml_support :: dispatch :: ClassifyDispatch < ( & Value , & H256 ) > > :: classify_dispatch ( & :: srml_support :: dispatch :: SimpleDispatchInfo :: default ( ) , ( value , pubkey ) ) ;
                return ::srml_support::dispatch::DispatchInfo { weight, class };
            }
            if let Call::__PhantomItem(_, _) = self {
                {
                    {
                        {
                            ::std::rt::begin_panic_fmt(
                                &::core::fmt::Arguments::new_v1(
                                    &["internal error: entered unreachable code: "],
                                    &match (&"__PhantomItem should never be used.",) {
                                        (arg0,) => [::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        )],
                                    },
                                ),
                                &("utxo/runtime/src/utxo.rs", 104u32, 1u32),
                            )
                        }
                    }
                }
            }
            let weight = <dyn::srml_support::dispatch::WeighData<_>>::weigh_data(
                &::srml_support::dispatch::SimpleDispatchInfo::default(),
                (),
            );
            let class = <dyn::srml_support::dispatch::ClassifyDispatch<_>>::classify_dispatch(
                &::srml_support::dispatch::SimpleDispatchInfo::default(),
                (),
            );
            ::srml_support::dispatch::DispatchInfo { weight, class }
        }
    }
    impl<T: Trait> ::srml_support::dispatch::Clone for Call<T> {
        fn clone(&self) -> Self {
            match *self {
                Call::execute(ref transaction) => Call::execute((*transaction).clone()),
                Call::mint(ref value, ref pubkey) => {
                    Call::mint((*value).clone(), (*pubkey).clone())
                }
                _ => ::std::rt::begin_panic(
                    "internal error: entered unreachable code",
                    &("utxo/runtime/src/utxo.rs", 104u32, 1u32),
                ),
            }
        }
    }
    impl<T: Trait> ::srml_support::dispatch::PartialEq for Call<T> {
        fn eq(&self, _other: &Self) -> bool {
            match *self {
                Call::execute(ref transaction) => {
                    let self_params = (transaction,);
                    if let Call::execute(ref transaction) = *_other {
                        self_params == (transaction,)
                    } else {
                        match *_other {
                            Call::__PhantomItem(_, _) => ::std::rt::begin_panic(
                                "internal error: entered unreachable code",
                                &("utxo/runtime/src/utxo.rs", 104u32, 1u32),
                            ),
                            _ => false,
                        }
                    }
                }
                Call::mint(ref value, ref pubkey) => {
                    let self_params = (value, pubkey);
                    if let Call::mint(ref value, ref pubkey) = *_other {
                        self_params == (value, pubkey)
                    } else {
                        match *_other {
                            Call::__PhantomItem(_, _) => ::std::rt::begin_panic(
                                "internal error: entered unreachable code",
                                &("utxo/runtime/src/utxo.rs", 104u32, 1u32),
                            ),
                            _ => false,
                        }
                    }
                }
                _ => ::std::rt::begin_panic(
                    "internal error: entered unreachable code",
                    &("utxo/runtime/src/utxo.rs", 104u32, 1u32),
                ),
            }
        }
    }
    impl<T: Trait> ::srml_support::dispatch::Eq for Call<T> {}
    #[cfg(feature = "std")]
    impl<T: Trait> ::srml_support::dispatch::fmt::Debug for Call<T> {
        fn fmt(
            &self,
            _f: &mut ::srml_support::dispatch::fmt::Formatter,
        ) -> ::srml_support::dispatch::result::Result<(), ::srml_support::dispatch::fmt::Error>
        {
            match *self {
                Call::execute(ref transaction) => _f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["", ""],
                    &match (&"execute", &(transaction.clone(),)) {
                        (arg0, arg1) => [
                            ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                            ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                        ],
                    },
                )),
                Call::mint(ref value, ref pubkey) => _f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["", ""],
                    &match (&"mint", &(value.clone(), pubkey.clone())) {
                        (arg0, arg1) => [
                            ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                            ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                        ],
                    },
                )),
                _ => ::std::rt::begin_panic(
                    "internal error: entered unreachable code",
                    &("utxo/runtime/src/utxo.rs", 104u32, 1u32),
                ),
            }
        }
    }
    impl<T: Trait> ::srml_support::dispatch::Dispatchable for Call<T> {
        type Trait = T;
        type Origin = T::Origin;
        fn dispatch(self, _origin: Self::Origin) -> ::srml_support::dispatch::Result {
            match self {
                Call::execute(transaction) => <Module<T>>::execute(_origin, transaction),
                Call::mint(value, pubkey) => <Module<T>>::mint(_origin, value, pubkey),
                Call::__PhantomItem(_, _) => ::std::rt::begin_panic_fmt(
                    &::core::fmt::Arguments::new_v1(
                        &["internal error: entered unreachable code: "],
                        &match (&"__PhantomItem should never be used.",) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ),
                    &("utxo/runtime/src/utxo.rs", 104u32, 1u32),
                ),
            }
        }
    }
    impl<T: Trait> ::srml_support::dispatch::Callable<T> for Module<T> {
        type Call = Call<T>;
    }
    impl<T: Trait> Module<T> {
        #[doc(hidden)]
        pub fn dispatch<D: ::srml_support::dispatch::Dispatchable<Trait = T>>(
            d: D,
            origin: D::Origin,
        ) -> ::srml_support::dispatch::Result {
            d.dispatch(origin)
        }
    }
    impl<T: Trait> Module<T> {
        #[doc(hidden)]
        pub fn call_functions() -> &'static [::srml_support::dispatch::FunctionMetadata] {
            & [ :: srml_support :: dispatch :: FunctionMetadata { name : :: srml_support :: dispatch :: DecodeDifferent :: Encode ( "execute" ) , arguments : :: srml_support :: dispatch :: DecodeDifferent :: Encode ( & [ :: srml_support :: dispatch :: FunctionArgumentMetadata { name : :: srml_support :: dispatch :: DecodeDifferent :: Encode ( "transaction" ) , ty : :: srml_support :: dispatch :: DecodeDifferent :: Encode ( "Transaction<T::BlockNumber>" ) , } ] ) , documentation : :: srml_support :: dispatch :: DecodeDifferent :: Encode ( & [ r" Dispatch a single transaction and update UTXO set accordingly" ] ) , } , :: srml_support :: dispatch :: FunctionMetadata { name : :: srml_support :: dispatch :: DecodeDifferent :: Encode ( "mint" ) , arguments : :: srml_support :: dispatch :: DecodeDifferent :: Encode ( & [ :: srml_support :: dispatch :: FunctionArgumentMetadata { name : :: srml_support :: dispatch :: DecodeDifferent :: Encode ( "value" ) , ty : :: srml_support :: dispatch :: DecodeDifferent :: Encode ( "Value" ) , } , :: srml_support :: dispatch :: FunctionArgumentMetadata { name : :: srml_support :: dispatch :: DecodeDifferent :: Encode ( "pubkey" ) , ty : :: srml_support :: dispatch :: DecodeDifferent :: Encode ( "H256" ) , } ] ) , documentation : :: srml_support :: dispatch :: DecodeDifferent :: Encode ( & [ r" DANGEROUS! Adds specified output to the storage potentially overwriting existing one." , r" Does not perform enough checks. Must only be used for testing purposes." ] ) , } ]
        }
    }
    impl<T: 'static + Trait> Module<T> {
        #[doc(hidden)]
        pub fn module_constants_metadata(
        ) -> &'static [::srml_support::dispatch::ModuleConstantMetadata] {
            &[]
        }
    }
    /// [`RawEvent`] specialized for the configuration [`Trait`]
    ///
    /// [`RawEvent`]: enum.RawEvent.html
    /// [`Trait`]: trait.Trait.html
    pub type Event<T> = RawEvent<<T as system::Trait>::BlockNumber>;
    /// Events for this module.
    ///
    pub enum RawEvent<BlockNumber> {
        /// Transaction was executed successfully
        TransactionExecuted(Transaction<BlockNumber>),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<BlockNumber: ::core::clone::Clone> ::core::clone::Clone for RawEvent<BlockNumber> {
        #[inline]
        fn clone(&self) -> RawEvent<BlockNumber> {
            match (&*self,) {
                (&RawEvent::TransactionExecuted(ref __self_0),) => {
                    RawEvent::TransactionExecuted(::core::clone::Clone::clone(&(*__self_0)))
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<BlockNumber: ::core::cmp::PartialEq> ::core::cmp::PartialEq for RawEvent<BlockNumber> {
        #[inline]
        fn eq(&self, other: &RawEvent<BlockNumber>) -> bool {
            match (&*self, &*other) {
                (
                    &RawEvent::TransactionExecuted(ref __self_0),
                    &RawEvent::TransactionExecuted(ref __arg_1_0),
                ) => (*__self_0) == (*__arg_1_0),
            }
        }
        #[inline]
        fn ne(&self, other: &RawEvent<BlockNumber>) -> bool {
            match (&*self, &*other) {
                (
                    &RawEvent::TransactionExecuted(ref __self_0),
                    &RawEvent::TransactionExecuted(ref __arg_1_0),
                ) => (*__self_0) != (*__arg_1_0),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<BlockNumber: ::core::cmp::Eq> ::core::cmp::Eq for RawEvent<BlockNumber> {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<Transaction<BlockNumber>>;
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_RawEvent: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl<BlockNumber> _parity_scale_codec::Encode for RawEvent<BlockNumber>
        where
            Transaction<BlockNumber>: _parity_scale_codec::Encode,
            Transaction<BlockNumber>: _parity_scale_codec::Encode,
        {
            fn encode_to<EncOut: _parity_scale_codec::Output>(&self, dest: &mut EncOut) {
                match *self {
                    RawEvent::TransactionExecuted(ref aa) => {
                        dest.push_byte(0usize as u8);
                        dest.push(aa);
                    }
                    _ => (),
                }
            }
        }
        impl<BlockNumber> _parity_scale_codec::EncodeLike for RawEvent<BlockNumber>
        where
            Transaction<BlockNumber>: _parity_scale_codec::Encode,
            Transaction<BlockNumber>: _parity_scale_codec::Encode,
        {
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_RawEvent: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl<BlockNumber> _parity_scale_codec::Decode for RawEvent<BlockNumber>
        where
            Transaction<BlockNumber>: _parity_scale_codec::Decode,
            Transaction<BlockNumber>: _parity_scale_codec::Decode,
        {
            fn decode<DecIn: _parity_scale_codec::Input>(
                input: &mut DecIn,
            ) -> core::result::Result<Self, _parity_scale_codec::Error> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => Ok(RawEvent::TransactionExecuted({
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err(
                                    "Error decoding field RawEvent :: TransactionExecuted.0".into(),
                                )
                            }
                            Ok(a) => a,
                        }
                    })),
                    x => Err("No such variant in enum RawEvent".into()),
                }
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<BlockNumber: ::core::fmt::Debug> ::core::fmt::Debug for RawEvent<BlockNumber> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&RawEvent::TransactionExecuted(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("TransactionExecuted");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl<BlockNumber> From<RawEvent<BlockNumber>> for () {
        fn from(_: RawEvent<BlockNumber>) -> () {
            ()
        }
    }
    impl<BlockNumber> RawEvent<BlockNumber> {
        #[allow(dead_code)]
        pub fn metadata() -> &'static [::srml_support::event::EventMetadata] {
            &[::srml_support::event::EventMetadata {
                name: ::srml_support::event::DecodeDifferent::Encode("TransactionExecuted"),
                arguments: ::srml_support::event::DecodeDifferent::Encode(&[
                    "Transaction<BlockNumber>",
                ]),
                documentation: ::srml_support::event::DecodeDifferent::Encode(&[
                    r" Transaction was executed successfully",
                ]),
            }]
        }
    }
    /// Information collected during transaction verification
    pub enum CheckInfo<'a> {
        /// Combined value of all inputs and outputs
        Totals { input: Value, output: Value },
        /// Some referred UTXOs were missing
        MissingInputs(Vec<&'a H256>),
    }
    /// Result of transaction verification
    pub type CheckResult<'a> = rstd::result::Result<CheckInfo<'a>, &'static str>;
    impl<T: Trait> Module<T> {
        /// Check transaction for validity.
        ///
        /// Ensures that:
        /// - inputs and outputs are not empty
        /// - all inputs match to existing, unspent and unlocked outputs
        /// - each input is used exactly once
        /// - each output is defined exactly once and has nonzero value
        /// - total output value must not exceed total input value
        /// - new outputs do not collide with existing ones
        /// - sum of input and output values does not overflow
        /// - provided signatures are valid
        pub fn check_transaction(transaction: &Transaction<T::BlockNumber>) -> CheckResult<'_> {
            {
                if !!transaction.inputs.is_empty() {
                    {
                        return Err("no inputs");
                    };
                }
            };
            {
                if !!transaction.outputs.is_empty() {
                    {
                        return Err("no outputs");
                    };
                }
            };
            {
                let input_set: BTreeMap<_, ()> =
                    transaction.inputs.iter().map(|input| (input, ())).collect();
                {
                    if !(input_set.len() == transaction.inputs.len()) {
                        {
                            return Err("each input must only be used once");
                        };
                    }
                };
            }
            {
                let output_set: BTreeMap<_, ()> = transaction
                    .outputs
                    .iter()
                    .map(|output| (output, ()))
                    .collect();
                {
                    if !(output_set.len() == transaction.outputs.len()) {
                        {
                            return Err("each output must be defined only once");
                        };
                    }
                };
            }
            let mut total_input: Value = 0;
            let mut missing_utxo = Vec::new();
            for input in transaction.inputs.iter() {
                let temp: Option<TransactionOutput<T::BlockNumber>> =
                    <UnspentOutputs<T>>::get(&input.parent_output);
                if let Some(output) = temp {
                    {
                        if !!<LockedOutputs<T>>::exists(&input.parent_output) {
                            {
                                return Err("utxo is locked");
                            };
                        }
                    };
                    total_input = total_input
                        .checked_add(output.value)
                        .ok_or("input value overflow")?;
                } else {
                    missing_utxo.push(&input.parent_output);
                }
            }
            let mut total_output: Value = 0;
            for output in transaction.outputs.iter() {
                {
                    if !(output.value != 0) {
                        {
                            return Err("output value must be nonzero");
                        };
                    }
                };
                let hash = BlakeTwo256::hash_of(output);
                {
                    if !!<UnspentOutputs<T>>::exists(hash) {
                        {
                            return Err("output already exists");
                        };
                    }
                };
                total_output = total_output
                    .checked_add(output.value)
                    .ok_or("output value overflow")?;
            }
            if missing_utxo.is_empty() {
                {
                    if !(total_input >= total_output) {
                        {
                            return Err("output value must not exceed input value");
                        };
                    }
                };
                Ok(CheckInfo::Totals {
                    input: total_input,
                    output: total_output,
                })
            } else {
                Ok(CheckInfo::MissingInputs(missing_utxo))
            }
        }
        /// Redistribute combined leftover value evenly among chain authorities
        fn spend_leftover(authorities: &[H256]) {
            let leftover = LeftoverTotal::take();
            let share_value: Value = leftover
                .checked_div(authorities.len() as Value)
                .ok_or("No authorities")
                .unwrap();
            if share_value == 0 {
                return;
            }
            let remainder = leftover
                .checked_sub(share_value * authorities.len() as Value)
                .ok_or("Sub underflow")
                .unwrap();
            LeftoverTotal::put(remainder as Value);
            for authority in authorities {
                let utxo = TransactionOutput {
                    value: share_value,
                    pubkey: *authority,
                    salt: <system::Module<T>>::block_number(),
                };
                let hash = BlakeTwo256::hash_of(&utxo);
                if !<UnspentOutputs<T>>::exists(hash) {
                    <UnspentOutputs<T>>::insert(hash, utxo);
                    runtime_io::print("leftover share sent to");
                    runtime_io::print(hash.as_fixed_bytes() as &[u8]);
                } else {
                    runtime_io::print("leftover share wasted due to hash collision");
                }
            }
        }
        /// Update storage to reflect changes made by transaction
        fn update_storage(transaction: &Transaction<T::BlockNumber>, leftover: Value) -> Result {
            let new_total = LeftoverTotal::get()
                .checked_add(leftover)
                .ok_or("Leftover overflow")?;
            LeftoverTotal::put(new_total);
            for input in &transaction.inputs {
                <UnspentOutputs<T>>::remove(input.parent_output);
            }
            for output in &transaction.outputs {
                let hash = BlakeTwo256::hash_of(output);
                <UnspentOutputs<T>>::insert(hash, output);
            }
            Ok(())
        }
        pub fn lock_utxo(hash: &H256, until: Option<T::BlockNumber>) -> Result {
            {
                if !!<LockedOutputs<T>>::exists(hash) {
                    {
                        return Err("utxo is already locked");
                    };
                }
            };
            {
                if !<UnspentOutputs<T>>::exists(hash) {
                    {
                        return Err("utxo does not exist");
                    };
                }
            };
            if let Some(until) = until {
                {
                    if !(until > <system::Module<T>>::block_number()) {
                        {
                            return Err("block number is in the past");
                        };
                    }
                };
                <LockedOutputs<T>>::insert(hash, LockStatus::LockedUntil(until));
            } else {
                <LockedOutputs<T>>::insert(hash, LockStatus::Locked);
            }
            Ok(())
        }
        pub fn unlock_utxo(hash: &H256) -> Result {
            {
                if !!<LockedOutputs<T>>::exists(hash) {
                    {
                        return Err("utxo is not locked");
                    };
                }
            };
            <LockedOutputs<T>>::remove(hash);
            Ok(())
        }
    }
}
