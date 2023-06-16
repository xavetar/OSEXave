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

pub enum XACTENGINE {
    XACTENGINE_E_ALREADYINITIALIZED = 0x8AC70001,
    XACTENGINE_E_NOTINITIALIZED = 0x8AC70002,
    XACTENGINE_E_EXPIRED = 0x8AC70003,
    XACTENGINE_E_NONOTIFICATIONCALLBACK = 0x8AC70004,
    XACTENGINE_E_NOTIFICATIONREGISTERED = 0x8AC70005,
    XACTENGINE_E_INVALIDUSAGE = 0x8AC70006,
    XACTENGINE_E_INVALIDDATA = 0x8AC70007,
    XACTENGINE_E_INSTANCELIMITFAILTOPLAY = 0x8AC70008,
    XACTENGINE_E_NOGLOBALSETTINGS = 0x8AC70009,
    XACTENGINE_E_INVALIDVARIABLEINDEX = 0x8AC7000A,
    XACTENGINE_E_INVALIDCATEGORY = 0x8AC7000B,
    XACTENGINE_E_INVALIDCUEINDEX = 0x8AC7000C,
    XACTENGINE_E_INVALIDWAVEINDEX = 0x8AC7000D,
    XACTENGINE_E_INVALIDTRACKINDEX = 0x8AC7000E,
    XACTENGINE_E_INVALIDSOUNDOFFSETORINDEX = 0x8AC7000F,
    XACTENGINE_E_READFILE = 0x8AC70010,
    XACTENGINE_E_UNKNOWNEVENT = 0x8AC70011,
    XACTENGINE_E_INCALLBACK = 0x8AC70012,
    XACTENGINE_E_NOWAVEBANK = 0x8AC70013,
    XACTENGINE_E_SELECTVARIATION = 0x8AC70014,
    XACTENGINE_E_MULTIPLEAUDITIONENGINES = 0x8AC70015,
    XACTENGINE_E_WAVEBANKNOTPREPARED = 0x8AC70016,
    XACTENGINE_E_NORENDERER = 0x8AC70017,
    XACTENGINE_E_INVALIDENTRYCOUNT = 0x8AC70018,
    XACTENGINE_E_AUDITION_WRITEFILE = 0x8AC70101,
    XACTENGINE_E_AUDITION_NOSOUNDBANK = 0x8AC70102,
    XACTENGINE_E_AUDITION_INVALIDRPCINDEX = 0x8AC70103,
    XACTENGINE_E_AUDITION_MISSINGDATA = 0x8AC70104,
    XACTENGINE_E_AUDITION_UNKNOWNCOMMAND = 0x8AC70105,
    XACTENGINE_E_AUDITION_INVALIDDSPINDEX = 0x8AC70106,
}

impl XACTENGINE {
    pub fn description(&self) -> &'static str {
        match self {
            XACTENGINE::XACTENGINE_E_ALREADYINITIALIZED => "The engine is already initialized.",
            XACTENGINE::XACTENGINE_E_NOTINITIALIZED => "The engine has not been initialized.",
            XACTENGINE::XACTENGINE_E_EXPIRED => "The engine has expired (demo or pre-release version).",
            XACTENGINE::XACTENGINE_E_NONOTIFICATIONCALLBACK => "No notification callback.",
            XACTENGINE::XACTENGINE_E_NOTIFICATIONREGISTERED => "Notification already registered.",
            XACTENGINE::XACTENGINE_E_INVALIDUSAGE => "Invalid usage.",
            XACTENGINE::XACTENGINE_E_INVALIDDATA => "Invalid data.",
            XACTENGINE::XACTENGINE_E_INSTANCELIMITFAILTOPLAY => "A sound failed to play due to the instance limit.",
            XACTENGINE::XACTENGINE_E_NOGLOBALSETTINGS => "Global Settings are not loaded.",
            XACTENGINE::XACTENGINE_E_INVALIDVARIABLEINDEX => "Invalid variable index.",
            XACTENGINE::XACTENGINE_E_INVALIDCATEGORY => "Invalid category.",
            XACTENGINE::XACTENGINE_E_INVALIDCUEINDEX => "Invalid cue index.",
            XACTENGINE::XACTENGINE_E_INVALIDWAVEINDEX => "Invalid wave index.",
            XACTENGINE::XACTENGINE_E_INVALIDTRACKINDEX => "Invalid track index.",
            XACTENGINE::XACTENGINE_E_INVALIDSOUNDOFFSETORINDEX => "Invalid sound offset or index.",
            XACTENGINE::XACTENGINE_E_READFILE => "Error reading a file.",
            XACTENGINE::XACTENGINE_E_UNKNOWNEVENT => "Unknown event type.",
            XACTENGINE::XACTENGINE_E_INCALLBACK => "Invalid call of a method or function from callback.",
            XACTENGINE::XACTENGINE_E_NOWAVEBANK => "No wavebank exists for the specified operation.",
            XACTENGINE::XACTENGINE_E_SELECTVARIATION => "Unable to select a variation.",
            XACTENGINE::XACTENGINE_E_MULTIPLEAUDITIONENGINES => "Attempted to create more that one audition engine.",
            XACTENGINE::XACTENGINE_E_WAVEBANKNOTPREPARED => "The wavebank is not prepared.",
            XACTENGINE::XACTENGINE_E_NORENDERER => "No audio device can be found.",
            XACTENGINE::XACTENGINE_E_INVALIDENTRYCOUNT => "Invalid entry count for channel maps.",
            XACTENGINE::XACTENGINE_E_AUDITION_WRITEFILE => "An error occurred writing a file during auditioning.",
            XACTENGINE::XACTENGINE_E_AUDITION_NOSOUNDBANK => "Missing a sound bank.",
            XACTENGINE::XACTENGINE_E_AUDITION_INVALIDRPCINDEX => "Missing an RPC curve.",
            XACTENGINE::XACTENGINE_E_AUDITION_MISSINGDATA => "Missing data for an audition command.",
            XACTENGINE::XACTENGINE_E_AUDITION_UNKNOWNCOMMAND => "Unknown command.",
            XACTENGINE::XACTENGINE_E_AUDITION_INVALIDDSPINDEX => "Missing a DSP parameter.",
        }
    }
}
