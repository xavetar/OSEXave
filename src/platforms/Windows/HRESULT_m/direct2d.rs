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

pub enum DIRECT2D {
    D2DERR_WRONG_STATE = 0x88990001,
    D2DERR_NOT_INITIALIZED = 0x88990002,
    D2DERR_UNSUPPORTED_OPERATION = 0x88990003,
    D2DERR_SCANNER_FAILED = 0x88990004,
    D2DERR_SCREEN_ACCESS_DENIED = 0x88990005,
    D2DERR_DISPLAY_STATE_INVALID = 0x88990006,
    D2DERR_ZERO_VECTOR = 0x88990007,
    D2DERR_INTERNAL_ERROR = 0x88990008,
    D2DERR_DISPLAY_FORMAT_NOT_SUPPORTED = 0x88990009,
    D2DERR_INVALID_CALL = 0x8899000A,
    D2DERR_NO_HARDWARE_DEVICE = 0x8899000B,
    D2DERR_RECREATE_TARGET = 0x8899000C,
    D2DERR_TOO_MANY_SHADER_ELEMENTS = 0x8899000D,
    D2DERR_SHADER_COMPILE_FAILED = 0x8899000E,
    D2DERR_MAX_TEXTURE_SIZE_EXCEEDED = 0x8899000F,
    D2DERR_UNSUPPORTED_VERSION = 0x88990010,
    D2DERR_BAD_NUMBER = 0x88990011,
    D2DERR_WRONG_FACTORY = 0x88990012,
    D2DERR_LAYER_ALREADY_IN_USE = 0x88990013,
    D2DERR_POP_CALL_DID_NOT_MATCH_PUSH = 0x88990014,
    D2DERR_WRONG_RESOURCE_DOMAIN = 0x88990015,
    D2DERR_PUSH_POP_UNBALANCED = 0x88990016,
    D2DERR_RENDER_TARGET_HAS_LAYER_OR_CLIPRECT = 0x88990017,
    D2DERR_INCOMPATIBLE_BRUSH_TYPES = 0x88990018,
    D2DERR_WIN32_ERROR = 0x88990019,
    D2DERR_TARGET_NOT_GDI_COMPATIBLE = 0x8899001A,
    D2DERR_TEXT_EFFECT_IS_WRONG_TYPE = 0x8899001B,
    D2DERR_TEXT_RENDERER_NOT_RELEASED = 0x8899001C,
    D2DERR_EXCEEDS_MAX_BITMAP_SIZE = 0x8899001D,
    D2DERR_INVALID_GRAPH_CONFIGURATION = 0x8899001E,
    D2DERR_INVALID_INTERNAL_GRAPH_CONFIGURATION = 0x8899001F,
    D2DERR_CYCLIC_GRAPH = 0x88990020,
    D2DERR_BITMAP_CANNOT_DRAW = 0x88990021,
    D2DERR_OUTSTANDING_BITMAP_REFERENCES = 0x88990022,
    D2DERR_ORIGINAL_TARGET_NOT_BOUND = 0x88990023,
    D2DERR_INVALID_TARGET = 0x88990024,
    D2DERR_BITMAP_BOUND_AS_TARGET = 0x88990025,
    D2DERR_INSUFFICIENT_DEVICE_CAPABILITIES = 0x88990026,
    D2DERR_INTERMEDIATE_TOO_LARGE = 0x88990027,
    D2DERR_EFFECT_IS_NOT_REGISTERED = 0x88990028,
    D2DERR_INVALID_PROPERTY = 0x88990029,
    D2DERR_NO_SUBPROPERTIES = 0x8899002A,
    D2DERR_PRINT_JOB_CLOSED = 0x8899002B,
    D2DERR_PRINT_FORMAT_NOT_SUPPORTED = 0x8899002C,
    D2DERR_TOO_MANY_TRANSFORM_INPUTS = 0x8899002D,
    D2DERR_INVALID_GLYPH_IMAGE = 0x8899002E,
}

impl DIRECT2D {
    pub fn description(&self) -> &'static str {
        match self {
            DIRECT2D::D2DERR_WRONG_STATE => "The object was not in the correct state to process the method.",
            DIRECT2D::D2DERR_NOT_INITIALIZED => "The object has not yet been initialized.",
            DIRECT2D::D2DERR_UNSUPPORTED_OPERATION => "The requested operation is not supported.",
            DIRECT2D::D2DERR_SCANNER_FAILED => "The geometry scanner failed to process the data.",
            DIRECT2D::D2DERR_SCREEN_ACCESS_DENIED => "Direct2D could not access the screen.",
            DIRECT2D::D2DERR_DISPLAY_STATE_INVALID => "A valid display state could not be determined.",
            DIRECT2D::D2DERR_ZERO_VECTOR => "The supplied vector is zero.",
            DIRECT2D::D2DERR_INTERNAL_ERROR => "An internal error (Direct2D bug) occurred. On checked builds, we would assert. The application should close this instance of Direct2D and should consider restarting its process.",
            DIRECT2D::D2DERR_DISPLAY_FORMAT_NOT_SUPPORTED => "The display format Direct2D needs to render is not supported by the hardware device.",
            DIRECT2D::D2DERR_INVALID_CALL => "A call to this method is invalid.",
            DIRECT2D::D2DERR_NO_HARDWARE_DEVICE => "No hardware rendering device is available for this operation.",
            DIRECT2D::D2DERR_RECREATE_TARGET => "There has been a presentation error that may be recoverable. The caller needs to recreate, rerender the entire frame, and reattempt present.",
            DIRECT2D::D2DERR_TOO_MANY_SHADER_ELEMENTS => "Shader construction failed because it was too complex.",
            DIRECT2D::D2DERR_SHADER_COMPILE_FAILED => "Shader compilation failed.",
            DIRECT2D::D2DERR_MAX_TEXTURE_SIZE_EXCEEDED => "Requested DirectX surface size exceeded maximum texture size.",
            DIRECT2D::D2DERR_UNSUPPORTED_VERSION => "The requested Direct2D version is not supported.",
            DIRECT2D::D2DERR_BAD_NUMBER => "Invalid number.",
            DIRECT2D::D2DERR_WRONG_FACTORY => "Objects used together must be created from the same factory instance.",
            DIRECT2D::D2DERR_LAYER_ALREADY_IN_USE => "A layer resource can only be in use once at any point in time.",
            DIRECT2D::D2DERR_POP_CALL_DID_NOT_MATCH_PUSH => "The pop call did not match the corresponding push call.",
            DIRECT2D::D2DERR_WRONG_RESOURCE_DOMAIN => "The resource was realized on the wrong render target.",
            DIRECT2D::D2DERR_PUSH_POP_UNBALANCED => "The push and pop calls were unbalanced.",
            DIRECT2D::D2DERR_RENDER_TARGET_HAS_LAYER_OR_CLIPRECT => "Attempt to copy from a render target while a layer or clip rect is applied.",
            DIRECT2D::D2DERR_INCOMPATIBLE_BRUSH_TYPES => "The brush types are incompatible for the call.",
            DIRECT2D::D2DERR_WIN32_ERROR => "An unknown win32 failure occurred.",
            DIRECT2D::D2DERR_TARGET_NOT_GDI_COMPATIBLE => "The render target is not compatible with GDI.",
            DIRECT2D::D2DERR_TEXT_EFFECT_IS_WRONG_TYPE => "A text client drawing effect object is of the wrong type.",
            DIRECT2D::D2DERR_TEXT_RENDERER_NOT_RELEASED => "The application is holding a reference to the IDWriteTextRenderer interface after the corresponding DrawText or DrawTextLayout call has returned. The IDWriteTextRenderer instance will be invalid.",
            DIRECT2D::D2DERR_EXCEEDS_MAX_BITMAP_SIZE => "The requested size is larger than the guaranteed supported texture size at the Direct3D device's current feature level.",
            DIRECT2D::D2DERR_INVALID_GRAPH_CONFIGURATION => "There was a configuration error in the graph.",
            DIRECT2D::D2DERR_INVALID_INTERNAL_GRAPH_CONFIGURATION => "There was a internal configuration error in the graph.",
            DIRECT2D::D2DERR_CYCLIC_GRAPH => "There was a cycle in the graph.",
            DIRECT2D::D2DERR_BITMAP_CANNOT_DRAW => "Cannot draw with a bitmap that has the D2D1_BITMAP_OPTIONS_CANNOT_DRAW option.",
            DIRECT2D::D2DERR_OUTSTANDING_BITMAP_REFERENCES => "The operation cannot complete while there are outstanding references to the target bitmap.",
            DIRECT2D::D2DERR_ORIGINAL_TARGET_NOT_BOUND => "The operation failed because the original target is not currently bound as a target.",
            DIRECT2D::D2DERR_INVALID_TARGET => "Cannot set the image as a target because it is either an effect or is a bitmap that does not have the D2D1_BITMAP_OPTIONS_TARGET flag set.",
            DIRECT2D::D2DERR_BITMAP_BOUND_AS_TARGET => "Cannot draw with a bitmap that is currently bound as the target bitmap.",
            DIRECT2D::D2DERR_INSUFFICIENT_DEVICE_CAPABILITIES => "D3D Device does not have sufficient capabilities to perform the requested action.",
            DIRECT2D::D2DERR_INTERMEDIATE_TOO_LARGE => "The graph could not be rendered with the context's current tiling settings.",
            DIRECT2D::D2DERR_EFFECT_IS_NOT_REGISTERED => "The CLSID provided to Unregister did not correspond to a registered effect.",
            DIRECT2D::D2DERR_INVALID_PROPERTY => "The specified property does not exist.",
            DIRECT2D::D2DERR_NO_SUBPROPERTIES => "The specified sub-property does not exist.",
            DIRECT2D::D2DERR_PRINT_JOB_CLOSED => "AddPage or Close called after print job is already closed.",
            DIRECT2D::D2DERR_PRINT_FORMAT_NOT_SUPPORTED => "Error during print control creation. Indicates that none of the package target types (representing printer formats) are supported by Direct2D print control.",
            DIRECT2D::D2DERR_TOO_MANY_TRANSFORM_INPUTS => "An effect attempted to use a transform with too many inputs.",
            DIRECT2D::D2DERR_INVALID_GLYPH_IMAGE => "An error was encountered while decoding or parsing the requested glyph image.",
        }
    }
}
