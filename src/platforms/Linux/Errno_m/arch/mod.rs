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

#[cfg(any(all(not(target_arch = "alpha"), not(target_arch = "mips"), not(target_arch = "parisc"),
              not(target_arch = "parisc2.0"), not(target_arch = "powerpc"),
              not(target_arch = "powerpc64"), not(target_arch = "sparc"),
              not(target_arch = "sparc64"), not(target_arch = "x86"))))]
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64", target_arch = "arm",
          target_arch = "arm64"))]
#[cfg(target_os = "linux")]
mod base;
#[cfg(target_arch = "alpha")]
#[cfg(target_os = "linux")]
mod alpha;
#[cfg(target_arch = "mips")]
#[cfg(target_os = "linux")]
mod mips;
#[cfg(any(target_arch = "parisc", target_arch = "parisc2.0"))]
#[cfg(target_os = "linux")]
mod parisc;
#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
#[cfg(target_os = "linux")]
mod powerpc;
#[cfg(any(target_arch = "sparc", target_arch = "sparc64"))]
#[cfg(target_os = "linux")]
mod sparc;


macro_rules! import_linux_oserror {
    ($arch:ident) => {
        pub use self::$arch::OSError;
    };
}

#[cfg(any(all(not(target_arch = "alpha"), not(target_arch = "mips"), not(target_arch = "parisc"),
              not(target_arch = "parisc2.0"), not(target_arch = "powerpc"),
              not(target_arch = "powerpc64"), not(target_arch = "sparc"),
              not(target_arch = "sparc64"), not(target_arch = "x86"))))]
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64", target_arch = "arm",
target_arch = "arm64"))]
#[cfg(target_os = "linux")]
import_linux_oserror!(base);
#[cfg(target_arch = "alpha")]
#[cfg(target_os = "linux")]
import_linux_oserror!(alpha);
#[cfg(target_arch = "mips")]
#[cfg(target_os = "linux")]
import_linux_oserror!(mips);
#[cfg(any(target_arch = "parisc", target_arch = "parisc2.0"))]
#[cfg(target_os = "linux")]
import_linux_oserror!(parisc);
#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
#[cfg(target_os = "linux")]
import_linux_oserror!(powerpc);
#[cfg(any(target_arch = "sparc", target_arch = "sparc64"))]
#[cfg(target_os = "linux")]
import_linux_oserror!(sparc);
