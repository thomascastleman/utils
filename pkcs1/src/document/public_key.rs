//! PKCS#1 RSA public key document.

use crate::{error, Error, Result, RsaPublicKey};
use alloc::{borrow::ToOwned, vec::Vec};
use core::{
    convert::{TryFrom, TryInto},
    fmt,
};
use der::Encodable;

#[cfg(feature = "std")]
use std::{fs, path::Path, str};

#[cfg(feature = "pem")]
use {
    crate::{pem, public_key::PEM_TYPE_LABEL},
    alloc::string::String,
    core::str::FromStr,
};

/// PKCS#1 `RSA PUBLIC KEY` document.
///
/// This type provides storage for [`RsaPublicKey`] encoded as ASN.1
/// DER with the invariant that the contained-document is "well-formed", i.e.
/// it will parse successfully according to this crate's parsing rules.
#[derive(Clone)]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub struct RsaPublicKeyDocument(Vec<u8>);

impl RsaPublicKeyDocument {
    /// Parse the [`RsaPublicKey`] contained in this [`RsaPublicKeyDocument`]
    pub fn public_key(&self) -> RsaPublicKey<'_> {
        RsaPublicKey::try_from(self.0.as_slice()).expect("malformed PublicKeyDocument")
    }

    /// Parse [`RsaPublicKeyDocument`] from ASN.1 DER
    pub fn from_der(bytes: &[u8]) -> Result<Self> {
        bytes.try_into()
    }

    /// Parse [`RsaPublicKeyDocument`] from PEM
    ///
    /// PEM-encoded public keys can be identified by the leading delimiter:
    ///
    /// ```text
    /// -----BEGIN RSA PUBLIC KEY-----
    /// ```
    #[cfg(feature = "pem")]
    #[cfg_attr(docsrs, doc(cfg(feature = "pem")))]
    pub fn from_pem(s: &str) -> Result<Self> {
        let (label, der_bytes) = pem::decode_vec(s.as_bytes())?;

        if label != PEM_TYPE_LABEL {
            return Err(pem::Error::Label.into());
        }

        Self::from_der(&*der_bytes)
    }

    /// Serialize [`RsaPublicKeyDocument`] as PEM-encoded PKCS#8 string.
    #[cfg(feature = "pem")]
    #[cfg_attr(docsrs, doc(cfg(feature = "pem")))]
    pub fn to_pem(&self) -> String {
        pem::encode_string(PEM_TYPE_LABEL, &self.0).expect(error::PEM_ENCODING_MSG)
    }

    /// Load [`RsaPublicKeyDocument`] from an ASN.1 DER-encoded file on the local
    /// filesystem (binary format).
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn read_der_file(path: impl AsRef<Path>) -> Result<Self> {
        fs::read(path)?.try_into()
    }

    /// Load [`RsaPublicKeyDocument`] from a PEM-encoded file on the local filesystem.
    #[cfg(all(feature = "pem", feature = "std"))]
    #[cfg_attr(docsrs, doc(cfg(feature = "pem")))]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn read_pem_file(path: impl AsRef<Path>) -> Result<Self> {
        Self::from_pem(&fs::read_to_string(path)?)
    }

    /// Write ASN.1 DER-encoded public key to the given path
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn write_der_file(&self, path: impl AsRef<Path>) -> Result<()> {
        fs::write(path, self.as_ref())?;
        Ok(())
    }

    /// Write PEM-encoded public key to the given path
    #[cfg(all(feature = "pem", feature = "std"))]
    #[cfg_attr(docsrs, doc(cfg(feature = "pem")))]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn write_pem_file(&self, path: impl AsRef<Path>) -> Result<()> {
        fs::write(path, self.to_pem().as_bytes())?;
        Ok(())
    }
}

impl AsRef<[u8]> for RsaPublicKeyDocument {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl From<RsaPublicKey<'_>> for RsaPublicKeyDocument {
    fn from(spki: RsaPublicKey<'_>) -> RsaPublicKeyDocument {
        RsaPublicKeyDocument::from(&spki)
    }
}

impl From<&RsaPublicKey<'_>> for RsaPublicKeyDocument {
    fn from(spki: &RsaPublicKey<'_>) -> RsaPublicKeyDocument {
        spki.to_vec()
            .ok()
            .and_then(|buf| buf.try_into().ok())
            .expect(error::DER_ENCODING_MSG)
    }
}

impl TryFrom<&[u8]> for RsaPublicKeyDocument {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        // Ensure document is well-formed
        RsaPublicKey::try_from(bytes)?;
        Ok(Self(bytes.to_owned()))
    }
}

impl TryFrom<Vec<u8>> for RsaPublicKeyDocument {
    type Error = Error;

    fn try_from(bytes: Vec<u8>) -> Result<Self> {
        // Ensure document is well-formed
        RsaPublicKey::try_from(bytes.as_slice())?;
        Ok(Self(bytes))
    }
}

impl fmt::Debug for RsaPublicKeyDocument {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_tuple("RsaPublicKeyDocument")
            .field(&self.public_key())
            .finish()
    }
}

#[cfg(feature = "pem")]
#[cfg_attr(docsrs, doc(cfg(feature = "pem")))]
impl FromStr for RsaPublicKeyDocument {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_pem(s)
    }
}