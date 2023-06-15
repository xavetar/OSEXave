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

pub enum STATEREPOSITORY {
    STATEREPOSITORY_TRANSACTION_CALLER_ID_CHANGED = 0x00670013,
    STATEREPOSITORY_E_CONCURRENCY_LOCKING_FAILURE = 0x80670001,
    STATEREPOSITORY_E_STATEMENT_INPROGRESS = 0x80670002,
    STATEREPOSITORY_E_CONFIGURATION_INVALID = 0x80670003,
    STATEREPOSITORY_E_UNKNOWN_SCHEMA_VERSION = 0x80670004,
    STATEREPOSITORY_ERROR_DICTIONARY_CORRUPTED = 0x80670005,
    STATEREPOSITORY_E_BLOCKED = 0x80670006,
    STATEREPOSITORY_E_BUSY_RETRY = 0x80670007,
    STATEREPOSITORY_E_BUSY_RECOVERY_RETRY = 0x80670008,
    STATEREPOSITORY_E_LOCKED_RETRY = 0x80670009,
    STATEREPOSITORY_E_LOCKED_SHAREDCACHE_RETRY = 0x8067000A,
    STATEREPOSITORY_E_TRANSACTION_REQUIRED = 0x8067000B,
    STATEREPOSITORY_E_BUSY_TIMEOUT_EXCEEDED = 0x8067000C,
    STATEREPOSITORY_E_BUSY_RECOVERY_TIMEOUT_EXCEEDED = 0x8067000D,
    STATEREPOSITORY_E_LOCKED_TIMEOUT_EXCEEDED = 0x8067000E,
    STATEREPOSITORY_E_LOCKED_SHAREDCACHE_TIMEOUT_EXCEEDED = 0x8067000F,
    STATEREPOSITORY_E_SERVICE_STOP_IN_PROGRESS = 0x80670010,
    STATEREPOSTORY_E_NESTED_TRANSACTION_NOT_SUPPORTED = 0x80670011,
    STATEREPOSITORY_ERROR_CACHE_CORRUPTED = 0x80670012,
    STATEREPOSITORY_TRANSACTION_IN_PROGRESS = 0x80670014,
    STATEREPOSITORY_E_CACHE_NOT_INIITALIZED = 0x80670015,
    STATEREPOSITORY_E_DEPENDENCY_NOT_RESOLVED = 0x80670016,
}

impl STATEREPOSITORY {
    pub fn description(&self) -> &'static str {
        match self {
            STATEREPOSITORY::STATEREPOSITORY_TRANSACTION_CALLER_ID_CHANGED => "The transaction caller id has changed.",
            STATEREPOSITORY::STATEREPOSITORY_E_CONCURRENCY_LOCKING_FAILURE => "Optimistic locking failure. Data cannot be updated if it has changed since it was read.",
            STATEREPOSITORY::STATEREPOSITORY_E_STATEMENT_INPROGRESS => "A prepared statement has been stepped at least once but not run to completion or reset. This may result in busy waits.",
            STATEREPOSITORY::STATEREPOSITORY_E_CONFIGURATION_INVALID => "The StateRepository configuration is not valid.",
            STATEREPOSITORY::STATEREPOSITORY_E_UNKNOWN_SCHEMA_VERSION => "The StateRepository schema version is not known.",
            STATEREPOSITORY::STATEREPOSITORY_ERROR_DICTIONARY_CORRUPTED => "A StateRepository dictionary is not valid.",
            STATEREPOSITORY::STATEREPOSITORY_E_BLOCKED => "The request failed because the StateRepository is actively blocking requests.",
            STATEREPOSITORY::STATEREPOSITORY_E_BUSY_RETRY => "The database file is locked. The request will be retried.",
            STATEREPOSITORY::STATEREPOSITORY_E_BUSY_RECOVERY_RETRY => "The database file is locked because another process is busy recovering the database. The request will be retried.",
            STATEREPOSITORY::STATEREPOSITORY_E_LOCKED_RETRY => "A table in the database is locked. The request will be retried.",
            STATEREPOSITORY::STATEREPOSITORY_E_LOCKED_SHAREDCACHE_RETRY => "The shared cache for the database is locked by another connection. The request will be retried.",
            STATEREPOSITORY::STATEREPOSITORY_E_TRANSACTION_REQUIRED => "A transaction is required to perform the request operation.",
            STATEREPOSITORY::STATEREPOSITORY_E_BUSY_TIMEOUT_EXCEEDED => "The database file is locked. The request has exceeded the allowed threshold.",
            STATEREPOSITORY::STATEREPOSITORY_E_BUSY_RECOVERY_TIMEOUT_EXCEEDED => "The database file is locked because another process is busy recovering the database. The request has exceeded the allowed threshold.",
            STATEREPOSITORY::STATEREPOSITORY_E_LOCKED_TIMEOUT_EXCEEDED => "A table in the database is locked. The request has exceeded the allowed threshold.",
            STATEREPOSITORY::STATEREPOSITORY_E_LOCKED_SHAREDCACHE_TIMEOUT_EXCEEDED => "The shared cache for the database is locked by another connection. The request has exceeded the allowed threshold.",
            STATEREPOSITORY::STATEREPOSITORY_E_SERVICE_STOP_IN_PROGRESS => "The StateRepository service Stop event is in progress.",
            STATEREPOSITORY::STATEREPOSTORY_E_NESTED_TRANSACTION_NOT_SUPPORTED => "Nested transactions are not supported.",
            STATEREPOSITORY::STATEREPOSITORY_ERROR_CACHE_CORRUPTED => "The StateRepository cache is not valid.",
            STATEREPOSITORY::STATEREPOSITORY_TRANSACTION_IN_PROGRESS => "A transaction is in progress for the database connection.",
            STATEREPOSITORY::STATEREPOSITORY_E_CACHE_NOT_INIITALIZED => "The StateRepository cache is not initialized.",
            STATEREPOSITORY::STATEREPOSITORY_E_DEPENDENCY_NOT_RESOLVED => "Package dependency criteria could not be resolved.",
        }
    }
}
