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

#[derive(Clone, Debug, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OSError {
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
    EAGAIN = 11,
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
    EDEADLK = 35,
    ENAMETOOLONG = 36,
    ENOLCK = 37,
    ENOSYS = 38,
    ENOTEMPTY = 39,
    ELOOP = 40,
    ENOMSG = 42,
    EIDRM = 43,
    ECHRNG = 44,
    EL2NSYNC = 45,
    EL3HLT = 46,
    EL3RST = 47,
    ELNRNG = 48,
    EUNATCH = 49,
    ENOCSI = 50,
    EL2HLT = 51,
    EBADE = 52,
    EBADR = 53,
    EXFULL = 54,
    ENOANO = 55,
    EBADRQC = 56,
    EBADSLT = 57,
    EBFONT = 59,
    ENOSTR = 60,
    ENODATA = 61,
    ETIME = 62,
    ENOSR = 63,
    ENONET = 64,
    ENOPKG = 65,
    EREMOTE = 66,
    ENOLINK = 67,
    EADV = 68,
    ESRMNT = 69,
    ECOMM = 70,
    EPROTO = 71,
    EMULTIHOP = 72,
    EDOTDOT = 73,
    EBADMSG = 74,
    EOVERFLOW = 75,
    ENOTUNIQ = 76,
    EBADFD = 77,
    EREMCHG = 78,
    ELIBACC = 79,
    ELIBBAD = 80,
    ELIBSCN = 81,
    ELIBMAX = 82,
    ELIBEXEC = 83,
    EILSEQ = 84,
    ERESTART = 85,
    ESTRPIPE = 86,
    EUSERS = 87,
    ENOTSOCK = 88,
    EDESTADDRREQ = 89,
    EMSGSIZE = 90,
    EPROTOTYPE = 91,
    ENOPROTOOPT = 92,
    EPROTONOSUPPORT = 93,
    ESOCKTNOSUPPORT = 94,
    EOPNOTSUPP = 95,
    EPFNOSUPPORT = 96,
    EAFNOSUPPORT = 97,
    EADDRINUSE = 98,
    EADDRNOTAVAIL = 99,
    ENETDOWN = 100,
    ENETUNREACH = 101,
    ENETRESET = 102,
    ECONNABORTED = 103,
    ECONNRESET = 104,
    ENOBUFS = 105,
    EISCONN = 106,
    ENOTCONN = 107,
    ESHUTDOWN = 108,
    ETOOMANYREFS = 109,
    ETIMEDOUT = 110,
    ECONNREFUSED = 111,
    EHOSTDOWN = 112,
    EHOSTUNREACH = 113,
    EALREADY = 114,
    EINPROGRESS = 115,
    ESTALE = 116,
    EUCLEAN = 117,
    ENOTNAM = 118,
    ENAVAIL = 119,
    EISNAM = 120,
    EREMOTEIO = 121,
    EDQUOT = 122,
    ENOMEDIUM = 123,
    EMEDIUMTYPE = 124,
    ECANCELED = 125,
    ENOKEY = 126,
    EKEYEXPIRED = 127,
    EKEYREVOKED = 128,
    EKEYREJECTED = 129,
    EOWNERDEAD = 130,
    ENOTRECOVERABLE = 131,
}

impl OSError {
    pub fn code(&self) -> u32 {
        match self {
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
            OSError::EAGAIN => OSError::EAGAIN as u32,
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
            OSError::EDEADLK => OSError::EDEADLK as u32,
            OSError::ENAMETOOLONG => OSError::ENAMETOOLONG as u32,
            OSError::ENOLCK => OSError::ENOLCK as u32,
            OSError::ENOSYS => OSError::ENOSYS as u32,
            OSError::ENOTEMPTY => OSError::ENOTEMPTY as u32,
            OSError::ELOOP => OSError::ELOOP as u32,
            OSError::ENOMSG => OSError::ENOMSG as u32,
            OSError::EIDRM => OSError::EIDRM as u32,
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
            OSError::ENOSTR => OSError::ENOSTR as u32,
            OSError::ENODATA => OSError::ENODATA as u32,
            OSError::ETIME => OSError::ETIME as u32,
            OSError::ENOSR => OSError::ENOSR as u32,
            OSError::ENONET => OSError::ENONET as u32,
            OSError::ENOPKG => OSError::ENOPKG as u32,
            OSError::EREMOTE => OSError::EREMOTE as u32,
            OSError::ENOLINK => OSError::ENOLINK as u32,
            OSError::EADV => OSError::EADV as u32,
            OSError::ESRMNT => OSError::ESRMNT as u32,
            OSError::ECOMM => OSError::ECOMM as u32,
            OSError::EPROTO => OSError::EPROTO as u32,
            OSError::EMULTIHOP => OSError::EMULTIHOP as u32,
            OSError::EDOTDOT => OSError::EDOTDOT as u32,
            OSError::EBADMSG => OSError::EBADMSG as u32,
            OSError::EOVERFLOW => OSError::EOVERFLOW as u32,
            OSError::ENOTUNIQ => OSError::ENOTUNIQ as u32,
            OSError::EBADFD => OSError::EBADFD as u32,
            OSError::EREMCHG => OSError::EREMCHG as u32,
            OSError::ELIBACC => OSError::ELIBACC as u32,
            OSError::ELIBBAD => OSError::ELIBBAD as u32,
            OSError::ELIBSCN => OSError::ELIBSCN as u32,
            OSError::ELIBMAX => OSError::ELIBMAX as u32,
            OSError::ELIBEXEC => OSError::ELIBEXEC as u32,
            OSError::EILSEQ => OSError::EILSEQ as u32,
            OSError::ERESTART => OSError::ERESTART as u32,
            OSError::ESTRPIPE => OSError::ESTRPIPE as u32,
            OSError::EUSERS => OSError::EUSERS as u32,
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
            OSError::EHOSTDOWN => OSError::EHOSTDOWN as u32,
            OSError::EHOSTUNREACH => OSError::EHOSTUNREACH as u32,
            OSError::EALREADY => OSError::EALREADY as u32,
            OSError::EINPROGRESS => OSError::EINPROGRESS as u32,
            OSError::ESTALE => OSError::ESTALE as u32,
            OSError::EUCLEAN => OSError::EUCLEAN as u32,
            OSError::ENOTNAM => OSError::ENOTNAM as u32,
            OSError::ENAVAIL => OSError::ENAVAIL as u32,
            OSError::EISNAM => OSError::EISNAM as u32,
            OSError::EREMOTEIO => OSError::EREMOTEIO as u32,
            OSError::EDQUOT => OSError::EDQUOT as u32,
            OSError::ENOMEDIUM => OSError::ENOMEDIUM as u32,
            OSError::EMEDIUMTYPE => OSError::EMEDIUMTYPE as u32,
            OSError::ECANCELED => OSError::ECANCELED as u32,
            OSError::ENOKEY => OSError::ENOKEY as u32,
            OSError::EKEYEXPIRED => OSError::EKEYEXPIRED as u32,
            OSError::EKEYREVOKED => OSError::EKEYREVOKED as u32,
            OSError::EKEYREJECTED => OSError::EKEYREJECTED as u32,
            OSError::EOWNERDEAD => OSError::EOWNERDEAD as u32,
            OSError::ENOTRECOVERABLE => OSError::ENOTRECOVERABLE as u32,
            _ => panic!("Invalid OSError kind! (Linux)")
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
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
            OSError::EAGAIN => "Try again",
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
            OSError::EDEADLK => "Resource deadlock would occur",
            OSError::ENAMETOOLONG => "File name too long",
            OSError::ENOLCK => "No record locks available",
            OSError::ENOSYS => "Function not implemented",
            OSError::ENOTEMPTY => "Directory not empty",
            OSError::ELOOP => "Too many symbolic links encountered",
            OSError::ENOMSG => "No message of desired type",
            OSError::EIDRM => "Identifier removed",
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
            OSError::ENOSTR => "Device not a stream",
            OSError::ENODATA => "No data available",
            OSError::ETIME => "Timer expired",
            OSError::ENOSR => "Out of streams resources",
            OSError::ENONET => "Machine is not on the network",
            OSError::ENOPKG => "Package not installed",
            OSError::EREMOTE => "Object is remote",
            OSError::ENOLINK => "Link has been severed",
            OSError::EADV => "Advertise error",
            OSError::ESRMNT => "Srmount error",
            OSError::ECOMM => "Communication error on send",
            OSError::EPROTO => "Protocol error",
            OSError::EMULTIHOP => "Multihop attempted",
            OSError::EDOTDOT => "RFS specific error",
            OSError::EBADMSG => "Not a data message",
            OSError::EOVERFLOW => "Value too large for defined data type",
            OSError::ENOTUNIQ => "Name not unique on network",
            OSError::EBADFD => "File descriptor in bad state",
            OSError::EREMCHG => "Remote address changed",
            OSError::ELIBACC => "Can not access a needed shared library",
            OSError::ELIBBAD => "Accessing a corrupted shared library",
            OSError::ELIBSCN => ".lib section in a.out corrupted",
            OSError::ELIBMAX => "Attempting to link in too many shared libraries",
            OSError::ELIBEXEC => "Cannot exec a shared library directly",
            OSError::EILSEQ => "Illegal byte sequence",
            OSError::ERESTART => "Interrupted system call should be restarted",
            OSError::ESTRPIPE => "Streams pipe error",
            OSError::EUSERS => "Too many users",
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
            OSError::EHOSTDOWN => "Host is down",
            OSError::EHOSTUNREACH => "No route to host",
            OSError::EALREADY => "Operation already in progress",
            OSError::EINPROGRESS => "Operation now in progress",
            OSError::ESTALE => "Stale file handle",
            OSError::EUCLEAN => "Structure needs cleaning",
            OSError::ENOTNAM => "Not a XENIX named type file",
            OSError::ENAVAIL => "No XENIX semaphores available",
            OSError::EISNAM => "Is a named type file",
            OSError::EREMOTEIO => "Remote I/O error",
            OSError::EDQUOT => "Quota exceeded",
            OSError::ENOMEDIUM => "No medium found",
            OSError::EMEDIUMTYPE => "Wrong medium type",
            OSError::ECANCELED => "Operation canceled",
            OSError::ENOKEY => "Required key not available",
            OSError::EKEYEXPIRED => "Key has expired",
            OSError::EKEYREVOKED => "Key has been revoked",
            OSError::EKEYREJECTED => "Key was rejected by service",
            OSError::EOWNERDEAD => "Owner died",
            OSError::ENOTRECOVERABLE => "State not recoverable",
            _ => panic!("Invalid OSError kind! (Linux)")
        }
    }

    pub fn kind(&self) -> OSError {
        match self {
            OSError::EPERM => OSError::EPERM,
            OSError::ENOENT => OSError::ENOENT,
            OSError::ESRCH => OSError::ESRCH,
            OSError::EINTR => OSError::EINTR,
            OSError::EIO => OSError::EIO,
            OSError::ENXIO => OSError::ENXIO,
            OSError::E2BIG => OSError::E2BIG,
            OSError::ENOEXEC => OSError::ENOEXEC,
            OSError::EBADF => OSError::EBADF,
            OSError::ECHILD => OSError::ECHILD,
            OSError::EAGAIN => OSError::EAGAIN,
            OSError::ENOMEM => OSError::ENOMEM,
            OSError::EACCES => OSError::EACCES,
            OSError::EFAULT => OSError::EFAULT,
            OSError::ENOTBLK => OSError::ENOTBLK,
            OSError::EBUSY => OSError::EBUSY,
            OSError::EEXIST => OSError::EEXIST,
            OSError::EXDEV => OSError::EXDEV,
            OSError::ENODEV => OSError::ENODEV,
            OSError::ENOTDIR => OSError::ENOTDIR,
            OSError::EISDIR => OSError::EISDIR,
            OSError::EINVAL => OSError::EINVAL,
            OSError::ENFILE => OSError::ENFILE,
            OSError::EMFILE => OSError::EMFILE,
            OSError::ENOTTY => OSError::ENOTTY,
            OSError::ETXTBSY => OSError::ETXTBSY,
            OSError::EFBIG => OSError::EFBIG,
            OSError::ENOSPC => OSError::ENOSPC,
            OSError::ESPIPE => OSError::ESPIPE,
            OSError::EROFS => OSError::EROFS,
            OSError::EMLINK => OSError::EMLINK,
            OSError::EPIPE => OSError::EPIPE,
            OSError::EDOM => OSError::EDOM,
            OSError::ERANGE => OSError::ERANGE,
            OSError::EDEADLK => OSError::EDEADLK,
            OSError::ENAMETOOLONG => OSError::ENAMETOOLONG,
            OSError::ENOLCK => OSError::ENOLCK,
            OSError::ENOSYS => OSError::ENOSYS,
            OSError::ENOTEMPTY => OSError::ENOTEMPTY,
            OSError::ELOOP => OSError::ELOOP,
            OSError::ENOMSG => OSError::ENOMSG,
            OSError::EIDRM => OSError::EIDRM,
            OSError::ECHRNG => OSError::ECHRNG,
            OSError::EL2NSYNC => OSError::EL2NSYNC,
            OSError::EL3HLT => OSError::EL3HLT,
            OSError::EL3RST => OSError::EL3RST,
            OSError::ELNRNG => OSError::ELNRNG,
            OSError::EUNATCH => OSError::EUNATCH,
            OSError::ENOCSI => OSError::ENOCSI,
            OSError::EL2HLT => OSError::EL2HLT,
            OSError::EBADE => OSError::EBADE,
            OSError::EBADR => OSError::EBADR,
            OSError::EXFULL => OSError::EXFULL,
            OSError::ENOANO => OSError::ENOANO,
            OSError::EBADRQC => OSError::EBADRQC,
            OSError::EBADSLT => OSError::EBADSLT,
            OSError::EBFONT => OSError::EBFONT,
            OSError::ENOSTR => OSError::ENOSTR,
            OSError::ENODATA => OSError::ENODATA,
            OSError::ETIME => OSError::ETIME,
            OSError::ENOSR => OSError::ENOSR,
            OSError::ENONET => OSError::ENONET,
            OSError::ENOPKG => OSError::ENOPKG,
            OSError::EREMOTE => OSError::EREMOTE,
            OSError::ENOLINK => OSError::ENOLINK,
            OSError::EADV => OSError::EADV,
            OSError::ESRMNT => OSError::ESRMNT,
            OSError::ECOMM => OSError::ECOMM,
            OSError::EPROTO => OSError::EPROTO,
            OSError::EMULTIHOP => OSError::EMULTIHOP,
            OSError::EDOTDOT => OSError::EDOTDOT,
            OSError::EBADMSG => OSError::EBADMSG,
            OSError::EOVERFLOW => OSError::EOVERFLOW,
            OSError::ENOTUNIQ => OSError::ENOTUNIQ,
            OSError::EBADFD => OSError::EBADFD,
            OSError::EREMCHG => OSError::EREMCHG,
            OSError::ELIBACC => OSError::ELIBACC,
            OSError::ELIBBAD => OSError::ELIBBAD,
            OSError::ELIBSCN => OSError::ELIBSCN,
            OSError::ELIBMAX => OSError::ELIBMAX,
            OSError::ELIBEXEC => OSError::ELIBEXEC,
            OSError::EILSEQ => OSError::EILSEQ,
            OSError::ERESTART => OSError::ERESTART,
            OSError::ESTRPIPE => OSError::ESTRPIPE,
            OSError::EUSERS => OSError::EUSERS,
            OSError::ENOTSOCK => OSError::ENOTSOCK,
            OSError::EDESTADDRREQ => OSError::EDESTADDRREQ,
            OSError::EMSGSIZE => OSError::EMSGSIZE,
            OSError::EPROTOTYPE => OSError::EPROTOTYPE,
            OSError::ENOPROTOOPT => OSError::ENOPROTOOPT,
            OSError::EPROTONOSUPPORT => OSError::EPROTONOSUPPORT,
            OSError::ESOCKTNOSUPPORT => OSError::ESOCKTNOSUPPORT,
            OSError::EOPNOTSUPP => OSError::EOPNOTSUPP,
            OSError::EPFNOSUPPORT => OSError::EPFNOSUPPORT,
            OSError::EAFNOSUPPORT => OSError::EAFNOSUPPORT,
            OSError::EADDRINUSE => OSError::EADDRINUSE,
            OSError::EADDRNOTAVAIL => OSError::EADDRNOTAVAIL,
            OSError::ENETDOWN => OSError::ENETDOWN,
            OSError::ENETUNREACH => OSError::ENETUNREACH,
            OSError::ENETRESET => OSError::ENETRESET,
            OSError::ECONNABORTED => OSError::ECONNABORTED,
            OSError::ECONNRESET => OSError::ECONNRESET,
            OSError::ENOBUFS => OSError::ENOBUFS,
            OSError::EISCONN => OSError::EISCONN,
            OSError::ENOTCONN => OSError::ENOTCONN,
            OSError::ESHUTDOWN => OSError::ESHUTDOWN,
            OSError::ETOOMANYREFS => OSError::ETOOMANYREFS,
            OSError::ETIMEDOUT => OSError::ETIMEDOUT,
            OSError::ECONNREFUSED => OSError::ECONNREFUSED,
            OSError::EHOSTDOWN => OSError::EHOSTDOWN,
            OSError::EHOSTUNREACH => OSError::EHOSTUNREACH,
            OSError::EALREADY => OSError::EALREADY,
            OSError::EINPROGRESS => OSError::EINPROGRESS,
            OSError::ESTALE => OSError::ESTALE,
            OSError::EUCLEAN => OSError::EUCLEAN,
            OSError::ENOTNAM => OSError::ENOTNAM,
            OSError::ENAVAIL => OSError::ENAVAIL,
            OSError::EISNAM => OSError::EISNAM,
            OSError::EREMOTEIO => OSError::EREMOTEIO,
            OSError::EDQUOT => OSError::EDQUOT,
            OSError::ENOMEDIUM => OSError::ENOMEDIUM,
            OSError::EMEDIUMTYPE => OSError::EMEDIUMTYPE,
            OSError::ECANCELED => OSError::ECANCELED,
            OSError::ENOKEY => OSError::ENOKEY,
            OSError::EKEYEXPIRED => OSError::EKEYEXPIRED,
            OSError::EKEYREVOKED => OSError::EKEYREVOKED,
            OSError::EKEYREJECTED => OSError::EKEYREJECTED,
            OSError::EOWNERDEAD => OSError::EOWNERDEAD,
            OSError::ENOTRECOVERABLE => OSError::ENOTRECOVERABLE,
            _ => panic!("Invalid OSError kind! (Linux)")
        }
    }

    pub fn kind_from_name(name: &str) -> OSError {
        match name {
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
            "EAGAIN" => OSError::EAGAIN,
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
            "EDEADLK" => OSError::EDEADLK,
            "ENAMETOOLONG" => OSError::ENAMETOOLONG,
            "ENOLCK" => OSError::ENOLCK,
            "ENOSYS" => OSError::ENOSYS,
            "ENOTEMPTY" => OSError::ENOTEMPTY,
            "ELOOP" => OSError::ELOOP,
            "ENOMSG" => OSError::ENOMSG,
            "EIDRM" => OSError::EIDRM,
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
            "ENOSTR" => OSError::ENOSTR,
            "ENODATA" => OSError::ENODATA,
            "ETIME" => OSError::ETIME,
            "ENOSR" => OSError::ENOSR,
            "ENONET" => OSError::ENONET,
            "ENOPKG" => OSError::ENOPKG,
            "EREMOTE" => OSError::EREMOTE,
            "ENOLINK" => OSError::ENOLINK,
            "EADV" => OSError::EADV,
            "ESRMNT" => OSError::ESRMNT,
            "ECOMM" => OSError::ECOMM,
            "EPROTO" => OSError::EPROTO,
            "EMULTIHOP" => OSError::EMULTIHOP,
            "EDOTDOT" => OSError::EDOTDOT,
            "EBADMSG" => OSError::EBADMSG,
            "EOVERFLOW" => OSError::EOVERFLOW,
            "ENOTUNIQ" => OSError::ENOTUNIQ,
            "EBADFD" => OSError::EBADFD,
            "EREMCHG" => OSError::EREMCHG,
            "ELIBACC" => OSError::ELIBACC,
            "ELIBBAD" => OSError::ELIBBAD,
            "ELIBSCN" => OSError::ELIBSCN,
            "ELIBMAX" => OSError::ELIBMAX,
            "ELIBEXEC" => OSError::ELIBEXEC,
            "EILSEQ" => OSError::EILSEQ,
            "ERESTART" => OSError::ERESTART,
            "ESTRPIPE" => OSError::ESTRPIPE,
            "EUSERS" => OSError::EUSERS,
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
            "EHOSTDOWN" => OSError::EHOSTDOWN,
            "EHOSTUNREACH" => OSError::EHOSTUNREACH,
            "EALREADY" => OSError::EALREADY,
            "EINPROGRESS" => OSError::EINPROGRESS,
            "ESTALE" => OSError::ESTALE,
            "EUCLEAN" => OSError::EUCLEAN,
            "ENOTNAM" => OSError::ENOTNAM,
            "ENAVAIL" => OSError::ENAVAIL,
            "EISNAM" => OSError::EISNAM,
            "EREMOTEIO" => OSError::EREMOTEIO,
            "EDQUOT" => OSError::EDQUOT,
            "ENOMEDIUM" => OSError::ENOMEDIUM,
            "EMEDIUMTYPE" => OSError::EMEDIUMTYPE,
            "ECANCELED" => OSError::ECANCELED,
            "ENOKEY" => OSError::ENOKEY,
            "EKEYEXPIRED" => OSError::EKEYEXPIRED,
            "EKEYREVOKED" => OSError::EKEYREVOKED,
            "EKEYREJECTED" => OSError::EKEYREJECTED,
            "EOWNERDEAD" => OSError::EOWNERDEAD,
            "ENOTRECOVERABLE" => OSError::ENOTRECOVERABLE,
            _ => panic!("Invalid OSError name: {}! (Linux)", name)
        }
    }

    pub fn kind_from_code(code: &u32) -> OSError {
        match code {
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
            11 => OSError::EAGAIN,
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
            35 => OSError::EDEADLK,
            36 => OSError::ENAMETOOLONG,
            37 => OSError::ENOLCK,
            38 => OSError::ENOSYS,
            39 => OSError::ENOTEMPTY,
            40 => OSError::ELOOP,
            42 => OSError::ENOMSG,
            43 => OSError::EIDRM,
            44 => OSError::ECHRNG,
            45 => OSError::EL2NSYNC,
            46 => OSError::EL3HLT,
            47 => OSError::EL3RST,
            48 => OSError::ELNRNG,
            49 => OSError::EUNATCH,
            50 => OSError::ENOCSI,
            51 => OSError::EL2HLT,
            52 => OSError::EBADE,
            53 => OSError::EBADR,
            54 => OSError::EXFULL,
            55 => OSError::ENOANO,
            56 => OSError::EBADRQC,
            57 => OSError::EBADSLT,
            59 => OSError::EBFONT,
            60 => OSError::ENOSTR,
            61 => OSError::ENODATA,
            62 => OSError::ETIME,
            63 => OSError::ENOSR,
            64 => OSError::ENONET,
            65 => OSError::ENOPKG,
            66 => OSError::EREMOTE,
            67 => OSError::ENOLINK,
            68 => OSError::EADV,
            69 => OSError::ESRMNT,
            70 => OSError::ECOMM,
            71 => OSError::EPROTO,
            72 => OSError::EMULTIHOP,
            73 => OSError::EDOTDOT,
            74 => OSError::EBADMSG,
            75 => OSError::EOVERFLOW,
            76 => OSError::ENOTUNIQ,
            77 => OSError::EBADFD,
            78 => OSError::EREMCHG,
            79 => OSError::ELIBACC,
            80 => OSError::ELIBBAD,
            81 => OSError::ELIBSCN,
            82 => OSError::ELIBMAX,
            83 => OSError::ELIBEXEC,
            84 => OSError::EILSEQ,
            85 => OSError::ERESTART,
            86 => OSError::ESTRPIPE,
            87 => OSError::EUSERS,
            88 => OSError::ENOTSOCK,
            89 => OSError::EDESTADDRREQ,
            90 => OSError::EMSGSIZE,
            91 => OSError::EPROTOTYPE,
            92 => OSError::ENOPROTOOPT,
            93 => OSError::EPROTONOSUPPORT,
            94 => OSError::ESOCKTNOSUPPORT,
            95 => OSError::EOPNOTSUPP,
            96 => OSError::EPFNOSUPPORT,
            97 => OSError::EAFNOSUPPORT,
            98 => OSError::EADDRINUSE,
            99 => OSError::EADDRNOTAVAIL,
            100 => OSError::ENETDOWN,
            101 => OSError::ENETUNREACH,
            102 => OSError::ENETRESET,
            103 => OSError::ECONNABORTED,
            104 => OSError::ECONNRESET,
            105 => OSError::ENOBUFS,
            106 => OSError::EISCONN,
            107 => OSError::ENOTCONN,
            108 => OSError::ESHUTDOWN,
            109 => OSError::ETOOMANYREFS,
            110 => OSError::ETIMEDOUT,
            111 => OSError::ECONNREFUSED,
            112 => OSError::EHOSTDOWN,
            113 => OSError::EHOSTUNREACH,
            114 => OSError::EALREADY,
            115 => OSError::EINPROGRESS,
            116 => OSError::ESTALE,
            117 => OSError::EUCLEAN,
            118 => OSError::ENOTNAM,
            119 => OSError::ENAVAIL,
            120 => OSError::EISNAM,
            121 => OSError::EREMOTEIO,
            122 => OSError::EDQUOT,
            123 => OSError::ENOMEDIUM,
            124 => OSError::EMEDIUMTYPE,
            125 => OSError::ECANCELED,
            126 => OSError::ENOKEY,
            127 => OSError::EKEYEXPIRED,
            128 => OSError::EKEYREVOKED,
            129 => OSError::EKEYREJECTED,
            130 => OSError::EOWNERDEAD,
            131 => OSError::ENOTRECOVERABLE,
            _ => panic!("Invalid OSError code: {}! (Linux)", code)
        }
    }
}