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

pub enum DIRECTMUSIC {
    DSERR_CONTROLUNAVAIL = 0x8878001E,
    DSERR_INVALIDCALL = 0x88780032,
    DSERR_PRIOLEVELNEEDED = 0x88780046,
    DSERR_BADFORMAT = 0x88780064,
    DSERR_NODRIVER = 0x88780078,
    DSERR_BUFFERLOST = 0x88780096,
    DSERR_OTHERAPPHASPRIO = 0x887800A0,
    DSERR_UNINITIALIZED = 0x887800AA,
    DSERR_DS8_REQUIRED = 0x887800BE,
    DMUS_E_DRIVER_FAILED = 0x88781101,
    DMUS_E_PORTS_OPEN = 0x88781102,
    DMUS_E_DEVICE_IN_USE = 0x88781103,
    DMUS_E_INSUFFICIENTBUFFER = 0x88781104,
    DMUS_E_BUFFERNOTSET = 0x88781105,
    DMUS_E_BUFFERNOTAVAILABLE = 0x88781106,
    DMUS_E_NOTADLSCOL = 0x88781108,
    DMUS_E_INVALIDOFFSET = 0x88781109,
    DMUS_E_ALREADY_LOADED = 0x88781111,
    DMUS_E_INVALIDPOS = 0x88781113,
    DMUS_E_INVALIDPATCH = 0x88781114,
    DMUS_E_CANNOTSEEK = 0x88781115,
    DMUS_E_CANNOTWRITE = 0x88781116,
    DMUS_E_CHUNKNOTFOUND = 0x88781117,
    DMUS_E_INVALID_DOWNLOADID = 0x88781119,
    DMUS_E_NOT_DOWNLOADED_TO_PORT = 0x88781120,
    DMUS_E_ALREADY_DOWNLOADED = 0x88781121,
    DMUS_E_UNKNOWN_PROPERTY = 0x88781122,
    DMUS_E_SET_UNSUPPORTED = 0x88781123,
    DMUS_E_GET_UNSUPPORTED = 0x88781124,
    DMUS_E_NOTMONO = 0x88781125,
    DMUS_E_BADARTICULATION = 0x88781126,
    DMUS_E_BADINSTRUMENT = 0x88781127,
    DMUS_E_BADWAVELINK = 0x88781128,
    DMUS_E_NOARTICULATION = 0x88781129,
    DMUS_E_NOTPCM = 0x8878112A,
    DMUS_E_BADWAVE = 0x8878112B,
    DMUS_E_BADOFFSETTABLE = 0x8878112C,
    DMUS_E_UNKNOWNDOWNLOAD = 0x8878112D,
    DMUS_E_NOSYNTHSINK = 0x8878112E,
    DMUS_E_ALREADYOPEN = 0x8878112F,
    DMUS_E_ALREADYCLOSED = 0x88781130,
    DMUS_E_SYNTHNOTCONFIGURED = 0x88781131,
    DMUS_E_SYNTHACTIVE = 0x88781132,
    DMUS_E_CANNOTREAD = 0x88781133,
    DMUS_E_DMUSIC_RELEASED = 0x88781134,
    DMUS_E_BUFFER_EMPTY = 0x88781135,
    DMUS_E_BUFFER_FULL = 0x88781136,
    DMUS_E_PORT_NOT_CAPTURE = 0x88781137,
    DMUS_E_PORT_NOT_RENDER = 0x88781138,
    DMUS_E_DSOUND_NOT_SET = 0x88781139,
    DMUS_E_ALREADY_ACTIVATED = 0x8878113A,
    DMUS_E_INVALIDBUFFER = 0x8878113B,
    DMUS_E_WAVEFORMATNOTSUPPORTED = 0x8878113C,
    DMUS_E_SYNTHINACTIVE = 0x8878113D,
    DMUS_E_DSOUND_ALREADY_SET = 0x8878113E,
    DMUS_E_INVALID_EVENT = 0x8878113F,
    DMUS_E_UNSUPPORTED_STREAM = 0x88781150,
    DMUS_E_ALREADY_INITED = 0x88781151,
    DMUS_E_INVALID_BAND = 0x88781152,
    DMUS_E_TRACK_HDR_NOT_FIRST_CK = 0x88781155,
    DMUS_E_TOOL_HDR_NOT_FIRST_CK = 0x88781156,
    DMUS_E_INVALID_TRACK_HDR = 0x88781157,
    DMUS_E_INVALID_TOOL_HDR = 0x88781158,
    DMUS_E_ALL_TOOLS_FAILED = 0x88781159,
    DMUS_E_ALL_TRACKS_FAILED = 0x88781160,
    DMUS_E_NOT_FOUND = 0x88781161,
    DMUS_E_NOT_INIT = 0x88781162,
    DMUS_E_TYPE_DISABLED = 0x88781163,
    DMUS_E_TYPE_UNSUPPORTED = 0x88781164,
    DMUS_E_TIME_PAST = 0x88781165,
    DMUS_E_TRACK_NOT_FOUND = 0x88781166,
    DMUS_E_TRACK_NO_CLOCKTIME_SUPPORT = 0x88781167,
    DMUS_E_NO_MASTER_CLOCK = 0x88781170,
    DMUS_E_LOADER_NOCLASSID = 0x88781180,
    DMUS_E_LOADER_BADPATH = 0x88781181,
    DMUS_E_LOADER_FAILEDOPEN = 0x88781182,
    DMUS_E_LOADER_FORMATNOTSUPPORTED = 0x88781183,
    DMUS_E_LOADER_FAILEDCREATE = 0x88781184,
    DMUS_E_LOADER_OBJECTNOTFOUND = 0x88781185,
    DMUS_E_LOADER_NOFILENAME = 0x88781186,
    DMUS_E_INVALIDFILE = 0x88781200,
    DMUS_E_ALREADY_EXISTS = 0x88781201,
    DMUS_E_OUT_OF_RANGE = 0x88781202,
    DMUS_E_SEGMENT_INIT_FAILED = 0x88781203,
    DMUS_E_ALREADY_SENT = 0x88781204,
    DMUS_E_CANNOT_FREE = 0x88781205,
    DMUS_E_CANNOT_OPEN_PORT = 0x88781206,
    DMUS_E_CANNOT_CONVERT = 0x88781207,
    DMUS_E_DESCEND_CHUNK_FAIL = 0x88781210,
    DMUS_E_NOT_LOADED = 0x88781211,
    DMUS_E_SCRIPT_LANGUAGE_INCOMPATIBLE = 0x88781213,
    DMUS_E_SCRIPT_UNSUPPORTED_VARTYPE = 0x88781214,
    DMUS_E_SCRIPT_ERROR_IN_SCRIPT = 0x88781215,
    DMUS_E_SCRIPT_CANTLOAD_OLEAUT32 = 0x88781216,
    DMUS_E_SCRIPT_LOADSCRIPT_ERROR = 0x88781217,
    DMUS_E_SCRIPT_INVALID_FILE = 0x88781218,
    DMUS_E_INVALID_SCRIPTTRACK = 0x88781219,
    DMUS_E_SCRIPT_VARIABLE_NOT_FOUND = 0x8878121A,
    DMUS_E_SCRIPT_ROUTINE_NOT_FOUND = 0x8878121B,
    DMUS_E_SCRIPT_CONTENT_READONLY = 0x8878121C,
    DMUS_E_SCRIPT_NOT_A_REFERENCE = 0x8878121D,
    DMUS_E_SCRIPT_VALUE_NOT_SUPPORTED = 0x8878121E,
    DMUS_E_INVALID_SEGMENTTRIGGERTRACK = 0x88781220,
    DMUS_E_INVALID_LYRICSTRACK = 0x88781221,
    DMUS_E_INVALID_PARAMCONTROLTRACK = 0x88781222,
    DMUS_E_AUDIOVBSCRIPT_SYNTAXERROR = 0x88781223,
    DMUS_E_AUDIOVBSCRIPT_RUNTIMEERROR = 0x88781224,
    DMUS_E_AUDIOVBSCRIPT_OPERATIONFAILURE = 0x88781225,
    DMUS_E_AUDIOPATHS_NOT_VALID = 0x88781226,
    DMUS_E_AUDIOPATHS_IN_USE = 0x88781227,
    DMUS_E_NO_AUDIOPATH_CONFIG = 0x88781228,
    DMUS_E_AUDIOPATH_INACTIVE = 0x88781229,
    DMUS_E_AUDIOPATH_NOBUFFER = 0x8878122A,
    DMUS_E_AUDIOPATH_NOPORT = 0x8878122B,
    DMUS_E_NO_AUDIOPATH = 0x8878122C,
    DMUS_E_INVALIDCHUNK = 0x8878122D,
    DMUS_E_AUDIOPATH_NOGLOBALFXBUFFER = 0x8878122E,
    DMUS_E_INVALID_CONTAINER_OBJECT = 0x8878122F,
}

impl DIRECTMUSIC {
    pub fn description(&self) -> &'static str {
        match self {
            DIRECTMUSIC::DSERR_CONTROLUNAVAIL => "The requested control is not available.",
            DIRECTMUSIC::DSERR_INVALIDCALL => "The call is not valid for the current state of this object.",
            DIRECTMUSIC::DSERR_PRIOLEVELNEEDED => "The function requires a higher priority level.",
            DIRECTMUSIC::DSERR_BADFORMAT => "The specified wave format is not supported.",
            DIRECTMUSIC::DSERR_NODRIVER => "No sound driver is available for use.",
            DIRECTMUSIC::DSERR_BUFFERLOST => "Buffer memory has been lost, and must be restored.",
            DIRECTMUSIC::DSERR_OTHERAPPHASPRIO => "Another dsound app has focus, preventing this call from succeeding.",
            DIRECTMUSIC::DSERR_UNINITIALIZED => "This object has not been initialized.",
            DIRECTMUSIC::DSERR_DS8_REQUIRED => "Attempted to use DirectSound 8+ functionality on an older DirectSound object",
            DIRECTMUSIC::DMUS_E_DRIVER_FAILED => "An unexpected error was returned from a device driver, indicating possible failure of the driver or hardware.",
            DIRECTMUSIC::DMUS_E_PORTS_OPEN => "The requested operation cannot be performed while there are instantiated ports in any process in the system.",
            DIRECTMUSIC::DMUS_E_DEVICE_IN_USE => "The requested device is already in use (possibly by a non-DirectMusic client), and cannot be opened again.",
            DIRECTMUSIC::DMUS_E_INSUFFICIENTBUFFER => "Buffer is not large enough for the requested operation.",
            DIRECTMUSIC::DMUS_E_BUFFERNOTSET => "No buffer was prepared for the download data.",
            DIRECTMUSIC::DMUS_E_BUFFERNOTAVAILABLE => "Download failed due to inability to access or create a download buffer.",
            DIRECTMUSIC::DMUS_E_NOTADLSCOL => "Error parsing DLS collection. File is corrupt.",
            DIRECTMUSIC::DMUS_E_INVALIDOFFSET => "Wave chunks in DLS collection file are at incorrect offsets.",
            DIRECTMUSIC::DMUS_E_ALREADY_LOADED => "Attempted to load a DLS collection that has already been loaded.",
            DIRECTMUSIC::DMUS_E_INVALIDPOS => "Error reading wave data from DLS collection. Indicates a bad file.",
            DIRECTMUSIC::DMUS_E_INVALIDPATCH => "There is no instrument in the collection that matches the specified patch number.",
            DIRECTMUSIC::DMUS_E_CANNOTSEEK => "The IStream* doesn't support Seek().",
            DIRECTMUSIC::DMUS_E_CANNOTWRITE => "The IStream* doesn't support Write().",
            DIRECTMUSIC::DMUS_E_CHUNKNOTFOUND => "A required RIFF chunk could not be found while parsing a file.",
            DIRECTMUSIC::DMUS_E_INVALID_DOWNLOADID => "An invalid download id was used in the process of creating a download buffer.",
            DIRECTMUSIC::DMUS_E_NOT_DOWNLOADED_TO_PORT => "Attempted to unload an object that was not downloaded or that was previously unloaded.",
            DIRECTMUSIC::DMUS_E_ALREADY_DOWNLOADED => "Buffer was already downloaded to synth.",
            DIRECTMUSIC::DMUS_E_UNKNOWN_PROPERTY => "The specified property item was not recognized by the target object.",
            DIRECTMUSIC::DMUS_E_SET_UNSUPPORTED => "The specified property item may not be set on the target object.",
            DIRECTMUSIC::DMUS_E_GET_UNSUPPORTED => "The specified property item may not be retrieved from the target object.",
            DIRECTMUSIC::DMUS_E_NOTMONO => "Wave chunk has more than one interleaved channel. DLS format requires MONO.",
            DIRECTMUSIC::DMUS_E_BADARTICULATION => "Invalid articulation chunk in DLS collection.",
            DIRECTMUSIC::DMUS_E_BADINSTRUMENT => "Invalid instrument chunk in DLS collection.",
            DIRECTMUSIC::DMUS_E_BADWAVELINK => "Wavelink chunk in DLS collection points to invalid wave.",
            DIRECTMUSIC::DMUS_E_NOARTICULATION => "Articulation missing from instrument in DLS collection.",
            DIRECTMUSIC::DMUS_E_NOTPCM => "Downoaded DLS wave is not in PCM format.",
            DIRECTMUSIC::DMUS_E_BADWAVE => "Bad wave chunk in DLS collection.",
            DIRECTMUSIC::DMUS_E_BADOFFSETTABLE => "Offset Table for download buffer has errors.",
            DIRECTMUSIC::DMUS_E_UNKNOWNDOWNLOAD => "Attempted to download unknown data type.",
            DIRECTMUSIC::DMUS_E_NOSYNTHSINK => "An operation could not be completed because no sink was connected to the synthesizer.",
            DIRECTMUSIC::DMUS_E_ALREADYOPEN => "An attempt was made to open the software synthesizer while it was already open.",
            DIRECTMUSIC::DMUS_E_ALREADYCLOSED => "An attempt was made to close the software synthesizer while it was already closed.",
            DIRECTMUSIC::DMUS_E_SYNTHNOTCONFIGURED => "The operation could not be completed because the software synthesizer has not yet been fully configured.",
            DIRECTMUSIC::DMUS_E_SYNTHACTIVE => "The operation cannot be carried out while the synthesizer is active.",
            DIRECTMUSIC::DMUS_E_CANNOTREAD => "An error occurred while attempting to read from the IStream* object.",
            DIRECTMUSIC::DMUS_E_DMUSIC_RELEASED => "The operation cannot be performed because the final instance of the DirectMusic object was released. Ports cannot be used after final release of the DirectMusic object.",
            DIRECTMUSIC::DMUS_E_BUFFER_EMPTY => "There was no data in the referenced buffer.",
            DIRECTMUSIC::DMUS_E_BUFFER_FULL => "There is insufficient space to insert the specified event into the buffer.",
            DIRECTMUSIC::DMUS_E_PORT_NOT_CAPTURE => "The specified operation could not be carried out because the port is a capture port.",
            DIRECTMUSIC::DMUS_E_PORT_NOT_RENDER => "The specified operation could not be carried out because the port is a render port.",
            DIRECTMUSIC::DMUS_E_DSOUND_NOT_SET => "The port could not be created because no DirectSound instance has been specified. Specify a DirectSound interface via the IDirectMusic::SetDirectSound method; pass NULL to have DirectMusic manage usage of DirectSound.",
            DIRECTMUSIC::DMUS_E_ALREADY_ACTIVATED => "The operation cannot be carried out while the port is active.",
            DIRECTMUSIC::DMUS_E_INVALIDBUFFER => "An invalid DirectSound buffer was passed to the port.",
            DIRECTMUSIC::DMUS_E_WAVEFORMATNOTSUPPORTED => "An invalid buffer format was handed to the synth sink.",
            DIRECTMUSIC::DMUS_E_SYNTHINACTIVE => "The operation cannot be carried out while the synthesizer is inactive.",
            DIRECTMUSIC::DMUS_E_DSOUND_ALREADY_SET => "IDirectMusic::SetDirectSound has already been called. It may not be changed while in use.",
            DIRECTMUSIC::DMUS_E_INVALID_EVENT => "The given event is invalid (either it is not a valid MIDI message or it makes use of running status). The event cannot be packed into the buffer.",
            DIRECTMUSIC::DMUS_E_UNSUPPORTED_STREAM => "The IStream* object does not contain data supported by the loading object.",
            DIRECTMUSIC::DMUS_E_ALREADY_INITED => "The object has already been initialized.",
            DIRECTMUSIC::DMUS_E_INVALID_BAND => "The file does not contain a valid band.",
            DIRECTMUSIC::DMUS_E_TRACK_HDR_NOT_FIRST_CK => "The IStream* object's data does not have a track header as the first chunk. Therefore, it can't be read by the segment object.",
            DIRECTMUSIC::DMUS_E_TOOL_HDR_NOT_FIRST_CK => "The IStream* object's data does not have a tool header as the first chunk. Therefore, it can't be read by the graph object.",
            DIRECTMUSIC::DMUS_E_INVALID_TRACK_HDR => "The IStream* object's data contains an invalid track header (ckid is 0 and fccType is NULL). Therefore, it can't be read by the segment object.",
            DIRECTMUSIC::DMUS_E_INVALID_TOOL_HDR => "The IStream* object's data contains an invalid tool header (ckid is 0 and fccType is NULL). Therefore, it can't be read by the graph object.",
            DIRECTMUSIC::DMUS_E_ALL_TOOLS_FAILED => "The graph object was unable to load all tools from the IStream* object data. This may be due to errors in the stream, or the tools being incorrectly registered on the client.",
            DIRECTMUSIC::DMUS_E_ALL_TRACKS_FAILED => "The segment object was unable to load all tracks from the IStream* object data. This may be due to errors in the stream, or the tracks being incorrectly registered on the client.",
            DIRECTMUSIC::DMUS_E_NOT_FOUND => "The requested item was not contained by the object.",
            DIRECTMUSIC::DMUS_E_NOT_INIT => "A required object is not initialized or failed to initialize.",
            DIRECTMUSIC::DMUS_E_TYPE_DISABLED => "The requested parameter type is currently disabled. Parameter types may be enabled and disabled by certain calls to SetParam().",
            DIRECTMUSIC::DMUS_E_TYPE_UNSUPPORTED => "The requested parameter type is not supported on the object.",
            DIRECTMUSIC::DMUS_E_TIME_PAST => "The specified time is in the past, and the operation can not succeed.",
            DIRECTMUSIC::DMUS_E_TRACK_NOT_FOUND => "The requested track is not contained by the segment.",
            DIRECTMUSIC::DMUS_E_TRACK_NO_CLOCKTIME_SUPPORT => "The track does not support clock time playback or getparam.",
            DIRECTMUSIC::DMUS_E_NO_MASTER_CLOCK => "There is no master clock in the performance. Be sure to call IDirectMusicPerformance::Init().",
            DIRECTMUSIC::DMUS_E_LOADER_NOCLASSID => "The class id field is required and is missing in the DMUS_OBJECTDESC.",
            DIRECTMUSIC::DMUS_E_LOADER_BADPATH => "The specified file path is invalid.",
            DIRECTMUSIC::DMUS_E_LOADER_FAILEDOPEN => "File open failed.",
            DIRECTMUSIC::DMUS_E_LOADER_FORMATNOTSUPPORTED => "Search data type is not supported.",
            DIRECTMUSIC::DMUS_E_LOADER_FAILEDCREATE => "Unable to find or create object.",
            DIRECTMUSIC::DMUS_E_LOADER_OBJECTNOTFOUND => "Object was not found.",
            DIRECTMUSIC::DMUS_E_LOADER_NOFILENAME => "The file name is missing from the DMUS_OBJECTDESC.",
            DIRECTMUSIC::DMUS_E_INVALIDFILE => "The specified file is not a valid file.",
            DIRECTMUSIC::DMUS_E_ALREADY_EXISTS => "The tool is already contained in the graph. Create a new instance.",
            DIRECTMUSIC::DMUS_E_OUT_OF_RANGE => "Value is out of range.",
            DIRECTMUSIC::DMUS_E_SEGMENT_INIT_FAILED => "Segment initialization failed.",
            DIRECTMUSIC::DMUS_E_ALREADY_SENT => "The DMUS_PMSG has already been sent to the performance object via IDirectMusicPerformance::SendPMsg().",
            DIRECTMUSIC::DMUS_E_CANNOT_FREE => "The DMUS_PMSG was either not allocated by the performance via IDirectMusicPerformance::AllocPMsg(), or it was already freed via IDirectMusicPerformance::FreePMsg().",
            DIRECTMUSIC::DMUS_E_CANNOT_OPEN_PORT => "The default system port could not be opened.",
            DIRECTMUSIC::DMUS_E_CANNOT_CONVERT => "A call to MIDIToMusic() or MusicToMIDI() resulted in an error because the requested conversion could not happen. This usually occurs when the provided DMUS_CHORD_KEY structure has an invalid chord or scale pattern.",
            DIRECTMUSIC::DMUS_E_DESCEND_CHUNK_FAIL => "The end of the file was reached before the specified chunk was found.",
            DIRECTMUSIC::DMUS_E_NOT_LOADED => "An attempt to use this object failed because the object has not been loaded.",
            DIRECTMUSIC::DMUS_E_SCRIPT_LANGUAGE_INCOMPATIBLE => "The activeX scripting engine for the script's language is not compatible with DirectMusic.",
            DIRECTMUSIC::DMUS_E_SCRIPT_UNSUPPORTED_VARTYPE => "A variant was used that had a type that is not supported by DirectMusic.",
            DIRECTMUSIC::DMUS_E_SCRIPT_ERROR_IN_SCRIPT => "An error was encountered while parsing or executing the script.",
            DIRECTMUSIC::DMUS_E_SCRIPT_CANTLOAD_OLEAUT32 => "Loading of oleaut32.dll failed.",
            DIRECTMUSIC::DMUS_E_SCRIPT_LOADSCRIPT_ERROR => "An error occurred while parsing a script loaded using LoadScript. The script being loaded contains an error.",
            DIRECTMUSIC::DMUS_E_SCRIPT_INVALID_FILE => "The script file is invalid.",
            DIRECTMUSIC::DMUS_E_INVALID_SCRIPTTRACK => "The file contains an invalid script track.",
            DIRECTMUSIC::DMUS_E_SCRIPT_VARIABLE_NOT_FOUND => "The script does not contain a variable with the specified name.",
            DIRECTMUSIC::DMUS_E_SCRIPT_ROUTINE_NOT_FOUND => "The script does not contain a routine with the specified name.",
            DIRECTMUSIC::DMUS_E_SCRIPT_CONTENT_READONLY => "Attempted to set variables for content referenced or embedded in a script.",
            DIRECTMUSIC::DMUS_E_SCRIPT_NOT_A_REFERENCE => "Attempt was made to set a script's variable by reference to a value that was not an object type.",
            DIRECTMUSIC::DMUS_E_SCRIPT_VALUE_NOT_SUPPORTED => "Attempt was made to set a script's variable by value to an object that does not support a default value property.",
            DIRECTMUSIC::DMUS_E_INVALID_SEGMENTTRIGGERTRACK => "The file contains an invalid segment trigger track.",
            DIRECTMUSIC::DMUS_E_INVALID_LYRICSTRACK => "The file contains an invalid lyrics track.",
            DIRECTMUSIC::DMUS_E_INVALID_PARAMCONTROLTRACK => "The file contains an invalid parameter control track.",
            DIRECTMUSIC::DMUS_E_AUDIOVBSCRIPT_SYNTAXERROR => "A script written in AudioVBScript could not be read because it contained a statement that is not allowed by the AudioVBScript language.",
            DIRECTMUSIC::DMUS_E_AUDIOVBSCRIPT_RUNTIMEERROR => "A script routine written in AudioVBScript failed because an invalid operation occurred.",
            DIRECTMUSIC::DMUS_E_AUDIOVBSCRIPT_OPERATIONFAILURE => "A script routine written in AudioVBScript failed because a function outside of a script failed to complete.",
            DIRECTMUSIC::DMUS_E_AUDIOPATHS_NOT_VALID => "Audio paths are invalid because pchannels have been set.",
            DIRECTMUSIC::DMUS_E_AUDIOPATHS_IN_USE => "Pchannels are invalid because audio paths have been set.",
            DIRECTMUSIC::DMUS_E_NO_AUDIOPATH_CONFIG => "The specified segment has no embedded audio path configuration.",
            DIRECTMUSIC::DMUS_E_AUDIOPATH_INACTIVE => "An audiopath is inactive.",
            DIRECTMUSIC::DMUS_E_AUDIOPATH_NOBUFFER => "An audiopath failed to create because a requested buffer could not be created.",
            DIRECTMUSIC::DMUS_E_AUDIOPATH_NOPORT => "An audiopath could not be used for playback because it lacked port assignments.",
            DIRECTMUSIC::DMUS_E_NO_AUDIOPATH => "Attempted to play a segment in audiopath mode when there was no audiopath.",
            DIRECTMUSIC::DMUS_E_INVALIDCHUNK => "Invalid data was found in a RIFF file chunk.",
            DIRECTMUSIC::DMUS_E_AUDIOPATH_NOGLOBALFXBUFFER => "Attempted to create an audiopath that sends to a nonexistent global effects buffer.",
            DIRECTMUSIC::DMUS_E_INVALID_CONTAINER_OBJECT => "The file does not contain a valid container object.",
        }
    }
}
