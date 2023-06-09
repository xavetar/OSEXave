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
pub enum DISPATCH {
    DISP_E_UNKNOWNINTERFACE,
    DISP_E_MEMBERNOTFOUND,
    DISP_E_PARAMNOTFOUND,
    DISP_E_TYPEMISMATCH,
    DISP_E_UNKNOWNNAME,
    DISP_E_NONAMEDARGS,
    DISP_E_BADVARTYPE,
    DISP_E_EXCEPTION,
    DISP_E_OVERFLOW,
    DISP_E_BADINDEX,
    DISP_E_UNKNOWNLCID,
    DISP_E_ARRAYISLOCKED,
    DISP_E_BADPARAMCOUNT,
    DISP_E_PARAMNOTOPTIONAL,
    DISP_E_BADCALLEE,
    DISP_E_NOTACOLLECTION,
    DISP_E_DIVBYZERO,
    DISP_E_BUFFERTOOSMALL,
    TYPE_E_BUFFERTOOSMALL,
    TYPE_E_FIELDNOTFOUND,
    TYPE_E_INVDATAREAD,
    TYPE_E_UNSUPFORMAT,
    TYPE_E_REGISTRYACCESS,
    TYPE_E_LIBNOTREGISTERED,
    TYPE_E_UNDEFINEDTYPE,
    TYPE_E_QUALIFIEDNAMEDISALLOWED,
    TYPE_E_INVALIDSTATE,
    TYPE_E_WRONGTYPEKIND,
    TYPE_E_ELEMENTNOTFOUND,
    TYPE_E_AMBIGUOUSNAME,
    TYPE_E_NAMECONFLICT,
    TYPE_E_UNKNOWNLCID,
    TYPE_E_DLLFUNCTIONNOTFOUND,
    TYPE_E_BADMODULEKIND,
    TYPE_E_SIZETOOBIG,
    TYPE_E_DUPLICATEID,
    TYPE_E_INVALIDID,
    TYPE_E_TYPEMISMATCH,
    TYPE_E_OUTOFBOUNDS,
    TYPE_E_IOERROR,
    TYPE_E_CANTCREATETMPFILE,
    TYPE_E_CANTLOADLIBRARY,
    TYPE_E_INCONSISTENTPROPFUNCS,
    TYPE_E_CIRCULARTYPE,
}

impl DISPATCH {
    pub fn code(&self) -> u32 {
        return match self {
            DISPATCH::DISP_E_UNKNOWNINTERFACE => 0x80020001 as u32,
            DISPATCH::DISP_E_MEMBERNOTFOUND => 0x80020003 as u32,
            DISPATCH::DISP_E_PARAMNOTFOUND => 0x80020004 as u32,
            DISPATCH::DISP_E_TYPEMISMATCH => 0x80020005 as u32,
            DISPATCH::DISP_E_UNKNOWNNAME => 0x80020006 as u32,
            DISPATCH::DISP_E_NONAMEDARGS => 0x80020007 as u32,
            DISPATCH::DISP_E_BADVARTYPE => 0x80020008 as u32,
            DISPATCH::DISP_E_EXCEPTION => 0x80020009 as u32,
            DISPATCH::DISP_E_OVERFLOW => 0x8002000A as u32,
            DISPATCH::DISP_E_BADINDEX => 0x8002000B as u32,
            DISPATCH::DISP_E_UNKNOWNLCID => 0x8002000C as u32,
            DISPATCH::DISP_E_ARRAYISLOCKED => 0x8002000D as u32,
            DISPATCH::DISP_E_BADPARAMCOUNT => 0x8002000E as u32,
            DISPATCH::DISP_E_PARAMNOTOPTIONAL => 0x8002000F as u32,
            DISPATCH::DISP_E_BADCALLEE => 0x80020010 as u32,
            DISPATCH::DISP_E_NOTACOLLECTION => 0x80020011 as u32,
            DISPATCH::DISP_E_DIVBYZERO => 0x80020012 as u32,
            DISPATCH::DISP_E_BUFFERTOOSMALL => 0x80020013 as u32,
            DISPATCH::TYPE_E_BUFFERTOOSMALL => 0x80028016 as u32,
            DISPATCH::TYPE_E_FIELDNOTFOUND => 0x80028017 as u32,
            DISPATCH::TYPE_E_INVDATAREAD => 0x80028018 as u32,
            DISPATCH::TYPE_E_UNSUPFORMAT => 0x80028019 as u32,
            DISPATCH::TYPE_E_REGISTRYACCESS => 0x8002801C as u32,
            DISPATCH::TYPE_E_LIBNOTREGISTERED => 0x8002801D as u32,
            DISPATCH::TYPE_E_UNDEFINEDTYPE => 0x80028027 as u32,
            DISPATCH::TYPE_E_QUALIFIEDNAMEDISALLOWED => 0x80028028 as u32,
            DISPATCH::TYPE_E_INVALIDSTATE => 0x80028029 as u32,
            DISPATCH::TYPE_E_WRONGTYPEKIND => 0x8002802A as u32,
            DISPATCH::TYPE_E_ELEMENTNOTFOUND => 0x8002802B as u32,
            DISPATCH::TYPE_E_AMBIGUOUSNAME => 0x8002802C as u32,
            DISPATCH::TYPE_E_NAMECONFLICT => 0x8002802D as u32,
            DISPATCH::TYPE_E_UNKNOWNLCID => 0x8002802E as u32,
            DISPATCH::TYPE_E_DLLFUNCTIONNOTFOUND => 0x8002802F as u32,
            DISPATCH::TYPE_E_BADMODULEKIND => 0x800288BD as u32,
            DISPATCH::TYPE_E_SIZETOOBIG => 0x800288C5 as u32,
            DISPATCH::TYPE_E_DUPLICATEID => 0x800288C6 as u32,
            DISPATCH::TYPE_E_INVALIDID => 0x800288CF as u32,
            DISPATCH::TYPE_E_TYPEMISMATCH => 0x80028CA0 as u32,
            DISPATCH::TYPE_E_OUTOFBOUNDS => 0x80028CA1 as u32,
            DISPATCH::TYPE_E_IOERROR => 0x80028CA2 as u32,
            DISPATCH::TYPE_E_CANTCREATETMPFILE => 0x80028CA3 as u32,
            DISPATCH::TYPE_E_CANTLOADLIBRARY => 0x80029C4A as u32,
            DISPATCH::TYPE_E_INCONSISTENTPROPFUNCS => 0x80029C83 as u32,
            DISPATCH::TYPE_E_CIRCULARTYPE => 0x80029C84 as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            DISPATCH::DISP_E_UNKNOWNINTERFACE => RawError::Kind(DISPATCH::DISP_E_UNKNOWNINTERFACE),
            DISPATCH::DISP_E_MEMBERNOTFOUND => RawError::Kind(DISPATCH::DISP_E_MEMBERNOTFOUND),
            DISPATCH::DISP_E_PARAMNOTFOUND => RawError::Kind(DISPATCH::DISP_E_PARAMNOTFOUND),
            DISPATCH::DISP_E_TYPEMISMATCH => RawError::Kind(DISPATCH::DISP_E_TYPEMISMATCH),
            DISPATCH::DISP_E_UNKNOWNNAME => RawError::Kind(DISPATCH::DISP_E_UNKNOWNNAME),
            DISPATCH::DISP_E_NONAMEDARGS => RawError::Kind(DISPATCH::DISP_E_NONAMEDARGS),
            DISPATCH::DISP_E_BADVARTYPE => RawError::Kind(DISPATCH::DISP_E_BADVARTYPE),
            DISPATCH::DISP_E_EXCEPTION => RawError::Kind(DISPATCH::DISP_E_EXCEPTION),
            DISPATCH::DISP_E_OVERFLOW => RawError::Kind(DISPATCH::DISP_E_OVERFLOW),
            DISPATCH::DISP_E_BADINDEX => RawError::Kind(DISPATCH::DISP_E_BADINDEX),
            DISPATCH::DISP_E_UNKNOWNLCID => RawError::Kind(DISPATCH::DISP_E_UNKNOWNLCID),
            DISPATCH::DISP_E_ARRAYISLOCKED => RawError::Kind(DISPATCH::DISP_E_ARRAYISLOCKED),
            DISPATCH::DISP_E_BADPARAMCOUNT => RawError::Kind(DISPATCH::DISP_E_BADPARAMCOUNT),
            DISPATCH::DISP_E_PARAMNOTOPTIONAL => RawError::Kind(DISPATCH::DISP_E_PARAMNOTOPTIONAL),
            DISPATCH::DISP_E_BADCALLEE => RawError::Kind(DISPATCH::DISP_E_BADCALLEE),
            DISPATCH::DISP_E_NOTACOLLECTION => RawError::Kind(DISPATCH::DISP_E_NOTACOLLECTION),
            DISPATCH::DISP_E_DIVBYZERO => RawError::Kind(DISPATCH::DISP_E_DIVBYZERO),
            DISPATCH::DISP_E_BUFFERTOOSMALL => RawError::Kind(DISPATCH::DISP_E_BUFFERTOOSMALL),
            DISPATCH::TYPE_E_BUFFERTOOSMALL => RawError::Kind(DISPATCH::TYPE_E_BUFFERTOOSMALL),
            DISPATCH::TYPE_E_FIELDNOTFOUND => RawError::Kind(DISPATCH::TYPE_E_FIELDNOTFOUND),
            DISPATCH::TYPE_E_INVDATAREAD => RawError::Kind(DISPATCH::TYPE_E_INVDATAREAD),
            DISPATCH::TYPE_E_UNSUPFORMAT => RawError::Kind(DISPATCH::TYPE_E_UNSUPFORMAT),
            DISPATCH::TYPE_E_REGISTRYACCESS => RawError::Kind(DISPATCH::TYPE_E_REGISTRYACCESS),
            DISPATCH::TYPE_E_LIBNOTREGISTERED => RawError::Kind(DISPATCH::TYPE_E_LIBNOTREGISTERED),
            DISPATCH::TYPE_E_UNDEFINEDTYPE => RawError::Kind(DISPATCH::TYPE_E_UNDEFINEDTYPE),
            DISPATCH::TYPE_E_QUALIFIEDNAMEDISALLOWED => RawError::Kind(DISPATCH::TYPE_E_QUALIFIEDNAMEDISALLOWED),
            DISPATCH::TYPE_E_INVALIDSTATE => RawError::Kind(DISPATCH::TYPE_E_INVALIDSTATE),
            DISPATCH::TYPE_E_WRONGTYPEKIND => RawError::Kind(DISPATCH::TYPE_E_WRONGTYPEKIND),
            DISPATCH::TYPE_E_ELEMENTNOTFOUND => RawError::Kind(DISPATCH::TYPE_E_ELEMENTNOTFOUND),
            DISPATCH::TYPE_E_AMBIGUOUSNAME => RawError::Kind(DISPATCH::TYPE_E_AMBIGUOUSNAME),
            DISPATCH::TYPE_E_NAMECONFLICT => RawError::Kind(DISPATCH::TYPE_E_NAMECONFLICT),
            DISPATCH::TYPE_E_UNKNOWNLCID => RawError::Kind(DISPATCH::TYPE_E_UNKNOWNLCID),
            DISPATCH::TYPE_E_DLLFUNCTIONNOTFOUND => RawError::Kind(DISPATCH::TYPE_E_DLLFUNCTIONNOTFOUND),
            DISPATCH::TYPE_E_BADMODULEKIND => RawError::Kind(DISPATCH::TYPE_E_BADMODULEKIND),
            DISPATCH::TYPE_E_SIZETOOBIG => RawError::Kind(DISPATCH::TYPE_E_SIZETOOBIG),
            DISPATCH::TYPE_E_DUPLICATEID => RawError::Kind(DISPATCH::TYPE_E_DUPLICATEID),
            DISPATCH::TYPE_E_INVALIDID => RawError::Kind(DISPATCH::TYPE_E_INVALIDID),
            DISPATCH::TYPE_E_TYPEMISMATCH => RawError::Kind(DISPATCH::TYPE_E_TYPEMISMATCH),
            DISPATCH::TYPE_E_OUTOFBOUNDS => RawError::Kind(DISPATCH::TYPE_E_OUTOFBOUNDS),
            DISPATCH::TYPE_E_IOERROR => RawError::Kind(DISPATCH::TYPE_E_IOERROR),
            DISPATCH::TYPE_E_CANTCREATETMPFILE => RawError::Kind(DISPATCH::TYPE_E_CANTCREATETMPFILE),
            DISPATCH::TYPE_E_CANTLOADLIBRARY => RawError::Kind(DISPATCH::TYPE_E_CANTLOADLIBRARY),
            DISPATCH::TYPE_E_INCONSISTENTPROPFUNCS => RawError::Kind(DISPATCH::TYPE_E_INCONSISTENTPROPFUNCS),
            DISPATCH::TYPE_E_CIRCULARTYPE => RawError::Kind(DISPATCH::TYPE_E_CIRCULARTYPE),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            DISPATCH::DISP_E_UNKNOWNINTERFACE => "Unknown interface.",
            DISPATCH::DISP_E_MEMBERNOTFOUND => "Member not found.",
            DISPATCH::DISP_E_PARAMNOTFOUND => "Parameter not found.",
            DISPATCH::DISP_E_TYPEMISMATCH => "Type mismatch.",
            DISPATCH::DISP_E_UNKNOWNNAME => "Unknown name.",
            DISPATCH::DISP_E_NONAMEDARGS => "No named arguments.",
            DISPATCH::DISP_E_BADVARTYPE => "Bad variable type.",
            DISPATCH::DISP_E_EXCEPTION => "Exception occurred.",
            DISPATCH::DISP_E_OVERFLOW => "Out of present range.",
            DISPATCH::DISP_E_BADINDEX => "Invalid index.",
            DISPATCH::DISP_E_UNKNOWNLCID => "Unknown language.",
            DISPATCH::DISP_E_ARRAYISLOCKED => "Memory is locked.",
            DISPATCH::DISP_E_BADPARAMCOUNT => "Invalid number of parameters.",
            DISPATCH::DISP_E_PARAMNOTOPTIONAL => "Parameter not optional.",
            DISPATCH::DISP_E_BADCALLEE => "Invalid callee.",
            DISPATCH::DISP_E_NOTACOLLECTION => "Does not support a collection.",
            DISPATCH::DISP_E_DIVBYZERO => "Division by zero.",
            DISPATCH::DISP_E_BUFFERTOOSMALL => "Buffer too small",
            DISPATCH::TYPE_E_BUFFERTOOSMALL => "Buffer too small.",
            DISPATCH::TYPE_E_FIELDNOTFOUND => "Field name not defined in the record.",
            DISPATCH::TYPE_E_INVDATAREAD => "Old format or invalid type library.",
            DISPATCH::TYPE_E_UNSUPFORMAT => "Old format or invalid type library.",
            DISPATCH::TYPE_E_REGISTRYACCESS => "Error accessing the OLE registry.",
            DISPATCH::TYPE_E_LIBNOTREGISTERED => "Library not registered.",
            DISPATCH::TYPE_E_UNDEFINEDTYPE => "Bound to unknown type.",
            DISPATCH::TYPE_E_QUALIFIEDNAMEDISALLOWED => "Qualified name disallowed.",
            DISPATCH::TYPE_E_INVALIDSTATE => "Invalid forward reference, or reference to uncompiled type.",
            DISPATCH::TYPE_E_WRONGTYPEKIND => "Type mismatch.",
            DISPATCH::TYPE_E_ELEMENTNOTFOUND => "Element not found.",
            DISPATCH::TYPE_E_AMBIGUOUSNAME => "Ambiguous name.",
            DISPATCH::TYPE_E_NAMECONFLICT => "Name already exists in the library.",
            DISPATCH::TYPE_E_UNKNOWNLCID => "Unknown LCID.",
            DISPATCH::TYPE_E_DLLFUNCTIONNOTFOUND => "Function not defined in specified DLL.",
            DISPATCH::TYPE_E_BADMODULEKIND => "Wrong module kind for the operation.",
            DISPATCH::TYPE_E_SIZETOOBIG => "Size may not exceed 64K.",
            DISPATCH::TYPE_E_DUPLICATEID => "Duplicate ID in inheritance hierarchy.",
            DISPATCH::TYPE_E_INVALIDID => "Incorrect inheritance depth in standard OLE hmember.",
            DISPATCH::TYPE_E_TYPEMISMATCH => "Type mismatch.",
            DISPATCH::TYPE_E_OUTOFBOUNDS => "Invalid number of arguments.",
            DISPATCH::TYPE_E_IOERROR => "I/O Error.",
            DISPATCH::TYPE_E_CANTCREATETMPFILE => "Error creating unique tmp file.",
            DISPATCH::TYPE_E_CANTLOADLIBRARY => "Error loading type library/DLL.",
            DISPATCH::TYPE_E_INCONSISTENTPROPFUNCS => "Inconsistent property functions.",
            DISPATCH::TYPE_E_CIRCULARTYPE => "Circular dependency between types/modules.",
        }
    }

    pub fn from_name(name: &str) -> DISPATCH {
        return match name {
            "DISP_E_UNKNOWNINTERFACE" => DISPATCH::DISP_E_UNKNOWNINTERFACE,
            "DISP_E_MEMBERNOTFOUND" => DISPATCH::DISP_E_MEMBERNOTFOUND,
            "DISP_E_PARAMNOTFOUND" => DISPATCH::DISP_E_PARAMNOTFOUND,
            "DISP_E_TYPEMISMATCH" => DISPATCH::DISP_E_TYPEMISMATCH,
            "DISP_E_UNKNOWNNAME" => DISPATCH::DISP_E_UNKNOWNNAME,
            "DISP_E_NONAMEDARGS" => DISPATCH::DISP_E_NONAMEDARGS,
            "DISP_E_BADVARTYPE" => DISPATCH::DISP_E_BADVARTYPE,
            "DISP_E_EXCEPTION" => DISPATCH::DISP_E_EXCEPTION,
            "DISP_E_OVERFLOW" => DISPATCH::DISP_E_OVERFLOW,
            "DISP_E_BADINDEX" => DISPATCH::DISP_E_BADINDEX,
            "DISP_E_UNKNOWNLCID" => DISPATCH::DISP_E_UNKNOWNLCID,
            "DISP_E_ARRAYISLOCKED" => DISPATCH::DISP_E_ARRAYISLOCKED,
            "DISP_E_BADPARAMCOUNT" => DISPATCH::DISP_E_BADPARAMCOUNT,
            "DISP_E_PARAMNOTOPTIONAL" => DISPATCH::DISP_E_PARAMNOTOPTIONAL,
            "DISP_E_BADCALLEE" => DISPATCH::DISP_E_BADCALLEE,
            "DISP_E_NOTACOLLECTION" => DISPATCH::DISP_E_NOTACOLLECTION,
            "DISP_E_DIVBYZERO" => DISPATCH::DISP_E_DIVBYZERO,
            "DISP_E_BUFFERTOOSMALL" => DISPATCH::DISP_E_BUFFERTOOSMALL,
            "TYPE_E_BUFFERTOOSMALL" => DISPATCH::TYPE_E_BUFFERTOOSMALL,
            "TYPE_E_FIELDNOTFOUND" => DISPATCH::TYPE_E_FIELDNOTFOUND,
            "TYPE_E_INVDATAREAD" => DISPATCH::TYPE_E_INVDATAREAD,
            "TYPE_E_UNSUPFORMAT" => DISPATCH::TYPE_E_UNSUPFORMAT,
            "TYPE_E_REGISTRYACCESS" => DISPATCH::TYPE_E_REGISTRYACCESS,
            "TYPE_E_LIBNOTREGISTERED" => DISPATCH::TYPE_E_LIBNOTREGISTERED,
            "TYPE_E_UNDEFINEDTYPE" => DISPATCH::TYPE_E_UNDEFINEDTYPE,
            "TYPE_E_QUALIFIEDNAMEDISALLOWED" => DISPATCH::TYPE_E_QUALIFIEDNAMEDISALLOWED,
            "TYPE_E_INVALIDSTATE" => DISPATCH::TYPE_E_INVALIDSTATE,
            "TYPE_E_WRONGTYPEKIND" => DISPATCH::TYPE_E_WRONGTYPEKIND,
            "TYPE_E_ELEMENTNOTFOUND" => DISPATCH::TYPE_E_ELEMENTNOTFOUND,
            "TYPE_E_AMBIGUOUSNAME" => DISPATCH::TYPE_E_AMBIGUOUSNAME,
            "TYPE_E_NAMECONFLICT" => DISPATCH::TYPE_E_NAMECONFLICT,
            "TYPE_E_UNKNOWNLCID" => DISPATCH::TYPE_E_UNKNOWNLCID,
            "TYPE_E_DLLFUNCTIONNOTFOUND" => DISPATCH::TYPE_E_DLLFUNCTIONNOTFOUND,
            "TYPE_E_BADMODULEKIND" => DISPATCH::TYPE_E_BADMODULEKIND,
            "TYPE_E_SIZETOOBIG" => DISPATCH::TYPE_E_SIZETOOBIG,
            "TYPE_E_DUPLICATEID" => DISPATCH::TYPE_E_DUPLICATEID,
            "TYPE_E_INVALIDID" => DISPATCH::TYPE_E_INVALIDID,
            "TYPE_E_TYPEMISMATCH" => DISPATCH::TYPE_E_TYPEMISMATCH,
            "TYPE_E_OUTOFBOUNDS" => DISPATCH::TYPE_E_OUTOFBOUNDS,
            "TYPE_E_IOERROR" => DISPATCH::TYPE_E_IOERROR,
            "TYPE_E_CANTCREATETMPFILE" => DISPATCH::TYPE_E_CANTCREATETMPFILE,
            "TYPE_E_CANTLOADLIBRARY" => DISPATCH::TYPE_E_CANTLOADLIBRARY,
            "TYPE_E_INCONSISTENTPROPFUNCS" => DISPATCH::TYPE_E_INCONSISTENTPROPFUNCS,
            "TYPE_E_CIRCULARTYPE" => DISPATCH::TYPE_E_CIRCULARTYPE,
        }
    }
    pub fn from_code(code: u32) -> DISPATCH {
        return match code {
            0x80020001 => DISPATCH::DISP_E_UNKNOWNINTERFACE,
            0x80020003 => DISPATCH::DISP_E_MEMBERNOTFOUND,
            0x80020004 => DISPATCH::DISP_E_PARAMNOTFOUND,
            0x80020005 => DISPATCH::DISP_E_TYPEMISMATCH,
            0x80020006 => DISPATCH::DISP_E_UNKNOWNNAME,
            0x80020007 => DISPATCH::DISP_E_NONAMEDARGS,
            0x80020008 => DISPATCH::DISP_E_BADVARTYPE,
            0x80020009 => DISPATCH::DISP_E_EXCEPTION,
            0x8002000A => DISPATCH::DISP_E_OVERFLOW,
            0x8002000B => DISPATCH::DISP_E_BADINDEX,
            0x8002000C => DISPATCH::DISP_E_UNKNOWNLCID,
            0x8002000D => DISPATCH::DISP_E_ARRAYISLOCKED,
            0x8002000E => DISPATCH::DISP_E_BADPARAMCOUNT,
            0x8002000F => DISPATCH::DISP_E_PARAMNOTOPTIONAL,
            0x80020010 => DISPATCH::DISP_E_BADCALLEE,
            0x80020011 => DISPATCH::DISP_E_NOTACOLLECTION,
            0x80020012 => DISPATCH::DISP_E_DIVBYZERO,
            0x80020013 => DISPATCH::DISP_E_BUFFERTOOSMALL,
            0x80028016 => DISPATCH::TYPE_E_BUFFERTOOSMALL,
            0x80028017 => DISPATCH::TYPE_E_FIELDNOTFOUND,
            0x80028018 => DISPATCH::TYPE_E_INVDATAREAD,
            0x80028019 => DISPATCH::TYPE_E_UNSUPFORMAT,
            0x8002801C => DISPATCH::TYPE_E_REGISTRYACCESS,
            0x8002801D => DISPATCH::TYPE_E_LIBNOTREGISTERED,
            0x80028027 => DISPATCH::TYPE_E_UNDEFINEDTYPE,
            0x80028028 => DISPATCH::TYPE_E_QUALIFIEDNAMEDISALLOWED,
            0x80028029 => DISPATCH::TYPE_E_INVALIDSTATE,
            0x8002802A => DISPATCH::TYPE_E_WRONGTYPEKIND,
            0x8002802B => DISPATCH::TYPE_E_ELEMENTNOTFOUND,
            0x8002802C => DISPATCH::TYPE_E_AMBIGUOUSNAME,
            0x8002802D => DISPATCH::TYPE_E_NAMECONFLICT,
            0x8002802E => DISPATCH::TYPE_E_UNKNOWNLCID,
            0x8002802F => DISPATCH::TYPE_E_DLLFUNCTIONNOTFOUND,
            0x800288BD => DISPATCH::TYPE_E_BADMODULEKIND,
            0x800288C5 => DISPATCH::TYPE_E_SIZETOOBIG,
            0x800288C6 => DISPATCH::TYPE_E_DUPLICATEID,
            0x800288CF => DISPATCH::TYPE_E_INVALIDID,
            0x80028CA0 => DISPATCH::TYPE_E_TYPEMISMATCH,
            0x80028CA1 => DISPATCH::TYPE_E_OUTOFBOUNDS,
            0x80028CA2 => DISPATCH::TYPE_E_IOERROR,
            0x80028CA3 => DISPATCH::TYPE_E_CANTCREATETMPFILE,
            0x80029C4A => DISPATCH::TYPE_E_CANTLOADLIBRARY,
            0x80029C83 => DISPATCH::TYPE_E_INCONSISTENTPROPFUNCS,
            0x80029C84 => DISPATCH::TYPE_E_CIRCULARTYPE,
        }
    }
}
