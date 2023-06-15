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

pub enum BCD {
    ERROR_BCD_NOT_ALL_ENTRIES_IMPORTED = 0x80390001,
    ERROR_BCD_NOT_ALL_ENTRIES_SYNCHRONIZED = 0x80390003,
    ERROR_BCD_TOO_MANY_ELEMENTS = 0xC0390002,
}

impl BCD {
    pub fn description(&self) -> &'static str {
        match self {
            BCD::ERROR_BCD_NOT_ALL_ENTRIES_IMPORTED => "Some BCD entries were not imported correctly from the BCD store.",
            BCD::ERROR_BCD_NOT_ALL_ENTRIES_SYNCHRONIZED => "Some BCD entries were not synchronized correctly with the firmware.",
            BCD::ERROR_BCD_TOO_MANY_ELEMENTS => "Entries enumerated have exceeded the allowed threshold.",
        }
    }
}
