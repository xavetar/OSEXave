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

#[derive(Clone, Debug, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OSError {
    ERROR_INVALID_FUNCTION = 1,
    ERROR_FILE_NOT_FOUND = 2,
    ERROR_PATH_NOT_FOUND = 3,
    ERROR_TOO_MANY_OPEN_FILES = 4,
    ERROR_ACCESS_DENIED = 5,
    ERROR_INVALID_HANDLE = 6,
    ERROR_ARENA_TRASHED = 7,
    ERROR_NOT_ENOUGH_MEMORY = 8,
    ERROR_INVALID_BLOCK = 9,
    ERROR_BAD_ENVIRONMENT = 10,
    ERROR_BAD_FORMAT = 11,
    ERROR_INVALID_ACCESS = 12,
    ERROR_INVALID_DATA = 13,
    ERROR_OUTOFMEMORY = 14,
    ERROR_INVALID_DRIVE = 15,
    ERROR_CURRENT_DIRECTORY = 16,
    ERROR_NOT_SAME_DEVICE = 17,
    ERROR_NO_MORE_FILES = 18,
    ERROR_WRITE_PROTECT = 19,
    ERROR_BAD_UNIT = 20,
    ERROR_NOT_READY = 21,
    ERROR_BAD_COMMAND = 22,
    ERROR_CRC = 23,
    ERROR_BAD_LENGTH = 24,
    ERROR_SEEK = 25,
    ERROR_NOT_DOS_DISK = 26,
    ERROR_SECTOR_NOT_FOUND = 27,
    ERROR_OUT_OF_PAPER = 28,
    ERROR_WRITE_FAULT = 29,
    ERROR_READ_FAULT = 30,
    ERROR_GEN_FAILURE = 31,
    ERROR_SHARING_VIOLATION = 32,
    ERROR_LOCK_VIOLATION = 33,
    ERROR_WRONG_DISK = 34,
    ERROR_SHARING_BUFFER_EXCEEDED = 36,
    ERROR_HANDLE_EOF = 38,
    ERROR_HANDLE_DISK_FULL = 39,
    ERROR_INVALID_PARAMETER = 87,
    ERROR_DISK_FULL = 112,
    ERROR_INVALID_NAME = 123,
    ERROR_NO_SYSTEM_RESOURCES = 1450,
}

impl OSError {
    pub fn code(&self) -> u32 {
        match self {
            OSError::ERROR_INVALID_FUNCTION => OSError::ERROR_INVALID_FUNCTION as u32,
            OSError::ERROR_FILE_NOT_FOUND => OSError::ERROR_FILE_NOT_FOUND as u32,
            OSError::ERROR_PATH_NOT_FOUND => OSError::ERROR_PATH_NOT_FOUND as u32,
            OSError::ERROR_TOO_MANY_OPEN_FILES => OSError::ERROR_TOO_MANY_OPEN_FILES as u32,
            OSError::ERROR_ACCESS_DENIED => OSError::ERROR_ACCESS_DENIED as u32,
            OSError::ERROR_INVALID_HANDLE => OSError::ERROR_INVALID_HANDLE as u32,
            OSError::ERROR_ARENA_TRASHED => OSError::ERROR_ARENA_TRASHED as u32,
            OSError::ERROR_NOT_ENOUGH_MEMORY => OSError::ERROR_NOT_ENOUGH_MEMORY as u32,
            OSError::ERROR_INVALID_BLOCK => OSError::ERROR_INVALID_BLOCK as u32,
            OSError::ERROR_BAD_ENVIRONMENT => OSError::ERROR_BAD_ENVIRONMENT as u32,
            OSError::ERROR_BAD_FORMAT => OSError::ERROR_BAD_FORMAT as u32,
            OSError::ERROR_INVALID_ACCESS => OSError::ERROR_INVALID_ACCESS as u32,
            OSError::ERROR_INVALID_DATA => OSError::ERROR_INVALID_DATA as u32,
            OSError::ERROR_OUTOFMEMORY => OSError::ERROR_OUTOFMEMORY as u32,
            OSError::ERROR_INVALID_DRIVE => OSError::ERROR_INVALID_DRIVE as u32,
            OSError::ERROR_CURRENT_DIRECTORY => OSError::ERROR_CURRENT_DIRECTORY as u32,
            OSError::ERROR_NOT_SAME_DEVICE => OSError::ERROR_NOT_SAME_DEVICE as u32,
            OSError::ERROR_NO_MORE_FILES => OSError::ERROR_NO_MORE_FILES as u32,
            OSError::ERROR_WRITE_PROTECT => OSError::ERROR_WRITE_PROTECT as u32,
            OSError::ERROR_BAD_UNIT => OSError::ERROR_BAD_UNIT as u32,
            OSError::ERROR_NOT_READY => OSError::ERROR_NOT_READY as u32,
            OSError::ERROR_BAD_COMMAND => OSError::ERROR_BAD_COMMAND as u32,
            OSError::ERROR_CRC => OSError::ERROR_CRC as u32,
            OSError::ERROR_BAD_LENGTH => OSError::ERROR_BAD_LENGTH as u32,
            OSError::ERROR_SEEK => OSError::ERROR_SEEK as u32,
            OSError::ERROR_NOT_DOS_DISK => OSError::ERROR_NOT_DOS_DISK as u32,
            OSError::ERROR_SECTOR_NOT_FOUND => OSError::ERROR_SECTOR_NOT_FOUND as u32,
            OSError::ERROR_OUT_OF_PAPER => OSError::ERROR_OUT_OF_PAPER as u32,
            OSError::ERROR_WRITE_FAULT => OSError::ERROR_WRITE_FAULT as u32,
            OSError::ERROR_READ_FAULT => OSError::ERROR_READ_FAULT as u32,
            OSError::ERROR_GEN_FAILURE => OSError::ERROR_GEN_FAILURE as u32,
            OSError::ERROR_SHARING_VIOLATION => OSError::ERROR_SHARING_VIOLATION as u32,
            OSError::ERROR_LOCK_VIOLATION => OSError::ERROR_LOCK_VIOLATION as u32,
            OSError::ERROR_WRONG_DISK => OSError::ERROR_WRONG_DISK as u32,
            OSError::ERROR_SHARING_BUFFER_EXCEEDED => OSError::ERROR_SHARING_BUFFER_EXCEEDED as u32,
            OSError::ERROR_HANDLE_EOF => OSError::ERROR_HANDLE_EOF as u32,
            OSError::ERROR_HANDLE_DISK_FULL => OSError::ERROR_HANDLE_DISK_FULL as u32,
            OSError::ERROR_INVALID_PARAMETER => OSError::ERROR_INVALID_PARAMETER as u32,
            OSError::ERROR_DISK_FULL => OSError::ERROR_DISK_FULL as u32,
            OSError::ERROR_INVALID_NAME => OSError::ERROR_INVALID_NAME as u32,
            OSError::ERROR_NO_SYSTEM_RESOURCES => OSError::ERROR_NO_SYSTEM_RESOURCES as u32,
            _ => panic!("Invalid OSError kind! (Windows)")
        }
    }

    pub fn kind(&self) -> OSError {
        match self {
            OSError::ERROR_INVALID_FUNCTION => OSError::ERROR_INVALID_FUNCTION,
            OSError::ERROR_FILE_NOT_FOUND => OSError::ERROR_FILE_NOT_FOUND,
            OSError::ERROR_PATH_NOT_FOUND => OSError::ERROR_PATH_NOT_FOUND,
            OSError::ERROR_TOO_MANY_OPEN_FILES => OSError::ERROR_TOO_MANY_OPEN_FILES,
            OSError::ERROR_ACCESS_DENIED => OSError::ERROR_ACCESS_DENIED,
            OSError::ERROR_INVALID_HANDLE => OSError::ERROR_INVALID_HANDLE,
            OSError::ERROR_ARENA_TRASHED => OSError::ERROR_ARENA_TRASHED,
            OSError::ERROR_NOT_ENOUGH_MEMORY => OSError::ERROR_NOT_ENOUGH_MEMORY,
            OSError::ERROR_INVALID_BLOCK => OSError::ERROR_INVALID_BLOCK,
            OSError::ERROR_BAD_ENVIRONMENT => OSError::ERROR_BAD_ENVIRONMENT,
            OSError::ERROR_BAD_FORMAT => OSError::ERROR_BAD_FORMAT,
            OSError::ERROR_INVALID_ACCESS => OSError::ERROR_INVALID_ACCESS,
            OSError::ERROR_INVALID_DATA => OSError::ERROR_INVALID_DATA,
            OSError::ERROR_OUTOFMEMORY => OSError::ERROR_OUTOFMEMORY,
            OSError::ERROR_INVALID_DRIVE => OSError::ERROR_INVALID_DRIVE,
            OSError::ERROR_CURRENT_DIRECTORY => OSError::ERROR_CURRENT_DIRECTORY,
            OSError::ERROR_NOT_SAME_DEVICE => OSError::ERROR_NOT_SAME_DEVICE,
            OSError::ERROR_NO_MORE_FILES => OSError::ERROR_NO_MORE_FILES,
            OSError::ERROR_WRITE_PROTECT => OSError::ERROR_WRITE_PROTECT,
            OSError::ERROR_BAD_UNIT => OSError::ERROR_BAD_UNIT,
            OSError::ERROR_NOT_READY => OSError::ERROR_NOT_READY,
            OSError::ERROR_BAD_COMMAND => OSError::ERROR_BAD_COMMAND,
            OSError::ERROR_CRC => OSError::ERROR_CRC,
            OSError::ERROR_BAD_LENGTH => OSError::ERROR_BAD_LENGTH,
            OSError::ERROR_SEEK => OSError::ERROR_SEEK,
            OSError::ERROR_NOT_DOS_DISK => OSError::ERROR_NOT_DOS_DISK,
            OSError::ERROR_SECTOR_NOT_FOUND => OSError::ERROR_SECTOR_NOT_FOUND,
            OSError::ERROR_OUT_OF_PAPER => OSError::ERROR_OUT_OF_PAPER,
            OSError::ERROR_WRITE_FAULT => OSError::ERROR_WRITE_FAULT,
            OSError::ERROR_READ_FAULT => OSError::ERROR_READ_FAULT,
            OSError::ERROR_GEN_FAILURE => OSError::ERROR_GEN_FAILURE,
            OSError::ERROR_SHARING_VIOLATION => OSError::ERROR_SHARING_VIOLATION,
            OSError::ERROR_LOCK_VIOLATION => OSError::ERROR_LOCK_VIOLATION,
            OSError::ERROR_WRONG_DISK => OSError::ERROR_WRONG_DISK,
            OSError::ERROR_SHARING_BUFFER_EXCEEDED => OSError::ERROR_SHARING_BUFFER_EXCEEDED,
            OSError::ERROR_HANDLE_EOF => OSError::ERROR_HANDLE_EOF,
            OSError::ERROR_HANDLE_DISK_FULL => OSError::ERROR_HANDLE_DISK_FULL,
            OSError::ERROR_INVALID_PARAMETER => OSError::ERROR_INVALID_PARAMETER,
            OSError::ERROR_DISK_FULL => OSError::ERROR_DISK_FULL,
            OSError::ERROR_INVALID_NAME => OSError::ERROR_INVALID_NAME,
            OSError::ERROR_NO_SYSTEM_RESOURCES => OSError::ERROR_NO_SYSTEM_RESOURCES,
            _ => panic!("Invalid OSError kind! (MacOS)")
        }
    }

    pub fn description(&self) -> &'static str {
        return match self {
            OSError::ERROR_INVALID_FUNCTION => "Incorrect function.",
            OSError::ERROR_FILE_NOT_FOUND => "The system cannot find the file specified.",
            OSError::ERROR_PATH_NOT_FOUND => "The system cannot find the path specified.",
            OSError::ERROR_TOO_MANY_OPEN_FILES => "The system cannot open the file.",
            OSError::ERROR_ACCESS_DENIED => "Access is denied.",
            OSError::ERROR_INVALID_HANDLE => "The handle is invalid.",
            OSError::ERROR_ARENA_TRASHED => "The storage control blocks were destroyed.",
            OSError::ERROR_NOT_ENOUGH_MEMORY => "Not enough storage is available to process this command.",
            OSError::ERROR_INVALID_BLOCK => "The storage control block address is invalid.",
            OSError::ERROR_BAD_ENVIRONMENT => "The environment is incorrect.",
            OSError::ERROR_BAD_FORMAT => "An attempt was made to load a program with an incorrect format.",
            OSError::ERROR_INVALID_ACCESS => "The access code is invalid.",
            OSError::ERROR_INVALID_DATA => "The data is invalid.",
            OSError::ERROR_OUTOFMEMORY => "Not enough storage is available to complete this operation.",
            OSError::ERROR_INVALID_DRIVE => "The system cannot find the drive specified.",
            OSError::ERROR_CURRENT_DIRECTORY => "The directory cannot be removed.",
            OSError::ERROR_NOT_SAME_DEVICE => "The system cannot move the file to a different disk drive.",
            OSError::ERROR_NO_MORE_FILES => "There are no more files.",
            OSError::ERROR_WRITE_PROTECT => "The media is write protected.",
            OSError::ERROR_BAD_UNIT => "The system cannot find the device specified.",
            OSError::ERROR_NOT_READY => "The device is not ready.",
            OSError::ERROR_BAD_COMMAND => "The device does not recognize the command.",
            OSError::ERROR_CRC => "Data error (cyclic redundancy check).",
            OSError::ERROR_BAD_LENGTH => "The program issued a command but the command length is incorrect.",
            OSError::ERROR_SEEK => "The drive cannot locate a specific area or track on the disk.",
            OSError::ERROR_NOT_DOS_DISK => "The specified disk or diskette cannot be accessed.",
            OSError::ERROR_SECTOR_NOT_FOUND => "The drive cannot find the sector requested.",
            OSError::ERROR_OUT_OF_PAPER => "The printer is out of paper.",
            OSError::ERROR_WRITE_FAULT => "The system cannot write to the specified device.",
            OSError::ERROR_READ_FAULT => "The system cannot read from the specified device.",
            OSError::ERROR_GEN_FAILURE => "A device attached to the system is not functioning.",
            OSError::ERROR_SHARING_VIOLATION => "The process cannot access the file because it is being used by another process.",
            OSError::ERROR_LOCK_VIOLATION => "The process cannot access the file because another process has locked a portion of the file.",
            OSError::ERROR_WRONG_DISK => "The wrong diskette is in the drive. Insert %2 (Volume Serial Number: %3) into drive %1.",
            OSError::ERROR_SHARING_BUFFER_EXCEEDED => "Too many files opened for sharing.",
            OSError::ERROR_HANDLE_EOF => "Reached the end of the file.",
            OSError::ERROR_HANDLE_DISK_FULL => "The disk is full.",
            OSError::ERROR_INVALID_PARAMETER => "The parameter is incorrect.",
            OSError::ERROR_DISK_FULL => "The disk is full.",
            OSError::ERROR_INVALID_NAME => "The file name, directory name, or volume label syntax is incorrect.",
            OSError::ERROR_NO_SYSTEM_RESOURCES => "Insufficient system resources exist to complete the requested service.",
            _ => panic!("Invalid OSError kind! (Windows)")
        }
    }

    pub fn kind_from_name(name: &str) -> OSError {
        match name {
            "ERROR_INVALID_FUNCTION" => OSError::ERROR_INVALID_FUNCTION,
            "ERROR_FILE_NOT_FOUND" => OSError::ERROR_FILE_NOT_FOUND,
            "ERROR_PATH_NOT_FOUND" => OSError::ERROR_PATH_NOT_FOUND,
            "ERROR_TOO_MANY_OPEN_FILES" => OSError::ERROR_TOO_MANY_OPEN_FILES,
            "ERROR_ACCESS_DENIED" => OSError::ERROR_ACCESS_DENIED,
            "ERROR_INVALID_HANDLE" => OSError::ERROR_INVALID_HANDLE,
            "ERROR_ARENA_TRASHED" => OSError::ERROR_ARENA_TRASHED,
            "ERROR_NOT_ENOUGH_MEMORY" => OSError::ERROR_NOT_ENOUGH_MEMORY,
            "ERROR_INVALID_BLOCK" => OSError::ERROR_INVALID_BLOCK,
            "ERROR_BAD_ENVIRONMENT" => OSError::ERROR_BAD_ENVIRONMENT,
            "ERROR_BAD_FORMAT" => OSError::ERROR_BAD_FORMAT,
            "ERROR_INVALID_ACCESS" => OSError::ERROR_INVALID_ACCESS,
            "ERROR_INVALID_DATA" => OSError::ERROR_INVALID_DATA,
            "ERROR_OUTOFMEMORY" => OSError::ERROR_OUTOFMEMORY,
            "ERROR_INVALID_DRIVE" => OSError::ERROR_INVALID_DRIVE,
            "ERROR_CURRENT_DIRECTORY" => OSError::ERROR_CURRENT_DIRECTORY,
            "ERROR_NOT_SAME_DEVICE" => OSError::ERROR_NOT_SAME_DEVICE,
            "ERROR_NO_MORE_FILES" => OSError::ERROR_NO_MORE_FILES,
            "ERROR_WRITE_PROTECT" => OSError::ERROR_WRITE_PROTECT,
            "ERROR_BAD_UNIT" => OSError::ERROR_BAD_UNIT,
            "ERROR_NOT_READY" => OSError::ERROR_NOT_READY,
            "ERROR_BAD_COMMAND" => OSError::ERROR_BAD_COMMAND,
            "ERROR_CRC" => OSError::ERROR_CRC,
            "ERROR_BAD_LENGTH" => OSError::ERROR_BAD_LENGTH,
            "ERROR_SEEK" => OSError::ERROR_SEEK,
            "ERROR_NOT_DOS_DISK" => OSError::ERROR_NOT_DOS_DISK,
            "ERROR_SECTOR_NOT_FOUND" => OSError::ERROR_SECTOR_NOT_FOUND,
            "ERROR_OUT_OF_PAPER" => OSError::ERROR_OUT_OF_PAPER,
            "ERROR_WRITE_FAULT" => OSError::ERROR_WRITE_FAULT,
            "ERROR_READ_FAULT" => OSError::ERROR_READ_FAULT,
            "ERROR_GEN_FAILURE" => OSError::ERROR_GEN_FAILURE,
            "ERROR_SHARING_VIOLATION" => OSError::ERROR_SHARING_VIOLATION,
            "ERROR_LOCK_VIOLATION" => OSError::ERROR_LOCK_VIOLATION,
            "ERROR_WRONG_DISK" => OSError::ERROR_WRONG_DISK,
            "ERROR_SHARING_BUFFER_EXCEEDED" => OSError::ERROR_SHARING_BUFFER_EXCEEDED,
            "ERROR_HANDLE_EOF" => OSError::ERROR_HANDLE_EOF,
            "ERROR_HANDLE_DISK_FULL" => OSError::ERROR_HANDLE_DISK_FULL,
            "ERROR_INVALID_PARAMETER" => OSError::ERROR_INVALID_PARAMETER,
            "ERROR_DISK_FULL" => OSError::ERROR_DISK_FULL,
            "ERROR_INVALID_NAME" => OSError::ERROR_INVALID_NAME,
            "ERROR_NO_SYSTEM_RESOURCES" => OSError::ERROR_NO_SYSTEM_RESOURCES,
            _ => panic!("Invalid OSError name: {}! (Windows)", name)
        }
    }

    pub fn kind_from_code(code: &u32) -> OSError {
        match code {
            1 => OSError::ERROR_INVALID_FUNCTION,
            2 => OSError::ERROR_FILE_NOT_FOUND,
            3 => OSError::ERROR_PATH_NOT_FOUND,
            4 => OSError::ERROR_TOO_MANY_OPEN_FILES,
            5 => OSError::ERROR_ACCESS_DENIED,
            6 => OSError::ERROR_INVALID_HANDLE,
            7 => OSError::ERROR_ARENA_TRASHED,
            8 => OSError::ERROR_NOT_ENOUGH_MEMORY,
            9 => OSError::ERROR_INVALID_BLOCK,
            10 => OSError::ERROR_BAD_ENVIRONMENT,
            11 => OSError::ERROR_BAD_FORMAT,
            12 => OSError::ERROR_INVALID_ACCESS,
            13 => OSError::ERROR_INVALID_DATA,
            14 => OSError::ERROR_OUTOFMEMORY,
            15 => OSError::ERROR_INVALID_DRIVE,
            16 => OSError::ERROR_CURRENT_DIRECTORY,
            17 => OSError::ERROR_NOT_SAME_DEVICE,
            18 => OSError::ERROR_NO_MORE_FILES,
            19 => OSError::ERROR_WRITE_PROTECT,
            20 => OSError::ERROR_BAD_UNIT,
            21 => OSError::ERROR_NOT_READY,
            22 => OSError::ERROR_BAD_COMMAND,
            23 => OSError::ERROR_CRC,
            24 => OSError::ERROR_BAD_LENGTH,
            25 => OSError::ERROR_SEEK,
            26 => OSError::ERROR_NOT_DOS_DISK,
            27 => OSError::ERROR_SECTOR_NOT_FOUND,
            28 => OSError::ERROR_OUT_OF_PAPER,
            29 => OSError::ERROR_WRITE_FAULT,
            30 => OSError::ERROR_READ_FAULT,
            31 => OSError::ERROR_GEN_FAILURE,
            32 => OSError::ERROR_SHARING_VIOLATION,
            33 => OSError::ERROR_LOCK_VIOLATION,
            34 => OSError::ERROR_WRONG_DISK,
            36 => OSError::ERROR_SHARING_BUFFER_EXCEEDED,
            38 => OSError::ERROR_HANDLE_EOF,
            39 => OSError::ERROR_HANDLE_DISK_FULL,
            87 => OSError::ERROR_INVALID_PARAMETER,
            112 => OSError::ERROR_DISK_FULL,
            123 => OSError::ERROR_INVALID_NAME,
            1450 => OSError::ERROR_NO_SYSTEM_RESOURCES,
            _ => panic!("Invalid OSError code: {}! (Windows)", code)
        }
    }
}