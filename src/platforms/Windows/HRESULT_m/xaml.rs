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
pub enum XAML {
    E_XAMLPARSEFAILED,
    E_LAYOUTCYCLE,
    E_ELEMENTNOTENABLED,
    E_ELEMENTNOTAVAILABLE,
    E_UNKNOWNTYPE,
}

impl XAML {
    pub fn code(&self) -> u32 {
        return match self {
            XAML::E_XAMLPARSEFAILED => 0x802B000A as u32,
            XAML::E_LAYOUTCYCLE => 0x802B0014 as u32,
            XAML::E_ELEMENTNOTENABLED => 0x802B001E as u32,
            XAML::E_ELEMENTNOTAVAILABLE => 0x802B001F as u32,
            XAML::E_UNKNOWNTYPE => 0x802B0028 as u32,
        }
    }

    pub fn error(&self) -> RawError {
        return match self {
            XAML::E_XAMLPARSEFAILED => RawError::Kind(XAML::E_XAMLPARSEFAILED),
            XAML::E_LAYOUTCYCLE => RawError::Kind(XAML::E_LAYOUTCYCLE),
            XAML::E_ELEMENTNOTENABLED => RawError::Kind(XAML::E_ELEMENTNOTENABLED),
            XAML::E_ELEMENTNOTAVAILABLE => RawError::Kind(XAML::E_ELEMENTNOTAVAILABLE),
            XAML::E_UNKNOWNTYPE => RawError::Kind(XAML::E_UNKNOWNTYPE),
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            XAML::E_XAMLPARSEFAILED => "XAML parsing failed.",
            XAML::E_LAYOUTCYCLE => "Layout cycle detected. Layout could not complete.",
            XAML::E_ELEMENTNOTENABLED => "The operation is not allowed on a nonenabled element.",
            XAML::E_ELEMENTNOTAVAILABLE => "Element not available.",
            XAML::E_UNKNOWNTYPE => "Unknown Type.",
        }
    }

    pub fn from_name(name: &str) -> XAML {
        return match name {
            "E_XAMLPARSEFAILED" => XAML::E_XAMLPARSEFAILED,
            "E_LAYOUTCYCLE" => XAML::E_LAYOUTCYCLE,
            "E_ELEMENTNOTENABLED" => XAML::E_ELEMENTNOTENABLED,
            "E_ELEMENTNOTAVAILABLE" => XAML::E_ELEMENTNOTAVAILABLE,
            "E_UNKNOWNTYPE" => XAML::E_UNKNOWNTYPE,
        }
    }
    pub fn from_code(code: u32) -> XAML {
        return match code {
            0x802B000A => XAML::E_XAMLPARSEFAILED,
            0x802B0014 => XAML::E_LAYOUTCYCLE,
            0x802B001E => XAML::E_ELEMENTNOTENABLED,
            0x802B001F => XAML::E_ELEMENTNOTAVAILABLE,
            0x802B0028 => XAML::E_UNKNOWNTYPE,
        }
    }
}
