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

pub enum JSCRIPT {
    JSCRIPT_E_CANTEXECUTE = 0x89020001,
}

impl JSCRIPT {
    pub fn description(&self) -> &'static str {
        match self {
            JSCRIPT::JSCRIPT_E_CANTEXECUTE => "Function could not execute because it was deleted or garbage collected.",
        }
    }
}
