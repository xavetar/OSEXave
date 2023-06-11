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
    ESUCCESS = 0,
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
    EDEADLK_EDEADLOCK = 11,
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
    EAGAIN_EWOULDBLOCK = 35,
    EINPROGRESS = 36,
    EALREADY = 37,
    ENOTSOCK = 38,
    EDESTADDRREQ = 39,
    EMSGSIZE = 40,
    EPROTOTYPE = 41,
    ENOPROTOOPT = 42,
    EPROTONOSUPPORT = 43,
    ESOCKTNOSUPPORT = 44,
    EOPNOTSUPP = 45,
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
    EUSERS = 68,
    EDQUOT = 69,
    ESTALE = 70,
    EREMOTE = 71,
    ENOLCK = 77,
    ENOSYS = 78,
    ENOMSG = 80,
    EIDRM = 81,
    ENOSR = 82,
    ETIME = 83,
    EBADMSG = 84,
    EPROTO = 85,
    ENODATA = 86,
    ENOSTR = 87,
    ENOPKG = 92,
    EILSEQ = 116,
    ECHRNG = 88,
    EL2NSYNC = 89,
    EL3HLT = 90,
    EL3RST = 91,
    ELNRNG = 93,
    EUNATCH = 94,
    ENOCSI = 95,
    EL2HLT = 96,
    EBADE = 97,
    EBADR = 98,
    EXFULL = 99,
    ENOANO = 100,
    EBADRQC = 101,
    EBADSLT = 102,
    EBFONT = 104,
    ENONET = 105,
    ENOLINK = 106,
    EADV = 107,
    ESRMNT = 108,
    ECOMM = 109,
    EMULTIHOP = 110,
    EDOTDOT = 111,
    EOVERFLOW = 112,
    ENOTUNIQ = 113,
    EBADFD = 114,
    EREMCHG = 115,
    EUCLEAN = 117,
    ENOTNAM = 118,
    ENAVAIL = 119,
    EISNAM = 120,
    EREMOTEIO = 121,
    ELIBACC = 122,
    ELIBBAD = 123,
    ELIBSCN = 124,
    ELIBMAX = 125,
    ELIBEXEC = 126,
    ERESTART = 127,
    ESTRPIPE = 128,
    ENOMEDIUM = 129,
    EMEDIUMTYPE = 130,
    ECANCELED = 131,
    ENOKEY = 132,
    EKEYEXPIRED = 133,
    EKEYREVOKED = 134,
    EKEYREJECTED = 135,
    EOWNERDEAD = 136,
    ENOTRECOVERABLE = 137,
    ERFKILL = 138,
    EHWPOISON = 139,
}

impl OSError {
    pub fn code(&self) -> u32 {
        match self {
            OSError::ESUCCESS => OSError::ESUCCESS as u32,
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
            OSError::EDEADLK_EDEADLOCK => OSError::EDEADLK_EDEADLOCK as u32,
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
            OSError::EAGAIN_EWOULDBLOCK => OSError::EAGAIN_EWOULDBLOCK as u32,
            OSError::EINPROGRESS => OSError::EINPROGRESS as u32,
            OSError::EALREADY => OSError::EALREADY as u32,
            OSError::ENOTSOCK => OSError::ENOTSOCK as u32,
            OSError::EDESTADDRREQ => OSError::EDESTADDRREQ as u32,
            OSError::EMSGSIZE => OSError::EMSGSIZE as u32,
            OSError::EPROTOTYPE => OSError::EPROTOTYPE as u32,
            OSError::ENOPROTOOPT => OSError::ENOPROTOOPT as u32,
            OSError::EPROTONOSUPPORT => OSError::EPROTONOSUPPORT as u32,
            OSError::ESOCKTNOSUPPORT => OSError::ESOCKTNOSUPPORT as u32,
            OSError::EOPNOTSUPP => OSError::EOPNOTSUPP as u32,
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
            OSError::EUSERS => OSError::EUSERS as u32,
            OSError::EDQUOT => OSError::EDQUOT as u32,
            OSError::ESTALE => OSError::ESTALE as u32,
            OSError::EREMOTE => OSError::EREMOTE as u32,
            OSError::ENOLCK => OSError::ENOLCK as u32,
            OSError::ENOSYS => OSError::ENOSYS as u32,
            OSError::ENOMSG => OSError::ENOMSG as u32,
            OSError::EIDRM => OSError::EIDRM as u32,
            OSError::ENOSR => OSError::ENOSR as u32,
            OSError::ETIME => OSError::ETIME as u32,
            OSError::EBADMSG => OSError::EBADMSG as u32,
            OSError::EPROTO => OSError::EPROTO as u32,
            OSError::ENODATA => OSError::ENODATA as u32,
            OSError::ENOSTR => OSError::ENOSTR as u32,
            OSError::ENOPKG => OSError::ENOPKG as u32,
            OSError::EILSEQ => OSError::EILSEQ as u32,
            OSError::ECHRNG => OSError::ECHRNG as u32,
            OSError::EL2NSYNC => OSError::EL2NSYNC as u32,
            OSError::EL3HLT => OSError::EL3HLT as u32,
            OSError::EL3RST => OSError::EL3RST as u32,
            OSError::ELNRNG => OSError::ELNRNG as u32,
            OSError::EUNATCH => OSError::EUNATCH as u32,
            OSError::ENOCSI => OSError::ENOCSI as u32,
            OSError::EL2HLT => OSError::EL2HLT as u32,
            OSError::EBADE => OSError::EBADE as u32,
            OSError::EBADR => OSError::EBADR as u32,
            OSError::EXFULL => OSError::EXFULL as u32,
            OSError::ENOANO => OSError::ENOANO as u32,
            OSError::EBADRQC => OSError::EBADRQC as u32,
            OSError::EBADSLT => OSError::EBADSLT as u32,
            OSError::EBFONT => OSError::EBFONT as u32,
            OSError::ENONET => OSError::ENONET as u32,
            OSError::ENOLINK => OSError::ENOLINK as u32,
            OSError::EADV => OSError::EADV as u32,
            OSError::ESRMNT => OSError::ESRMNT as u32,
            OSError::ECOMM => OSError::ECOMM as u32,
            OSError::EMULTIHOP => OSError::EMULTIHOP as u32,
            OSError::EDOTDOT => OSError::EDOTDOT as u32,
            OSError::EOVERFLOW => OSError::EOVERFLOW as u32,
            OSError::ENOTUNIQ => OSError::ENOTUNIQ as u32,
            OSError::EBADFD => OSError::EBADFD as u32,
            OSError::EREMCHG => OSError::EREMCHG as u32,
            OSError::EUCLEAN => OSError::EUCLEAN as u32,
            OSError::ENOTNAM => OSError::ENOTNAM as u32,
            OSError::ENAVAIL => OSError::ENAVAIL as u32,
            OSError::EISNAM => OSError::EISNAM as u32,
            OSError::EREMOTEIO => OSError::EREMOTEIO as u32,
            OSError::ELIBACC => OSError::ELIBACC as u32,
            OSError::ELIBBAD => OSError::ELIBBAD as u32,
            OSError::ELIBSCN => OSError::ELIBSCN as u32,
            OSError::ELIBMAX => OSError::ELIBMAX as u32,
            OSError::ELIBEXEC => OSError::ELIBEXEC as u32,
            OSError::ERESTART => OSError::ERESTART as u32,
            OSError::ESTRPIPE => OSError::ESTRPIPE as u32,
            OSError::ENOMEDIUM => OSError::ENOMEDIUM as u32,
            OSError::EMEDIUMTYPE => OSError::EMEDIUMTYPE as u32,
            OSError::ECANCELED => OSError::ECANCELED as u32,
            OSError::ENOKEY => OSError::ENOKEY as u32,
            OSError::EKEYEXPIRED => OSError::EKEYEXPIRED as u32,
            OSError::EKEYREVOKED => OSError::EKEYREVOKED as u32,
            OSError::EKEYREJECTED => OSError::EKEYREJECTED as u32,
            OSError::EOWNERDEAD => OSError::EOWNERDEAD as u32,
            OSError::ENOTRECOVERABLE => OSError::ENOTRECOVERABLE as u32,
            OSError::ERFKILL => OSError::ERFKILL as u32,
            OSError::EHWPOISON => OSError::EHWPOISON as u32,
            _ => panic!("Invalid OSError kind! (Linux, arch: Alpha)")
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            OSError::ESUCCESS => "Success",
            OSError::EPERM => "Operation not permitted",
            OSError::ENOENT => "No such file or directory",
            OSError::ESRCH => "No such process",
            OSError::EINTR => "Interrupted system call",
            OSError::EIO => "I/O error",
            OSError::ENXIO => "No such device or address",
            OSError::E2BIG => "Argument list too long",
            OSError::ENOEXEC => "Exec format error",
            OSError::EBADF => "Bad file number",
            OSError::ECHILD => "No child processes",
            OSError::EDEADLK_EDEADLOCK => "Resource deadlock would occur",
            OSError::ENOMEM => "Out of memory",
            OSError::EACCES => "Permission denied",
            OSError::EFAULT => "Bad address",
            OSError::ENOTBLK => "Block device required",
            OSError::EBUSY => "Device or resource busy",
            OSError::EEXIST => "File exists",
            OSError::EXDEV => "Cross-device link",
            OSError::ENODEV => "No such device",
            OSError::ENOTDIR => "Not a directory",
            OSError::EISDIR => "Is a directory",
            OSError::EINVAL => "Invalid argument",
            OSError::ENFILE => "File table overflow",
            OSError::EMFILE => "Too many open files",
            OSError::ENOTTY => "Not a typewriter",
            OSError::ETXTBSY => "Text file busy",
            OSError::EFBIG => "File too large",
            OSError::ENOSPC => "No space left on device",
            OSError::ESPIPE => "Illegal seek",
            OSError::EROFS => "Read-only file system",
            OSError::EMLINK => "Too many links",
            OSError::EPIPE => "Broken pipe",
            OSError::EDOM => "Math argument out of domain of func",
            OSError::ERANGE => "Math result not representable",
            OSError::EAGAIN_EWOULDBLOCK => "Try again/Operation would block",
            OSError::EINPROGRESS => "Operation now in progress",
            OSError::EALREADY => "Operation already in progress",
            OSError::ENOTSOCK => "Socket operation on non-socket",
            OSError::EDESTADDRREQ => "Destination address required",
            OSError::EMSGSIZE => "Message too long",
            OSError::EPROTOTYPE => "Protocol wrong type for socket",
            OSError::ENOPROTOOPT => "Protocol not available",
            OSError::EPROTONOSUPPORT => "Protocol not supported",
            OSError::ESOCKTNOSUPPORT => "Socket type not supported",
            OSError::EOPNOTSUPP => "Operation not supported on transport endpoint",
            OSError::EPFNOSUPPORT => "Protocol family not supported",
            OSError::EAFNOSUPPORT => "Address family not supported by protocol",
            OSError::EADDRINUSE => "Address already in use",
            OSError::EADDRNOTAVAIL => "Cannot assign requested address",
            OSError::ENETDOWN => "Network is down",
            OSError::ENETUNREACH => "Network is unreachable",
            OSError::ENETRESET => "Network dropped connection because of reset",
            OSError::ECONNABORTED => "Software caused connection abort",
            OSError::ECONNRESET => "Connection reset by peer",
            OSError::ENOBUFS => "No buffer space available",
            OSError::EISCONN => "Transport endpoint is already connected",
            OSError::ENOTCONN => "Transport endpoint is not connected",
            OSError::ESHUTDOWN => "Cannot send after transport endpoint shutdown",
            OSError::ETOOMANYREFS => "Too many references: cannot splice",
            OSError::ETIMEDOUT => "Connection timed out",
            OSError::ECONNREFUSED => "Connection refused",
            OSError::ELOOP => "Too many symbolic links encountered",
            OSError::ENAMETOOLONG => "File name too long",
            OSError::EHOSTDOWN => "Host is down",
            OSError::EHOSTUNREACH => "No route to host",
            OSError::ENOTEMPTY => "Directory not empty",
            OSError::EUSERS => "Too many users",
            OSError::EDQUOT => "Quota exceeded",
            OSError::ESTALE => "Stale file handle",
            OSError::EREMOTE => "Object is remote",
            OSError::ENOLCK => "No record locks available",
            OSError::ENOSYS => "Function not implemented",
            OSError::ENOMSG => "No message of desired type",
            OSError::EIDRM => "Identifier removed",
            OSError::ENOSR => "Out of streams resources",
            OSError::ETIME => "Timer expired",
            OSError::EBADMSG => "Not a data message",
            OSError::EPROTO => "Protocol error",
            OSError::ENODATA => "No data available",
            OSError::ENOSTR => "Device not a stream",
            OSError::ENOPKG => "Package not installed",
            OSError::EILSEQ => "Illegal byte sequence",
            OSError::ECHRNG => "Channel number out of range",
            OSError::EL2NSYNC => "Level 2 not synchronized",
            OSError::EL3HLT => "Level 3 halted",
            OSError::EL3RST => "Level 3 reset",
            OSError::ELNRNG => "Link number out of range",
            OSError::EUNATCH => "Protocol driver not attached",
            OSError::ENOCSI => "No CSI structure available",
            OSError::EL2HLT => "Level 2 halted",
            OSError::EBADE => "Invalid exchange",
            OSError::EBADR => "Invalid request descriptor",
            OSError::EXFULL => "Exchange full",
            OSError::ENOANO => "No anode",
            OSError::EBADRQC => "Invalid request code",
            OSError::EBADSLT => "Invalid slot",
            OSError::EBFONT => "Bad font file format",
            OSError::ENONET => "Machine is not on the network",
            OSError::ENOLINK => "Link has been severed",
            OSError::EADV => "Advertise error",
            OSError::ESRMNT => "Srmount error",
            OSError::ECOMM => "Communication error on send",
            OSError::EMULTIHOP => "Multihop attempted",
            OSError::EDOTDOT => "RFS specific error",
            OSError::EOVERFLOW => "Value too large for defined data type",
            OSError::ENOTUNIQ => "Name not unique on network",
            OSError::EBADFD => "File descriptor in bad state",
            OSError::EREMCHG => "Remote address changed",
            OSError::EUCLEAN => "Structure needs cleaning",
            OSError::ENOTNAM => "Not a XENIX named type file",
            OSError::ENAVAIL => "No XENIX semaphores available",
            OSError::EISNAM => "Is a named type file",
            OSError::EREMOTEIO => "Remote I/O error",
            OSError::ELIBACC => "Can not access a needed shared library",
            OSError::ELIBBAD => "Accessing a corrupted shared library",
            OSError::ELIBSCN => ".lib section in a.out corrupted",
            OSError::ELIBMAX => "Attempting to link in too many shared libraries",
            OSError::ELIBEXEC => "Cannot exec a shared library directly",
            OSError::ERESTART => "Interrupted system call should be restarted",
            OSError::ESTRPIPE => "Streams pipe error",
            OSError::ENOMEDIUM => "No medium found",
            OSError::EMEDIUMTYPE => "Wrong medium type",
            OSError::ECANCELED => "Operation Cancelled",
            OSError::ENOKEY => "Required key not available",
            OSError::EKEYEXPIRED => "Key has expired",
            OSError::EKEYREVOKED => "Key has been revoked",
            OSError::EKEYREJECTED => "Key was rejected by service",
            OSError::EOWNERDEAD => "Owner died",
            OSError::ENOTRECOVERABLE => "State not recoverable",
            OSError::ERFKILL => "Operation not possible due to RF-kill",
            OSError::EHWPOISON => "Memory page has hardware error",
            _ => panic!("Invalid OSError kind! (Linux, arch: Alpha)")
        }
    }

    pub fn error(&self) -> RawError {
        match self {
            OSError::ESUCCESS => RawError::RawOSError(OSError::ESUCCESS),
            OSError::EPERM => RawError::RawOSError(OSError::EPERM),
            OSError::ENOENT => RawError::RawOSError(OSError::ENOENT),
            OSError::ESRCH => RawError::RawOSError(OSError::ESRCH),
            OSError::EINTR => RawError::RawOSError(OSError::EINTR),
            OSError::EIO => RawError::RawOSError(OSError::EIO),
            OSError::ENXIO => RawError::RawOSError(OSError::ENXIO),
            OSError::E2BIG => RawError::RawOSError(OSError::E2BIG),
            OSError::ENOEXEC => RawError::RawOSError(OSError::ENOEXEC),
            OSError::EBADF => RawError::RawOSError(OSError::EBADF),
            OSError::ECHILD => RawError::RawOSError(OSError::ECHILD),
            OSError::EDEADLK_EDEADLOCK => RawError::RawOSError(OSError::EDEADLK_EDEADLOCK),
            OSError::ENOMEM => RawError::RawOSError(OSError::ENOMEM),
            OSError::EACCES => RawError::RawOSError(OSError::EACCES),
            OSError::EFAULT => RawError::RawOSError(OSError::EFAULT),
            OSError::ENOTBLK => RawError::RawOSError(OSError::ENOTBLK),
            OSError::EBUSY => RawError::RawOSError(OSError::EBUSY),
            OSError::EEXIST => RawError::RawOSError(OSError::EEXIST),
            OSError::EXDEV => RawError::RawOSError(OSError::EXDEV),
            OSError::ENODEV => RawError::RawOSError(OSError::ENODEV),
            OSError::ENOTDIR => RawError::RawOSError(OSError::ENOTDIR),
            OSError::EISDIR => RawError::RawOSError(OSError::EISDIR),
            OSError::EINVAL => RawError::RawOSError(OSError::EINVAL),
            OSError::ENFILE => RawError::RawOSError(OSError::ENFILE),
            OSError::EMFILE => RawError::RawOSError(OSError::EMFILE),
            OSError::ENOTTY => RawError::RawOSError(OSError::ENOTTY),
            OSError::ETXTBSY => RawError::RawOSError(OSError::ETXTBSY),
            OSError::EFBIG => RawError::RawOSError(OSError::EFBIG),
            OSError::ENOSPC => RawError::RawOSError(OSError::ENOSPC),
            OSError::ESPIPE => RawError::RawOSError(OSError::ESPIPE),
            OSError::EROFS => RawError::RawOSError(OSError::EROFS),
            OSError::EMLINK => RawError::RawOSError(OSError::EMLINK),
            OSError::EPIPE => RawError::RawOSError(OSError::EPIPE),
            OSError::EDOM => RawError::RawOSError(OSError::EDOM),
            OSError::ERANGE => RawError::RawOSError(OSError::ERANGE),
            OSError::EAGAIN_EWOULDBLOCK => RawError::RawOSError(OSError::EAGAIN_EWOULDBLOCK),
            OSError::EINPROGRESS => RawError::RawOSError(OSError::EINPROGRESS),
            OSError::EALREADY => RawError::RawOSError(OSError::EALREADY),
            OSError::ENOTSOCK => RawError::RawOSError(OSError::ENOTSOCK),
            OSError::EDESTADDRREQ => RawError::RawOSError(OSError::EDESTADDRREQ),
            OSError::EMSGSIZE => RawError::RawOSError(OSError::EMSGSIZE),
            OSError::EPROTOTYPE => RawError::RawOSError(OSError::EPROTOTYPE),
            OSError::ENOPROTOOPT => RawError::RawOSError(OSError::ENOPROTOOPT),
            OSError::EPROTONOSUPPORT => RawError::RawOSError(OSError::EPROTONOSUPPORT),
            OSError::ESOCKTNOSUPPORT => RawError::RawOSError(OSError::ESOCKTNOSUPPORT),
            OSError::EOPNOTSUPP => RawError::RawOSError(OSError::EOPNOTSUPP),
            OSError::EPFNOSUPPORT => RawError::RawOSError(OSError::EPFNOSUPPORT),
            OSError::EAFNOSUPPORT => RawError::RawOSError(OSError::EAFNOSUPPORT),
            OSError::EADDRINUSE => RawError::RawOSError(OSError::EADDRINUSE),
            OSError::EADDRNOTAVAIL => RawError::RawOSError(OSError::EADDRNOTAVAIL),
            OSError::ENETDOWN => RawError::RawOSError(OSError::ENETDOWN),
            OSError::ENETUNREACH => RawError::RawOSError(OSError::ENETUNREACH),
            OSError::ENETRESET => RawError::RawOSError(OSError::ENETRESET),
            OSError::ECONNABORTED => RawError::RawOSError(OSError::ECONNABORTED),
            OSError::ECONNRESET => RawError::RawOSError(OSError::ECONNRESET),
            OSError::ENOBUFS => RawError::RawOSError(OSError::ENOBUFS),
            OSError::EISCONN => RawError::RawOSError(OSError::EISCONN),
            OSError::ENOTCONN => RawError::RawOSError(OSError::ENOTCONN),
            OSError::ESHUTDOWN => RawError::RawOSError(OSError::ESHUTDOWN),
            OSError::ETOOMANYREFS => RawError::RawOSError(OSError::ETOOMANYREFS),
            OSError::ETIMEDOUT => RawError::RawOSError(OSError::ETIMEDOUT),
            OSError::ECONNREFUSED => RawError::RawOSError(OSError::ECONNREFUSED),
            OSError::ELOOP => RawError::RawOSError(OSError::ELOOP),
            OSError::ENAMETOOLONG => RawError::RawOSError(OSError::ENAMETOOLONG),
            OSError::EHOSTDOWN => RawError::RawOSError(OSError::EHOSTDOWN),
            OSError::EHOSTUNREACH => RawError::RawOSError(OSError::EHOSTUNREACH),
            OSError::ENOTEMPTY => RawError::RawOSError(OSError::ENOTEMPTY),
            OSError::EUSERS => RawError::RawOSError(OSError::EUSERS),
            OSError::EDQUOT => RawError::RawOSError(OSError::EDQUOT),
            OSError::ESTALE => RawError::RawOSError(OSError::ESTALE),
            OSError::EREMOTE => RawError::RawOSError(OSError::EREMOTE),
            OSError::ENOLCK => RawError::RawOSError(OSError::ENOLCK),
            OSError::ENOSYS => RawError::RawOSError(OSError::ENOSYS),
            OSError::ENOMSG => RawError::RawOSError(OSError::ENOMSG),
            OSError::EIDRM => RawError::RawOSError(OSError::EIDRM),
            OSError::ENOSR => RawError::RawOSError(OSError::ENOSR),
            OSError::ETIME => RawError::RawOSError(OSError::ETIME),
            OSError::EBADMSG => RawError::RawOSError(OSError::EBADMSG),
            OSError::EPROTO => RawError::RawOSError(OSError::EPROTO),
            OSError::ENODATA => RawError::RawOSError(OSError::ENODATA),
            OSError::ENOSTR => RawError::RawOSError(OSError::ENOSTR),
            OSError::ENOPKG => RawError::RawOSError(OSError::ENOPKG),
            OSError::EILSEQ => RawError::RawOSError(OSError::EILSEQ),
            OSError::ECHRNG => RawError::RawOSError(OSError::ECHRNG),
            OSError::EL2NSYNC => RawError::RawOSError(OSError::EL2NSYNC),
            OSError::EL3HLT => RawError::RawOSError(OSError::EL3HLT),
            OSError::EL3RST => RawError::RawOSError(OSError::EL3RST),
            OSError::ELNRNG => RawError::RawOSError(OSError::ELNRNG),
            OSError::EUNATCH => RawError::RawOSError(OSError::EUNATCH),
            OSError::ENOCSI => RawError::RawOSError(OSError::ENOCSI),
            OSError::EL2HLT => RawError::RawOSError(OSError::EL2HLT),
            OSError::EBADE => RawError::RawOSError(OSError::EBADE),
            OSError::EBADR => RawError::RawOSError(OSError::EBADR),
            OSError::EXFULL => RawError::RawOSError(OSError::EXFULL),
            OSError::ENOANO => RawError::RawOSError(OSError::ENOANO),
            OSError::EBADRQC => RawError::RawOSError(OSError::EBADRQC),
            OSError::EBADSLT => RawError::RawOSError(OSError::EBADSLT),
            OSError::EBFONT => RawError::RawOSError(OSError::EBFONT),
            OSError::ENONET => RawError::RawOSError(OSError::ENONET),
            OSError::ENOLINK => RawError::RawOSError(OSError::ENOLINK),
            OSError::EADV => RawError::RawOSError(OSError::EADV),
            OSError::ESRMNT => RawError::RawOSError(OSError::ESRMNT),
            OSError::ECOMM => RawError::RawOSError(OSError::ECOMM),
            OSError::EMULTIHOP => RawError::RawOSError(OSError::EMULTIHOP),
            OSError::EDOTDOT => RawError::RawOSError(OSError::EDOTDOT),
            OSError::EOVERFLOW => RawError::RawOSError(OSError::EOVERFLOW),
            OSError::ENOTUNIQ => RawError::RawOSError(OSError::ENOTUNIQ),
            OSError::EBADFD => RawError::RawOSError(OSError::EBADFD),
            OSError::EREMCHG => RawError::RawOSError(OSError::EREMCHG),
            OSError::EUCLEAN => RawError::RawOSError(OSError::EUCLEAN),
            OSError::ENOTNAM => RawError::RawOSError(OSError::ENOTNAM),
            OSError::ENAVAIL => RawError::RawOSError(OSError::ENAVAIL),
            OSError::EISNAM => RawError::RawOSError(OSError::EISNAM),
            OSError::EREMOTEIO => RawError::RawOSError(OSError::EREMOTEIO),
            OSError::ELIBACC => RawError::RawOSError(OSError::ELIBACC),
            OSError::ELIBBAD => RawError::RawOSError(OSError::ELIBBAD),
            OSError::ELIBSCN => RawError::RawOSError(OSError::ELIBSCN),
            OSError::ELIBMAX => RawError::RawOSError(OSError::ELIBMAX),
            OSError::ELIBEXEC => RawError::RawOSError(OSError::ELIBEXEC),
            OSError::ERESTART => RawError::RawOSError(OSError::ERESTART),
            OSError::ESTRPIPE => RawError::RawOSError(OSError::ESTRPIPE),
            OSError::ENOMEDIUM => RawError::RawOSError(OSError::ENOMEDIUM),
            OSError::EMEDIUMTYPE => RawError::RawOSError(OSError::EMEDIUMTYPE),
            OSError::ECANCELED => RawError::RawOSError(OSError::ECANCELED),
            OSError::ENOKEY => RawError::RawOSError(OSError::ENOKEY),
            OSError::EKEYEXPIRED => RawError::RawOSError(OSError::EKEYEXPIRED),
            OSError::EKEYREVOKED => RawError::RawOSError(OSError::EKEYREVOKED),
            OSError::EKEYREJECTED => RawError::RawOSError(OSError::EKEYREJECTED),
            OSError::EOWNERDEAD => RawError::RawOSError(OSError::EOWNERDEAD),
            OSError::ENOTRECOVERABLE => RawError::RawOSError(OSError::ENOTRECOVERABLE),
            OSError::ERFKILL => RawError::RawOSError(OSError::ERFKILL),
            OSError::EHWPOISON => RawError::RawOSError(OSError::EHWPOISON),
            _ => panic!("Invalid OSError kind! (Linux, arch: Alpha)")
        }
    }

    pub fn kind_from_name(name: &str) -> OSError {
        match name {
            "ESUCCESS" => OSError::ESUCCESS,
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
            "EDEADLK" | "EDEADLOCK" | "EDEADLK_EDEADLOCK" => OSError::EDEADLK_EDEADLOCK,
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
            "EAGAIN" | "EWOULDBLOCK" | "EAGAIN_EWOULDBLOCK" => OSError::EAGAIN_EWOULDBLOCK,
            "EINPROGRESS" => OSError::EINPROGRESS,
            "EALREADY" => OSError::EALREADY,
            "ENOTSOCK" => OSError::ENOTSOCK,
            "EDESTADDRREQ" => OSError::EDESTADDRREQ,
            "EMSGSIZE" => OSError::EMSGSIZE,
            "EPROTOTYPE" => OSError::EPROTOTYPE,
            "ENOPROTOOPT" => OSError::ENOPROTOOPT,
            "EPROTONOSUPPORT" => OSError::EPROTONOSUPPORT,
            "ESOCKTNOSUPPORT" => OSError::ESOCKTNOSUPPORT,
            "EOPNOTSUPP" => OSError::EOPNOTSUPP,
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
            "EUSERS" => OSError::EUSERS,
            "EDQUOT" => OSError::EDQUOT,
            "ESTALE" => OSError::ESTALE,
            "EREMOTE" => OSError::EREMOTE,
            "ENOLCK" => OSError::ENOLCK,
            "ENOSYS" => OSError::ENOSYS,
            "ENOMSG" => OSError::ENOMSG,
            "EIDRM" => OSError::EIDRM,
            "ENOSR" => OSError::ENOSR,
            "ETIME" => OSError::ETIME,
            "EBADMSG" => OSError::EBADMSG,
            "EPROTO" => OSError::EPROTO,
            "ENODATA" => OSError::ENODATA,
            "ENOSTR" => OSError::ENOSTR,
            "ENOPKG" => OSError::ENOPKG,
            "EILSEQ" => OSError::EILSEQ,
            "ECHRNG" => OSError::ECHRNG,
            "EL2NSYNC" => OSError::EL2NSYNC,
            "EL3HLT" => OSError::EL3HLT,
            "EL3RST" => OSError::EL3RST,
            "ELNRNG" => OSError::ELNRNG,
            "EUNATCH" => OSError::EUNATCH,
            "ENOCSI" => OSError::ENOCSI,
            "EL2HLT" => OSError::EL2HLT,
            "EBADE" => OSError::EBADE,
            "EBADR" => OSError::EBADR,
            "EXFULL" => OSError::EXFULL,
            "ENOANO" => OSError::ENOANO,
            "EBADRQC" => OSError::EBADRQC,
            "EBADSLT" => OSError::EBADSLT,
            "EBFONT" => OSError::EBFONT,
            "ENONET" => OSError::ENONET,
            "ENOLINK" => OSError::ENOLINK,
            "EADV" => OSError::EADV,
            "ESRMNT" => OSError::ESRMNT,
            "ECOMM" => OSError::ECOMM,
            "EMULTIHOP" => OSError::EMULTIHOP,
            "EDOTDOT" => OSError::EDOTDOT,
            "EOVERFLOW" => OSError::EOVERFLOW,
            "ENOTUNIQ" => OSError::ENOTUNIQ,
            "EBADFD" => OSError::EBADFD,
            "EREMCHG" => OSError::EREMCHG,
            "EUCLEAN" => OSError::EUCLEAN,
            "ENOTNAM" => OSError::ENOTNAM,
            "ENAVAIL" => OSError::ENAVAIL,
            "EISNAM" => OSError::EISNAM,
            "EREMOTEIO" => OSError::EREMOTEIO,
            "ELIBACC" => OSError::ELIBACC,
            "ELIBBAD" => OSError::ELIBBAD,
            "ELIBSCN" => OSError::ELIBSCN,
            "ELIBMAX" => OSError::ELIBMAX,
            "ELIBEXEC" => OSError::ELIBEXEC,
            "ERESTART" => OSError::ERESTART,
            "ESTRPIPE" => OSError::ESTRPIPE,
            "ENOMEDIUM" => OSError::ENOMEDIUM,
            "EMEDIUMTYPE" => OSError::EMEDIUMTYPE,
            "ECANCELED" => OSError::ECANCELED,
            "ENOKEY" => OSError::ENOKEY,
            "EKEYEXPIRED" => OSError::EKEYEXPIRED,
            "EKEYREVOKED" => OSError::EKEYREVOKED,
            "EKEYREJECTED" => OSError::EKEYREJECTED,
            "EOWNERDEAD" => OSError::EOWNERDEAD,
            "ENOTRECOVERABLE" => OSError::ENOTRECOVERABLE,
            "ERFKILL" => OSError::ERFKILL,
            "EHWPOISON" => OSError::EHWPOISON,
            _ => panic!("Invalid OSError name: {}! (Linux, arch: Alpha)", name)
        }
    }

    pub fn kind_from_code(code: &u32) -> OSError {
        match code {
            0 => OSError::ESUCCESS,
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
            11 => OSError::EDEADLK_EDEADLOCK,
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
            35 => OSError::EAGAIN_EWOULDBLOCK,
            36 => OSError::EINPROGRESS,
            37 => OSError::EALREADY,
            38 => OSError::ENOTSOCK,
            39 => OSError::EDESTADDRREQ,
            40 => OSError::EMSGSIZE,
            41 => OSError::EPROTOTYPE,
            42 => OSError::ENOPROTOOPT,
            43 => OSError::EPROTONOSUPPORT,
            44 => OSError::ESOCKTNOSUPPORT,
            45 => OSError::EOPNOTSUPP,
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
            68 => OSError::EUSERS,
            69 => OSError::EDQUOT,
            70 => OSError::ESTALE,
            71 => OSError::EREMOTE,
            77 => OSError::ENOLCK,
            78 => OSError::ENOSYS,
            80 => OSError::ENOMSG,
            81 => OSError::EIDRM,
            82 => OSError::ENOSR,
            83 => OSError::ETIME,
            84 => OSError::EBADMSG,
            85 => OSError::EPROTO,
            86 => OSError::ENODATA,
            87 => OSError::ENOSTR,
            92 => OSError::ENOPKG,
            116 => OSError::EILSEQ,
            88 => OSError::ECHRNG,
            89 => OSError::EL2NSYNC,
            90 => OSError::EL3HLT,
            91 => OSError::EL3RST,
            93 => OSError::ELNRNG,
            94 => OSError::EUNATCH,
            95 => OSError::ENOCSI,
            96 => OSError::EL2HLT,
            97 => OSError::EBADE,
            98 => OSError::EBADR,
            99 => OSError::EXFULL,
            100 => OSError::ENOANO,
            101 => OSError::EBADRQC,
            102 => OSError::EBADSLT,
            104 => OSError::EBFONT,
            105 => OSError::ENONET,
            106 => OSError::ENOLINK,
            107 => OSError::EADV,
            108 => OSError::ESRMNT,
            109 => OSError::ECOMM,
            110 => OSError::EMULTIHOP,
            111 => OSError::EDOTDOT,
            112 => OSError::EOVERFLOW,
            113 => OSError::ENOTUNIQ,
            114 => OSError::EBADFD,
            115 => OSError::EREMCHG,
            117 => OSError::EUCLEAN,
            118 => OSError::ENOTNAM,
            119 => OSError::ENAVAIL,
            120 => OSError::EISNAM,
            121 => OSError::EREMOTEIO,
            122 => OSError::ELIBACC,
            123 => OSError::ELIBBAD,
            124 => OSError::ELIBSCN,
            125 => OSError::ELIBMAX,
            126 => OSError::ELIBEXEC,
            127 => OSError::ERESTART,
            128 => OSError::ESTRPIPE,
            129 => OSError::ENOMEDIUM,
            130 => OSError::EMEDIUMTYPE,
            131 => OSError::ECANCELED,
            132 => OSError::ENOKEY,
            133 => OSError::EKEYEXPIRED,
            134 => OSError::EKEYREVOKED,
            135 => OSError::EKEYREJECTED,
            136 => OSError::EOWNERDEAD,
            137 => OSError::ENOTRECOVERABLE,
            138 => OSError::ERFKILL,
            139 => OSError::EHWPOISON,
            _ => panic!("Invalid OSError code: {}! (Linux, arch: Alpha)", code)
        }
    }
}