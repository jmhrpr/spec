// @generated
/// Predicate to match transactions based on addresses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressPredicate {
    /// Match transactions with the specified source address.
    #[prost(bytes="bytes", tag="1")]
    pub match_source: ::prost::bytes::Bytes,
    /// Match transactions with the specified target address.
    #[prost(bytes="bytes", tag="2")]
    pub match_target: ::prost::bytes::Bytes,
    /// Match transactions with the specified address as either source or target.
    #[prost(bytes="bytes", tag="3")]
    pub match_any: ::prost::bytes::Bytes,
}
/// Predicate to match transactions based on assets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetPredicate {
    /// Match transactions with the specified asset policy.
    #[prost(bytes="bytes", tag="1")]
    pub match_policy: ::prost::bytes::Bytes,
    /// Match transactions with the specified asset name.
    #[prost(bytes="bytes", tag="2")]
    pub match_name: ::prost::bytes::Bytes,
}
/// Predicate to match transactions based on UTXOs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UtxoPredicate {
    /// Match transactions with the specified UTXO hash.
    #[prost(bytes="bytes", tag="1")]
    pub match_hash: ::prost::bytes::Bytes,
    /// Match transactions with the specified UTXO index.
    #[prost(uint32, tag="2")]
    pub match_index: u32,
}
/// Predicate to match transactions based on datums.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatumPredicate {
    /// Match transactions with the specified datum hash.
    #[prost(bytes="bytes", tag="1")]
    pub match_hash: ::prost::bytes::Bytes,
}
/// Predicate to match transactions based on any of the specified types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnyPredicate {
    #[prost(oneof="any_predicate::Type", tags="1, 2, 3, 4")]
    pub r#type: ::core::option::Option<any_predicate::Type>,
}
/// Nested message and enum types in `AnyPredicate`.
pub mod any_predicate {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        /// Address-based predicate.
        #[prost(message, tag="1")]
        Address(super::AddressPredicate),
        /// Asset-based predicate.
        #[prost(message, tag="2")]
        Asset(super::AssetPredicate),
        /// UTXO-based predicate.
        #[prost(message, tag="3")]
        Utxo(super::UtxoPredicate),
        /// Datum-based predicate.
        #[prost(message, tag="4")]
        Datum(super::DatumPredicate),
    }
}
/// Request to watch transactions based on a set of predicates.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WatchTxRequest {
    /// List of predicates to filter transactions.
    #[prost(message, repeated, tag="1")]
    pub predicate: ::prost::alloc::vec::Vec<AnyPredicate>,
    /// Field mask to selectively return fields.
    #[prost(message, optional, tag="2")]
    pub field_mask: ::core::option::Option<::pbjson_types::FieldMask>,
}
/// Represents a transaction from any supported blockchain.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnyChainTx {
    #[prost(oneof="any_chain_tx::Chain", tags="1")]
    pub chain: ::core::option::Option<any_chain_tx::Chain>,
}
/// Nested message and enum types in `AnyChainTx`.
pub mod any_chain_tx {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Chain {
        /// A Cardano transaction.
        #[prost(message, tag="1")]
        Cardano(::utxorpc_spec_cardano::utxorpc::cardano::v1::Tx),
    }
}
/// Response containing the matching transactions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WatchTxResponse {
    #[prost(oneof="watch_tx_response::Action", tags="1, 2")]
    pub action: ::core::option::Option<watch_tx_response::Action>,
}
/// Nested message and enum types in `WatchTxResponse`.
pub mod watch_tx_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Action {
        /// Apply this transaction.
        #[prost(message, tag="1")]
        Apply(super::AnyChainTx),
        /// Undo this transaction.
        #[prost(message, tag="2")]
        Undo(super::AnyChainTx),
    }
}
/// Encoded file descriptor set for the `utxorpc.watch.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xcd, 0x1f, 0x0a, 0x1c, 0x75, 0x74, 0x78, 0x6f, 0x72, 0x70, 0x63, 0x2f, 0x77, 0x61, 0x74,
    0x63, 0x68, 0x2f, 0x76, 0x31, 0x2f, 0x77, 0x61, 0x74, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x12, 0x10, 0x75, 0x74, 0x78, 0x6f, 0x72, 0x70, 0x63, 0x2e, 0x77, 0x61, 0x74, 0x63, 0x68,
    0x2e, 0x76, 0x31, 0x1a, 0x20, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x62, 0x75, 0x66, 0x2f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x5f, 0x6d, 0x61, 0x73, 0x6b, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x20, 0x75, 0x74, 0x78, 0x6f, 0x72, 0x70, 0x63, 0x2f, 0x63,
    0x61, 0x72, 0x64, 0x61, 0x6e, 0x6f, 0x2f, 0x76, 0x31, 0x2f, 0x63, 0x61, 0x72, 0x64, 0x61, 0x6e,
    0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x75, 0x0a, 0x10, 0x41, 0x64, 0x64, 0x72, 0x65,
    0x73, 0x73, 0x50, 0x72, 0x65, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x12, 0x21, 0x0a, 0x0c, 0x6d,
    0x61, 0x74, 0x63, 0x68, 0x5f, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0c, 0x52, 0x0b, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x12, 0x21,
    0x0a, 0x0c, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0c, 0x52, 0x0b, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x54, 0x61, 0x72, 0x67, 0x65,
    0x74, 0x12, 0x1b, 0x0a, 0x09, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x61, 0x6e, 0x79, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0c, 0x52, 0x08, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x41, 0x6e, 0x79, 0x22, 0x52,
    0x0a, 0x0e, 0x41, 0x73, 0x73, 0x65, 0x74, 0x50, 0x72, 0x65, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65,
    0x12, 0x21, 0x0a, 0x0c, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x70, 0x6f, 0x6c, 0x69, 0x63, 0x79,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0b, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x50, 0x6f, 0x6c,
    0x69, 0x63, 0x79, 0x12, 0x1d, 0x0a, 0x0a, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x6e, 0x61, 0x6d,
    0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x09, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x4e, 0x61,
    0x6d, 0x65, 0x22, 0x4f, 0x0a, 0x0d, 0x55, 0x74, 0x78, 0x6f, 0x50, 0x72, 0x65, 0x64, 0x69, 0x63,
    0x61, 0x74, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x68, 0x61, 0x73,
    0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x09, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x48, 0x61,
    0x73, 0x68, 0x12, 0x1f, 0x0a, 0x0b, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x69, 0x6e, 0x64, 0x65,
    0x78, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0a, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x49, 0x6e,
    0x64, 0x65, 0x78, 0x22, 0x2f, 0x0a, 0x0e, 0x44, 0x61, 0x74, 0x75, 0x6d, 0x50, 0x72, 0x65, 0x64,
    0x69, 0x63, 0x61, 0x74, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x68,
    0x61, 0x73, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x09, 0x6d, 0x61, 0x74, 0x63, 0x68,
    0x48, 0x61, 0x73, 0x68, 0x22, 0x81, 0x02, 0x0a, 0x0c, 0x41, 0x6e, 0x79, 0x50, 0x72, 0x65, 0x64,
    0x69, 0x63, 0x61, 0x74, 0x65, 0x12, 0x3e, 0x0a, 0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x22, 0x2e, 0x75, 0x74, 0x78, 0x6f, 0x72, 0x70, 0x63,
    0x2e, 0x77, 0x61, 0x74, 0x63, 0x68, 0x2e, 0x76, 0x31, 0x2e, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73,
    0x73, 0x50, 0x72, 0x65, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x48, 0x00, 0x52, 0x07, 0x61, 0x64,
    0x64, 0x72, 0x65, 0x73, 0x73, 0x12, 0x38, 0x0a, 0x05, 0x61, 0x73, 0x73, 0x65, 0x74, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x75, 0x74, 0x78, 0x6f, 0x72, 0x70, 0x63, 0x2e, 0x77,
    0x61, 0x74, 0x63, 0x68, 0x2e, 0x76, 0x31, 0x2e, 0x41, 0x73, 0x73, 0x65, 0x74, 0x50, 0x72, 0x65,
    0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x48, 0x00, 0x52, 0x05, 0x61, 0x73, 0x73, 0x65, 0x74, 0x12,
    0x35, 0x0a, 0x04, 0x75, 0x74, 0x78, 0x6f, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1f, 0x2e,
    0x75, 0x74, 0x78, 0x6f, 0x72, 0x70, 0x63, 0x2e, 0x77, 0x61, 0x74, 0x63, 0x68, 0x2e, 0x76, 0x31,
    0x2e, 0x55, 0x74, 0x78, 0x6f, 0x50, 0x72, 0x65, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x48, 0x00,
    0x52, 0x04, 0x75, 0x74, 0x78, 0x6f, 0x12, 0x38, 0x0a, 0x05, 0x64, 0x61, 0x74, 0x75, 0x6d, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x75, 0x74, 0x78, 0x6f, 0x72, 0x70, 0x63, 0x2e,
    0x77, 0x61, 0x74, 0x63, 0x68, 0x2e, 0x76, 0x31, 0x2e, 0x44, 0x61, 0x74, 0x75, 0x6d, 0x50, 0x72,
    0x65, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x48, 0x00, 0x52, 0x05, 0x64, 0x61, 0x74, 0x75, 0x6d,
    0x42, 0x06, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x22, 0x89, 0x01, 0x0a, 0x0e, 0x57, 0x61, 0x74,
    0x63, 0x68, 0x54, 0x78, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x3c, 0x0a, 0x09, 0x70,
    0x72, 0x65, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1e,
    0x2e, 0x75, 0x74, 0x78, 0x6f, 0x72, 0x70, 0x63, 0x2e, 0x77, 0x61, 0x74, 0x63, 0x68, 0x2e, 0x76,
    0x31, 0x2e, 0x41, 0x6e, 0x79, 0x50, 0x72, 0x65, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x52, 0x09,
    0x70, 0x72, 0x65, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x12, 0x39, 0x0a, 0x0a, 0x66, 0x69, 0x65,
    0x6c, 0x64, 0x5f, 0x6d, 0x61, 0x73, 0x6b, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e,
    0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e,
    0x46, 0x69, 0x65, 0x6c, 0x64, 0x4d, 0x61, 0x73, 0x6b, 0x52, 0x09, 0x66, 0x69, 0x65, 0x6c, 0x64,
    0x4d, 0x61, 0x73, 0x6b, 0x22, 0x49, 0x0a, 0x0a, 0x41, 0x6e, 0x79, 0x43, 0x68, 0x61, 0x69, 0x6e,
    0x54, 0x78, 0x12, 0x32, 0x0a, 0x07, 0x63, 0x61, 0x72, 0x64, 0x61, 0x6e, 0x6f, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x75, 0x74, 0x78, 0x6f, 0x72, 0x70, 0x63, 0x2e, 0x63, 0x61,
    0x72, 0x64, 0x61, 0x6e, 0x6f, 0x2e, 0x76, 0x31, 0x2e, 0x54, 0x78, 0x48, 0x00, 0x52, 0x07, 0x63,
    0x61, 0x72, 0x64, 0x61, 0x6e, 0x6f, 0x42, 0x07, 0x0a, 0x05, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x22,
    0x85, 0x01, 0x0a, 0x0f, 0x57, 0x61, 0x74, 0x63, 0x68, 0x54, 0x78, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x12, 0x34, 0x0a, 0x05, 0x61, 0x70, 0x70, 0x6c, 0x79, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x75, 0x74, 0x78, 0x6f, 0x72, 0x70, 0x63, 0x2e, 0x77, 0x61, 0x74,
    0x63, 0x68, 0x2e, 0x76, 0x31, 0x2e, 0x41, 0x6e, 0x79, 0x43, 0x68, 0x61, 0x69, 0x6e, 0x54, 0x78,
    0x48, 0x00, 0x52, 0x05, 0x61, 0x70, 0x70, 0x6c, 0x79, 0x12, 0x32, 0x0a, 0x04, 0x75, 0x6e, 0x64,
    0x6f, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x75, 0x74, 0x78, 0x6f, 0x72, 0x70,
    0x63, 0x2e, 0x77, 0x61, 0x74, 0x63, 0x68, 0x2e, 0x76, 0x31, 0x2e, 0x41, 0x6e, 0x79, 0x43, 0x68,
    0x61, 0x69, 0x6e, 0x54, 0x78, 0x48, 0x00, 0x52, 0x04, 0x75, 0x6e, 0x64, 0x6f, 0x42, 0x08, 0x0a,
    0x06, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x32, 0x62, 0x0a, 0x0e, 0x54, 0x78, 0x57, 0x61, 0x74,
    0x63, 0x68, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x12, 0x50, 0x0a, 0x07, 0x57, 0x61, 0x74,
    0x63, 0x68, 0x54, 0x78, 0x12, 0x20, 0x2e, 0x75, 0x74, 0x78, 0x6f, 0x72, 0x70, 0x63, 0x2e, 0x77,
    0x61, 0x74, 0x63, 0x68, 0x2e, 0x76, 0x31, 0x2e, 0x57, 0x61, 0x74, 0x63, 0x68, 0x54, 0x78, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x21, 0x2e, 0x75, 0x74, 0x78, 0x6f, 0x72, 0x70, 0x63,
    0x2e, 0x77, 0x61, 0x74, 0x63, 0x68, 0x2e, 0x76, 0x31, 0x2e, 0x57, 0x61, 0x74, 0x63, 0x68, 0x54,
    0x78, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x30, 0x01, 0x42, 0xbf, 0x01, 0x0a, 0x14,
    0x63, 0x6f, 0x6d, 0x2e, 0x75, 0x74, 0x78, 0x6f, 0x72, 0x70, 0x63, 0x2e, 0x77, 0x61, 0x74, 0x63,
    0x68, 0x2e, 0x76, 0x31, 0x42, 0x0a, 0x57, 0x61, 0x74, 0x63, 0x68, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x50, 0x01, 0x5a, 0x39, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x62,
    0x75, 0x66, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x2f, 0x62, 0x75, 0x66, 0x2d, 0x74, 0x6f, 0x75, 0x72,
    0x2f, 0x67, 0x65, 0x6e, 0x2f, 0x75, 0x74, 0x78, 0x6f, 0x72, 0x70, 0x63, 0x2f, 0x77, 0x61, 0x74,
    0x63, 0x68, 0x2f, 0x76, 0x31, 0x3b, 0x77, 0x61, 0x74, 0x63, 0x68, 0x76, 0x31, 0xa2, 0x02, 0x03,
    0x55, 0x57, 0x58, 0xaa, 0x02, 0x10, 0x55, 0x74, 0x78, 0x6f, 0x72, 0x70, 0x63, 0x2e, 0x57, 0x61,
    0x74, 0x63, 0x68, 0x2e, 0x56, 0x31, 0xca, 0x02, 0x10, 0x55, 0x74, 0x78, 0x6f, 0x72, 0x70, 0x63,
    0x5c, 0x57, 0x61, 0x74, 0x63, 0x68, 0x5c, 0x56, 0x31, 0xe2, 0x02, 0x1c, 0x55, 0x74, 0x78, 0x6f,
    0x72, 0x70, 0x63, 0x5c, 0x57, 0x61, 0x74, 0x63, 0x68, 0x5c, 0x56, 0x31, 0x5c, 0x47, 0x50, 0x42,
    0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x12, 0x55, 0x74, 0x78, 0x6f, 0x72,
    0x70, 0x63, 0x3a, 0x3a, 0x57, 0x61, 0x74, 0x63, 0x68, 0x3a, 0x3a, 0x56, 0x31, 0x4a, 0xf8, 0x14,
    0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x41, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00,
    0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x19, 0x0a, 0x09, 0x0a, 0x02,
    0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x2a, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x05,
    0x00, 0x2a, 0x0a, 0x41, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x08, 0x00, 0x0c, 0x01, 0x1a, 0x35,
    0x20, 0x50, 0x72, 0x65, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x6d, 0x61,
    0x74, 0x63, 0x68, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x20, 0x62, 0x61, 0x73, 0x65, 0x64, 0x20, 0x6f, 0x6e, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73,
    0x73, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x08, 0x08,
    0x18, 0x0a, 0x44, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x09, 0x02, 0x19, 0x22, 0x37,
    0x20, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x73, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x70, 0x65,
    0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x20, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x20, 0x61, 0x64,
    0x64, 0x72, 0x65, 0x73, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x09, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x09, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x09, 0x17,
    0x18, 0x0a, 0x44, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x02, 0x19, 0x22, 0x37,
    0x20, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x73, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x70, 0x65,
    0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x20, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x20, 0x61, 0x64,
    0x64, 0x72, 0x65, 0x73, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x0a, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x0a, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0a, 0x17,
    0x18, 0x0a, 0x58, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0b, 0x02, 0x16, 0x22, 0x4b,
    0x20, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x73, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x70, 0x65,
    0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x61,
    0x73, 0x20, 0x65, 0x69, 0x74, 0x68, 0x65, 0x72, 0x20, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x20,
    0x6f, 0x72, 0x20, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0b, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x0b, 0x14, 0x15, 0x0a, 0x3e, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0f, 0x00, 0x12,
    0x01, 0x1a, 0x32, 0x20, 0x50, 0x72, 0x65, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x20, 0x74, 0x6f,
    0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x73, 0x20, 0x62, 0x61, 0x73, 0x65, 0x64, 0x20, 0x6f, 0x6e, 0x20, 0x61, 0x73, 0x73,
    0x65, 0x74, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0f, 0x08,
    0x16, 0x0a, 0x42, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x10, 0x02, 0x19, 0x22, 0x35,
    0x20, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x73, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x70, 0x65,
    0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x20, 0x61, 0x73, 0x73, 0x65, 0x74, 0x20, 0x70, 0x6f, 0x6c,
    0x69, 0x63, 0x79, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x10, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x10, 0x08,
    0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x10, 0x17, 0x18, 0x0a,
    0x40, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x11, 0x02, 0x17, 0x22, 0x33, 0x20, 0x4d,
    0x61, 0x74, 0x63, 0x68, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x73, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69,
    0x66, 0x69, 0x65, 0x64, 0x20, 0x61, 0x73, 0x73, 0x65, 0x74, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x2e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x11, 0x02, 0x07, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x11, 0x08, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x11, 0x15, 0x16, 0x0a, 0x3d, 0x0a, 0x02, 0x04,
    0x02, 0x12, 0x04, 0x15, 0x00, 0x18, 0x01, 0x1a, 0x31, 0x20, 0x50, 0x72, 0x65, 0x64, 0x69, 0x63,
    0x61, 0x74, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x20, 0x74, 0x72, 0x61,
    0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x62, 0x61, 0x73, 0x65, 0x64, 0x20,
    0x6f, 0x6e, 0x20, 0x55, 0x54, 0x58, 0x4f, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02,
    0x01, 0x12, 0x03, 0x15, 0x08, 0x15, 0x0a, 0x3f, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03,
    0x16, 0x02, 0x17, 0x22, 0x32, 0x20, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x20, 0x74, 0x72, 0x61, 0x6e,
    0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x20, 0x55, 0x54, 0x58, 0x4f,
    0x20, 0x68, 0x61, 0x73, 0x68, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x16, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x16, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x16, 0x15,
    0x16, 0x0a, 0x40, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x17, 0x02, 0x19, 0x22, 0x33,
    0x20, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x73, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x70, 0x65,
    0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x20, 0x55, 0x54, 0x58, 0x4f, 0x20, 0x69, 0x6e, 0x64, 0x65,
    0x78, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x17, 0x02,
    0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x17, 0x09, 0x14, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x17, 0x17, 0x18, 0x0a, 0x3e, 0x0a,
    0x02, 0x04, 0x03, 0x12, 0x04, 0x1b, 0x00, 0x1d, 0x01, 0x1a, 0x32, 0x20, 0x50, 0x72, 0x65, 0x64,
    0x69, 0x63, 0x61, 0x74, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x20, 0x74,
    0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x62, 0x61, 0x73, 0x65,
    0x64, 0x20, 0x6f, 0x6e, 0x20, 0x64, 0x61, 0x74, 0x75, 0x6d, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x1b, 0x08, 0x16, 0x0a, 0x40, 0x0a, 0x04, 0x04, 0x03, 0x02,
    0x00, 0x12, 0x03, 0x1c, 0x02, 0x17, 0x22, 0x33, 0x20, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x20, 0x74,
    0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x77, 0x69, 0x74, 0x68,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x20, 0x64,
    0x61, 0x74, 0x75, 0x6d, 0x20, 0x68, 0x61, 0x73, 0x68, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1c, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x1c, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x1c, 0x15, 0x16, 0x0a, 0x52, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x20, 0x00, 0x27,
    0x01, 0x1a, 0x46, 0x20, 0x50, 0x72, 0x65, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x20, 0x74, 0x6f,
    0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x73, 0x20, 0x62, 0x61, 0x73, 0x65, 0x64, 0x20, 0x6f, 0x6e, 0x20, 0x61, 0x6e, 0x79,
    0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65,
    0x64, 0x20, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01,
    0x12, 0x03, 0x20, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x04, 0x08, 0x00, 0x12, 0x04, 0x21,
    0x02, 0x26, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x08, 0x00, 0x01, 0x12, 0x03, 0x21, 0x08,
    0x0c, 0x0a, 0x27, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x22, 0x04, 0x21, 0x22, 0x1a,
    0x20, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x2d, 0x62, 0x61, 0x73, 0x65, 0x64, 0x20, 0x70,
    0x72, 0x65, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x22, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x22, 0x15, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x22, 0x1f, 0x20, 0x0a, 0x25, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x23, 0x04,
    0x1d, 0x22, 0x18, 0x20, 0x41, 0x73, 0x73, 0x65, 0x74, 0x2d, 0x62, 0x61, 0x73, 0x65, 0x64, 0x20,
    0x70, 0x72, 0x65, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x01, 0x06, 0x12, 0x03, 0x23, 0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x23, 0x13, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x23, 0x1b, 0x1c, 0x0a, 0x24, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12, 0x03, 0x24,
    0x04, 0x1b, 0x22, 0x17, 0x20, 0x55, 0x54, 0x58, 0x4f, 0x2d, 0x62, 0x61, 0x73, 0x65, 0x64, 0x20,
    0x70, 0x72, 0x65, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x02, 0x06, 0x12, 0x03, 0x24, 0x04, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x24, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x24, 0x19, 0x1a, 0x0a, 0x25, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x03, 0x12, 0x03, 0x25,
    0x04, 0x1d, 0x22, 0x18, 0x20, 0x44, 0x61, 0x74, 0x75, 0x6d, 0x2d, 0x62, 0x61, 0x73, 0x65, 0x64,
    0x20, 0x70, 0x72, 0x65, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x03, 0x06, 0x12, 0x03, 0x25, 0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x25, 0x13, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x25, 0x1b, 0x1c, 0x0a, 0x49, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x2a, 0x00,
    0x2d, 0x01, 0x1a, 0x3d, 0x20, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x74, 0x6f, 0x20,
    0x77, 0x61, 0x74, 0x63, 0x68, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x73, 0x20, 0x62, 0x61, 0x73, 0x65, 0x64, 0x20, 0x6f, 0x6e, 0x20, 0x61, 0x20, 0x73, 0x65,
    0x74, 0x20, 0x6f, 0x66, 0x20, 0x70, 0x72, 0x65, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x73, 0x2e,
    0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x2a, 0x08, 0x16, 0x0a, 0x39, 0x0a,
    0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x2b, 0x02, 0x26, 0x22, 0x2c, 0x20, 0x4c, 0x69, 0x73,
    0x74, 0x20, 0x6f, 0x66, 0x20, 0x70, 0x72, 0x65, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x73, 0x20,
    0x74, 0x6f, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x2b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x2b, 0x0b, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2b,
    0x18, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2b, 0x24, 0x25,
    0x0a, 0x37, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x2c, 0x02, 0x2b, 0x22, 0x2a, 0x20,
    0x46, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x6d, 0x61, 0x73, 0x6b, 0x20, 0x74, 0x6f, 0x20, 0x73, 0x65,
    0x6c, 0x65, 0x63, 0x74, 0x69, 0x76, 0x65, 0x6c, 0x79, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e,
    0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x01, 0x06, 0x12, 0x03, 0x2c, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x2c, 0x1c, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x2c, 0x29, 0x2a, 0x0a, 0x45, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x30, 0x00, 0x34, 0x01, 0x1a,
    0x39, 0x20, 0x52, 0x65, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x61, 0x20, 0x74,
    0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20,
    0x61, 0x6e, 0x79, 0x20, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x20, 0x62, 0x6c,
    0x6f, 0x63, 0x6b, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06,
    0x01, 0x12, 0x03, 0x30, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x06, 0x08, 0x00, 0x12, 0x04,
    0x31, 0x02, 0x33, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x08, 0x00, 0x01, 0x12, 0x03, 0x31,
    0x08, 0x0d, 0x0a, 0x25, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x32, 0x04, 0x26, 0x22,
    0x18, 0x20, 0x41, 0x20, 0x43, 0x61, 0x72, 0x64, 0x61, 0x6e, 0x6f, 0x20, 0x74, 0x72, 0x61, 0x6e,
    0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x32, 0x04, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x32, 0x1a, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x32, 0x24, 0x25, 0x0a, 0x3c, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x37, 0x00, 0x3c, 0x01, 0x1a,
    0x30, 0x20, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61,
    0x69, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x69,
    0x6e, 0x67, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e,
    0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x37, 0x08, 0x17, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x07, 0x08, 0x00, 0x12, 0x04, 0x38, 0x02, 0x3b, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x08, 0x00, 0x01, 0x12, 0x03, 0x38, 0x08, 0x0e, 0x0a, 0x26, 0x0a, 0x04, 0x04, 0x07, 0x02,
    0x00, 0x12, 0x03, 0x39, 0x04, 0x19, 0x22, 0x19, 0x20, 0x41, 0x70, 0x70, 0x6c, 0x79, 0x20, 0x74,
    0x68, 0x69, 0x73, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x06, 0x12, 0x03, 0x39, 0x04, 0x0e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x39, 0x0f, 0x14, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x39, 0x17, 0x18, 0x0a, 0x25, 0x0a, 0x04, 0x04,
    0x07, 0x02, 0x01, 0x12, 0x03, 0x3a, 0x04, 0x18, 0x22, 0x18, 0x20, 0x55, 0x6e, 0x64, 0x6f, 0x20,
    0x74, 0x68, 0x69, 0x73, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x06, 0x12, 0x03, 0x3a, 0x04, 0x0e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3a, 0x0f, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x03, 0x12, 0x03, 0x3a, 0x16, 0x17, 0x0a, 0x4f, 0x0a, 0x02,
    0x06, 0x00, 0x12, 0x04, 0x3f, 0x00, 0x41, 0x01, 0x1a, 0x43, 0x20, 0x53, 0x65, 0x72, 0x76, 0x69,
    0x63, 0x65, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66, 0x6f,
    0x72, 0x20, 0x77, 0x61, 0x74, 0x63, 0x68, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73,
    0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x62, 0x61, 0x73, 0x65, 0x64, 0x20, 0x6f, 0x6e,
    0x20, 0x70, 0x72, 0x65, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a,
    0x03, 0x06, 0x00, 0x01, 0x12, 0x03, 0x3f, 0x08, 0x16, 0x0a, 0x45, 0x0a, 0x04, 0x06, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x40, 0x02, 0x3f, 0x22, 0x38, 0x20, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x20,
    0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x6d, 0x61, 0x74,
    0x63, 0x68, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66,
    0x69, 0x65, 0x64, 0x20, 0x70, 0x72, 0x65, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x73, 0x2e, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x40, 0x06, 0x0d, 0x0a, 0x0c,
    0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x40, 0x0e, 0x1c, 0x0a, 0x0c, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x40, 0x27, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x40, 0x2e, 0x3d, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("utxorpc.watch.v1.serde.rs");
include!("utxorpc.watch.v1.tonic.rs");
// @@protoc_insertion_point(module)