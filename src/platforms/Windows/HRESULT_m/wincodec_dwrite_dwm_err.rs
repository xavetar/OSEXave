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

pub enum WINCODEC_DWRITE_DWM_ERR {
    MILERR_OBJECTBUSY = 0x88980001,
    MILERR_INSUFFICIENTBUFFER = 0x88980002,
    MILERR_WIN32ERROR = 0x88980003,
    MILERR_SCANNER_FAILED = 0x88980004,
    MILERR_SCREENACCESSDENIED = 0x88980005,
    MILERR_DISPLAYSTATEINVALID = 0x88980006,
    MILERR_NONINVERTIBLEMATRIX = 0x88980007,
    MILERR_ZEROVECTOR = 0x88980008,
    MILERR_TERMINATED = 0x88980009,
    MILERR_BADNUMBER = 0x8898000A,
    MILERR_INTERNALERROR = 0x88980080,
    MILERR_DISPLAYFORMATNOTSUPPORTED = 0x88980084,
    MILERR_INVALIDCALL = 0x88980085,
    MILERR_ALREADYLOCKED = 0x88980086,
    MILERR_NOTLOCKED = 0x88980087,
    MILERR_DEVICECANNOTRENDERTEXT = 0x88980088,
    MILERR_GLYPHBITMAPMISSED = 0x88980089,
    MILERR_MALFORMEDGLYPHCACHE = 0x8898008A,
    MILERR_GENERIC_IGNORE = 0x8898008B,
    MILERR_MALFORMED_GUIDELINE_DATA = 0x8898008C,
    MILERR_NO_HARDWARE_DEVICE = 0x8898008D,
    MILERR_NEED_RECREATE_AND_PRESENT = 0x8898008E,
    MILERR_ALREADY_INITIALIZED = 0x8898008F,
    MILERR_MISMATCHED_SIZE = 0x88980090,
    MILERR_NO_REDIRECTION_SURFACE_AVAILABLE = 0x88980091,
    MILERR_REMOTING_NOT_SUPPORTED = 0x88980092,
    MILERR_QUEUED_PRESENT_NOT_SUPPORTED = 0x88980093,
    MILERR_NOT_QUEUING_PRESENTS = 0x88980094,
    MILERR_NO_REDIRECTION_SURFACE_RETRY_LATER = 0x88980095,
    MILERR_TOOMANYSHADERELEMNTS = 0x88980096,
    MILERR_MROW_READLOCK_FAILED = 0x88980097,
    MILERR_MROW_UPDATE_FAILED = 0x88980098,
    MILERR_SHADER_COMPILE_FAILED = 0x88980099,
    MILERR_MAX_TEXTURE_SIZE_EXCEEDED = 0x8898009A,
    MILERR_QPC_TIME_WENT_BACKWARD = 0x8898009B,
    MILERR_DXGI_ENUMERATION_OUT_OF_SYNC = 0x8898009D,
    MILERR_ADAPTER_NOT_FOUND = 0x8898009E,
    MILERR_COLORSPACE_NOT_SUPPORTED = 0x8898009F,
    MILERR_PREFILTER_NOT_SUPPORTED = 0x889800A0,
    MILERR_DISPLAYID_ACCESS_DENIED = 0x889800A1,
    UCEERR_INVALIDPACKETHEADER = 0x88980400,
    UCEERR_UNKNOWNPACKET = 0x88980401,
    UCEERR_ILLEGALPACKET = 0x88980402,
    UCEERR_MALFORMEDPACKET = 0x88980403,
    UCEERR_ILLEGALHANDLE = 0x88980404,
    UCEERR_HANDLELOOKUPFAILED = 0x88980405,
    UCEERR_RENDERTHREADFAILURE = 0x88980406,
    UCEERR_CTXSTACKFRSTTARGETNULL = 0x88980407,
    UCEERR_CONNECTIONIDLOOKUPFAILED = 0x88980408,
    UCEERR_BLOCKSFULL = 0x88980409,
    UCEERR_MEMORYFAILURE = 0x8898040A,
    UCEERR_PACKETRECORDOUTOFRANGE = 0x8898040B,
    UCEERR_ILLEGALRECORDTYPE = 0x8898040C,
    UCEERR_OUTOFHANDLES = 0x8898040D,
    UCEERR_UNCHANGABLE_UPDATE_ATTEMPTED = 0x8898040E,
    UCEERR_NO_MULTIPLE_WORKER_THREADS = 0x8898040F,
    UCEERR_REMOTINGNOTSUPPORTED = 0x88980410,
    UCEERR_MISSINGENDCOMMAND = 0x88980411,
    UCEERR_MISSINGBEGINCOMMAND = 0x88980412,
    UCEERR_CHANNELSYNCTIMEDOUT = 0x88980413,
    UCEERR_CHANNELSYNCABANDONED = 0x88980414,
    UCEERR_UNSUPPORTEDTRANSPORTVERSION = 0x88980415,
    UCEERR_TRANSPORTUNAVAILABLE = 0x88980416,
    UCEERR_FEEDBACK_UNSUPPORTED = 0x88980417,
    UCEERR_COMMANDTRANSPORTDENIED = 0x88980418,
    UCEERR_GRAPHICSSTREAMUNAVAILABLE = 0x88980419,
    UCEERR_GRAPHICSSTREAMALREADYOPEN = 0x88980420,
    UCEERR_TRANSPORTDISCONNECTED = 0x88980421,
    UCEERR_TRANSPORTOVERLOADED = 0x88980422,
    UCEERR_PARTITION_ZOMBIED = 0x88980423,
    MILAVERR_NOCLOCK = 0x88980500,
    MILAVERR_NOMEDIATYPE = 0x88980501,
    MILAVERR_NOVIDEOMIXER = 0x88980502,
    MILAVERR_NOVIDEOPRESENTER = 0x88980503,
    MILAVERR_NOREADYFRAMES = 0x88980504,
    MILAVERR_MODULENOTLOADED = 0x88980505,
    MILAVERR_WMPFACTORYNOTREGISTERED = 0x88980506,
    MILAVERR_INVALIDWMPVERSION = 0x88980507,
    MILAVERR_INSUFFICIENTVIDEORESOURCES = 0x88980508,
    MILAVERR_VIDEOACCELERATIONNOTAVAILABLE = 0x88980509,
    MILAVERR_REQUESTEDTEXTURETOOBIG = 0x8898050A,
    MILAVERR_SEEKFAILED = 0x8898050B,
    MILAVERR_UNEXPECTEDWMPFAILURE = 0x8898050C,
    MILAVERR_MEDIAPLAYERCLOSED = 0x8898050D,
    MILAVERR_UNKNOWNHARDWAREERROR = 0x8898050E,
    MILEFFECTSERR_UNKNOWNPROPERTY = 0x8898060E,
    MILEFFECTSERR_EFFECTNOTPARTOFGROUP = 0x8898060F,
    MILEFFECTSERR_NOINPUTSOURCEATTACHED = 0x88980610,
    MILEFFECTSERR_CONNECTORNOTCONNECTED = 0x88980611,
    MILEFFECTSERR_CONNECTORNOTASSOCIATEDWITHEFFECT = 0x88980612,
    MILEFFECTSERR_RESERVED = 0x88980613,
    MILEFFECTSERR_CYCLEDETECTED = 0x88980614,
    MILEFFECTSERR_EFFECTINMORETHANONEGRAPH = 0x88980615,
    MILEFFECTSERR_EFFECTALREADYINAGRAPH = 0x88980616,
    MILEFFECTSERR_EFFECTHASNOCHILDREN = 0x88980617,
    MILEFFECTSERR_ALREADYATTACHEDTOLISTENER = 0x88980618,
    MILEFFECTSERR_NOTAFFINETRANSFORM = 0x88980619,
    MILEFFECTSERR_EMPTYBOUNDS = 0x8898061A,
    MILEFFECTSERR_OUTPUTSIZETOOLARGE = 0x8898061B,
    DWMERR_STATE_TRANSITION_FAILED = 0x88980700,
    DWMERR_THEME_FAILED = 0x88980701,
    DWMERR_CATASTROPHIC_FAILURE = 0x88980702,
    DCOMPOSITION_ERROR_WINDOW_ALREADY_COMPOSED = 0x88980800,
    DCOMPOSITION_ERROR_SURFACE_BEING_RENDERED = 0x88980801,
    DCOMPOSITION_ERROR_SURFACE_NOT_BEING_RENDERED = 0x88980802,
    WINCODEC_ERR_WRONGSTATE = 0x88982F04,
    WINCODEC_ERR_VALUEOUTOFRANGE = 0x88982F05,
    WINCODEC_ERR_UNKNOWNIMAGEFORMAT = 0x88982F07,
    WINCODEC_ERR_UNSUPPORTEDVERSION = 0x88982F0B,
    WINCODEC_ERR_NOTINITIALIZED = 0x88982F0C,
    WINCODEC_ERR_ALREADYLOCKED = 0x88982F0D,
    WINCODEC_ERR_PROPERTYNOTFOUND = 0x88982F40,
    WINCODEC_ERR_PROPERTYNOTSUPPORTED = 0x88982F41,
    WINCODEC_ERR_PROPERTYSIZE = 0x88982F42,
    WINCODEC_ERR_CODECPRESENT = 0x88982F43,
    WINCODEC_ERR_CODECNOTHUMBNAIL = 0x88982F44,
    WINCODEC_ERR_PALETTEUNAVAILABLE = 0x88982F45,
    WINCODEC_ERR_CODECTOOMANYSCANLINES = 0x88982F46,
    WINCODEC_ERR_INTERNALERROR = 0x88982F48,
    WINCODEC_ERR_SOURCERECTDOESNOTMATCHDIMENSIONS = 0x88982F49,
    WINCODEC_ERR_COMPONENTNOTFOUND = 0x88982F50,
    WINCODEC_ERR_IMAGESIZEOUTOFRANGE = 0x88982F51,
    WINCODEC_ERR_TOOMUCHMETADATA = 0x88982F52,
    WINCODEC_ERR_BADIMAGE = 0x88982F60,
    WINCODEC_ERR_BADHEADER = 0x88982F61,
    WINCODEC_ERR_FRAMEMISSING = 0x88982F62,
    WINCODEC_ERR_BADMETADATAHEADER = 0x88982F63,
    WINCODEC_ERR_BADSTREAMDATA = 0x88982F70,
    WINCODEC_ERR_STREAMWRITE = 0x88982F71,
    WINCODEC_ERR_STREAMREAD = 0x88982F72,
    WINCODEC_ERR_STREAMNOTAVAILABLE = 0x88982F73,
    WINCODEC_ERR_UNSUPPORTEDPIXELFORMAT = 0x88982F80,
    WINCODEC_ERR_UNSUPPORTEDOPERATION = 0x88982F81,
    WINCODEC_ERR_INVALIDREGISTRATION = 0x88982F8A,
    WINCODEC_ERR_COMPONENTINITIALIZEFAILURE = 0x88982F8B,
    WINCODEC_ERR_INSUFFICIENTBUFFER = 0x88982F8C,
    WINCODEC_ERR_DUPLICATEMETADATAPRESENT = 0x88982F8D,
    WINCODEC_ERR_PROPERTYUNEXPECTEDTYPE = 0x88982F8E,
    WINCODEC_ERR_UNEXPECTEDSIZE = 0x88982F8F,
    WINCODEC_ERR_INVALIDQUERYREQUEST = 0x88982F90,
    WINCODEC_ERR_UNEXPECTEDMETADATATYPE = 0x88982F91,
    WINCODEC_ERR_REQUESTONLYVALIDATMETADATAROOT = 0x88982F92,
    WINCODEC_ERR_INVALIDQUERYCHARACTER = 0x88982F93,
    WINCODEC_ERR_WIN32ERROR = 0x88982F94,
    WINCODEC_ERR_INVALIDPROGRESSIVELEVEL = 0x88982F95,
    WINCODEC_ERR_INVALIDJPEGSCANINDEX = 0x88982F96,
    DWRITE_E_FILEFORMAT = 0x88985000,
    DWRITE_E_UNEXPECTED = 0x88985001,
    DWRITE_E_NOFONT = 0x88985002,
    DWRITE_E_FILENOTFOUND = 0x88985003,
    DWRITE_E_FILEACCESS = 0x88985004,
    DWRITE_E_FONTCOLLECTIONOBSOLETE = 0x88985005,
    DWRITE_E_ALREADYREGISTERED = 0x88985006,
    DWRITE_E_CACHEFORMAT = 0x88985007,
    DWRITE_E_CACHEVERSION = 0x88985008,
    DWRITE_E_UNSUPPORTEDOPERATION = 0x88985009,
    DWRITE_E_TEXTRENDERERINCOMPATIBLE = 0x8898500A,
    DWRITE_E_FLOWDIRECTIONCONFLICTS = 0x8898500B,
    DWRITE_E_NOCOLOR = 0x8898500C,
    DWRITE_E_REMOTEFONT = 0x8898500D,
    DWRITE_E_DOWNLOADCANCELLED = 0x8898500E,
    DWRITE_E_DOWNLOADFAILED = 0x8898500F,
    DWRITE_E_TOOMANYDOWNLOADS = 0x88985010,
}

impl WINCODEC_DWRITE_DWM_ERR {
    pub fn description(&self) -> &'static str {
        match self {
            WINCODEC_DWRITE_DWM_ERR::MILERR_OBJECTBUSY => "MILERR_OBJECTBUSY",
            WINCODEC_DWRITE_DWM_ERR::MILERR_INSUFFICIENTBUFFER => "MILERR_INSUFFICIENTBUFFER",
            WINCODEC_DWRITE_DWM_ERR::MILERR_WIN32ERROR => "MILERR_WIN32ERROR",
            WINCODEC_DWRITE_DWM_ERR::MILERR_SCANNER_FAILED => "MILERR_SCANNER_FAILED",
            WINCODEC_DWRITE_DWM_ERR::MILERR_SCREENACCESSDENIED => "MILERR_SCREENACCESSDENIED",
            WINCODEC_DWRITE_DWM_ERR::MILERR_DISPLAYSTATEINVALID => "MILERR_DISPLAYSTATEINVALID",
            WINCODEC_DWRITE_DWM_ERR::MILERR_NONINVERTIBLEMATRIX => "MILERR_NONINVERTIBLEMATRIX",
            WINCODEC_DWRITE_DWM_ERR::MILERR_ZEROVECTOR => "MILERR_ZEROVECTOR",
            WINCODEC_DWRITE_DWM_ERR::MILERR_TERMINATED => "MILERR_TERMINATED",
            WINCODEC_DWRITE_DWM_ERR::MILERR_BADNUMBER => "MILERR_BADNUMBER",
            WINCODEC_DWRITE_DWM_ERR::MILERR_INTERNALERROR => "An internal error (MIL bug) occurred. On checked builds, an assert would be raised.",
            WINCODEC_DWRITE_DWM_ERR::MILERR_DISPLAYFORMATNOTSUPPORTED => "The display format we need to render is not supported by the hardware device.",
            WINCODEC_DWRITE_DWM_ERR::MILERR_INVALIDCALL => "A call to this method is invalid.",
            WINCODEC_DWRITE_DWM_ERR::MILERR_ALREADYLOCKED => "Lock attempted on an already locked object.",
            WINCODEC_DWRITE_DWM_ERR::MILERR_NOTLOCKED => "Unlock attempted on an unlocked object.",
            WINCODEC_DWRITE_DWM_ERR::MILERR_DEVICECANNOTRENDERTEXT => "No algorithm available to render text with this device",
            WINCODEC_DWRITE_DWM_ERR::MILERR_GLYPHBITMAPMISSED => "Some glyph bitmaps, required for glyph run rendering, are not contained in glyph cache.",
            WINCODEC_DWRITE_DWM_ERR::MILERR_MALFORMEDGLYPHCACHE => "Some glyph bitmaps in glyph cache are unexpectedly big.",
            WINCODEC_DWRITE_DWM_ERR::MILERR_GENERIC_IGNORE => "Marker error for known Win32 errors that are currently being ignored by the compositor. This is to avoid returning S_OK when an error has occurred, but still unwind the stack in the correct location.",
            WINCODEC_DWRITE_DWM_ERR::MILERR_MALFORMED_GUIDELINE_DATA => "Guideline coordinates are not sorted properly or contain NaNs.",
            WINCODEC_DWRITE_DWM_ERR::MILERR_NO_HARDWARE_DEVICE => "No HW rendering device is available for this operation.",
            WINCODEC_DWRITE_DWM_ERR::MILERR_NEED_RECREATE_AND_PRESENT => "There has been a presentation error that may be recoverable. The caller needs to recreate, rerender the entire frame, and reattempt present.",
            WINCODEC_DWRITE_DWM_ERR::MILERR_ALREADY_INITIALIZED => "The object has already been initialized.",
            WINCODEC_DWRITE_DWM_ERR::MILERR_MISMATCHED_SIZE => "The size of the object does not match the expected size.",
            WINCODEC_DWRITE_DWM_ERR::MILERR_NO_REDIRECTION_SURFACE_AVAILABLE => "No Redirection surface available.",
            WINCODEC_DWRITE_DWM_ERR::MILERR_REMOTING_NOT_SUPPORTED => "Remoting of this content is not supported.",
            WINCODEC_DWRITE_DWM_ERR::MILERR_QUEUED_PRESENT_NOT_SUPPORTED => "Queued Presents are not supported.",
            WINCODEC_DWRITE_DWM_ERR::MILERR_NOT_QUEUING_PRESENTS => "Queued Presents are not being used.",
            WINCODEC_DWRITE_DWM_ERR::MILERR_NO_REDIRECTION_SURFACE_RETRY_LATER => "No redirection surface was available. Caller should retry the call.",
            WINCODEC_DWRITE_DWM_ERR::MILERR_TOOMANYSHADERELEMNTS => "Shader construction failed because it was too complex.",
            WINCODEC_DWRITE_DWM_ERR::MILERR_MROW_READLOCK_FAILED => "MROW attempt to get a read lock failed.",
            WINCODEC_DWRITE_DWM_ERR::MILERR_MROW_UPDATE_FAILED => "MROW attempt to update the data failed because another update was outstanding.",
            WINCODEC_DWRITE_DWM_ERR::MILERR_SHADER_COMPILE_FAILED => "Shader compilation failed.",
            WINCODEC_DWRITE_DWM_ERR::MILERR_MAX_TEXTURE_SIZE_EXCEEDED => "Requested DX redirection surface size exceeded maximum texture size.",
            WINCODEC_DWRITE_DWM_ERR::MILERR_QPC_TIME_WENT_BACKWARD => "QueryPerformanceCounter returned a time in the past.",
            WINCODEC_DWRITE_DWM_ERR::MILERR_DXGI_ENUMERATION_OUT_OF_SYNC => "Primary Display device returned an invalid refresh rate.",
            WINCODEC_DWRITE_DWM_ERR::MILERR_ADAPTER_NOT_FOUND => "DWM can not find the adapter specified by the LUID.",
            WINCODEC_DWRITE_DWM_ERR::MILERR_COLORSPACE_NOT_SUPPORTED => "The requested bitmap color space is not supported.",
            WINCODEC_DWRITE_DWM_ERR::MILERR_PREFILTER_NOT_SUPPORTED => "The requested bitmap pre-filtering state is not supported.",
            WINCODEC_DWRITE_DWM_ERR::MILERR_DISPLAYID_ACCESS_DENIED => "Access is denied to the requested bitmap for the specified display id.",
            WINCODEC_DWRITE_DWM_ERR::UCEERR_INVALIDPACKETHEADER => "UCEERR_INVALIDPACKETHEADER",
            WINCODEC_DWRITE_DWM_ERR::UCEERR_UNKNOWNPACKET => "UCEERR_UNKNOWNPACKET",
            WINCODEC_DWRITE_DWM_ERR::UCEERR_ILLEGALPACKET => "UCEERR_ILLEGALPACKET",
            WINCODEC_DWRITE_DWM_ERR::UCEERR_MALFORMEDPACKET => "UCEERR_MALFORMEDPACKET",
            WINCODEC_DWRITE_DWM_ERR::UCEERR_ILLEGALHANDLE => "UCEERR_ILLEGALHANDLE",
            WINCODEC_DWRITE_DWM_ERR::UCEERR_HANDLELOOKUPFAILED => "UCEERR_HANDLELOOKUPFAILED",
            WINCODEC_DWRITE_DWM_ERR::UCEERR_RENDERTHREADFAILURE => "UCEERR_RENDERTHREADFAILURE",
            WINCODEC_DWRITE_DWM_ERR::UCEERR_CTXSTACKFRSTTARGETNULL => "UCEERR_CTXSTACKFRSTTARGETNULL",
            WINCODEC_DWRITE_DWM_ERR::UCEERR_CONNECTIONIDLOOKUPFAILED => "UCEERR_CONNECTIONIDLOOKUPFAILED",
            WINCODEC_DWRITE_DWM_ERR::UCEERR_BLOCKSFULL => "UCEERR_BLOCKSFULL",
            WINCODEC_DWRITE_DWM_ERR::UCEERR_MEMORYFAILURE => "UCEERR_MEMORYFAILURE",
            WINCODEC_DWRITE_DWM_ERR::UCEERR_PACKETRECORDOUTOFRANGE => "UCEERR_PACKETRECORDOUTOFRANGE",
            WINCODEC_DWRITE_DWM_ERR::UCEERR_ILLEGALRECORDTYPE => "UCEERR_ILLEGALRECORDTYPE",
            WINCODEC_DWRITE_DWM_ERR::UCEERR_OUTOFHANDLES => "UCEERR_OUTOFHANDLES",
            WINCODEC_DWRITE_DWM_ERR::UCEERR_UNCHANGABLE_UPDATE_ATTEMPTED => "UCEERR_UNCHANGABLE_UPDATE_ATTEMPTED",
            WINCODEC_DWRITE_DWM_ERR::UCEERR_NO_MULTIPLE_WORKER_THREADS => "UCEERR_NO_MULTIPLE_WORKER_THREADS",
            WINCODEC_DWRITE_DWM_ERR::UCEERR_REMOTINGNOTSUPPORTED => "UCEERR_REMOTINGNOTSUPPORTED",
            WINCODEC_DWRITE_DWM_ERR::UCEERR_MISSINGENDCOMMAND => "UCEERR_MISSINGENDCOMMAND",
            WINCODEC_DWRITE_DWM_ERR::UCEERR_MISSINGBEGINCOMMAND => "UCEERR_MISSINGBEGINCOMMAND",
            WINCODEC_DWRITE_DWM_ERR::UCEERR_CHANNELSYNCTIMEDOUT => "UCEERR_CHANNELSYNCTIMEDOUT",
            WINCODEC_DWRITE_DWM_ERR::UCEERR_CHANNELSYNCABANDONED => "UCEERR_CHANNELSYNCABANDONED",
            WINCODEC_DWRITE_DWM_ERR::UCEERR_UNSUPPORTEDTRANSPORTVERSION => "UCEERR_UNSUPPORTEDTRANSPORTVERSION",
            WINCODEC_DWRITE_DWM_ERR::UCEERR_TRANSPORTUNAVAILABLE => "UCEERR_TRANSPORTUNAVAILABLE",
            WINCODEC_DWRITE_DWM_ERR::UCEERR_FEEDBACK_UNSUPPORTED => "UCEERR_FEEDBACK_UNSUPPORTED",
            WINCODEC_DWRITE_DWM_ERR::UCEERR_COMMANDTRANSPORTDENIED => "UCEERR_COMMANDTRANSPORTDENIED",
            WINCODEC_DWRITE_DWM_ERR::UCEERR_GRAPHICSSTREAMUNAVAILABLE => "UCEERR_GRAPHICSSTREAMUNAVAILABLE",
            WINCODEC_DWRITE_DWM_ERR::UCEERR_GRAPHICSSTREAMALREADYOPEN => "UCEERR_GRAPHICSSTREAMALREADYOPEN",
            WINCODEC_DWRITE_DWM_ERR::UCEERR_TRANSPORTDISCONNECTED => "UCEERR_TRANSPORTDISCONNECTED",
            WINCODEC_DWRITE_DWM_ERR::UCEERR_TRANSPORTOVERLOADED => "UCEERR_TRANSPORTOVERLOADED",
            WINCODEC_DWRITE_DWM_ERR::UCEERR_PARTITION_ZOMBIED => "UCEERR_PARTITION_ZOMBIED",
            WINCODEC_DWRITE_DWM_ERR::MILAVERR_NOCLOCK => "MILAVERR_NOCLOCK",
            WINCODEC_DWRITE_DWM_ERR::MILAVERR_NOMEDIATYPE => "MILAVERR_NOMEDIATYPE",
            WINCODEC_DWRITE_DWM_ERR::MILAVERR_NOVIDEOMIXER => "MILAVERR_NOVIDEOMIXER",
            WINCODEC_DWRITE_DWM_ERR::MILAVERR_NOVIDEOPRESENTER => "MILAVERR_NOVIDEOPRESENTER",
            WINCODEC_DWRITE_DWM_ERR::MILAVERR_NOREADYFRAMES => "MILAVERR_NOREADYFRAMES",
            WINCODEC_DWRITE_DWM_ERR::MILAVERR_MODULENOTLOADED => "MILAVERR_MODULENOTLOADED",
            WINCODEC_DWRITE_DWM_ERR::MILAVERR_WMPFACTORYNOTREGISTERED => "MILAVERR_WMPFACTORYNOTREGISTERED",
            WINCODEC_DWRITE_DWM_ERR::MILAVERR_INVALIDWMPVERSION => "MILAVERR_INVALIDWMPVERSION",
            WINCODEC_DWRITE_DWM_ERR::MILAVERR_INSUFFICIENTVIDEORESOURCES => "MILAVERR_INSUFFICIENTVIDEORESOURCES",
            WINCODEC_DWRITE_DWM_ERR::MILAVERR_VIDEOACCELERATIONNOTAVAILABLE => "MILAVERR_VIDEOACCELERATIONNOTAVAILABLE",
            WINCODEC_DWRITE_DWM_ERR::MILAVERR_REQUESTEDTEXTURETOOBIG => "MILAVERR_REQUESTEDTEXTURETOOBIG",
            WINCODEC_DWRITE_DWM_ERR::MILAVERR_SEEKFAILED => "MILAVERR_SEEKFAILED",
            WINCODEC_DWRITE_DWM_ERR::MILAVERR_UNEXPECTEDWMPFAILURE => "MILAVERR_UNEXPECTEDWMPFAILURE",
            WINCODEC_DWRITE_DWM_ERR::MILAVERR_MEDIAPLAYERCLOSED => "MILAVERR_MEDIAPLAYERCLOSED",
            WINCODEC_DWRITE_DWM_ERR::MILAVERR_UNKNOWNHARDWAREERROR => "MILAVERR_UNKNOWNHARDWAREERROR",
            WINCODEC_DWRITE_DWM_ERR::MILEFFECTSERR_UNKNOWNPROPERTY => "MILEFFECTSERR_UNKNOWNPROPERTY",
            WINCODEC_DWRITE_DWM_ERR::MILEFFECTSERR_EFFECTNOTPARTOFGROUP => "MILEFFECTSERR_EFFECTNOTPARTOFGROUP",
            WINCODEC_DWRITE_DWM_ERR::MILEFFECTSERR_NOINPUTSOURCEATTACHED => "MILEFFECTSERR_NOINPUTSOURCEATTACHED",
            WINCODEC_DWRITE_DWM_ERR::MILEFFECTSERR_CONNECTORNOTCONNECTED => "MILEFFECTSERR_CONNECTORNOTCONNECTED",
            WINCODEC_DWRITE_DWM_ERR::MILEFFECTSERR_CONNECTORNOTASSOCIATEDWITHEFFECT => "MILEFFECTSERR_CONNECTORNOTASSOCIATEDWITHEFFECT",
            WINCODEC_DWRITE_DWM_ERR::MILEFFECTSERR_RESERVED => "MILEFFECTSERR_RESERVED",
            WINCODEC_DWRITE_DWM_ERR::MILEFFECTSERR_CYCLEDETECTED => "MILEFFECTSERR_CYCLEDETECTED",
            WINCODEC_DWRITE_DWM_ERR::MILEFFECTSERR_EFFECTINMORETHANONEGRAPH => "MILEFFECTSERR_EFFECTINMORETHANONEGRAPH",
            WINCODEC_DWRITE_DWM_ERR::MILEFFECTSERR_EFFECTALREADYINAGRAPH => "MILEFFECTSERR_EFFECTALREADYINAGRAPH",
            WINCODEC_DWRITE_DWM_ERR::MILEFFECTSERR_EFFECTHASNOCHILDREN => "MILEFFECTSERR_EFFECTHASNOCHILDREN",
            WINCODEC_DWRITE_DWM_ERR::MILEFFECTSERR_ALREADYATTACHEDTOLISTENER => "MILEFFECTSERR_ALREADYATTACHEDTOLISTENER",
            WINCODEC_DWRITE_DWM_ERR::MILEFFECTSERR_NOTAFFINETRANSFORM => "MILEFFECTSERR_NOTAFFINETRANSFORM",
            WINCODEC_DWRITE_DWM_ERR::MILEFFECTSERR_EMPTYBOUNDS => "MILEFFECTSERR_EMPTYBOUNDS",
            WINCODEC_DWRITE_DWM_ERR::MILEFFECTSERR_OUTPUTSIZETOOLARGE => "MILEFFECTSERR_OUTPUTSIZETOOLARGE",
            WINCODEC_DWRITE_DWM_ERR::DWMERR_STATE_TRANSITION_FAILED => "DWMERR_STATE_TRANSITION_FAILED",
            WINCODEC_DWRITE_DWM_ERR::DWMERR_THEME_FAILED => "DWMERR_THEME_FAILED",
            WINCODEC_DWRITE_DWM_ERR::DWMERR_CATASTROPHIC_FAILURE => "DWMERR_CATASTROPHIC_FAILURE",
            WINCODEC_DWRITE_DWM_ERR::DCOMPOSITION_ERROR_WINDOW_ALREADY_COMPOSED => "DCOMPOSITION_ERROR_WINDOW_ALREADY_COMPOSED",
            WINCODEC_DWRITE_DWM_ERR::DCOMPOSITION_ERROR_SURFACE_BEING_RENDERED => "DCOMPOSITION_ERROR_SURFACE_BEING_RENDERED",
            WINCODEC_DWRITE_DWM_ERR::DCOMPOSITION_ERROR_SURFACE_NOT_BEING_RENDERED => "DCOMPOSITION_ERROR_SURFACE_NOT_BEING_RENDERED",
            WINCODEC_DWRITE_DWM_ERR::WINCODEC_ERR_WRONGSTATE => "The codec is in the wrong state.",
            WINCODEC_DWRITE_DWM_ERR::WINCODEC_ERR_VALUEOUTOFRANGE => "The value is out of range.",
            WINCODEC_DWRITE_DWM_ERR::WINCODEC_ERR_UNKNOWNIMAGEFORMAT => "The image format is unknown.",
            WINCODEC_DWRITE_DWM_ERR::WINCODEC_ERR_UNSUPPORTEDVERSION => "The SDK version is unsupported.",
            WINCODEC_DWRITE_DWM_ERR::WINCODEC_ERR_NOTINITIALIZED => "The component is not initialized.",
            WINCODEC_DWRITE_DWM_ERR::WINCODEC_ERR_ALREADYLOCKED => "There is already an outstanding read or write lock.",
            WINCODEC_DWRITE_DWM_ERR::WINCODEC_ERR_PROPERTYNOTFOUND => "The specified bitmap property cannot be found.",
            WINCODEC_DWRITE_DWM_ERR::WINCODEC_ERR_PROPERTYNOTSUPPORTED => "The bitmap codec does not support the bitmap property.",
            WINCODEC_DWRITE_DWM_ERR::WINCODEC_ERR_PROPERTYSIZE => "The bitmap property size is invalid.",
            WINCODEC_DWRITE_DWM_ERR::WINCODEC_ERR_CODECPRESENT => "An unknown error has occurred.",
            WINCODEC_DWRITE_DWM_ERR::WINCODEC_ERR_CODECNOTHUMBNAIL => "The bitmap codec does not support a thumbnail.",
            WINCODEC_DWRITE_DWM_ERR::WINCODEC_ERR_PALETTEUNAVAILABLE => "The bitmap palette is unavailable.",
            WINCODEC_DWRITE_DWM_ERR::WINCODEC_ERR_CODECTOOMANYSCANLINES => "Too many scanlines were requested.",
            WINCODEC_DWRITE_DWM_ERR::WINCODEC_ERR_INTERNALERROR => "An internal error occurred.",
            WINCODEC_DWRITE_DWM_ERR::WINCODEC_ERR_SOURCERECTDOESNOTMATCHDIMENSIONS => "The bitmap bounds do not match the bitmap dimensions.",
            WINCODEC_DWRITE_DWM_ERR::WINCODEC_ERR_COMPONENTNOTFOUND => "The component cannot be found.",
            WINCODEC_DWRITE_DWM_ERR::WINCODEC_ERR_IMAGESIZEOUTOFRANGE => "The bitmap size is outside the valid range.",
            WINCODEC_DWRITE_DWM_ERR::WINCODEC_ERR_TOOMUCHMETADATA => "There is too much metadata to be written to the bitmap.",
            WINCODEC_DWRITE_DWM_ERR::WINCODEC_ERR_BADIMAGE => "The image is unrecognized.",
            WINCODEC_DWRITE_DWM_ERR::WINCODEC_ERR_BADHEADER => "The image header is unrecognized.",
            WINCODEC_DWRITE_DWM_ERR::WINCODEC_ERR_FRAMEMISSING => "The bitmap frame is missing.",
            WINCODEC_DWRITE_DWM_ERR::WINCODEC_ERR_BADMETADATAHEADER => "The image metadata header is unrecognized.",
            WINCODEC_DWRITE_DWM_ERR::WINCODEC_ERR_BADSTREAMDATA => "The stream data is unrecognized.",
            WINCODEC_DWRITE_DWM_ERR::WINCODEC_ERR_STREAMWRITE => "Failed to write to the stream.",
            WINCODEC_DWRITE_DWM_ERR::WINCODEC_ERR_STREAMREAD => "Failed to read from the stream.",
            WINCODEC_DWRITE_DWM_ERR::WINCODEC_ERR_STREAMNOTAVAILABLE => "The stream is not available.",
            WINCODEC_DWRITE_DWM_ERR::WINCODEC_ERR_UNSUPPORTEDPIXELFORMAT => "The bitmap pixel format is unsupported.",
            WINCODEC_DWRITE_DWM_ERR::WINCODEC_ERR_UNSUPPORTEDOPERATION => "The operation is unsupported.",
            WINCODEC_DWRITE_DWM_ERR::WINCODEC_ERR_INVALIDREGISTRATION => "The component registration is invalid.",
            WINCODEC_DWRITE_DWM_ERR::WINCODEC_ERR_COMPONENTINITIALIZEFAILURE => "The component initialization has failed.",
            WINCODEC_DWRITE_DWM_ERR::WINCODEC_ERR_INSUFFICIENTBUFFER => "The buffer allocated is insufficient.",
            WINCODEC_DWRITE_DWM_ERR::WINCODEC_ERR_DUPLICATEMETADATAPRESENT => "Duplicate metadata is present.",
            WINCODEC_DWRITE_DWM_ERR::WINCODEC_ERR_PROPERTYUNEXPECTEDTYPE => "The bitmap property type is unexpected.",
            WINCODEC_DWRITE_DWM_ERR::WINCODEC_ERR_UNEXPECTEDSIZE => "The size is unexpected.",
            WINCODEC_DWRITE_DWM_ERR::WINCODEC_ERR_INVALIDQUERYREQUEST => "The property query is invalid.",
            WINCODEC_DWRITE_DWM_ERR::WINCODEC_ERR_UNEXPECTEDMETADATATYPE => "The metadata type is unexpected.",
            WINCODEC_DWRITE_DWM_ERR::WINCODEC_ERR_REQUESTONLYVALIDATMETADATAROOT => "The specified bitmap property is only valid at root level.",
            WINCODEC_DWRITE_DWM_ERR::WINCODEC_ERR_INVALIDQUERYCHARACTER => "The query string contains an invalid character.",
            WINCODEC_DWRITE_DWM_ERR::WINCODEC_ERR_WIN32ERROR => "Windows Codecs received an error from the Win32 system.",
            WINCODEC_DWRITE_DWM_ERR::WINCODEC_ERR_INVALIDPROGRESSIVELEVEL => "The requested level of detail is not present.",
            WINCODEC_DWRITE_DWM_ERR::WINCODEC_ERR_INVALIDJPEGSCANINDEX => "The scan index is invalid.",
            WINCODEC_DWRITE_DWM_ERR::DWRITE_E_FILEFORMAT => "Indicates an error in an input file such as a font file.",
            WINCODEC_DWRITE_DWM_ERR::DWRITE_E_UNEXPECTED => "Indicates an error originating in DirectWrite code, which is not expected to occur but is safe to recover from.",
            WINCODEC_DWRITE_DWM_ERR::DWRITE_E_NOFONT => "Indicates the specified font does not exist.",
            WINCODEC_DWRITE_DWM_ERR::DWRITE_E_FILENOTFOUND => "A font file could not be opened because the file, directory, network location, drive, or other storage location does not exist or is unavailable.",
            WINCODEC_DWRITE_DWM_ERR::DWRITE_E_FILEACCESS => "A font file exists but could not be opened due to access denied, sharing violation, or similar error.",
            WINCODEC_DWRITE_DWM_ERR::DWRITE_E_FONTCOLLECTIONOBSOLETE => "A font collection is obsolete due to changes in the system.",
            WINCODEC_DWRITE_DWM_ERR::DWRITE_E_ALREADYREGISTERED => "The given interface is already registered.",
            WINCODEC_DWRITE_DWM_ERR::DWRITE_E_CACHEFORMAT => "The font cache contains invalid data.",
            WINCODEC_DWRITE_DWM_ERR::DWRITE_E_CACHEVERSION => "A font cache file corresponds to a different version of DirectWrite.",
            WINCODEC_DWRITE_DWM_ERR::DWRITE_E_UNSUPPORTEDOPERATION => "The operation is not supported for this type of font.",
            WINCODEC_DWRITE_DWM_ERR::DWRITE_E_TEXTRENDERERINCOMPATIBLE => "The version of the text renderer interface is not compatible.",
            WINCODEC_DWRITE_DWM_ERR::DWRITE_E_FLOWDIRECTIONCONFLICTS => "The flow direction conflicts with the reading direction. They must be perpendicular to each other.",
            WINCODEC_DWRITE_DWM_ERR::DWRITE_E_NOCOLOR => "The font or glyph run does not contain any colored glyphs.",
            WINCODEC_DWRITE_DWM_ERR::DWRITE_E_REMOTEFONT => "A font resource could not be accessed because it is remote.",
            WINCODEC_DWRITE_DWM_ERR::DWRITE_E_DOWNLOADCANCELLED => "A font download was canceled.",
            WINCODEC_DWRITE_DWM_ERR::DWRITE_E_DOWNLOADFAILED => "A font download failed.",
            WINCODEC_DWRITE_DWM_ERR::DWRITE_E_TOOMANYDOWNLOADS => "A font download request was not added or a download failed because there are too many active downloads.",
        }
    }
}