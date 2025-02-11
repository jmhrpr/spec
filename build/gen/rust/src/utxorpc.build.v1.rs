// @generated
/// Represents a specific point in the blockchain.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChainPoint {
    /// Slot number.
    #[prost(uint64, tag="1")]
    pub slot: u64,
    /// Block height.
    #[prost(uint64, tag="2")]
    pub height: u64,
    /// Block hash.
    #[prost(bytes="bytes", tag="3")]
    pub hash: ::prost::bytes::Bytes,
}
/// Request to get the current chain tip.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChainTipRequest {
}
/// Response containing the current chain tip.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChainTipResponse {
    /// Current chain tip.
    #[prost(message, optional, tag="1")]
    pub tip: ::core::option::Option<ChainPoint>,
}
/// Request to get specific chain parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChainParamRequest {
    /// List of requested parameters.
    #[prost(string, repeated, tag="1")]
    pub param: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Represents a key-value pair for a chain parameter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChainParam {
    /// Parameter key.
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    /// Parameter value.
    #[prost(bytes="bytes", tag="2")]
    pub value: ::prost::bytes::Bytes,
}
/// Response containing the requested chain parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChainParamResponse {
    /// List of requested chain parameters.
    #[prost(message, repeated, tag="1")]
    pub param: ::prost::alloc::vec::Vec<ChainParam>,
}
/// Request to get UTxOs by their associated addresses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUtxoByAddressRequest {
    /// List of addresses to query.
    #[prost(bytes="bytes", repeated, tag="1")]
    pub address: ::prost::alloc::vec::Vec<::prost::bytes::Bytes>,
    /// Point in the chain to query from.
    #[prost(message, optional, tag="2")]
    pub acquire_point: ::core::option::Option<ChainPoint>,
}
/// An evenlope that holds an UTxO from any of compatible chains
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnyChainUtxo {
    #[prost(oneof="any_chain_utxo::Chain", tags="1")]
    pub chain: ::core::option::Option<any_chain_utxo::Chain>,
}
/// Nested message and enum types in `AnyChainUtxo`.
pub mod any_chain_utxo {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Chain {
        #[prost(message, tag="1")]
        Cardano(::utxorpc_spec_cardano::utxorpc::cardano::v1::TxOutput),
    }
}
/// Response containing the UTxOs associated with the requested addresses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUtxoByAddressResponse {
    /// List of UTxOs.
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<AnyChainUtxo>,
    /// Token for pagination.
    #[prost(string, tag="2")]
    pub next_token: ::prost::alloc::string::String,
}
/// Represents a reference to a UTxO.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UtxoRef {
    /// Transaction hash.
    #[prost(bytes="bytes", tag="1")]
    pub hash: ::prost::bytes::Bytes,
    /// Output index.
    #[prost(uint32, tag="2")]
    pub index: u32,
}
/// Request to get UTxOs by their references.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUtxoByRefRequest {
    /// List of UTxO references to query.
    #[prost(message, repeated, tag="1")]
    pub r#ref: ::prost::alloc::vec::Vec<UtxoRef>,
    /// Point in the chain to query from.
    #[prost(message, optional, tag="2")]
    pub acquire_point: ::core::option::Option<ChainPoint>,
}
/// Response containing the UTxOs associated with the requested references.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUtxoByRefResponse {
    /// List of UTxOs.
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<AnyChainUtxo>,
    /// Token for pagination.
    #[prost(string, tag="2")]
    pub next_token: ::prost::alloc::string::String,
}
/// Request to hold UTxOs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HoldUtxoRequest {
    /// List of UTxO references to hold.
    #[prost(message, repeated, tag="1")]
    pub refs: ::prost::alloc::vec::Vec<UtxoRef>,
}
/// Response containing information about lost UTxOs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HoldUtxoResponse {
    /// List of lost UTxO references.
    #[prost(message, repeated, tag="1")]
    pub lost: ::prost::alloc::vec::Vec<UtxoRef>,
}
/// Encoded file descriptor set for the `utxorpc.build.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xc6, 0x2a, 0x0a, 0x1c, 0x75, 0x74, 0x78, 0x6f, 0x72, 0x70, 0x63, 0x2f, 0x62, 0x75, 0x69,
    0x6c, 0x64, 0x2f, 0x76, 0x31, 0x2f, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x12, 0x10, 0x75, 0x74, 0x78, 0x6f, 0x72, 0x70, 0x63, 0x2e, 0x62, 0x75, 0x69, 0x6c, 0x64,
    0x2e, 0x76, 0x31, 0x1a, 0x20, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x62, 0x75, 0x66, 0x2f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x5f, 0x6d, 0x61, 0x73, 0x6b, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x20, 0x75, 0x74, 0x78, 0x6f, 0x72, 0x70, 0x63, 0x2f, 0x63,
    0x61, 0x72, 0x64, 0x61, 0x6e, 0x6f, 0x2f, 0x76, 0x31, 0x2f, 0x63, 0x61, 0x72, 0x64, 0x61, 0x6e,
    0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x4c, 0x0a, 0x0a, 0x43, 0x68, 0x61, 0x69, 0x6e,
    0x50, 0x6f, 0x69, 0x6e, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x73, 0x6c, 0x6f, 0x74, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x04, 0x52, 0x04, 0x73, 0x6c, 0x6f, 0x74, 0x12, 0x16, 0x0a, 0x06, 0x68, 0x65, 0x69,
    0x67, 0x68, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x06, 0x68, 0x65, 0x69, 0x67, 0x68,
    0x74, 0x12, 0x12, 0x0a, 0x04, 0x68, 0x61, 0x73, 0x68, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x52,
    0x04, 0x68, 0x61, 0x73, 0x68, 0x22, 0x14, 0x0a, 0x12, 0x47, 0x65, 0x74, 0x43, 0x68, 0x61, 0x69,
    0x6e, 0x54, 0x69, 0x70, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0x45, 0x0a, 0x13, 0x47,
    0x65, 0x74, 0x43, 0x68, 0x61, 0x69, 0x6e, 0x54, 0x69, 0x70, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x12, 0x2e, 0x0a, 0x03, 0x74, 0x69, 0x70, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x1c, 0x2e, 0x75, 0x74, 0x78, 0x6f, 0x72, 0x70, 0x63, 0x2e, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x2e,
    0x76, 0x31, 0x2e, 0x43, 0x68, 0x61, 0x69, 0x6e, 0x50, 0x6f, 0x69, 0x6e, 0x74, 0x52, 0x03, 0x74,
    0x69, 0x70, 0x22, 0x2c, 0x0a, 0x14, 0x47, 0x65, 0x74, 0x43, 0x68, 0x61, 0x69, 0x6e, 0x50, 0x61,
    0x72, 0x61, 0x6d, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x14, 0x0a, 0x05, 0x70, 0x61,
    0x72, 0x61, 0x6d, 0x18, 0x01, 0x20, 0x03, 0x28, 0x09, 0x52, 0x05, 0x70, 0x61, 0x72, 0x61, 0x6d,
    0x22, 0x34, 0x0a, 0x0a, 0x43, 0x68, 0x61, 0x69, 0x6e, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x12, 0x10,
    0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x6b, 0x65, 0x79,
    0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52,
    0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x4b, 0x0a, 0x15, 0x47, 0x65, 0x74, 0x43, 0x68, 0x61,
    0x69, 0x6e, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12,
    0x32, 0x0a, 0x05, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1c,
    0x2e, 0x75, 0x74, 0x78, 0x6f, 0x72, 0x70, 0x63, 0x2e, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x2e, 0x76,
    0x31, 0x2e, 0x43, 0x68, 0x61, 0x69, 0x6e, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x52, 0x05, 0x70, 0x61,
    0x72, 0x61, 0x6d, 0x22, 0x76, 0x0a, 0x17, 0x47, 0x65, 0x74, 0x55, 0x74, 0x78, 0x6f, 0x42, 0x79,
    0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x18,
    0x0a, 0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0c, 0x52,
    0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x12, 0x41, 0x0a, 0x0d, 0x61, 0x63, 0x71, 0x75,
    0x69, 0x72, 0x65, 0x5f, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x1c, 0x2e, 0x75, 0x74, 0x78, 0x6f, 0x72, 0x70, 0x63, 0x2e, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x2e,
    0x76, 0x31, 0x2e, 0x43, 0x68, 0x61, 0x69, 0x6e, 0x50, 0x6f, 0x69, 0x6e, 0x74, 0x52, 0x0c, 0x61,
    0x63, 0x71, 0x75, 0x69, 0x72, 0x65, 0x50, 0x6f, 0x69, 0x6e, 0x74, 0x22, 0x51, 0x0a, 0x0c, 0x41,
    0x6e, 0x79, 0x43, 0x68, 0x61, 0x69, 0x6e, 0x55, 0x74, 0x78, 0x6f, 0x12, 0x38, 0x0a, 0x07, 0x63,
    0x61, 0x72, 0x64, 0x61, 0x6e, 0x6f, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x75,
    0x74, 0x78, 0x6f, 0x72, 0x70, 0x63, 0x2e, 0x63, 0x61, 0x72, 0x64, 0x61, 0x6e, 0x6f, 0x2e, 0x76,
    0x31, 0x2e, 0x54, 0x78, 0x4f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x48, 0x00, 0x52, 0x07, 0x63, 0x61,
    0x72, 0x64, 0x61, 0x6e, 0x6f, 0x42, 0x07, 0x0a, 0x05, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x22, 0x6f,
    0x0a, 0x18, 0x47, 0x65, 0x74, 0x55, 0x74, 0x78, 0x6f, 0x42, 0x79, 0x41, 0x64, 0x64, 0x72, 0x65,
    0x73, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x34, 0x0a, 0x05, 0x69, 0x74,
    0x65, 0x6d, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x75, 0x74, 0x78, 0x6f,
    0x72, 0x70, 0x63, 0x2e, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x2e, 0x76, 0x31, 0x2e, 0x41, 0x6e, 0x79,
    0x43, 0x68, 0x61, 0x69, 0x6e, 0x55, 0x74, 0x78, 0x6f, 0x52, 0x05, 0x69, 0x74, 0x65, 0x6d, 0x73,
    0x12, 0x1d, 0x0a, 0x0a, 0x6e, 0x65, 0x78, 0x74, 0x5f, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x6e, 0x65, 0x78, 0x74, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x22,
    0x33, 0x0a, 0x07, 0x55, 0x74, 0x78, 0x6f, 0x52, 0x65, 0x66, 0x12, 0x12, 0x0a, 0x04, 0x68, 0x61,
    0x73, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x68, 0x61, 0x73, 0x68, 0x12, 0x14,
    0x0a, 0x05, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x05, 0x69,
    0x6e, 0x64, 0x65, 0x78, 0x22, 0x85, 0x01, 0x0a, 0x13, 0x47, 0x65, 0x74, 0x55, 0x74, 0x78, 0x6f,
    0x42, 0x79, 0x52, 0x65, 0x66, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x2b, 0x0a, 0x03,
    0x72, 0x65, 0x66, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x75, 0x74, 0x78, 0x6f,
    0x72, 0x70, 0x63, 0x2e, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x2e, 0x76, 0x31, 0x2e, 0x55, 0x74, 0x78,
    0x6f, 0x52, 0x65, 0x66, 0x52, 0x03, 0x72, 0x65, 0x66, 0x12, 0x41, 0x0a, 0x0d, 0x61, 0x63, 0x71,
    0x75, 0x69, 0x72, 0x65, 0x5f, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x1c, 0x2e, 0x75, 0x74, 0x78, 0x6f, 0x72, 0x70, 0x63, 0x2e, 0x62, 0x75, 0x69, 0x6c, 0x64,
    0x2e, 0x76, 0x31, 0x2e, 0x43, 0x68, 0x61, 0x69, 0x6e, 0x50, 0x6f, 0x69, 0x6e, 0x74, 0x52, 0x0c,
    0x61, 0x63, 0x71, 0x75, 0x69, 0x72, 0x65, 0x50, 0x6f, 0x69, 0x6e, 0x74, 0x22, 0x6b, 0x0a, 0x14,
    0x47, 0x65, 0x74, 0x55, 0x74, 0x78, 0x6f, 0x42, 0x79, 0x52, 0x65, 0x66, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x12, 0x34, 0x0a, 0x05, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x18, 0x01, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x75, 0x74, 0x78, 0x6f, 0x72, 0x70, 0x63, 0x2e, 0x62, 0x75,
    0x69, 0x6c, 0x64, 0x2e, 0x76, 0x31, 0x2e, 0x41, 0x6e, 0x79, 0x43, 0x68, 0x61, 0x69, 0x6e, 0x55,
    0x74, 0x78, 0x6f, 0x52, 0x05, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x12, 0x1d, 0x0a, 0x0a, 0x6e, 0x65,
    0x78, 0x74, 0x5f, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09,
    0x6e, 0x65, 0x78, 0x74, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x22, 0x40, 0x0a, 0x0f, 0x48, 0x6f, 0x6c,
    0x64, 0x55, 0x74, 0x78, 0x6f, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x2d, 0x0a, 0x04,
    0x72, 0x65, 0x66, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x75, 0x74, 0x78,
    0x6f, 0x72, 0x70, 0x63, 0x2e, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x2e, 0x76, 0x31, 0x2e, 0x55, 0x74,
    0x78, 0x6f, 0x52, 0x65, 0x66, 0x52, 0x04, 0x72, 0x65, 0x66, 0x73, 0x22, 0x41, 0x0a, 0x10, 0x48,
    0x6f, 0x6c, 0x64, 0x55, 0x74, 0x78, 0x6f, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12,
    0x2d, 0x0a, 0x04, 0x6c, 0x6f, 0x73, 0x74, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x19, 0x2e,
    0x75, 0x74, 0x78, 0x6f, 0x72, 0x70, 0x63, 0x2e, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x2e, 0x76, 0x31,
    0x2e, 0x55, 0x74, 0x78, 0x6f, 0x52, 0x65, 0x66, 0x52, 0x04, 0x6c, 0x6f, 0x73, 0x74, 0x32, 0xf1,
    0x03, 0x0a, 0x12, 0x4c, 0x65, 0x64, 0x67, 0x65, 0x72, 0x53, 0x74, 0x61, 0x74, 0x65, 0x53, 0x65,
    0x72, 0x76, 0x69, 0x63, 0x65, 0x12, 0x5a, 0x0a, 0x0b, 0x47, 0x65, 0x74, 0x43, 0x68, 0x61, 0x69,
    0x6e, 0x54, 0x69, 0x70, 0x12, 0x24, 0x2e, 0x75, 0x74, 0x78, 0x6f, 0x72, 0x70, 0x63, 0x2e, 0x62,
    0x75, 0x69, 0x6c, 0x64, 0x2e, 0x76, 0x31, 0x2e, 0x47, 0x65, 0x74, 0x43, 0x68, 0x61, 0x69, 0x6e,
    0x54, 0x69, 0x70, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x25, 0x2e, 0x75, 0x74, 0x78,
    0x6f, 0x72, 0x70, 0x63, 0x2e, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x2e, 0x76, 0x31, 0x2e, 0x47, 0x65,
    0x74, 0x43, 0x68, 0x61, 0x69, 0x6e, 0x54, 0x69, 0x70, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x12, 0x60, 0x0a, 0x0d, 0x47, 0x65, 0x74, 0x43, 0x68, 0x61, 0x69, 0x6e, 0x50, 0x61, 0x72,
    0x61, 0x6d, 0x12, 0x26, 0x2e, 0x75, 0x74, 0x78, 0x6f, 0x72, 0x70, 0x63, 0x2e, 0x62, 0x75, 0x69,
    0x6c, 0x64, 0x2e, 0x76, 0x31, 0x2e, 0x47, 0x65, 0x74, 0x43, 0x68, 0x61, 0x69, 0x6e, 0x50, 0x61,
    0x72, 0x61, 0x6d, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x27, 0x2e, 0x75, 0x74, 0x78,
    0x6f, 0x72, 0x70, 0x63, 0x2e, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x2e, 0x76, 0x31, 0x2e, 0x47, 0x65,
    0x74, 0x43, 0x68, 0x61, 0x69, 0x6e, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x12, 0x69, 0x0a, 0x10, 0x47, 0x65, 0x74, 0x55, 0x74, 0x78, 0x6f, 0x42, 0x79,
    0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x12, 0x29, 0x2e, 0x75, 0x74, 0x78, 0x6f, 0x72, 0x70,
    0x63, 0x2e, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x2e, 0x76, 0x31, 0x2e, 0x47, 0x65, 0x74, 0x55, 0x74,
    0x78, 0x6f, 0x42, 0x79, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x1a, 0x2a, 0x2e, 0x75, 0x74, 0x78, 0x6f, 0x72, 0x70, 0x63, 0x2e, 0x62, 0x75, 0x69,
    0x6c, 0x64, 0x2e, 0x76, 0x31, 0x2e, 0x47, 0x65, 0x74, 0x55, 0x74, 0x78, 0x6f, 0x42, 0x79, 0x41,
    0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x5d,
    0x0a, 0x0c, 0x47, 0x65, 0x74, 0x55, 0x74, 0x78, 0x6f, 0x42, 0x79, 0x52, 0x65, 0x66, 0x12, 0x25,
    0x2e, 0x75, 0x74, 0x78, 0x6f, 0x72, 0x70, 0x63, 0x2e, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x2e, 0x76,
    0x31, 0x2e, 0x47, 0x65, 0x74, 0x55, 0x74, 0x78, 0x6f, 0x42, 0x79, 0x52, 0x65, 0x66, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x26, 0x2e, 0x75, 0x74, 0x78, 0x6f, 0x72, 0x70, 0x63, 0x2e,
    0x62, 0x75, 0x69, 0x6c, 0x64, 0x2e, 0x76, 0x31, 0x2e, 0x47, 0x65, 0x74, 0x55, 0x74, 0x78, 0x6f,
    0x42, 0x79, 0x52, 0x65, 0x66, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x53, 0x0a,
    0x08, 0x48, 0x6f, 0x6c, 0x64, 0x55, 0x74, 0x78, 0x6f, 0x12, 0x21, 0x2e, 0x75, 0x74, 0x78, 0x6f,
    0x72, 0x70, 0x63, 0x2e, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x2e, 0x76, 0x31, 0x2e, 0x48, 0x6f, 0x6c,
    0x64, 0x55, 0x74, 0x78, 0x6f, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x22, 0x2e, 0x75,
    0x74, 0x78, 0x6f, 0x72, 0x70, 0x63, 0x2e, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x2e, 0x76, 0x31, 0x2e,
    0x48, 0x6f, 0x6c, 0x64, 0x55, 0x74, 0x78, 0x6f, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x30, 0x01, 0x42, 0xbf, 0x01, 0x0a, 0x14, 0x63, 0x6f, 0x6d, 0x2e, 0x75, 0x74, 0x78, 0x6f, 0x72,
    0x70, 0x63, 0x2e, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x2e, 0x76, 0x31, 0x42, 0x0a, 0x42, 0x75, 0x69,
    0x6c, 0x64, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x39, 0x67, 0x69, 0x74, 0x68, 0x75,
    0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x62, 0x75, 0x66, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x2f, 0x62,
    0x75, 0x66, 0x2d, 0x74, 0x6f, 0x75, 0x72, 0x2f, 0x67, 0x65, 0x6e, 0x2f, 0x75, 0x74, 0x78, 0x6f,
    0x72, 0x70, 0x63, 0x2f, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x2f, 0x76, 0x31, 0x3b, 0x62, 0x75, 0x69,
    0x6c, 0x64, 0x76, 0x31, 0xa2, 0x02, 0x03, 0x55, 0x42, 0x58, 0xaa, 0x02, 0x10, 0x55, 0x74, 0x78,
    0x6f, 0x72, 0x70, 0x63, 0x2e, 0x42, 0x75, 0x69, 0x6c, 0x64, 0x2e, 0x56, 0x31, 0xca, 0x02, 0x10,
    0x55, 0x74, 0x78, 0x6f, 0x72, 0x70, 0x63, 0x5c, 0x42, 0x75, 0x69, 0x6c, 0x64, 0x5c, 0x56, 0x31,
    0xe2, 0x02, 0x1c, 0x55, 0x74, 0x78, 0x6f, 0x72, 0x70, 0x63, 0x5c, 0x42, 0x75, 0x69, 0x6c, 0x64,
    0x5c, 0x56, 0x31, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea,
    0x02, 0x12, 0x55, 0x74, 0x78, 0x6f, 0x72, 0x70, 0x63, 0x3a, 0x3a, 0x42, 0x75, 0x69, 0x6c, 0x64,
    0x3a, 0x3a, 0x56, 0x31, 0x4a, 0xca, 0x1b, 0x0a, 0x06, 0x12, 0x04, 0x02, 0x00, 0x5e, 0x01, 0x0a,
    0x39, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x02, 0x00, 0x12, 0x32, 0x2f, 0x2f, 0x20, 0x41, 0x20, 0x63,
    0x6f, 0x6e, 0x73, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x74, 0x20, 0x76, 0x69, 0x65, 0x77, 0x20, 0x6f,
    0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x6c, 0x65, 0x64, 0x67, 0x65, 0x72, 0x0a, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12,
    0x03, 0x04, 0x00, 0x19, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x06, 0x00, 0x2a, 0x0a,
    0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x07, 0x00, 0x2a, 0x0a, 0x3c, 0x0a, 0x02, 0x04, 0x00,
    0x12, 0x04, 0x0a, 0x00, 0x0e, 0x01, 0x1a, 0x30, 0x20, 0x52, 0x65, 0x70, 0x72, 0x65, 0x73, 0x65,
    0x6e, 0x74, 0x73, 0x20, 0x61, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x63, 0x20, 0x70,
    0x6f, 0x69, 0x6e, 0x74, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x62, 0x6c, 0x6f, 0x63,
    0x6b, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12,
    0x03, 0x0a, 0x08, 0x12, 0x0a, 0x1b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0b, 0x02,
    0x12, 0x22, 0x0e, 0x20, 0x53, 0x6c, 0x6f, 0x74, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x2e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0b, 0x02, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x09, 0x0d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0b, 0x10, 0x11, 0x0a, 0x1c, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x02, 0x14, 0x22, 0x0f, 0x20, 0x42, 0x6c, 0x6f, 0x63, 0x6b,
    0x20, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x0c, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x0c, 0x09, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x0c, 0x12, 0x13, 0x0a, 0x1a, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0d, 0x02, 0x11,
    0x22, 0x0d, 0x20, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x68, 0x61, 0x73, 0x68, 0x2e, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0d, 0x02, 0x07, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0d, 0x0f, 0x10, 0x0a, 0x32, 0x0a, 0x02, 0x04, 0x01, 0x12,
    0x03, 0x11, 0x00, 0x1d, 0x1a, 0x27, 0x20, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x74,
    0x6f, 0x20, 0x67, 0x65, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e,
    0x74, 0x20, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x20, 0x74, 0x69, 0x70, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x11, 0x08, 0x1a, 0x0a, 0x38, 0x0a, 0x02, 0x04, 0x02, 0x12,
    0x04, 0x14, 0x00, 0x16, 0x01, 0x1a, 0x2c, 0x20, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x20, 0x74, 0x69,
    0x70, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x14, 0x08, 0x1b, 0x0a,
    0x21, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x15, 0x02, 0x15, 0x22, 0x14, 0x20, 0x43,
    0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x20, 0x74, 0x69, 0x70,
    0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x15, 0x02, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x15, 0x0d, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x15, 0x13, 0x14, 0x0a, 0x37, 0x0a, 0x02,
    0x04, 0x03, 0x12, 0x04, 0x19, 0x00, 0x1b, 0x01, 0x1a, 0x2b, 0x20, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x67, 0x65, 0x74, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66,
    0x69, 0x63, 0x20, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x20, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74,
    0x65, 0x72, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x19, 0x08,
    0x1c, 0x0a, 0x2c, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x1a, 0x02, 0x1c, 0x22, 0x1f,
    0x20, 0x4c, 0x69, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x65, 0x64, 0x20, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1a, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x1a, 0x1a, 0x1b, 0x0a, 0x40, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x1e,
    0x00, 0x21, 0x01, 0x1a, 0x34, 0x20, 0x52, 0x65, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x73,
    0x20, 0x61, 0x20, 0x6b, 0x65, 0x79, 0x2d, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x70, 0x61, 0x69,
    0x72, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x20, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x20, 0x70, 0x61,
    0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01,
    0x12, 0x03, 0x1e, 0x08, 0x12, 0x0a, 0x1d, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x1f,
    0x02, 0x11, 0x22, 0x10, 0x20, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x20, 0x6b,
    0x65, 0x79, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1f,
    0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1f, 0x09, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1f, 0x0f, 0x10, 0x0a, 0x1f,
    0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x20, 0x02, 0x12, 0x22, 0x12, 0x20, 0x50, 0x61,
    0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x2e, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x20, 0x02, 0x07, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x20, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x20, 0x10, 0x11, 0x0a, 0x41, 0x0a, 0x02, 0x04, 0x05, 0x12,
    0x04, 0x24, 0x00, 0x26, 0x01, 0x1a, 0x35, 0x20, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x65, 0x64, 0x20, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x20,
    0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x05, 0x01, 0x12, 0x03, 0x24, 0x08, 0x1d, 0x0a, 0x32, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00,
    0x12, 0x03, 0x25, 0x02, 0x20, 0x22, 0x25, 0x20, 0x4c, 0x69, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20,
    0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x65, 0x64, 0x20, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x20,
    0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x25, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x25, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x25, 0x16, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x25, 0x1e, 0x1f, 0x0a, 0x41, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x29, 0x00, 0x2c, 0x01,
    0x1a, 0x35, 0x20, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x67, 0x65,
    0x74, 0x20, 0x55, 0x54, 0x78, 0x4f, 0x73, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x69, 0x72,
    0x20, 0x61, 0x73, 0x73, 0x6f, 0x63, 0x69, 0x61, 0x74, 0x65, 0x64, 0x20, 0x61, 0x64, 0x64, 0x72,
    0x65, 0x73, 0x73, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03,
    0x29, 0x08, 0x1f, 0x0a, 0x2a, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x2a, 0x02, 0x1d,
    0x22, 0x1d, 0x20, 0x4c, 0x69, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65,
    0x73, 0x73, 0x65, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x71, 0x75, 0x65, 0x72, 0x79, 0x2e, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2a, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2a, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x2a, 0x1b, 0x1c, 0x0a, 0x30, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12,
    0x03, 0x2b, 0x02, 0x1f, 0x22, 0x23, 0x20, 0x50, 0x6f, 0x69, 0x6e, 0x74, 0x20, 0x69, 0x6e, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x20, 0x74, 0x6f, 0x20, 0x71, 0x75, 0x65,
    0x72, 0x79, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x01, 0x06, 0x12, 0x03, 0x2b, 0x02, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x2b, 0x0d, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x2b, 0x1d, 0x1e, 0x0a, 0x4a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x2f, 0x00, 0x33, 0x01, 0x1a,
    0x3e, 0x20, 0x41, 0x6e, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x6c, 0x6f, 0x70, 0x65, 0x20, 0x74, 0x68,
    0x61, 0x74, 0x20, 0x68, 0x6f, 0x6c, 0x64, 0x73, 0x20, 0x61, 0x6e, 0x20, 0x55, 0x54, 0x78, 0x4f,
    0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x61, 0x6e, 0x79, 0x20, 0x6f, 0x66, 0x20, 0x63, 0x6f, 0x6d,
    0x70, 0x61, 0x74, 0x69, 0x62, 0x6c, 0x65, 0x20, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x73, 0x0a, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x2f, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x07, 0x08, 0x00, 0x12, 0x04, 0x30, 0x02, 0x32, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x08,
    0x00, 0x01, 0x12, 0x03, 0x30, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12,
    0x03, 0x31, 0x04, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x06, 0x12, 0x03, 0x31,
    0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x31, 0x20, 0x27,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x31, 0x2a, 0x2b, 0x0a, 0x54,
    0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x36, 0x00, 0x39, 0x01, 0x1a, 0x48, 0x20, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x55, 0x54, 0x78, 0x4f, 0x73, 0x20, 0x61, 0x73, 0x73, 0x6f, 0x63,
    0x69, 0x61, 0x74, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x65, 0x64, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73,
    0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x36, 0x08, 0x20,
    0x0a, 0x1d, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x37, 0x02, 0x22, 0x22, 0x10, 0x20,
    0x4c, 0x69, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x55, 0x54, 0x78, 0x4f, 0x73, 0x2e, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x03, 0x37, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x00, 0x06, 0x12, 0x03, 0x37, 0x0b, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x37, 0x18, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x37, 0x20, 0x21, 0x0a, 0x24, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01, 0x12,
    0x03, 0x38, 0x02, 0x18, 0x22, 0x17, 0x20, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x20, 0x66, 0x6f, 0x72,
    0x20, 0x70, 0x61, 0x67, 0x69, 0x6e, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x01, 0x05, 0x12, 0x03, 0x38, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x01, 0x01, 0x12, 0x03, 0x38, 0x09, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x38, 0x16, 0x17, 0x0a, 0x2f, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x3c,
    0x00, 0x3f, 0x01, 0x1a, 0x23, 0x20, 0x52, 0x65, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x73,
    0x20, 0x61, 0x20, 0x72, 0x65, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x63, 0x65, 0x20, 0x74, 0x6f, 0x20,
    0x61, 0x20, 0x55, 0x54, 0x78, 0x4f, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12,
    0x03, 0x3c, 0x08, 0x0f, 0x0a, 0x20, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x03, 0x3d, 0x02,
    0x11, 0x22, 0x13, 0x20, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20,
    0x68, 0x61, 0x73, 0x68, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x3d, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3d,
    0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3d, 0x0f, 0x10,
    0x0a, 0x1c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12, 0x03, 0x3e, 0x02, 0x13, 0x22, 0x0f, 0x20,
    0x4f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x20, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x2e, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x05, 0x12, 0x03, 0x3e, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3e, 0x09, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x3e, 0x11, 0x12, 0x0a, 0x37, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x04,
    0x42, 0x00, 0x45, 0x01, 0x1a, 0x2b, 0x20, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x74,
    0x6f, 0x20, 0x67, 0x65, 0x74, 0x20, 0x55, 0x54, 0x78, 0x4f, 0x73, 0x20, 0x62, 0x79, 0x20, 0x74,
    0x68, 0x65, 0x69, 0x72, 0x20, 0x72, 0x65, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x63, 0x65, 0x73, 0x2e,
    0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x03, 0x42, 0x08, 0x1b, 0x0a, 0x30, 0x0a,
    0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x03, 0x43, 0x02, 0x1b, 0x22, 0x23, 0x20, 0x4c, 0x69, 0x73,
    0x74, 0x20, 0x6f, 0x66, 0x20, 0x55, 0x54, 0x78, 0x4f, 0x20, 0x72, 0x65, 0x66, 0x65, 0x72, 0x65,
    0x6e, 0x63, 0x65, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x71, 0x75, 0x65, 0x72, 0x79, 0x2e, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x03, 0x43, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0a, 0x02, 0x00, 0x06, 0x12, 0x03, 0x43, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0a, 0x02, 0x00, 0x01, 0x12, 0x03, 0x43, 0x13, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x43, 0x19, 0x1a, 0x0a, 0x30, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x01, 0x12,
    0x03, 0x44, 0x02, 0x1f, 0x22, 0x23, 0x20, 0x50, 0x6f, 0x69, 0x6e, 0x74, 0x20, 0x69, 0x6e, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x20, 0x74, 0x6f, 0x20, 0x71, 0x75, 0x65,
    0x72, 0x79, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02,
    0x01, 0x06, 0x12, 0x03, 0x44, 0x02, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x44, 0x0d, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x44, 0x1d, 0x1e, 0x0a, 0x55, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x04, 0x48, 0x00, 0x4b, 0x01, 0x1a,
    0x49, 0x20, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61,
    0x69, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x55, 0x54, 0x78, 0x4f, 0x73, 0x20,
    0x61, 0x73, 0x73, 0x6f, 0x63, 0x69, 0x61, 0x74, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x65, 0x64, 0x20, 0x72, 0x65,
    0x66, 0x65, 0x72, 0x65, 0x6e, 0x63, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0b,
    0x01, 0x12, 0x03, 0x48, 0x08, 0x1c, 0x0a, 0x1d, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x03,
    0x49, 0x02, 0x22, 0x22, 0x10, 0x20, 0x4c, 0x69, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x55, 0x54,
    0x78, 0x4f, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x49, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x06, 0x12, 0x03, 0x49, 0x0b,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x03, 0x49, 0x18, 0x1d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03, 0x12, 0x03, 0x49, 0x20, 0x21, 0x0a, 0x24, 0x0a,
    0x04, 0x04, 0x0b, 0x02, 0x01, 0x12, 0x03, 0x4a, 0x02, 0x18, 0x22, 0x17, 0x20, 0x54, 0x6f, 0x6b,
    0x65, 0x6e, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x70, 0x61, 0x67, 0x69, 0x6e, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x05, 0x12, 0x03, 0x4a, 0x02,
    0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x01, 0x12, 0x03, 0x4a, 0x09, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x03, 0x12, 0x03, 0x4a, 0x16, 0x17, 0x0a, 0x24, 0x0a,
    0x02, 0x04, 0x0c, 0x12, 0x04, 0x4e, 0x00, 0x50, 0x01, 0x1a, 0x18, 0x20, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x68, 0x6f, 0x6c, 0x64, 0x20, 0x55, 0x54, 0x78, 0x4f,
    0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12, 0x03, 0x4e, 0x08, 0x17, 0x0a,
    0x2f, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x00, 0x12, 0x03, 0x4f, 0x02, 0x1c, 0x22, 0x22, 0x20, 0x4c,
    0x69, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x55, 0x54, 0x78, 0x4f, 0x20, 0x72, 0x65, 0x66, 0x65,
    0x72, 0x65, 0x6e, 0x63, 0x65, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x68, 0x6f, 0x6c, 0x64, 0x2e, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x04, 0x12, 0x03, 0x4f, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x06, 0x12, 0x03, 0x4f, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4f, 0x13, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x4f, 0x1a, 0x1b, 0x0a, 0x3f, 0x0a, 0x02, 0x04, 0x0d, 0x12, 0x04,
    0x53, 0x00, 0x55, 0x01, 0x1a, 0x33, 0x20, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20,
    0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72,
    0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x61, 0x62, 0x6f, 0x75, 0x74, 0x20, 0x6c, 0x6f, 0x73,
    0x74, 0x20, 0x55, 0x54, 0x78, 0x4f, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0d, 0x01,
    0x12, 0x03, 0x53, 0x08, 0x18, 0x0a, 0x2c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00, 0x12, 0x03, 0x54,
    0x02, 0x1c, 0x22, 0x1f, 0x20, 0x4c, 0x69, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x6c, 0x6f, 0x73,
    0x74, 0x20, 0x55, 0x54, 0x78, 0x4f, 0x20, 0x72, 0x65, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x63, 0x65,
    0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x04, 0x12, 0x03, 0x54, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x06, 0x12, 0x03, 0x54, 0x0b, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x01, 0x12, 0x03, 0x54, 0x13, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x00, 0x03, 0x12, 0x03, 0x54, 0x1a, 0x1b, 0x0a, 0x46, 0x0a, 0x02, 0x06,
    0x00, 0x12, 0x04, 0x58, 0x00, 0x5e, 0x01, 0x1a, 0x3a, 0x20, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63,
    0x65, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66, 0x6f, 0x72,
    0x20, 0x71, 0x75, 0x65, 0x72, 0x79, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x74,
    0x61, 0x74, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x65, 0x64, 0x67, 0x65,
    0x72, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00, 0x01, 0x12, 0x03, 0x58, 0x08, 0x1a, 0x0a,
    0x29, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x03, 0x59, 0x02, 0x44, 0x22, 0x1c, 0x20, 0x47,
    0x65, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x63,
    0x68, 0x61, 0x69, 0x6e, 0x20, 0x74, 0x69, 0x70, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x59, 0x06, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00,
    0x02, 0x12, 0x03, 0x59, 0x12, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x59, 0x2f, 0x42, 0x0a, 0x2d, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x01, 0x12, 0x03, 0x5a, 0x02,
    0x4a, 0x22, 0x20, 0x20, 0x47, 0x65, 0x74, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x63,
    0x20, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x20, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72,
    0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x5a, 0x06,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x5a, 0x14, 0x28, 0x0a,
    0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x5a, 0x33, 0x48, 0x0a, 0x37, 0x0a,
    0x04, 0x06, 0x00, 0x02, 0x02, 0x12, 0x03, 0x5b, 0x02, 0x53, 0x22, 0x2a, 0x20, 0x47, 0x65, 0x74,
    0x20, 0x55, 0x54, 0x78, 0x4f, 0x73, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x69, 0x72, 0x20,
    0x61, 0x73, 0x73, 0x6f, 0x63, 0x69, 0x61, 0x74, 0x65, 0x64, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65,
    0x73, 0x73, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x5b, 0x06, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x5b,
    0x17, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x5b, 0x39, 0x51,
    0x0a, 0x2d, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x03, 0x12, 0x03, 0x5c, 0x02, 0x47, 0x22, 0x20, 0x20,
    0x47, 0x65, 0x74, 0x20, 0x55, 0x54, 0x78, 0x4f, 0x73, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65,
    0x69, 0x72, 0x20, 0x72, 0x65, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x63, 0x65, 0x73, 0x2e, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x5c, 0x06, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x06, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x5c, 0x13, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x06,
    0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x5c, 0x31, 0x45, 0x0a, 0x3e, 0x0a, 0x04, 0x06, 0x00, 0x02,
    0x04, 0x12, 0x03, 0x5d, 0x02, 0x42, 0x22, 0x31, 0x20, 0x48, 0x6f, 0x6c, 0x64, 0x20, 0x55, 0x54,
    0x78, 0x4f, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x72, 0x65, 0x63, 0x65, 0x69, 0x76, 0x65, 0x20,
    0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x73, 0x20, 0x61, 0x62, 0x6f, 0x75, 0x74, 0x20, 0x6c, 0x6f,
    0x73, 0x74, 0x20, 0x55, 0x54, 0x78, 0x4f, 0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02,
    0x04, 0x01, 0x12, 0x03, 0x5d, 0x06, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x04, 0x02,
    0x12, 0x03, 0x5d, 0x0f, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x04, 0x06, 0x12, 0x03,
    0x5d, 0x29, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x5d, 0x30,
    0x40, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("utxorpc.build.v1.serde.rs");
include!("utxorpc.build.v1.tonic.rs");
// @@protoc_insertion_point(module)