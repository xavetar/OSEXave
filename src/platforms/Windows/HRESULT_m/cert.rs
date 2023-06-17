/*
 * Copyright 2023 Stanislav Mikhailov (xavetar)
 *
 * Licensed under the Creative Commons Zero v1.0 Universal (CC0) License.
 * You may obtain a copy of the License at
 *
 *     http://creativecommons.org/publicdomain/zero/1.0/
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the CC0 license is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// It is strictly forbidden to use the from_code method, respected Microsoft decided to define
// duplicate codes, but by defining different constant names, therefore, when using these codes,
// it is mandatory to use through from_name, and not from_code. Otherwise, it may cause undefined
// behavior or an unknown exception. Because one code corresponds to several constants.

use super::{RawError};

#[derive(Clone, Debug, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CERT {
    TRUST_E_PROVIDER_UNKNOWN,
    TRUST_E_ACTION_UNKNOWN,
    TRUST_E_SUBJECT_FORM_UNKNOWN,
    TRUST_E_SUBJECT_NOT_TRUSTED,
    DIGSIG_E_ENCODE,
    DIGSIG_E_DECODE,
    DIGSIG_E_EXTENSIBILITY,
    DIGSIG_E_CRYPTO,
    PERSIST_E_SIZEDEFINITE,
    PERSIST_E_SIZEINDEFINITE,
    PERSIST_E_NOTSELFSIZING,
    TRUST_E_NOSIGNATURE,
    CERT_E_EXPIRED,
    CERT_E_VALIDITYPERIODNESTING,
    CERT_E_ROLE,
    CERT_E_PATHLENCONST,
    CERT_E_CRITICAL,
    CERT_E_PURPOSE,
    CERT_E_ISSUERCHAINING,
    CERT_E_MALFORMED,
    CERT_E_UNTRUSTEDROOT,
    CERT_E_CHAINING,
    TRUST_E_FAIL,
    CERT_E_REVOKED,
    CERT_E_UNTRUSTEDTESTROOT,
    CERT_E_REVOCATION_FAILURE,
    CERT_E_CN_NO_MATCH,
    CERT_E_WRONG_USAGE,
    TRUST_E_EXPLICIT_DISTRUST,
    CERT_E_UNTRUSTEDCA,
    CERT_E_INVALID_POLICY,
    CERT_E_INVALID_NAME,
}

impl CERT {
    pub fn code(&self) -> u32 {
        return match self {
            CERT::TRUST_E_PROVIDER_UNKNOWN => 0x800B0001 as u32,
            CERT::TRUST_E_ACTION_UNKNOWN => 0x800B0002 as u32,
            CERT::TRUST_E_SUBJECT_FORM_UNKNOWN => 0x800B0003 as u32,
            CERT::TRUST_E_SUBJECT_NOT_TRUSTED => 0x800B0004 as u32,
            CERT::DIGSIG_E_ENCODE => 0x800B0005 as u32,
            CERT::DIGSIG_E_DECODE => 0x800B0006 as u32,
            CERT::DIGSIG_E_EXTENSIBILITY => 0x800B0007 as u32,
            CERT::DIGSIG_E_CRYPTO => 0x800B0008 as u32,
            CERT::PERSIST_E_SIZEDEFINITE => 0x800B0009 as u32,
            CERT::PERSIST_E_SIZEINDEFINITE => 0x800B000A as u32,
            CERT::PERSIST_E_NOTSELFSIZING => 0x800B000B as u32,
            CERT::TRUST_E_NOSIGNATURE => 0x800B0100 as u32,
            CERT::CERT_E_EXPIRED => 0x800B0101 as u32,
            CERT::CERT_E_VALIDITYPERIODNESTING => 0x800B0102 as u32,
            CERT::CERT_E_ROLE => 0x800B0103 as u32,
            CERT::CERT_E_PATHLENCONST => 0x800B0104 as u32,
            CERT::CERT_E_CRITICAL => 0x800B0105 as u32,
            CERT::CERT_E_PURPOSE => 0x800B0106 as u32,
            CERT::CERT_E_ISSUERCHAINING => 0x800B0107 as u32,
            CERT::CERT_E_MALFORMED => 0x800B0108 as u32,
            CERT::CERT_E_UNTRUSTEDROOT => 0x800B0109 as u32,
            CERT::CERT_E_CHAINING => 0x800B010A as u32,
            CERT::TRUST_E_FAIL => 0x800B010B as u32,
            CERT::CERT_E_REVOKED => 0x800B010C as u32,
            CERT::CERT_E_UNTRUSTEDTESTROOT => 0x800B010D as u32,
            CERT::CERT_E_REVOCATION_FAILURE => 0x800B010E as u32,
            CERT::CERT_E_CN_NO_MATCH => 0x800B010F as u32,
            CERT::CERT_E_WRONG_USAGE => 0x800B0110 as u32,
            CERT::TRUST_E_EXPLICIT_DISTRUST => 0x800B0111 as u32,
            CERT::CERT_E_UNTRUSTEDCA => 0x800B0112 as u32,
            CERT::CERT_E_INVALID_POLICY => 0x800B0113 as u32,
            CERT::CERT_E_INVALID_NAME => 0x800B0114 as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            CERT::TRUST_E_PROVIDER_UNKNOWN => RawError::Kind(CERT::TRUST_E_PROVIDER_UNKNOWN),
            CERT::TRUST_E_ACTION_UNKNOWN => RawError::Kind(CERT::TRUST_E_ACTION_UNKNOWN),
            CERT::TRUST_E_SUBJECT_FORM_UNKNOWN => RawError::Kind(CERT::TRUST_E_SUBJECT_FORM_UNKNOWN),
            CERT::TRUST_E_SUBJECT_NOT_TRUSTED => RawError::Kind(CERT::TRUST_E_SUBJECT_NOT_TRUSTED),
            CERT::DIGSIG_E_ENCODE => RawError::Kind(CERT::DIGSIG_E_ENCODE),
            CERT::DIGSIG_E_DECODE => RawError::Kind(CERT::DIGSIG_E_DECODE),
            CERT::DIGSIG_E_EXTENSIBILITY => RawError::Kind(CERT::DIGSIG_E_EXTENSIBILITY),
            CERT::DIGSIG_E_CRYPTO => RawError::Kind(CERT::DIGSIG_E_CRYPTO),
            CERT::PERSIST_E_SIZEDEFINITE => RawError::Kind(CERT::PERSIST_E_SIZEDEFINITE),
            CERT::PERSIST_E_SIZEINDEFINITE => RawError::Kind(CERT::PERSIST_E_SIZEINDEFINITE),
            CERT::PERSIST_E_NOTSELFSIZING => RawError::Kind(CERT::PERSIST_E_NOTSELFSIZING),
            CERT::TRUST_E_NOSIGNATURE => RawError::Kind(CERT::TRUST_E_NOSIGNATURE),
            CERT::CERT_E_EXPIRED => RawError::Kind(CERT::CERT_E_EXPIRED),
            CERT::CERT_E_VALIDITYPERIODNESTING => RawError::Kind(CERT::CERT_E_VALIDITYPERIODNESTING),
            CERT::CERT_E_ROLE => RawError::Kind(CERT::CERT_E_ROLE),
            CERT::CERT_E_PATHLENCONST => RawError::Kind(CERT::CERT_E_PATHLENCONST),
            CERT::CERT_E_CRITICAL => RawError::Kind(CERT::CERT_E_CRITICAL),
            CERT::CERT_E_PURPOSE => RawError::Kind(CERT::CERT_E_PURPOSE),
            CERT::CERT_E_ISSUERCHAINING => RawError::Kind(CERT::CERT_E_ISSUERCHAINING),
            CERT::CERT_E_MALFORMED => RawError::Kind(CERT::CERT_E_MALFORMED),
            CERT::CERT_E_UNTRUSTEDROOT => RawError::Kind(CERT::CERT_E_UNTRUSTEDROOT),
            CERT::CERT_E_CHAINING => RawError::Kind(CERT::CERT_E_CHAINING),
            CERT::TRUST_E_FAIL => RawError::Kind(CERT::TRUST_E_FAIL),
            CERT::CERT_E_REVOKED => RawError::Kind(CERT::CERT_E_REVOKED),
            CERT::CERT_E_UNTRUSTEDTESTROOT => RawError::Kind(CERT::CERT_E_UNTRUSTEDTESTROOT),
            CERT::CERT_E_REVOCATION_FAILURE => RawError::Kind(CERT::CERT_E_REVOCATION_FAILURE),
            CERT::CERT_E_CN_NO_MATCH => RawError::Kind(CERT::CERT_E_CN_NO_MATCH),
            CERT::CERT_E_WRONG_USAGE => RawError::Kind(CERT::CERT_E_WRONG_USAGE),
            CERT::TRUST_E_EXPLICIT_DISTRUST => RawError::Kind(CERT::TRUST_E_EXPLICIT_DISTRUST),
            CERT::CERT_E_UNTRUSTEDCA => RawError::Kind(CERT::CERT_E_UNTRUSTEDCA),
            CERT::CERT_E_INVALID_POLICY => RawError::Kind(CERT::CERT_E_INVALID_POLICY),
            CERT::CERT_E_INVALID_NAME => RawError::Kind(CERT::CERT_E_INVALID_NAME),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            CERT::TRUST_E_PROVIDER_UNKNOWN => "Unknown trust provider.",
            CERT::TRUST_E_ACTION_UNKNOWN => "The trust verification action specified is not supported by the specified trust provider.",
            CERT::TRUST_E_SUBJECT_FORM_UNKNOWN => "The form specified for the subject is not one supported or known by the specified trust provider.",
            CERT::TRUST_E_SUBJECT_NOT_TRUSTED => "The subject is not trusted for the specified action.",
            CERT::DIGSIG_E_ENCODE => "Error due to problem in ASN.1 encoding process.",
            CERT::DIGSIG_E_DECODE => "Error due to problem in ASN.1 decoding process.",
            CERT::DIGSIG_E_EXTENSIBILITY => "Reading / writing Extensions where Attributes are appropriate, and vice versa.",
            CERT::DIGSIG_E_CRYPTO => "Unspecified cryptographic failure.",
            CERT::PERSIST_E_SIZEDEFINITE => "The size of the data could not be determined.",
            CERT::PERSIST_E_SIZEINDEFINITE => "The size of the indefinite-sized data could not be determined.",
            CERT::PERSIST_E_NOTSELFSIZING => "This object does not read and write self-sizing data.",
            CERT::TRUST_E_NOSIGNATURE => "No signature was present in the subject.",
            CERT::CERT_E_EXPIRED => "A required certificate is not within its validity period when verifying against the current system clock or the timestamp in the signed file.",
            CERT::CERT_E_VALIDITYPERIODNESTING => "The validity periods of the certification chain do not nest correctly.",
            CERT::CERT_E_ROLE => "A certificate that can only be used as an end-entity is being used as a CA or vice versa.",
            CERT::CERT_E_PATHLENCONST => "A path length constraint in the certification chain has been violated.",
            CERT::CERT_E_CRITICAL => "A certificate contains an unknown extension that is marked 'critical'.",
            CERT::CERT_E_PURPOSE => "A certificate being used for a purpose other than the ones specified by its CA.",
            CERT::CERT_E_ISSUERCHAINING => "A parent of a given certificate in fact did not issue that child certificate.",
            CERT::CERT_E_MALFORMED => "A certificate is missing or has an empty value for an important field, such as a subject or issuer name.",
            CERT::CERT_E_UNTRUSTEDROOT => "A certificate chain processed, but terminated in a root certificate which is not trusted by the trust provider.",
            CERT::CERT_E_CHAINING => "A certificate chain could not be built to a trusted root authority.",
            CERT::TRUST_E_FAIL => "Generic trust failure.",
            CERT::CERT_E_REVOKED => "A certificate was explicitly revoked by its issuer.",
            CERT::CERT_E_UNTRUSTEDTESTROOT => "The certification path terminates with the test root which is not trusted with the current policy settings.",
            CERT::CERT_E_REVOCATION_FAILURE => "The revocation process could not continue - the certificate(s) could not be checked.",
            CERT::CERT_E_CN_NO_MATCH => "The certificate's CN name does not match the passed value.",
            CERT::CERT_E_WRONG_USAGE => "The certificate is not valid for the requested usage.",
            CERT::TRUST_E_EXPLICIT_DISTRUST => "The certificate was explicitly marked as untrusted by the user.",
            CERT::CERT_E_UNTRUSTEDCA => "A certification chain processed correctly, but one of the CA certificates is not trusted by the policy provider.",
            CERT::CERT_E_INVALID_POLICY => "The certificate has invalid policy.",
            CERT::CERT_E_INVALID_NAME => "The certificate has an invalid name. The name is not included in the permitted list or is explicitly excluded.",
        }
    }

    pub fn from_name(name: &str) -> CERT {
        return match name {
            "TRUST_E_PROVIDER_UNKNOWN" => CERT::TRUST_E_PROVIDER_UNKNOWN,
            "TRUST_E_ACTION_UNKNOWN" => CERT::TRUST_E_ACTION_UNKNOWN,
            "TRUST_E_SUBJECT_FORM_UNKNOWN" => CERT::TRUST_E_SUBJECT_FORM_UNKNOWN,
            "TRUST_E_SUBJECT_NOT_TRUSTED" => CERT::TRUST_E_SUBJECT_NOT_TRUSTED,
            "DIGSIG_E_ENCODE" => CERT::DIGSIG_E_ENCODE,
            "DIGSIG_E_DECODE" => CERT::DIGSIG_E_DECODE,
            "DIGSIG_E_EXTENSIBILITY" => CERT::DIGSIG_E_EXTENSIBILITY,
            "DIGSIG_E_CRYPTO" => CERT::DIGSIG_E_CRYPTO,
            "PERSIST_E_SIZEDEFINITE" => CERT::PERSIST_E_SIZEDEFINITE,
            "PERSIST_E_SIZEINDEFINITE" => CERT::PERSIST_E_SIZEINDEFINITE,
            "PERSIST_E_NOTSELFSIZING" => CERT::PERSIST_E_NOTSELFSIZING,
            "TRUST_E_NOSIGNATURE" => CERT::TRUST_E_NOSIGNATURE,
            "CERT_E_EXPIRED" => CERT::CERT_E_EXPIRED,
            "CERT_E_VALIDITYPERIODNESTING" => CERT::CERT_E_VALIDITYPERIODNESTING,
            "CERT_E_ROLE" => CERT::CERT_E_ROLE,
            "CERT_E_PATHLENCONST" => CERT::CERT_E_PATHLENCONST,
            "CERT_E_CRITICAL" => CERT::CERT_E_CRITICAL,
            "CERT_E_PURPOSE" => CERT::CERT_E_PURPOSE,
            "CERT_E_ISSUERCHAINING" => CERT::CERT_E_ISSUERCHAINING,
            "CERT_E_MALFORMED" => CERT::CERT_E_MALFORMED,
            "CERT_E_UNTRUSTEDROOT" => CERT::CERT_E_UNTRUSTEDROOT,
            "CERT_E_CHAINING" => CERT::CERT_E_CHAINING,
            "TRUST_E_FAIL" => CERT::TRUST_E_FAIL,
            "CERT_E_REVOKED" => CERT::CERT_E_REVOKED,
            "CERT_E_UNTRUSTEDTESTROOT" => CERT::CERT_E_UNTRUSTEDTESTROOT,
            "CERT_E_REVOCATION_FAILURE" => CERT::CERT_E_REVOCATION_FAILURE,
            "CERT_E_CN_NO_MATCH" => CERT::CERT_E_CN_NO_MATCH,
            "CERT_E_WRONG_USAGE" => CERT::CERT_E_WRONG_USAGE,
            "TRUST_E_EXPLICIT_DISTRUST" => CERT::TRUST_E_EXPLICIT_DISTRUST,
            "CERT_E_UNTRUSTEDCA" => CERT::CERT_E_UNTRUSTEDCA,
            "CERT_E_INVALID_POLICY" => CERT::CERT_E_INVALID_POLICY,
            "CERT_E_INVALID_NAME" => CERT::CERT_E_INVALID_NAME,
        }
    }
    pub fn from_code(code: u32) -> CERT {
        return match code {
            0x800B0001 => CERT::TRUST_E_PROVIDER_UNKNOWN,
            0x800B0002 => CERT::TRUST_E_ACTION_UNKNOWN,
            0x800B0003 => CERT::TRUST_E_SUBJECT_FORM_UNKNOWN,
            0x800B0004 => CERT::TRUST_E_SUBJECT_NOT_TRUSTED,
            0x800B0005 => CERT::DIGSIG_E_ENCODE,
            0x800B0006 => CERT::DIGSIG_E_DECODE,
            0x800B0007 => CERT::DIGSIG_E_EXTENSIBILITY,
            0x800B0008 => CERT::DIGSIG_E_CRYPTO,
            0x800B0009 => CERT::PERSIST_E_SIZEDEFINITE,
            0x800B000A => CERT::PERSIST_E_SIZEINDEFINITE,
            0x800B000B => CERT::PERSIST_E_NOTSELFSIZING,
            0x800B0100 => CERT::TRUST_E_NOSIGNATURE,
            0x800B0101 => CERT::CERT_E_EXPIRED,
            0x800B0102 => CERT::CERT_E_VALIDITYPERIODNESTING,
            0x800B0103 => CERT::CERT_E_ROLE,
            0x800B0104 => CERT::CERT_E_PATHLENCONST,
            0x800B0105 => CERT::CERT_E_CRITICAL,
            0x800B0106 => CERT::CERT_E_PURPOSE,
            0x800B0107 => CERT::CERT_E_ISSUERCHAINING,
            0x800B0108 => CERT::CERT_E_MALFORMED,
            0x800B0109 => CERT::CERT_E_UNTRUSTEDROOT,
            0x800B010A => CERT::CERT_E_CHAINING,
            0x800B010B => CERT::TRUST_E_FAIL,
            0x800B010C => CERT::CERT_E_REVOKED,
            0x800B010D => CERT::CERT_E_UNTRUSTEDTESTROOT,
            0x800B010E => CERT::CERT_E_REVOCATION_FAILURE,
            0x800B010F => CERT::CERT_E_CN_NO_MATCH,
            0x800B0110 => CERT::CERT_E_WRONG_USAGE,
            0x800B0111 => CERT::TRUST_E_EXPLICIT_DISTRUST,
            0x800B0112 => CERT::CERT_E_UNTRUSTEDCA,
            0x800B0113 => CERT::CERT_E_INVALID_POLICY,
            0x800B0114 => CERT::CERT_E_INVALID_NAME,
        }
    }
}
