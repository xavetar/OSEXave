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

pub enum RDDB {
    E_RDDB_LOGIN_FAILED = 0x88250001,
    E_RDDB_DATA_INTEGRITY_VIOLATION = 0x88250002,
    E_RDDB_FATAL_ERR = 0x88250003,
    E_RDDB_DB_OFFLINE = 0x88250004,
    E_RDDB_INVALID_ARG = 0x88250005,
    E_RDDB_SYNCH_CONFLICT = 0x88250006,
    E_RDDB_VM_ALREADY_ASSIGNED = 0x88250007,
    E_RDDB_USER_ALREADY_ASSIGNED = 0x88250008,
    E_RDDB_USER_CONN_PENDING = 0x88250009,
    E_RDDB_TOO_MANY_RECORDS = 0x8825000A,
    E_RDDB_NO_RESOURCE_AVAILABLE = 0x8825000B,
    E_RDDB_TARGET_ENDPOINT_DOWN = 0x8825000C,
    E_RDDB_UNKNOWN_ERR = 0x8825000D,
    E_RDDB_ALREADY_EXISTS = 0x8825000E,
    E_RDDB_KEYVALUE_MISMATCH = 0x8825000F,
}

impl RDDB {
    pub fn description(&self) -> &'static str {
        match self {
            RDDB::E_RDDB_LOGIN_FAILED => "User failed to logon to the DB.",
            RDDB::E_RDDB_DATA_INTEGRITY_VIOLATION => "DB operation failed due to violation of data integrity constraint.",
            RDDB::E_RDDB_FATAL_ERR => "DB internal error.",
            RDDB::E_RDDB_DB_OFFLINE => "DB is offline.",
            RDDB::E_RDDB_INVALID_ARG => "Invalid argument was supplied.",
            RDDB::E_RDDB_SYNCH_CONFLICT => "This error is returned to RD Connection Broker runtime, when there is a synch conflict. RD Connection Broker is expected to re-read the record from the DB & call update with latest record.",
            RDDB::E_RDDB_VM_ALREADY_ASSIGNED => "This VM is already assigned to a another user.",
            RDDB::E_RDDB_USER_ALREADY_ASSIGNED => "This user has already been assigned another VM.",
            RDDB::E_RDDB_USER_CONN_PENDING => "This error is returned to RD Connection Broker runtime, when the same user connects to the same farm more than once simultaneously.",
            RDDB::E_RDDB_TOO_MANY_RECORDS => "Number of records in the DB exceeds the maximum limit allowed for.",
            RDDB::E_RDDB_NO_RESOURCE_AVAILABLE => "No more resource available in Pool for connection.",
            RDDB::E_RDDB_TARGET_ENDPOINT_DOWN => "Target endpoint is not running currently.",
            RDDB::E_RDDB_UNKNOWN_ERR => "Generic database operation error.",
            RDDB::E_RDDB_ALREADY_EXISTS => "Value already exists.",
            RDDB::E_RDDB_KEYVALUE_MISMATCH => "Value mismatched.",
        }
    }
}
