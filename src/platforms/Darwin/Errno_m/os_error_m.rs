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

use super::{RawError};

#[derive(Clone, Debug, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OSError {
    BASE = 0,
    EPERM = 1,
    ENOENT = 2,
    ESRCH = 3,
    EINTR = 4,
    EIO = 5,
    ENXIO = 6,
    E2BIG = 7,
    ENOEXEC = 8,
    EBADF = 9,
    ECHILD = 10,
    EDEADLK = 11,
    ENOMEM = 12,
    EACCES = 13,
    EFAULT = 14,
    ENOTBLK = 15,
    EBUSY = 16,
    EEXIST = 17,
    EXDEV = 18,
    ENODEV = 19,
    ENOTDIR = 20,
    EISDIR = 21,
    EINVAL = 22,
    ENFILE = 23,
    EMFILE = 24,
    ENOTTY = 25,
    ETXTBSY = 26,
    EFBIG = 27,
    ENOSPC = 28,
    ESPIPE = 29,
    EROFS = 30,
    EMLINK = 31,
    EPIPE = 32,
    EDOM = 33,
    ERANGE = 34,
    EAGAIN = 35,
    EINPROGRESS = 36,
    EALREADY = 37,
    ENOTSOCK = 38,
    EDESTADDRREQ = 39,
    EMSGSIZE = 40,
    EPROTOTYPE = 41,
    ENOPROTOOPT = 42,
    EPROTONOSUPPORT = 43,
    ESOCKTNOSUPPORT = 44,
    ENOTSUP = 45,
    EPFNOSUPPORT = 46,
    EAFNOSUPPORT = 47,
    EADDRINUSE = 48,
    EADDRNOTAVAIL = 49,
    ENETDOWN = 50,
    ENETUNREACH = 51,
    ENETRESET = 52,
    ECONNABORTED = 53,
    ECONNRESET = 54,
    ENOBUFS = 55,
    EISCONN = 56,
    ENOTCONN = 57,
    ESHUTDOWN = 58,
    ETOOMANYREFS = 59,
    ETIMEDOUT = 60,
    ECONNREFUSED = 61,
    ELOOP = 62,
    ENAMETOOLONG = 63,
    EHOSTDOWN = 64,
    EHOSTUNREACH = 65,
    ENOTEMPTY = 66,
    EPROCLIM = 67,
    EUSERS = 68,
    EDQUOT = 69,
    ESTALE = 70,
    EREMOTE = 71,
    EBADRPC = 72,
    ERPCMISMATCH = 73,
    EPROGUNAVAIL = 74,
    EPROGMISMATCH = 75,
    EPROCUNAVAIL = 76,
    ENOLCK = 77,
    ENOSYS = 78,
    EFTYPE = 79,
    EAUTH = 80,
    ENEEDAUTH = 81,
    EPWROFF = 82,
    EDEVERR = 83,
    EOVERFLOW = 84,
    EBADEXEC = 85,
    EBADARCH = 86,
    ESHLIBVERS = 87,
    EBADMACHO = 88,
    ECANCELED = 89,
    EIDRM = 90,
    ENOMSG = 91,
    EILSEQ = 92,
    ENOATTR = 93,
    EBADMSG = 94,
    EMULTIHOP = 95,
    ENODATA = 96,
    ENOLINK = 97,
    ENOSR = 98,
    ENOSTR = 99,
    EPROTO = 100,
    ETIME = 101,
    EOPNOTSUPP = 102,
    ENOPOLICY = 103,
    ENOTRECOVERABLE = 104,
    EOWNERDEAD = 105,
    EQFULL = 106,
}

impl OSError {
    pub fn code(&self) -> u32 {
        return match self {
            OSError::BASE => OSError::BASE as u32,
            OSError::EPERM => OSError::EPERM as u32,
            OSError::ENOENT => OSError::ENOENT as u32,
            OSError::ESRCH => OSError::ESRCH as u32,
            OSError::EINTR => OSError::EINTR as u32,
            OSError::EIO => OSError::EIO as u32,
            OSError::ENXIO => OSError::ENXIO as u32,
            OSError::E2BIG => OSError::E2BIG as u32,
            OSError::ENOEXEC => OSError::ENOEXEC as u32,
            OSError::EBADF => OSError::EBADF as u32,
            OSError::ECHILD => OSError::ECHILD as u32,
            OSError::EDEADLK => OSError::EDEADLK as u32,
            OSError::ENOMEM => OSError::ENOMEM as u32,
            OSError::EACCES => OSError::EACCES as u32,
            OSError::EFAULT => OSError::EFAULT as u32,
            OSError::ENOTBLK => OSError::ENOTBLK as u32,
            OSError::EBUSY => OSError::EBUSY as u32,
            OSError::EEXIST => OSError::EEXIST as u32,
            OSError::EXDEV => OSError::EXDEV as u32,
            OSError::ENODEV => OSError::ENODEV as u32,
            OSError::ENOTDIR => OSError::ENOTDIR as u32,
            OSError::EISDIR => OSError::EISDIR as u32,
            OSError::EINVAL => OSError::EINVAL as u32,
            OSError::ENFILE => OSError::ENFILE as u32,
            OSError::EMFILE => OSError::EMFILE as u32,
            OSError::ENOTTY => OSError::ENOTTY as u32,
            OSError::ETXTBSY => OSError::ETXTBSY as u32,
            OSError::EFBIG => OSError::EFBIG as u32,
            OSError::ENOSPC => OSError::ENOSPC as u32,
            OSError::ESPIPE => OSError::ESPIPE as u32,
            OSError::EROFS => OSError::EROFS as u32,
            OSError::EMLINK => OSError::EMLINK as u32,
            OSError::EPIPE => OSError::EPIPE as u32,
            OSError::EDOM => OSError::EDOM as u32,
            OSError::ERANGE => OSError::ERANGE as u32,
            OSError::EAGAIN => OSError::EAGAIN as u32,
            OSError::EINPROGRESS => OSError::EINPROGRESS as u32,
            OSError::EALREADY => OSError::EALREADY as u32,
            OSError::ENOTSOCK => OSError::ENOTSOCK as u32,
            OSError::EDESTADDRREQ => OSError::EDESTADDRREQ as u32,
            OSError::EMSGSIZE => OSError::EMSGSIZE as u32,
            OSError::EPROTOTYPE => OSError::EPROTOTYPE as u32,
            OSError::ENOPROTOOPT => OSError::ENOPROTOOPT as u32,
            OSError::EPROTONOSUPPORT => OSError::EPROTONOSUPPORT as u32,
            OSError::ESOCKTNOSUPPORT => OSError::ESOCKTNOSUPPORT as u32,
            OSError::ENOTSUP => OSError::ENOTSUP as u32,
            OSError::EPFNOSUPPORT => OSError::EPFNOSUPPORT as u32,
            OSError::EAFNOSUPPORT => OSError::EAFNOSUPPORT as u32,
            OSError::EADDRINUSE => OSError::EADDRINUSE as u32,
            OSError::EADDRNOTAVAIL => OSError::EADDRNOTAVAIL as u32,
            OSError::ENETDOWN => OSError::ENETDOWN as u32,
            OSError::ENETUNREACH => OSError::ENETUNREACH as u32,
            OSError::ENETRESET => OSError::ENETRESET as u32,
            OSError::ECONNABORTED => OSError::ECONNABORTED as u32,
            OSError::ECONNRESET => OSError::ECONNRESET as u32,
            OSError::ENOBUFS => OSError::ENOBUFS as u32,
            OSError::EISCONN => OSError::EISCONN as u32,
            OSError::ENOTCONN => OSError::ENOTCONN as u32,
            OSError::ESHUTDOWN => OSError::ESHUTDOWN as u32,
            OSError::ETOOMANYREFS => OSError::ETOOMANYREFS as u32,
            OSError::ETIMEDOUT => OSError::ETIMEDOUT as u32,
            OSError::ECONNREFUSED => OSError::ECONNREFUSED as u32,
            OSError::ELOOP => OSError::ELOOP as u32,
            OSError::ENAMETOOLONG => OSError::ENAMETOOLONG as u32,
            OSError::EHOSTDOWN => OSError::EHOSTDOWN as u32,
            OSError::EHOSTUNREACH => OSError::EHOSTUNREACH as u32,
            OSError::ENOTEMPTY => OSError::ENOTEMPTY as u32,
            OSError::EPROCLIM => OSError::EPROCLIM as u32,
            OSError::EUSERS => OSError::EUSERS as u32,
            OSError::EDQUOT => OSError::EDQUOT as u32,
            OSError::ESTALE => OSError::ESTALE as u32,
            OSError::EREMOTE => OSError::EREMOTE as u32,
            OSError::EBADRPC => OSError::EBADRPC as u32,
            OSError::ERPCMISMATCH => OSError::ERPCMISMATCH as u32,
            OSError::EPROGUNAVAIL => OSError::EPROGUNAVAIL as u32,
            OSError::EPROGMISMATCH => OSError::EPROGMISMATCH as u32,
            OSError::EPROCUNAVAIL => OSError::EPROCUNAVAIL as u32,
            OSError::ENOLCK => OSError::ENOLCK as u32,
            OSError::ENOSYS => OSError::ENOSYS as u32,
            OSError::EFTYPE => OSError::EFTYPE as u32,
            OSError::EAUTH => OSError::EAUTH as u32,
            OSError::ENEEDAUTH => OSError::ENEEDAUTH as u32,
            OSError::EPWROFF => OSError::EPWROFF as u32,
            OSError::EDEVERR => OSError::EDEVERR as u32,
            OSError::EOVERFLOW => OSError::EOVERFLOW as u32,
            OSError::EBADEXEC => OSError::EBADEXEC as u32,
            OSError::EBADARCH => OSError::EBADARCH as u32,
            OSError::ESHLIBVERS => OSError::ESHLIBVERS as u32,
            OSError::EBADMACHO => OSError::EBADMACHO as u32,
            OSError::ECANCELED => OSError::ECANCELED as u32,
            OSError::EIDRM => OSError::EIDRM as u32,
            OSError::ENOMSG => OSError::ENOMSG as u32,
            OSError::EILSEQ => OSError::EILSEQ as u32,
            OSError::ENOATTR => OSError::ENOATTR as u32,
            OSError::EBADMSG => OSError::EBADMSG as u32,
            OSError::EMULTIHOP => OSError::EMULTIHOP as u32,
            OSError::ENODATA => OSError::ENODATA as u32,
            OSError::ENOLINK => OSError::ENOLINK as u32,
            OSError::ENOSR => OSError::ENOSR as u32,
            OSError::ENOSTR => OSError::ENOSTR as u32,
            OSError::EPROTO => OSError::EPROTO as u32,
            OSError::ETIME => OSError::ETIME as u32,
            OSError::EOPNOTSUPP => OSError::EOPNOTSUPP as u32,
            OSError::ENOPOLICY => OSError::ENOPOLICY as u32,
            OSError::ENOTRECOVERABLE => OSError::ENOTRECOVERABLE as u32,
            OSError::EOWNERDEAD => OSError::EOWNERDEAD as u32,
            OSError::EQFULL => OSError::EQFULL as u32,
            _ => panic!("Invalid OSError kind! (Darwin)")
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            OSError::BASE => RawError::Kind(OSError::BASE),
            OSError::EPERM => RawError::Kind(OSError::EPERM),
            OSError::ENOENT => RawError::Kind(OSError::ENOENT),
            OSError::ESRCH => RawError::Kind(OSError::ESRCH),
            OSError::EINTR => RawError::Kind(OSError::EINTR),
            OSError::EIO => RawError::Kind(OSError::EIO),
            OSError::ENXIO => RawError::Kind(OSError::ENXIO),
            OSError::E2BIG => RawError::Kind(OSError::E2BIG),
            OSError::ENOEXEC => RawError::Kind(OSError::ENOEXEC),
            OSError::EBADF => RawError::Kind(OSError::EBADF),
            OSError::ECHILD => RawError::Kind(OSError::ECHILD),
            OSError::EDEADLK => RawError::Kind(OSError::EDEADLK),
            OSError::ENOMEM => RawError::Kind(OSError::ENOMEM),
            OSError::EACCES => RawError::Kind(OSError::EACCES),
            OSError::EFAULT => RawError::Kind(OSError::EFAULT),
            OSError::ENOTBLK => RawError::Kind(OSError::ENOTBLK),
            OSError::EBUSY => RawError::Kind(OSError::EBUSY),
            OSError::EEXIST => RawError::Kind(OSError::EEXIST),
            OSError::EXDEV => RawError::Kind(OSError::EXDEV),
            OSError::ENODEV => RawError::Kind(OSError::ENODEV),
            OSError::ENOTDIR => RawError::Kind(OSError::ENOTDIR),
            OSError::EISDIR => RawError::Kind(OSError::EISDIR),
            OSError::EINVAL => RawError::Kind(OSError::EINVAL),
            OSError::ENFILE => RawError::Kind(OSError::ENFILE),
            OSError::EMFILE => RawError::Kind(OSError::EMFILE),
            OSError::ENOTTY => RawError::Kind(OSError::ENOTTY),
            OSError::ETXTBSY => RawError::Kind(OSError::ETXTBSY),
            OSError::EFBIG => RawError::Kind(OSError::EFBIG),
            OSError::ENOSPC => RawError::Kind(OSError::ENOSPC),
            OSError::ESPIPE => RawError::Kind(OSError::ESPIPE),
            OSError::EROFS => RawError::Kind(OSError::EROFS),
            OSError::EMLINK => RawError::Kind(OSError::EMLINK),
            OSError::EPIPE => RawError::Kind(OSError::EPIPE),
            OSError::EDOM => RawError::Kind(OSError::EDOM),
            OSError::ERANGE => RawError::Kind(OSError::ERANGE),
            OSError::EAGAIN => RawError::Kind(OSError::EAGAIN),
            OSError::EINPROGRESS => RawError::Kind(OSError::EINPROGRESS),
            OSError::EALREADY => RawError::Kind(OSError::EALREADY),
            OSError::ENOTSOCK => RawError::Kind(OSError::ENOTSOCK),
            OSError::EDESTADDRREQ => RawError::Kind(OSError::EDESTADDRREQ),
            OSError::EMSGSIZE => RawError::Kind(OSError::EMSGSIZE),
            OSError::EPROTOTYPE => RawError::Kind(OSError::EPROTOTYPE),
            OSError::ENOPROTOOPT => RawError::Kind(OSError::ENOPROTOOPT),
            OSError::EPROTONOSUPPORT => RawError::Kind(OSError::EPROTONOSUPPORT),
            OSError::ESOCKTNOSUPPORT => RawError::Kind(OSError::ESOCKTNOSUPPORT),
            OSError::ENOTSUP => RawError::Kind(OSError::ENOTSUP),
            OSError::EPFNOSUPPORT => RawError::Kind(OSError::EPFNOSUPPORT),
            OSError::EAFNOSUPPORT => RawError::Kind(OSError::EAFNOSUPPORT),
            OSError::EADDRINUSE => RawError::Kind(OSError::EADDRINUSE),
            OSError::EADDRNOTAVAIL => RawError::Kind(OSError::EADDRNOTAVAIL),
            OSError::ENETDOWN => RawError::Kind(OSError::ENETDOWN),
            OSError::ENETUNREACH => RawError::Kind(OSError::ENETUNREACH),
            OSError::ENETRESET => RawError::Kind(OSError::ENETRESET),
            OSError::ECONNABORTED => RawError::Kind(OSError::ECONNABORTED),
            OSError::ECONNRESET => RawError::Kind(OSError::ECONNRESET),
            OSError::ENOBUFS => RawError::Kind(OSError::ENOBUFS),
            OSError::EISCONN => RawError::Kind(OSError::EISCONN),
            OSError::ENOTCONN => RawError::Kind(OSError::ENOTCONN),
            OSError::ESHUTDOWN => RawError::Kind(OSError::ESHUTDOWN),
            OSError::ETOOMANYREFS => RawError::Kind(OSError::ETOOMANYREFS),
            OSError::ETIMEDOUT => RawError::Kind(OSError::ETIMEDOUT),
            OSError::ECONNREFUSED => RawError::Kind(OSError::ECONNREFUSED),
            OSError::ELOOP => RawError::Kind(OSError::ELOOP),
            OSError::ENAMETOOLONG => RawError::Kind(OSError::ENAMETOOLONG),
            OSError::EHOSTDOWN => RawError::Kind(OSError::EHOSTDOWN),
            OSError::EHOSTUNREACH => RawError::Kind(OSError::EHOSTUNREACH),
            OSError::ENOTEMPTY => RawError::Kind(OSError::ENOTEMPTY),
            OSError::EPROCLIM => RawError::Kind(OSError::EPROCLIM),
            OSError::EUSERS => RawError::Kind(OSError::EUSERS),
            OSError::EDQUOT => RawError::Kind(OSError::EDQUOT),
            OSError::ESTALE => RawError::Kind(OSError::ESTALE),
            OSError::EREMOTE => RawError::Kind(OSError::EREMOTE),
            OSError::EBADRPC => RawError::Kind(OSError::EBADRPC),
            OSError::ERPCMISMATCH => RawError::Kind(OSError::ERPCMISMATCH),
            OSError::EPROGUNAVAIL => RawError::Kind(OSError::EPROGUNAVAIL),
            OSError::EPROGMISMATCH => RawError::Kind(OSError::EPROGMISMATCH),
            OSError::EPROCUNAVAIL => RawError::Kind(OSError::EPROCUNAVAIL),
            OSError::ENOLCK => RawError::Kind(OSError::ENOLCK),
            OSError::ENOSYS => RawError::Kind(OSError::ENOSYS),
            OSError::EFTYPE => RawError::Kind(OSError::EFTYPE),
            OSError::EAUTH => RawError::Kind(OSError::EAUTH),
            OSError::ENEEDAUTH => RawError::Kind(OSError::ENEEDAUTH),
            OSError::EPWROFF => RawError::Kind(OSError::EPWROFF),
            OSError::EDEVERR => RawError::Kind(OSError::EDEVERR),
            OSError::EOVERFLOW => RawError::Kind(OSError::EOVERFLOW),
            OSError::EBADEXEC => RawError::Kind(OSError::EBADEXEC),
            OSError::EBADARCH => RawError::Kind(OSError::EBADARCH),
            OSError::ESHLIBVERS => RawError::Kind(OSError::ESHLIBVERS),
            OSError::EBADMACHO => RawError::Kind(OSError::EBADMACHO),
            OSError::ECANCELED => RawError::Kind(OSError::ECANCELED),
            OSError::EIDRM => RawError::Kind(OSError::EIDRM),
            OSError::ENOMSG => RawError::Kind(OSError::ENOMSG),
            OSError::EILSEQ => RawError::Kind(OSError::EILSEQ),
            OSError::ENOATTR => RawError::Kind(OSError::ENOATTR),
            OSError::EBADMSG => RawError::Kind(OSError::EBADMSG),
            OSError::EMULTIHOP => RawError::Kind(OSError::EMULTIHOP),
            OSError::ENODATA => RawError::Kind(OSError::ENODATA),
            OSError::ENOLINK => RawError::Kind(OSError::ENOLINK),
            OSError::ENOSR => RawError::Kind(OSError::ENOSR),
            OSError::ENOSTR => RawError::Kind(OSError::ENOSTR),
            OSError::EPROTO => RawError::Kind(OSError::EPROTO),
            OSError::ETIME => RawError::Kind(OSError::ETIME),
            OSError::EOPNOTSUPP => RawError::Kind(OSError::EOPNOTSUPP),
            OSError::ENOPOLICY => RawError::Kind(OSError::ENOPOLICY),
            OSError::ENOTRECOVERABLE => RawError::Kind(OSError::ENOTRECOVERABLE),
            OSError::EOWNERDEAD => RawError::Kind(OSError::EOWNERDEAD),
            OSError::EQFULL => RawError::Kind(OSError::EQFULL),
            _ => panic!("Invalid Errno! (Darwin)")
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            OSError::BASE => "Undefined error: 0",
            OSError::EPERM => "Operation not permitted",
            OSError::ENOENT => "No such file or directory",
            OSError::ESRCH => "No such process",
            OSError::EINTR => "Interrupted system call",
            OSError::EIO => "Input/output error",
            OSError::ENXIO => "Device not configured",
            OSError::E2BIG => "Argument list too long",
            OSError::ENOEXEC => "Exec format error",
            OSError::EBADF => "Bad file descriptor",
            OSError::ECHILD => "No child processes",
            OSError::EDEADLK => "Resource deadlock avoided",
            OSError::ENOMEM => "Cannot allocate memory",
            OSError::EACCES => "Permission denied",
            OSError::EFAULT => "Bad address",
            OSError::ENOTBLK => "Block device required",
            OSError::EBUSY => "Device busy",
            OSError::EEXIST => "File exists",
            OSError::EXDEV => "Cross-device link",
            OSError::ENODEV => "Operation not supported by device",
            OSError::ENOTDIR => "Not a directory",
            OSError::EISDIR => "Is a directory",
            OSError::EINVAL => "Invalid argument",
            OSError::ENFILE => "Too many open files in system",
            OSError::EMFILE => "Too many open files",
            OSError::ENOTTY => "Inappropriate ioctl for device",
            OSError::ETXTBSY => "Text file busy",
            OSError::EFBIG => "File too large",
            OSError::ENOSPC => "No space left on device",
            OSError::ESPIPE => "Illegal seek",
            OSError::EROFS => "Read-only file system",
            OSError::EMLINK => "Too many links",
            OSError::EPIPE => "Broken pipe",
            OSError::EDOM => "Numerical argument out of domain",
            OSError::ERANGE => "Result too large",
            OSError::EAGAIN => "Resource temporarily unavailable",
            OSError::EINPROGRESS => "Operation now in progress",
            OSError::EALREADY => "Operation already in progress",
            OSError::ENOTSOCK => "Socket operation on non-socket",
            OSError::EDESTADDRREQ => "Destination address required",
            OSError::EMSGSIZE => "Message too long",
            OSError::EPROTOTYPE => "Protocol wrong type for socket",
            OSError::ENOPROTOOPT => "Protocol not available",
            OSError::EPROTONOSUPPORT => "Protocol not supported",
            OSError::ESOCKTNOSUPPORT => "Socket type not supported",
            OSError::ENOTSUP => "Operation not supported",
            OSError::EPFNOSUPPORT => "Protocol family not supported",
            OSError::EAFNOSUPPORT => "Address family not supported by protocol family",
            OSError::EADDRINUSE => "Address already in use",
            OSError::EADDRNOTAVAIL => "Can't assign requested address",
            OSError::ENETDOWN => "Network is down",
            OSError::ENETUNREACH => "Network is unreachable",
            OSError::ENETRESET => "Network dropped connection on reset",
            OSError::ECONNABORTED => "Software caused connection abort",
            OSError::ECONNRESET => "Connection reset by peer",
            OSError::ENOBUFS => "No buffer space available",
            OSError::EISCONN => "Socket is already connected",
            OSError::ENOTCONN => "Socket is not connected",
            OSError::ESHUTDOWN => "Can't send after socket shutdown",
            OSError::ETOOMANYREFS => "Too many references: can't splice",
            OSError::ETIMEDOUT => "Operation timed out",
            OSError::ECONNREFUSED => "Connection refused",
            OSError::ELOOP => "Too many levels of symbolic links",
            OSError::ENAMETOOLONG => "File name too long",
            OSError::EHOSTDOWN => "Host is down",
            OSError::EHOSTUNREACH => "No route to host",
            OSError::ENOTEMPTY => "Directory not empty",
            OSError::EPROCLIM => "Too many processes",
            OSError::EUSERS => "Too many users",
            OSError::EDQUOT => "Disc quota exceeded",
            OSError::ESTALE => "Stale NFS file handle",
            OSError::EREMOTE => "Too many levels of remote in path",
            OSError::EBADRPC => "RPC struct is bad",
            OSError::ERPCMISMATCH => "RPC version wrong",
            OSError::EPROGUNAVAIL => "RPC prog. not avail",
            OSError::EPROGMISMATCH => "Program version wrong",
            OSError::EPROCUNAVAIL => "Bad procedure for program",
            OSError::ENOLCK => "No locks available",
            OSError::ENOSYS => "Function not implemented",
            OSError::EFTYPE => "Inappropriate file type or format",
            OSError::EAUTH => "Authentication error",
            OSError::ENEEDAUTH => "Need authenticator",
            OSError::EPWROFF => "Device power is off",
            OSError::EDEVERR => "Device error",
            OSError::EOVERFLOW => "Value too large to be stored in data type",
            OSError::EBADEXEC => "Bad executable",
            OSError::EBADARCH => "Bad CPU type in executable",
            OSError::ESHLIBVERS => "Shared library version mismatch",
            OSError::EBADMACHO => "Malformed Macho file",
            OSError::ECANCELED => "Operation canceled",
            OSError::EIDRM => "Identifier removed",
            OSError::ENOMSG => "No message of desired type",
            OSError::EILSEQ => "Illegal byte sequence",
            OSError::ENOATTR => "Attribute not found",
            OSError::EBADMSG => "Bad message",
            OSError::EMULTIHOP => "EMULTIHOP (Reserved)",
            OSError::ENODATA => "No message available on STREAM",
            OSError::ENOLINK => "ENOLINK (Reserved)",
            OSError::ENOSR => "No STREAM resources",
            OSError::ENOSTR => "Not a STREAM",
            OSError::EPROTO => "Protocol error",
            OSError::ETIME => "STREAM ioctl timeout",
            OSError::EOPNOTSUPP => "Operation not supported on socket",
            OSError::ENOPOLICY => "Policy not found",
            OSError::ENOTRECOVERABLE => "State not recoverable",
            OSError::EOWNERDEAD => "Previous owner died",
            OSError::EQFULL => "Interface output queue is full",
            _ => panic!("Invalid OSError kind! (Darwin)")
        }
    }

    pub fn from_name(name: &str) -> OSError {
        return match name {
            "BASE" => OSError::BASE,
            "EPERM" => OSError::EPERM,
            "ENOENT" => OSError::ENOENT,
            "ESRCH" => OSError::ESRCH,
            "EINTR" => OSError::EINTR,
            "EIO" => OSError::EIO,
            "ENXIO" => OSError::ENXIO,
            "E2BIG" => OSError::E2BIG,
            "ENOEXEC" => OSError::ENOEXEC,
            "EBADF" => OSError::EBADF,
            "ECHILD" => OSError::ECHILD,
            "EDEADLK" => OSError::EDEADLK,
            "ENOMEM" => OSError::ENOMEM,
            "EACCES" => OSError::EACCES,
            "EFAULT" => OSError::EFAULT,
            "ENOTBLK" => OSError::ENOTBLK,
            "EBUSY" => OSError::EBUSY,
            "EEXIST" => OSError::EEXIST,
            "EXDEV" => OSError::EXDEV,
            "ENODEV" => OSError::ENODEV,
            "ENOTDIR" => OSError::ENOTDIR,
            "EISDIR" => OSError::EISDIR,
            "EINVAL" => OSError::EINVAL,
            "ENFILE" => OSError::ENFILE,
            "EMFILE" => OSError::EMFILE,
            "ENOTTY" => OSError::ENOTTY,
            "ETXTBSY" => OSError::ETXTBSY,
            "EFBIG" => OSError::EFBIG,
            "ENOSPC" => OSError::ENOSPC,
            "ESPIPE" => OSError::ESPIPE,
            "EROFS" => OSError::EROFS,
            "EMLINK" => OSError::EMLINK,
            "EPIPE" => OSError::EPIPE,
            "EDOM" => OSError::EDOM,
            "ERANGE" => OSError::ERANGE,
            "EAGAIN" => OSError::EAGAIN,
            "EINPROGRESS" => OSError::EINPROGRESS,
            "EALREADY" => OSError::EALREADY,
            "ENOTSOCK" => OSError::ENOTSOCK,
            "EDESTADDRREQ" => OSError::EDESTADDRREQ,
            "EMSGSIZE" => OSError::EMSGSIZE,
            "EPROTOTYPE" => OSError::EPROTOTYPE,
            "ENOPROTOOPT" => OSError::ENOPROTOOPT,
            "EPROTONOSUPPORT" => OSError::EPROTONOSUPPORT,
            "ESOCKTNOSUPPORT" => OSError::ESOCKTNOSUPPORT,
            "ENOTSUP" => OSError::ENOTSUP,
            "EPFNOSUPPORT" => OSError::EPFNOSUPPORT,
            "EAFNOSUPPORT" => OSError::EAFNOSUPPORT,
            "EADDRINUSE" => OSError::EADDRINUSE,
            "EADDRNOTAVAIL" => OSError::EADDRNOTAVAIL,
            "ENETDOWN" => OSError::ENETDOWN,
            "ENETUNREACH" => OSError::ENETUNREACH,
            "ENETRESET" => OSError::ENETRESET,
            "ECONNABORTED" => OSError::ECONNABORTED,
            "ECONNRESET" => OSError::ECONNRESET,
            "ENOBUFS" => OSError::ENOBUFS,
            "EISCONN" => OSError::EISCONN,
            "ENOTCONN" => OSError::ENOTCONN,
            "ESHUTDOWN" => OSError::ESHUTDOWN,
            "ETOOMANYREFS" => OSError::ETOOMANYREFS,
            "ETIMEDOUT" => OSError::ETIMEDOUT,
            "ECONNREFUSED" => OSError::ECONNREFUSED,
            "ELOOP" => OSError::ELOOP,
            "ENAMETOOLONG" => OSError::ENAMETOOLONG,
            "EHOSTDOWN" => OSError::EHOSTDOWN,
            "EHOSTUNREACH" => OSError::EHOSTUNREACH,
            "ENOTEMPTY" => OSError::ENOTEMPTY,
            "EPROCLIM" => OSError::EPROCLIM,
            "EUSERS" => OSError::EUSERS,
            "EDQUOT" => OSError::EDQUOT,
            "ESTALE" => OSError::ESTALE,
            "EREMOTE" => OSError::EREMOTE,
            "EBADRPC" => OSError::EBADRPC,
            "ERPCMISMATCH" => OSError::ERPCMISMATCH,
            "EPROGUNAVAIL" => OSError::EPROGUNAVAIL,
            "EPROGMISMATCH" => OSError::EPROGMISMATCH,
            "EPROCUNAVAIL" => OSError::EPROCUNAVAIL,
            "ENOLCK" => OSError::ENOLCK,
            "ENOSYS" => OSError::ENOSYS,
            "EFTYPE" => OSError::EFTYPE,
            "EAUTH" => OSError::EAUTH,
            "ENEEDAUTH" => OSError::ENEEDAUTH,
            "EPWROFF" => OSError::EPWROFF,
            "EDEVERR" => OSError::EDEVERR,
            "EOVERFLOW" => OSError::EOVERFLOW,
            "EBADEXEC" => OSError::EBADEXEC,
            "EBADARCH" => OSError::EBADARCH,
            "ESHLIBVERS" => OSError::ESHLIBVERS,
            "EBADMACHO" => OSError::EBADMACHO,
            "ECANCELED" => OSError::ECANCELED,
            "EIDRM" => OSError::EIDRM,
            "ENOMSG" => OSError::ENOMSG,
            "EILSEQ" => OSError::EILSEQ,
            "ENOATTR" => OSError::ENOATTR,
            "EBADMSG" => OSError::EBADMSG,
            "EMULTIHOP" => OSError::EMULTIHOP,
            "ENODATA" => OSError::ENODATA,
            "ENOLINK" => OSError::ENOLINK,
            "ENOSR" => OSError::ENOSR,
            "ENOSTR" => OSError::ENOSTR,
            "EPROTO" => OSError::EPROTO,
            "ETIME" => OSError::ETIME,
            "EOPNOTSUPP" => OSError::EOPNOTSUPP,
            "ENOPOLICY" => OSError::ENOPOLICY,
            "ENOTRECOVERABLE" => OSError::ENOTRECOVERABLE,
            "EOWNERDEAD" => OSError::EOWNERDEAD,
            "EQFULL" => OSError::EQFULL,
            _ => panic!("Invalid OSError name: {}! (Darwin)", name)
        }
    }

    pub fn from_code(code: u32) -> OSError {
        return match code {
            0 => OSError::BASE,
            1 => OSError::EPERM,
            2 => OSError::ENOENT,
            3 => OSError::ESRCH,
            4 => OSError::EINTR,
            5 => OSError::EIO,
            6 => OSError::ENXIO,
            7 => OSError::E2BIG,
            8 => OSError::ENOEXEC,
            9 => OSError::EBADF,
            10 => OSError::ECHILD,
            11 => OSError::EDEADLK,
            12 => OSError::ENOMEM,
            13 => OSError::EACCES,
            14 => OSError::EFAULT,
            15 => OSError::ENOTBLK,
            16 => OSError::EBUSY,
            17 => OSError::EEXIST,
            18 => OSError::EXDEV,
            19 => OSError::ENODEV,
            20 => OSError::ENOTDIR,
            21 => OSError::EISDIR,
            22 => OSError::EINVAL,
            23 => OSError::ENFILE,
            24 => OSError::EMFILE,
            25 => OSError::ENOTTY,
            26 => OSError::ETXTBSY,
            27 => OSError::EFBIG,
            28 => OSError::ENOSPC,
            29 => OSError::ESPIPE,
            30 => OSError::EROFS,
            31 => OSError::EMLINK,
            32 => OSError::EPIPE,
            33 => OSError::EDOM,
            34 => OSError::ERANGE,
            35 => OSError::EAGAIN,
            36 => OSError::EINPROGRESS,
            37 => OSError::EALREADY,
            38 => OSError::ENOTSOCK,
            39 => OSError::EDESTADDRREQ,
            40 => OSError::EMSGSIZE,
            41 => OSError::EPROTOTYPE,
            42 => OSError::ENOPROTOOPT,
            43 => OSError::EPROTONOSUPPORT,
            44 => OSError::ESOCKTNOSUPPORT,
            45 => OSError::ENOTSUP,
            46 => OSError::EPFNOSUPPORT,
            47 => OSError::EAFNOSUPPORT,
            48 => OSError::EADDRINUSE,
            49 => OSError::EADDRNOTAVAIL,
            50 => OSError::ENETDOWN,
            51 => OSError::ENETUNREACH,
            52 => OSError::ENETRESET,
            53 => OSError::ECONNABORTED,
            54 => OSError::ECONNRESET,
            55 => OSError::ENOBUFS,
            56 => OSError::EISCONN,
            57 => OSError::ENOTCONN,
            58 => OSError::ESHUTDOWN,
            59 => OSError::ETOOMANYREFS,
            60 => OSError::ETIMEDOUT,
            61 => OSError::ECONNREFUSED,
            62 => OSError::ELOOP,
            63 => OSError::ENAMETOOLONG,
            64 => OSError::EHOSTDOWN,
            65 => OSError::EHOSTUNREACH,
            66 => OSError::ENOTEMPTY,
            67 => OSError::EPROCLIM,
            68 => OSError::EUSERS,
            69 => OSError::EDQUOT,
            70 => OSError::ESTALE,
            71 => OSError::EREMOTE,
            72 => OSError::EBADRPC,
            73 => OSError::ERPCMISMATCH,
            74 => OSError::EPROGUNAVAIL,
            75 => OSError::EPROGMISMATCH,
            76 => OSError::EPROCUNAVAIL,
            77 => OSError::ENOLCK,
            78 => OSError::ENOSYS,
            79 => OSError::EFTYPE,
            80 => OSError::EAUTH,
            81 => OSError::ENEEDAUTH,
            82 => OSError::EPWROFF,
            83 => OSError::EDEVERR,
            84 => OSError::EOVERFLOW,
            85 => OSError::EBADEXEC,
            86 => OSError::EBADARCH,
            87 => OSError::ESHLIBVERS,
            88 => OSError::EBADMACHO,
            89 => OSError::ECANCELED,
            90 => OSError::EIDRM,
            91 => OSError::ENOMSG,
            92 => OSError::EILSEQ,
            93 => OSError::ENOATTR,
            94 => OSError::EBADMSG,
            95 => OSError::EMULTIHOP,
            96 => OSError::ENODATA,
            97 => OSError::ENOLINK,
            98 => OSError::ENOSR,
            99 => OSError::ENOSTR,
            100 => OSError::EPROTO,
            101 => OSError::ETIME,
            102 => OSError::EOPNOTSUPP,
            103 => OSError::ENOPOLICY,
            104 => OSError::ENOTRECOVERABLE,
            105 => OSError::EOWNERDEAD,
            106 => OSError::EQFULL,
            _ => panic!("Invalid OSError code: {}! (Darwin)", code)
        }
    }
}
