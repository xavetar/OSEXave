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

pub enum XAML {
    E_XAMLPARSEFAILED = 0x802B000A,
    E_LAYOUTCYCLE = 0x802B0014,
    E_ELEMENTNOTENABLED = 0x802B001E,
    E_ELEMENTNOTAVAILABLE = 0x802B001F,
    E_UNKNOWNTYPE = 0x802B0028,
}

impl XAML {
    pub fn description(&self) -> &'static str {
        match self {
            XAML::E_XAMLPARSEFAILED => "XAML parsing failed.",
            XAML::E_LAYOUTCYCLE => "Layout cycle detected. Layout could not complete.",
            XAML::E_ELEMENTNOTENABLED => "The operation is not allowed on a nonenabled element.",
            XAML::E_ELEMENTNOTAVAILABLE => "Element not available.",
            XAML::E_UNKNOWNTYPE => "Unknown Type.",
        }
    }
}
