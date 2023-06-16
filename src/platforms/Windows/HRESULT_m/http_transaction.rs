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

pub enum HTTP_TRANSACTION {
    HTTP_E_STATUS_UNEXPECTED = 0x80190001,
    MREGISTER_E_DEVICE_MESSAGE_FORMAT_ERROR = 0x80190001,
    MREGISTER_E_DEVICE_AUTHENTICATION_ERROR = 0x80190002,
    HTTP_E_STATUS_UNEXPECTED_REDIRECTION = 0x80190003,
    MREGISTER_E_DEVICE_AUTHORIZATION_ERROR = 0x80190003,
    HTTP_E_STATUS_UNEXPECTED_CLIENT_ERROR = 0x80190004,
    MREGISTER_E_DEVICE_CERTIFCATEREQUEST_ERROR = 0x80190004,
    HTTP_E_STATUS_UNEXPECTED_SERVER_ERROR = 0x80190005,
    MREGISTER_E_DEVICE_CONFIGMGRSERVER_ERROR = 0x80190005,
    MREGISTER_E_DEVICE_INTERNALSERVICE_ERROR = 0x80190006,
    MREGISTER_E_DEVICE_INVALIDSECURITY_ERROR = 0x80190007,
    MREGISTER_E_DEVICE_UNKNOWN_ERROR = 0x80190008,
    MREGISTER_E_REGISTRATION_IN_PROGRESS = 0x80190009,
    MREGISTER_E_DEVICE_ALREADY_REGISTERED = 0x8019000A,
    MREGISTER_E_DEVICE_NOT_REGISTERED = 0x8019000B,
    MREGISTER_E_DISCOVERY_REDIRECTED = 0x8019000C,
    MREGISTER_E_DEVICE_NOT_AD_REGISTERED_ERROR = 0x8019000D,
    MREGISTER_E_DISCOVERY_FAILED = 0x8019000E,
    HTTP_E_STATUS_AMBIGUOUS = 0x8019012C,
    HTTP_E_STATUS_MOVED = 0x8019012D,
    HTTP_E_STATUS_REDIRECT = 0x8019012E,
    HTTP_E_STATUS_REDIRECT_METHOD = 0x8019012F,
    HTTP_E_STATUS_NOT_MODIFIED = 0x80190130,
    HTTP_E_STATUS_USE_PROXY = 0x80190131,
    HTTP_E_STATUS_REDIRECT_KEEP_VERB = 0x80190133,
    HTTP_E_STATUS_BAD_REQUEST = 0x80190190,
    HTTP_E_STATUS_DENIED = 0x80190191,
    HTTP_E_STATUS_PAYMENT_REQ = 0x80190192,
    HTTP_E_STATUS_FORBIDDEN = 0x80190193,
    HTTP_E_STATUS_NOT_FOUND = 0x80190194,
    HTTP_E_STATUS_BAD_METHOD = 0x80190195,
    HTTP_E_STATUS_NONE_ACCEPTABLE = 0x80190196,
    HTTP_E_STATUS_PROXY_AUTH_REQ = 0x80190197,
    HTTP_E_STATUS_REQUEST_TIMEOUT = 0x80190198,
    HTTP_E_STATUS_CONFLICT = 0x80190199,
    HTTP_E_STATUS_GONE = 0x8019019A,
    HTTP_E_STATUS_LENGTH_REQUIRED = 0x8019019B,
    HTTP_E_STATUS_PRECOND_FAILED = 0x8019019C,
    HTTP_E_STATUS_REQUEST_TOO_LARGE = 0x8019019D,
    HTTP_E_STATUS_URI_TOO_LONG = 0x8019019E,
    HTTP_E_STATUS_UNSUPPORTED_MEDIA = 0x8019019F,
    HTTP_E_STATUS_RANGE_NOT_SATISFIABLE = 0x801901A0,
    HTTP_E_STATUS_EXPECTATION_FAILED = 0x801901A1,
    HTTP_E_STATUS_SERVER_ERROR = 0x801901F4,
    HTTP_E_STATUS_NOT_SUPPORTED = 0x801901F5,
    HTTP_E_STATUS_BAD_GATEWAY = 0x801901F6,
    HTTP_E_STATUS_SERVICE_UNAVAIL = 0x801901F7,
    HTTP_E_STATUS_GATEWAY_TIMEOUT = 0x801901F8,
    HTTP_E_STATUS_VERSION_NOT_SUP = 0x801901F9,
}

impl HTTP_TRANSACTION {
    pub fn description(&self) -> &'static str {
        match self {
            HTTP_TRANSACTION::HTTP_E_STATUS_UNEXPECTED => "Unexpected HTTP status code.",
            HTTP_TRANSACTION::MREGISTER_E_DEVICE_MESSAGE_FORMAT_ERROR => "Windows MDE: Invalid Schema, Message Format Error from server.",
            HTTP_TRANSACTION::MREGISTER_E_DEVICE_AUTHENTICATION_ERROR => "Windows MDE: Server failed to authenticate the user.",
            HTTP_TRANSACTION::HTTP_E_STATUS_UNEXPECTED_REDIRECTION => "Unexpected redirection status code (3xx).",
            HTTP_TRANSACTION::MREGISTER_E_DEVICE_AUTHORIZATION_ERROR => "Windows MDE: User is not authorized to enroll.",
            HTTP_TRANSACTION::HTTP_E_STATUS_UNEXPECTED_CLIENT_ERROR => "Unexpected client error status code (4xx).",
            HTTP_TRANSACTION::MREGISTER_E_DEVICE_CERTIFCATEREQUEST_ERROR => "Windows MDE: User has no permission on the cert template or CA unreachable.",
            HTTP_TRANSACTION::HTTP_E_STATUS_UNEXPECTED_SERVER_ERROR => "Unexpected server error status code (5xx).",
            HTTP_TRANSACTION::MREGISTER_E_DEVICE_CONFIGMGRSERVER_ERROR => "Windows MDE: Generic Failure from management server, such as DB access error.",
            HTTP_TRANSACTION::MREGISTER_E_DEVICE_INTERNALSERVICE_ERROR => "Windows MDE: Unhandled exception from server.",
            HTTP_TRANSACTION::MREGISTER_E_DEVICE_INVALIDSECURITY_ERROR => "Windows MDE: Unhandled exception from server.",
            HTTP_TRANSACTION::MREGISTER_E_DEVICE_UNKNOWN_ERROR => "Windows MDE: Unknown server error.",
            HTTP_TRANSACTION::MREGISTER_E_REGISTRATION_IN_PROGRESS => "Windows MDE: Another enrollment operation is currently underway.",
            HTTP_TRANSACTION::MREGISTER_E_DEVICE_ALREADY_REGISTERED => "Windows MDE: Device is already enrolled.",
            HTTP_TRANSACTION::MREGISTER_E_DEVICE_NOT_REGISTERED => "Windows MDE: Device is not enrolled.",
            HTTP_TRANSACTION::MREGISTER_E_DISCOVERY_REDIRECTED => "Windows MDE: Redirection is needed.",
            HTTP_TRANSACTION::MREGISTER_E_DEVICE_NOT_AD_REGISTERED_ERROR => "Windows MDE: The device is not registered with Active Directory.",
            HTTP_TRANSACTION::MREGISTER_E_DISCOVERY_FAILED => "Windows MDE: Discovery failed; redirection is needed.",
            HTTP_TRANSACTION::HTTP_E_STATUS_AMBIGUOUS => "Multiple choices (300).",
            HTTP_TRANSACTION::HTTP_E_STATUS_MOVED => "Moved permanently (301).",
            HTTP_TRANSACTION::HTTP_E_STATUS_REDIRECT => "Found (302).",
            HTTP_TRANSACTION::HTTP_E_STATUS_REDIRECT_METHOD => "See Other (303).",
            HTTP_TRANSACTION::HTTP_E_STATUS_NOT_MODIFIED => "Not modified (304).",
            HTTP_TRANSACTION::HTTP_E_STATUS_USE_PROXY => "Use proxy (305).",
            HTTP_TRANSACTION::HTTP_E_STATUS_REDIRECT_KEEP_VERB => "Temporary redirect (307).",
            HTTP_TRANSACTION::HTTP_E_STATUS_BAD_REQUEST => "Bad request (400).",
            HTTP_TRANSACTION::HTTP_E_STATUS_DENIED => "Unauthorized (401).",
            HTTP_TRANSACTION::HTTP_E_STATUS_PAYMENT_REQ => "Payment required (402).",
            HTTP_TRANSACTION::HTTP_E_STATUS_FORBIDDEN => "Forbidden (403).",
            HTTP_TRANSACTION::HTTP_E_STATUS_NOT_FOUND => "Not found (404).",
            HTTP_TRANSACTION::HTTP_E_STATUS_BAD_METHOD => "Method not allowed (405).",
            HTTP_TRANSACTION::HTTP_E_STATUS_NONE_ACCEPTABLE => "Not acceptable (406).",
            HTTP_TRANSACTION::HTTP_E_STATUS_PROXY_AUTH_REQ => "Proxy authentication required (407).",
            HTTP_TRANSACTION::HTTP_E_STATUS_REQUEST_TIMEOUT => "Request timeout (408).",
            HTTP_TRANSACTION::HTTP_E_STATUS_CONFLICT => "Conflict (409).",
            HTTP_TRANSACTION::HTTP_E_STATUS_GONE => "Gone (410).",
            HTTP_TRANSACTION::HTTP_E_STATUS_LENGTH_REQUIRED => "Length required (411).",
            HTTP_TRANSACTION::HTTP_E_STATUS_PRECOND_FAILED => "Precondition failed (412).",
            HTTP_TRANSACTION::HTTP_E_STATUS_REQUEST_TOO_LARGE => "Request entity too large (413).",
            HTTP_TRANSACTION::HTTP_E_STATUS_URI_TOO_LONG => "Request-URI too long (414).",
            HTTP_TRANSACTION::HTTP_E_STATUS_UNSUPPORTED_MEDIA => "Unsupported media type (415).",
            HTTP_TRANSACTION::HTTP_E_STATUS_RANGE_NOT_SATISFIABLE => "Requested range not satisfiable (416).",
            HTTP_TRANSACTION::HTTP_E_STATUS_EXPECTATION_FAILED => "Expectation failed (417).",
            HTTP_TRANSACTION::HTTP_E_STATUS_SERVER_ERROR => "Internal server error (500).",
            HTTP_TRANSACTION::HTTP_E_STATUS_NOT_SUPPORTED => "Not implemented (501).",
            HTTP_TRANSACTION::HTTP_E_STATUS_BAD_GATEWAY => "Bad gateway (502).",
            HTTP_TRANSACTION::HTTP_E_STATUS_SERVICE_UNAVAIL => "Service unavailable (503).",
            HTTP_TRANSACTION::HTTP_E_STATUS_GATEWAY_TIMEOUT => "Gateway timeout (504).",
            HTTP_TRANSACTION::HTTP_E_STATUS_VERSION_NOT_SUP => "Version not supported (505).",
        }
    }
}