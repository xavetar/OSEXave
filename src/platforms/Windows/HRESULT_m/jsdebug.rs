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

pub enum JsDEBUG {
    E_JsDEBUG_MISMATCHED_RUNTIME = 0x8DC70001,
    E_JsDEBUG_UNKNOWN_THREAD = 0x8DC70002,
    E_JsDEBUG_OUTSIDE_OF_VM = 0x8DC70004,
    E_JsDEBUG_INVALID_MEMORY_ADDRESS = 0x8DC70005,
    E_JsDEBUG_SOURCE_LOCATION_NOT_FOUND = 0x8DC70006,
    E_JsDEBUG_RUNTIME_NOT_IN_DEBUG_MODE = 0x8DC70007,
}

impl JsDEBUG {
    pub fn description(&self) -> &'static str {
        match self {
            JsDEBUG::E_JsDEBUG_MISMATCHED_RUNTIME => "The Js runtime and the Js diag do not match.",
            JsDEBUG::E_JsDEBUG_UNKNOWN_THREAD => "Thread is not known to have any JS code, and will have no frames.",
            JsDEBUG::E_JsDEBUG_OUTSIDE_OF_VM => "Frame is outside of the interpreter. For example, portions of the Js dll which are logically not part of the interpreter.",
            JsDEBUG::E_JsDEBUG_INVALID_MEMORY_ADDRESS => "Specified memory address could not be written/read from",
            JsDEBUG::E_JsDEBUG_SOURCE_LOCATION_NOT_FOUND => "No source location found to bind the breakpoint",
            JsDEBUG::E_JsDEBUG_RUNTIME_NOT_IN_DEBUG_MODE => "Runtime not in debug mode",
        }
    }
}
