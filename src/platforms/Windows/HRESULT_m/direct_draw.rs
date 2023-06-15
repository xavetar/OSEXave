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

pub enum DIRECT_DRAW {
    DDERR_ALREADYINITIALIZED = 0x88760005,
    DDERR_CANNOTATTACHSURFACE = 0x8876000A,
    DDERR_CANNOTDETACHSURFACE = 0x88760014,
    DDERR_CURRENTLYNOTAVAIL = 0x88760028,
    DDERR_EXCEPTION = 0x88760037,
    DDERR_HEIGHTALIGN = 0x8876005A,
    DDERR_INCOMPATIBLEPRIMARY = 0x8876005F,
    DDERR_INVALIDCAPS = 0x88760064,
    DDERR_INVALIDCLIPLIST = 0x8876006E,
    DDERR_INVALIDMODE = 0x88760078,
    DDERR_INVALIDOBJECT = 0x88760082,
    DDERR_INVALIDPIXELFORMAT = 0x88760091,
    DDERR_INVALIDRECT = 0x88760096,
    DDERR_LOCKEDSURFACES = 0x887600A0,
    DDERR_NO3D = 0x887600AA,
    DDERR_NOALPHAHW = 0x887600B4,
    DDERR_NOSTEREOHARDWARE = 0x887600B5,
    DDERR_NOSURFACELEFT = 0x887600B6,
    DDERR_NOCLIPLIST = 0x887600CD,
    DDERR_NOCOLORCONVHW = 0x887600D2,
    DDERR_NOCOOPERATIVELEVELSET = 0x887600D4,
    DDERR_NOCOLORKEY = 0x887600D7,
    DDERR_NOCOLORKEYHW = 0x887600DC,
    DDERR_NODIRECTDRAWSUPPORT = 0x887600DE,
    DDERR_NOEXCLUSIVEMODE = 0x887600E1,
    DDERR_NOFLIPHW = 0x887600E6,
    DDERR_NOGDI = 0x887600F0,
    DDERR_NOMIRRORHW = 0x887600FA,
    DDERR_NOTFOUND = 0x887600FF,
    DDERR_NOOVERLAYHW = 0x88760104,
    DDERR_OVERLAPPINGRECTS = 0x8876010E,
    DDERR_NORASTEROPHW = 0x88760118,
    DDERR_NOROTATIONHW = 0x88760122,
    DDERR_NOSTRETCHHW = 0x88760136,
    DDERR_NOT4BITCOLOR = 0x8876013C,
    DDERR_NOT4BITCOLORINDEX = 0x8876013D,
    DDERR_NOT8BITCOLOR = 0x88760140,
    DDERR_NOTEXTUREHW = 0x8876014A,
    DDERR_NOVSYNCHW = 0x8876014F,
    DDERR_NOZBUFFERHW = 0x88760154,
    DDERR_NOZOVERLAYHW = 0x8876015E,
    DDERR_OUTOFCAPS = 0x88760168,
    DDERR_OUTOFVIDEOMEMORY = 0x8876017C,
    DDERR_OVERLAYCANTCLIP = 0x8876017E,
    DDERR_OVERLAYCOLORKEYONLYONEACTIVE = 0x88760180,
    DDERR_PALETTEBUSY = 0x88760183,
    DDERR_COLORKEYNOTSET = 0x88760190,
    DDERR_SURFACEALREADYATTACHED = 0x8876019A,
    DDERR_SURFACEALREADYDEPENDENT = 0x887601A4,
    DDERR_SURFACEBUSY = 0x887601AE,
    DDERR_CANTLOCKSURFACE = 0x887601B3,
    DDERR_SURFACEISOBSCURED = 0x887601B8,
    DDERR_SURFACELOST = 0x887601C2,
    DDERR_SURFACENOTATTACHED = 0x887601CC,
    DDERR_TOOBIGHEIGHT = 0x887601D6,
    DDERR_TOOBIGSIZE = 0x887601E0,
    DDERR_TOOBIGWIDTH = 0x887601EA,
    DDERR_UNSUPPORTEDFORMAT = 0x887601FE,
    DDERR_UNSUPPORTEDMASK = 0x88760208,
    DDERR_INVALIDSTREAM = 0x88760209,
    DDERR_VERTICALBLANKINPROGRESS = 0x88760219,
    DDERR_WASSTILLDRAWING = 0x8876021C,
    DDERR_DDSCAPSCOMPLEXREQUIRED = 0x8876021E,
    DDERR_XALIGN = 0x88760230,
    DDERR_INVALIDDIRECTDRAWGUID = 0x88760231,
    DDERR_DIRECTDRAWALREADYCREATED = 0x88760232,
    DDERR_NODIRECTDRAWHW = 0x88760233,
    DDERR_PRIMARYSURFACEALREADYEXISTS = 0x88760234,
    DDERR_NOEMULATION = 0x88760235,
    DDERR_REGIONTOOSMALL = 0x88760236,
    DDERR_CLIPPERISUSINGHWND = 0x88760237,
    DDERR_NOCLIPPERATTACHED = 0x88760238,
    DDERR_NOHWND = 0x88760239,
    DDERR_HWNDSUBCLASSED = 0x8876023A,
    DDERR_HWNDALREADYSET = 0x8876023B,
    DDERR_NOPALETTEATTACHED = 0x8876023C,
    DDERR_NOPALETTEHW = 0x8876023D,
    DDERR_BLTFASTCANTCLIP = 0x8876023E,
    DDERR_NOBLTHW = 0x8876023F,
    DDERR_NODDROPSHW = 0x88760240,
    DDERR_OVERLAYNOTVISIBLE = 0x88760241,
    DDERR_NOOVERLAYDEST = 0x88760242,
    DDERR_INVALIDPOSITION = 0x88760243,
    DDERR_NOTAOVERLAYSURFACE = 0x88760244,
    DDERR_EXCLUSIVEMODEALREADYSET = 0x88760245,
    DDERR_NOTFLIPPABLE = 0x88760246,
    DDERR_CANTDUPLICATE = 0x88760247,
    DDERR_NOTLOCKED = 0x88760248,
    DDERR_CANTCREATEDC = 0x88760249,
    DDERR_NODC = 0x8876024A,
    DDERR_WRONGMODE = 0x8876024B,
    DDERR_IMPLICITLYCREATED = 0x8876024C,
    DDERR_NOTPALETTIZED = 0x8876024D,
    DDERR_UNSUPPORTEDMODE = 0x8876024E,
    DDERR_NOMIPMAPHW = 0x8876024F,
    DDERR_INVALIDSURFACETYPE = 0x88760250,
    DDERR_NOOPTIMIZEHW = 0x88760258,
    DDERR_NOTLOADED = 0x88760259,
    DDERR_NOFOCUSWINDOW = 0x8876025A,
    DDERR_NOTONMIPMAPSUBLEVEL = 0x8876025B,
    DDERR_DCALREADYCREATED = 0x8876026C,
    DDERR_NONONLOCALVIDMEM = 0x88760276,
    DDERR_CANTPAGELOCK = 0x88760280,
    DDERR_CANTPAGEUNLOCK = 0x88760294,
    DDERR_NOTPAGELOCKED = 0x887602A8,
    DDERR_MOREDATA = 0x887602B2,
    DDERR_EXPIRED = 0x887602B3,
    DDERR_TESTFINISHED = 0x887602B4,
    DDERR_NEWMODE = 0x887602B5,
    DDERR_D3DNOTINITIALIZED = 0x887602B6,
    DDERR_VIDEONOTACTIVE = 0x887602B7,
    DDERR_NOMONITORINFORMATION = 0x887602B8,
    DDERR_NODRIVERSUPPORT = 0x887602B9,
    DDERR_DEVICEDOESNTOWNSURFACE = 0x887602BB,
}

impl DIRECT_DRAW {
    pub fn description(&self) -> &'static str {
        match self {
            DIRECT_DRAW::DDERR_ALREADYINITIALIZED => "The object has already been initialized.",
            DIRECT_DRAW::DDERR_CANNOTATTACHSURFACE => "A surface cannot be attached to another requested surface.",
            DIRECT_DRAW::DDERR_CANNOTDETACHSURFACE => "A surface cannot be detached from another requested surface.",
            DIRECT_DRAW::DDERR_CURRENTLYNOTAVAIL => "No support is currently available.",
            DIRECT_DRAW::DDERR_EXCEPTION => "An exception was encountered while performing the requested operation.",
            DIRECT_DRAW::DDERR_HEIGHTALIGN => "The height of the provided rectangle is not a multiple of the required alignment.",
            DIRECT_DRAW::DDERR_INCOMPATIBLEPRIMARY => "The primary surface creation request does not match the existing primary surface.",
            DIRECT_DRAW::DDERR_INVALIDCAPS => "One or more of the capability bits passed to the callback function are incorrect.",
            DIRECT_DRAW::DDERR_INVALIDCLIPLIST => "DirectDraw does not support the provided clip list.",
            DIRECT_DRAW::DDERR_INVALIDMODE => "DirectDraw does not support the requested mode.",
            DIRECT_DRAW::DDERR_INVALIDOBJECT => "DirectDraw received a pointer that was an invalid DirectDraw object.",
            DIRECT_DRAW::DDERR_INVALIDPIXELFORMAT => "The pixel format was invalid as specified.",
            DIRECT_DRAW::DDERR_INVALIDRECT => "The provided rectangle was invalid.",
            DIRECT_DRAW::DDERR_LOCKEDSURFACES => "One or more surfaces are locked, causing the failure of the requested operation.",
            DIRECT_DRAW::DDERR_NO3D => "No 3-D hardware or emulation is present.",
            DIRECT_DRAW::DDERR_NOALPHAHW => "No alpha-acceleration hardware is present or available, causing the failure of the requested operation.",
            DIRECT_DRAW::DDERR_NOSTEREOHARDWARE => "There is no stereo hardware present or available.",
            DIRECT_DRAW::DDERR_NOSURFACELEFT => "There is no hardware present that supports stereo surfaces.",
            DIRECT_DRAW::DDERR_NOCLIPLIST => "No clip list is available.",
            DIRECT_DRAW::DDERR_NOCOLORCONVHW => "No color-conversion hardware is present or available.",
            DIRECT_DRAW::DDERR_NOCOOPERATIVELEVELSET => "A create function was called without the IDirectDraw7::SetCooperativeLevel method.",
            DIRECT_DRAW::DDERR_NOCOLORKEY => "The surface does not currently have a color key.",
            DIRECT_DRAW::DDERR_NOCOLORKEYHW => "There is no hardware support for the destination color key.",
            DIRECT_DRAW::DDERR_NODIRECTDRAWSUPPORT => "DirectDraw support is not possible with the current display driver.",
            DIRECT_DRAW::DDERR_NOEXCLUSIVEMODE => "The operation requires the application to have exclusive mode, but the application does not have exclusive mode.",
            DIRECT_DRAW::DDERR_NOFLIPHW => "Flipping visible surfaces is not supported.",
            DIRECT_DRAW::DDERR_NOGDI => "No GDI is present.",
            DIRECT_DRAW::DDERR_NOMIRRORHW => "No mirroring hardware is present or available.",
            DIRECT_DRAW::DDERR_NOTFOUND => "The requested item was not found.",
            DIRECT_DRAW::DDERR_NOOVERLAYHW => "No overlay hardware is present or available.",
            DIRECT_DRAW::DDERR_OVERLAPPINGRECTS => "The source and destination rectangles are on the same surface and overlap each other.",
            DIRECT_DRAW::DDERR_NORASTEROPHW => "No appropriate raster-operation hardware is present or available.",
            DIRECT_DRAW::DDERR_NOROTATIONHW => "No rotation hardware is present or available.",
            DIRECT_DRAW::DDERR_NOSTRETCHHW => "There is no hardware support for stretching.",
            DIRECT_DRAW::DDERR_NOT4BITCOLOR => "The DirectDrawSurface object is not using a 4-bit color palette, and the requested operation requires a 4-bit color palette.",
            DIRECT_DRAW::DDERR_NOT4BITCOLORINDEX => "The DirectDrawSurface object is not using a 4-bit color index palette, and the requested operation requires a 4-bit color index palette.",
            DIRECT_DRAW::DDERR_NOT8BITCOLOR => "The DirectDrawSurface object is not using an 8-bit color palette, and the requested operation requires an 8-bit color palette.",
            DIRECT_DRAW::DDERR_NOTEXTUREHW => "The operation cannot be carried out because no texture-mapping hardware is present or available.",
            DIRECT_DRAW::DDERR_NOVSYNCHW => "There is no hardware support for vertical blank&#x2013;synchronized operations.",
            DIRECT_DRAW::DDERR_NOZBUFFERHW => "The operation to create a z-buffer in display memory or to perform a bit block transfer (bitblt), using a z-buffer cannot be carried out because there is no hardware support for z-buffers.",
            DIRECT_DRAW::DDERR_NOZOVERLAYHW => "The overlay surfaces cannot be z-layered, based on the z-order because the hardware does not support z-ordering of overlays.",
            DIRECT_DRAW::DDERR_OUTOFCAPS => "The hardware needed for the requested operation has already been allocated.",
            DIRECT_DRAW::DDERR_OUTOFVIDEOMEMORY => "DirectDraw does not have enough display memory to perform the operation.",
            DIRECT_DRAW::DDERR_OVERLAYCANTCLIP => "The hardware does not support clipped overlays.",
            DIRECT_DRAW::DDERR_OVERLAYCOLORKEYONLYONEACTIVE => "An attempt was made to have more than one color key active on an overlay.",
            DIRECT_DRAW::DDERR_PALETTEBUSY => "Access to this palette is refused because the palette is locked by another thread.",
            DIRECT_DRAW::DDERR_COLORKEYNOTSET => "No source color key is specified for this operation.",
            DIRECT_DRAW::DDERR_SURFACEALREADYATTACHED => "An attempt was made to attach a surface to another surface to which it is already attached.",
            DIRECT_DRAW::DDERR_SURFACEALREADYDEPENDENT => "An attempt was made to make a surface a dependency of another surface on which it is already dependent.",
            DIRECT_DRAW::DDERR_SURFACEBUSY => "Access to the surface is refused because the surface is locked by another thread.",
            DIRECT_DRAW::DDERR_CANTLOCKSURFACE => "Access to this surface is refused because an attempt was made to lock the primary surface without Display Control Interface (DCI) support.",
            DIRECT_DRAW::DDERR_SURFACEISOBSCURED => "Access to the surface is refused because the surface is obscured.",
            DIRECT_DRAW::DDERR_SURFACELOST => "Access to the surface is refused because the surface memory is gone. Call the IDirectDrawSurface7::Restore method on this surface to restore the memory associated with it.",
            DIRECT_DRAW::DDERR_SURFACENOTATTACHED => "The requested surface is not attached.",
            DIRECT_DRAW::DDERR_TOOBIGHEIGHT => "The height requested by DirectDraw is too large.",
            DIRECT_DRAW::DDERR_TOOBIGSIZE => "The size requested by DirectDraw is too large. However, the individual height and width are valid sizes.",
            DIRECT_DRAW::DDERR_TOOBIGWIDTH => "The width requested by DirectDraw is too large.",
            DIRECT_DRAW::DDERR_UNSUPPORTEDFORMAT => "The pixel format requested is not supported by DirectDraw.",
            DIRECT_DRAW::DDERR_UNSUPPORTEDMASK => "The bitmask in the pixel format requested is not supported by DirectDraw.",
            DIRECT_DRAW::DDERR_INVALIDSTREAM => "The specified stream contains invalid data.",
            DIRECT_DRAW::DDERR_VERTICALBLANKINPROGRESS => "A vertical blank is in progress.",
            DIRECT_DRAW::DDERR_WASSTILLDRAWING => "The previous bitblt operation that is transferring information to or from this surface is incomplete.",
            DIRECT_DRAW::DDERR_DDSCAPSCOMPLEXREQUIRED => "New for DirectX 7.0. The surface requires the DDSCAPS_COMPLEX flag.",
            DIRECT_DRAW::DDERR_XALIGN => "The provided rectangle was not horizontally aligned on a required boundary.",
            DIRECT_DRAW::DDERR_INVALIDDIRECTDRAWGUID => "The globally unique identifier (GUID) passed to the DirectDrawCreate function is not a valid DirectDraw driver identifier.",
            DIRECT_DRAW::DDERR_DIRECTDRAWALREADYCREATED => "A DirectDraw object representing this driver has already been created for this process.",
            DIRECT_DRAW::DDERR_NODIRECTDRAWHW => "Hardware-only DirectDraw object creation is not possible; the driver does not support any hardware.",
            DIRECT_DRAW::DDERR_PRIMARYSURFACEALREADYEXISTS => "This process has already created a primary surface.",
            DIRECT_DRAW::DDERR_NOEMULATION => "Software emulation is not available.",
            DIRECT_DRAW::DDERR_REGIONTOOSMALL => "The region passed to the IDirectDrawClipper::GetClipList method is too small.",
            DIRECT_DRAW::DDERR_CLIPPERISUSINGHWND => "An attempt was made to set a clip list for a DirectDrawClipper object that is already monitoring a window handle.",
            DIRECT_DRAW::DDERR_NOCLIPPERATTACHED => "No DirectDrawClipper object is attached to the surface object.",
            DIRECT_DRAW::DDERR_NOHWND => "Clipper notification requires a window handle, or no window handle has been previously set as the cooperative level window handle.",
            DIRECT_DRAW::DDERR_HWNDSUBCLASSED => "DirectDraw is prevented from restoring state because the DirectDraw cooperative-level window handle has been subclassed.",
            DIRECT_DRAW::DDERR_HWNDALREADYSET => "The DirectDraw cooperative-level window handle has already been set. It cannot be reset while the process has surfaces or palettes created.",
            DIRECT_DRAW::DDERR_NOPALETTEATTACHED => "No palette object is attached to this surface.",
            DIRECT_DRAW::DDERR_NOPALETTEHW => "There is no hardware support for 16- or 256-color palettes.",
            DIRECT_DRAW::DDERR_BLTFASTCANTCLIP => "A DirectDrawClipper object is attached to a source surface that has passed into a call to the IDirectDrawSurface7::BltFast method.",
            DIRECT_DRAW::DDERR_NOBLTHW => "No bit block transferring hardware is present.",
            DIRECT_DRAW::DDERR_NODDROPSHW => "No DirectDraw raster-operation (ROP) hardware is available.",
            DIRECT_DRAW::DDERR_OVERLAYNOTVISIBLE => "The IDirectDrawSurface7::GetOverlayPosition method was called on a hidden overlay.",
            DIRECT_DRAW::DDERR_NOOVERLAYDEST => "The IDirectDrawSurface7::GetOverlayPosition method is called on an overlay that the IDirectDrawSurface7::UpdateOverlay method has not been called on to establish as a destination.",
            DIRECT_DRAW::DDERR_INVALIDPOSITION => "The position of the overlay on the destination is no longer valid.",
            DIRECT_DRAW::DDERR_NOTAOVERLAYSURFACE => "An overlay component is called for a nonoverlay surface.",
            DIRECT_DRAW::DDERR_EXCLUSIVEMODEALREADYSET => "An attempt was made to set the cooperative level when it was already set to exclusive.",
            DIRECT_DRAW::DDERR_NOTFLIPPABLE => "An attempt was made to flip a surface that cannot be flipped.",
            DIRECT_DRAW::DDERR_CANTDUPLICATE => "Primary and 3-D surfaces, or surfaces that are implicitly created, cannot be duplicated.",
            DIRECT_DRAW::DDERR_NOTLOCKED => "An attempt was made to unlock a surface that was not locked.",
            DIRECT_DRAW::DDERR_CANTCREATEDC => "Windows cannot create any more device contexts (DCs), or a DC has requested a palette-indexed surface when the surface had no palette and the display mode was not palette-indexed (in this case, DirectDraw cannot select a proper palette into the DC).",
            DIRECT_DRAW::DDERR_NODC => "No device context (DC) has ever been created for this surface.",
            DIRECT_DRAW::DDERR_WRONGMODE => "This surface cannot be restored because it was created in a different mode.",
            DIRECT_DRAW::DDERR_IMPLICITLYCREATED => "The surface cannot be restored because it is an implicitly created surface.",
            DIRECT_DRAW::DDERR_NOTPALETTIZED => "The surface being used is not a palette-based surface.",
            DIRECT_DRAW::DDERR_UNSUPPORTEDMODE => "The display is currently in an unsupported mode.",
            DIRECT_DRAW::DDERR_NOMIPMAPHW => "No mipmap-capable texture mapping hardware is present or available.",
            DIRECT_DRAW::DDERR_INVALIDSURFACETYPE => "The surface was of the wrong type.",
            DIRECT_DRAW::DDERR_NOOPTIMIZEHW => "The device does not support optimized surfaces.",
            DIRECT_DRAW::DDERR_NOTLOADED => "The surface is an optimized surface, but it has not yet been allocated any memory.",
            DIRECT_DRAW::DDERR_NOFOCUSWINDOW => "An attempt was made to create or set a device window without first setting the focus window.",
            DIRECT_DRAW::DDERR_NOTONMIPMAPSUBLEVEL => "Attempt was made to set a palette on a mipmap sublevel",
            DIRECT_DRAW::DDERR_DCALREADYCREATED => "A device context (DC) has already been returned for this surface. Only one DC can be retrieved for each surface.",
            DIRECT_DRAW::DDERR_NONONLOCALVIDMEM => "An attempt was made to allocate nonlocal video memory from a device that does not support nonlocal video memory.",
            DIRECT_DRAW::DDERR_CANTPAGELOCK => "An attempt to page-lock a surface failed. Page lock does not work on a display-memory surface or an emulated primary surface.",
            DIRECT_DRAW::DDERR_CANTPAGEUNLOCK => "An attempt to page-unlock a surface failed. Page unlock does not work on a display-memory surface or an emulated primary surface.",
            DIRECT_DRAW::DDERR_NOTPAGELOCKED => "An attempt was made to page-unlock a surface with no outstanding page locks.",
            DIRECT_DRAW::DDERR_MOREDATA => "There is more data available than the specified buffer size can hold.",
            DIRECT_DRAW::DDERR_EXPIRED => "The data has expired and is therefore no longer valid.",
            DIRECT_DRAW::DDERR_TESTFINISHED => "New for DirectX 7.0. When returned by the IDirectDraw7::StartModeTest method, this value means that no test could be initiated because all the resolutions chosen for testing already have refresh rate information in the registry. When returned by IDirectDraw7::EvaluateMode, the value means that DirectDraw has completed a refresh rate test.",
            DIRECT_DRAW::DDERR_NEWMODE => "New for DirectX 7.0. When IDirectDraw7::StartModeTest is called with the DDSMT_ISTESTREQUIRED flag, it might return this value to denote that some or all of the resolutions can and should be tested. IDirectDraw7::EvaluateMode returns this value to indicate that the test has switched to a new display mode.",
            DIRECT_DRAW::DDERR_D3DNOTINITIALIZED => "D3D has not yet been initialized.",
            DIRECT_DRAW::DDERR_VIDEONOTACTIVE => "The video port is not active.",
            DIRECT_DRAW::DDERR_NOMONITORINFORMATION => "New for DirectX 7.0. Testing cannot proceed because the monitor has no associated EDID data.",
            DIRECT_DRAW::DDERR_NODRIVERSUPPORT => "New for DirectX 7.0. Testing cannot proceed because the display adapter driver does not enumerate refresh rates.",
            DIRECT_DRAW::DDERR_DEVICEDOESNTOWNSURFACE => "Surfaces created by one DirectDraw device cannot be used directly by another DirectDraw device.",
        }
    }
}
