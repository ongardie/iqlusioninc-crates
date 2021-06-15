//! Error type.

use core::fmt::{self, Display};

/// Result type.
pub type Result<T> = core::result::Result<T, Error>;

/// Error type.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// Base58 errors.
    Base58,

    /// Cryptographic errors.
    Crypto,

    /// Decoding errors (not related to Base58).
    Decode,

    /// Maximum derivation depth exceeded.
    Depth,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Base58 => f.write_str("base58 error"),
            Error::Crypto => f.write_str("cryptographic error"),
            Error::Decode => f.write_str("decoding error"),
            Error::Depth => f.write_str("maximum derivation depth exceeded"),
        }
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl std::error::Error for Error {}

impl From<bs58::decode::Error> for Error {
    fn from(_: bs58::decode::Error) -> Error {
        Error::Base58
    }
}

impl From<bs58::encode::Error> for Error {
    fn from(_: bs58::encode::Error) -> Error {
        Error::Base58
    }
}

impl From<core::array::TryFromSliceError> for Error {
    fn from(_: core::array::TryFromSliceError) -> Error {
        Error::Decode
    }
}

impl From<hmac::crypto_mac::InvalidKeyLength> for Error {
    fn from(_: hmac::crypto_mac::InvalidKeyLength) -> Error {
        Error::Crypto
    }
}

#[cfg(feature = "secp256k1")]
#[cfg_attr(docsrs, doc(cfg(feature = "secp256k1")))]
impl From<k256::elliptic_curve::Error> for Error {
    fn from(_: k256::elliptic_curve::Error) -> Error {
        Error::Crypto
    }
}

#[cfg(feature = "secp256k1")]
#[cfg_attr(docsrs, doc(cfg(feature = "secp256k1")))]
impl From<k256::ecdsa::Error> for Error {
    fn from(_: k256::ecdsa::Error) -> Error {
        Error::Crypto
    }
}