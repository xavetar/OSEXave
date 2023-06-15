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

pub enum CERT {
    TRUST_E_PROVIDER_UNKNOWN = 0x800B0001,
    TRUST_E_ACTION_UNKNOWN = 0x800B0002,
    TRUST_E_SUBJECT_FORM_UNKNOWN = 0x800B0003,
    TRUST_E_SUBJECT_NOT_TRUSTED = 0x800B0004,
    DIGSIG_E_ENCODE = 0x800B0005,
    DIGSIG_E_DECODE = 0x800B0006,
    DIGSIG_E_EXTENSIBILITY = 0x800B0007,
    DIGSIG_E_CRYPTO = 0x800B0008,
    PERSIST_E_SIZEDEFINITE = 0x800B0009,
    PERSIST_E_SIZEINDEFINITE = 0x800B000A,
    PERSIST_E_NOTSELFSIZING = 0x800B000B,
    TRUST_E_NOSIGNATURE = 0x800B0100,
    CERT_E_EXPIRED = 0x800B0101,
    CERT_E_VALIDITYPERIODNESTING = 0x800B0102,
    CERT_E_ROLE = 0x800B0103,
    CERT_E_PATHLENCONST = 0x800B0104,
    CERT_E_CRITICAL = 0x800B0105,
    CERT_E_PURPOSE = 0x800B0106,
    CERT_E_ISSUERCHAINING = 0x800B0107,
    CERT_E_MALFORMED = 0x800B0108,
    CERT_E_UNTRUSTEDROOT = 0x800B0109,
    CERT_E_CHAINING = 0x800B010A,
    TRUST_E_FAIL = 0x800B010B,
    CERT_E_REVOKED = 0x800B010C,
    CERT_E_UNTRUSTEDTESTROOT = 0x800B010D,
    CERT_E_REVOCATION_FAILURE = 0x800B010E,
    CERT_E_CN_NO_MATCH = 0x800B010F,
    CERT_E_WRONG_USAGE = 0x800B0110,
    TRUST_E_EXPLICIT_DISTRUST = 0x800B0111,
    CERT_E_UNTRUSTEDCA = 0x800B0112,
    CERT_E_INVALID_POLICY = 0x800B0113,
    CERT_E_INVALID_NAME = 0x800B0114,
}

impl CERT {
    pub fn description(&self) -> &'static str {
        match self {
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
}
