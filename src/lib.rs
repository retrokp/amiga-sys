//! Unsafe bindings for the Amiga (m68k) system libraries.
//!
//! Most functions take a pointer to its library as the first parameter.
//! Call `abs_exec_library()` to get a pointer to the Exec library and call `OpenLibrary()`
//! to open other libraries.

#![feature(asm_experimental_arch)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

#[cfg(any(target_arch = "m68k", doc))]
mod bindings;
#[cfg(any(target_arch = "m68k", doc))]
pub use bindings::*;

#[cfg(any(target_arch = "m68k", doc))]
mod constants;
#[cfg(any(target_arch = "m68k", doc))]
pub use constants::*;

/// Returns a pointer to the exec library by reading the memory address 0x00000004.
#[cfg(any(target_arch = "m68k", doc))]
pub unsafe fn abs_exec_library() -> *mut Library {
    unsafe { *(0x00000004 as *mut usize) as *mut Library }
}

// ensure structs have correct sizes: these sizes have been taken from NDK3.1 STRUCTURE.OFFSETS
#[cfg(any(target_arch = "m68k", doc))]
const _: () = {
    ["Size of AChain"][::core::mem::size_of::<AChain>() - 274usize];
    ["Size of AmigaGuideHost"][::core::mem::size_of::<AmigaGuideHost>() - 40usize];
    ["Size of AmigaGuideMsg"][::core::mem::size_of::<AmigaGuideMsg>() - 52usize];
    ["Size of AnalogSignalInterval"][::core::mem::size_of::<AnalogSignalInterval>() - 4usize];
    ["Size of AnchorPath"][::core::mem::size_of::<AnchorPath>() - 282usize];
    ["Size of AnimComp"][::core::mem::size_of::<AnimComp>() - 38usize];
    ["Size of AnimHeader"][::core::mem::size_of::<AnimHeader>() - 40usize];
    ["Size of AnimOb"][::core::mem::size_of::<AnimOb>() - 42usize];
    ["Size of AppIcon"][::core::mem::size_of::<AppIcon>() - 4usize];
    ["Size of AppMenuItem"][::core::mem::size_of::<AppMenuItem>() - 4usize];
    ["Size of AppMessage"][::core::mem::size_of::<AppMessage>() - 86usize];
    ["Size of AppWindow"][::core::mem::size_of::<AppWindow>() - 4usize];
    ["Size of AreaInfo"][::core::mem::size_of::<AreaInfo>() - 24usize];
    ["Size of AssignList"][::core::mem::size_of::<AssignList>() - 8usize];
    //["Size of AudChannel"][::core::mem::size_of::<AudChannel>() - 16usize]; // not found in NDK3.2
    ["Size of AvailFonts"][::core::mem::size_of::<AvailFonts>() - 10usize];
    ["Size of AvailFontsHeader"][::core::mem::size_of::<AvailFontsHeader>() - 2usize];
    ["Size of BadBlockBlock"][::core::mem::size_of::<BadBlockBlock>() - 512usize];
    ["Size of BadBlockEntry"][::core::mem::size_of::<BadBlockEntry>() - 8usize];
    ["Size of BitMap"][::core::mem::size_of::<BitMap>() - 40usize];
    ["Size of BitMapHeader"][::core::mem::size_of::<BitMapHeader>() - 20usize];
    ["Size of BitScaleArgs"][::core::mem::size_of::<BitScaleArgs>() - 48usize];
    ["Size of Bob"][::core::mem::size_of::<Bob>() - 32usize];
    ["Size of BoolInfo"][::core::mem::size_of::<BoolInfo>() - 10usize];
    ["Size of BootBlock"][::core::mem::size_of::<BootBlock>() - 12usize];
    ["Size of BootNode"][::core::mem::size_of::<BootNode>() - 20usize];
    ["Size of Border"][::core::mem::size_of::<Border>() - 16usize];
    ["Size of CDInfo"][::core::mem::size_of::<CDInfo>() - 34usize];
    ["Size of CDXL"][::core::mem::size_of::<CDXL>() - 28usize];
    ["Size of CIA"][::core::mem::size_of::<CIA>() - 3842usize];
    ["Size of CSource"][::core::mem::size_of::<CSource>() - 12usize];
    ["Size of CardHandle"][::core::mem::size_of::<CardHandle>() - 28usize];
    ["Size of CardMemoryMap"][::core::mem::size_of::<CardMemoryMap>() - 24usize];
    ["Size of Catalog"][::core::mem::size_of::<Catalog>() - 28usize];
    ["Size of CliProcList"][::core::mem::size_of::<CliProcList>() - 16usize];
    ["Size of ClipHookMsg"][::core::mem::size_of::<ClipHookMsg>() - 12usize];
    ["Size of ClipRect"][::core::mem::size_of::<ClipRect>() - 36usize];
    ["Size of ClipboardHandle"][::core::mem::size_of::<ClipboardHandle>() - 120usize];
    ["Size of ClipboardUnitPartial"][::core::mem::size_of::<ClipboardUnitPartial>() - 18usize];
    ["Size of ClockData"][::core::mem::size_of::<ClockData>() - 14usize];
    ["Size of CollectionItem"][::core::mem::size_of::<CollectionItem>() - 12usize];
    ["Size of ColorFontColors"][::core::mem::size_of::<ColorFontColors>() - 8usize];
    ["Size of ColorMap"][::core::mem::size_of::<ColorMap>() - 52usize];
    ["Size of ColorRegister"][::core::mem::size_of::<ColorRegister>() - 4usize];
    ["Size of ColorSpec"][::core::mem::size_of::<ColorSpec>() - 8usize];
    ["Size of ColorTextFont"][::core::mem::size_of::<ColorTextFont>() - 96usize];
    ["Size of ColorWheelHSB"][::core::mem::size_of::<ColorWheelHSB>() - 12usize];
    ["Size of ColorWheelRGB"][::core::mem::size_of::<ColorWheelRGB>() - 12usize];
    ["Size of CommandLineInterface"][::core::mem::size_of::<CommandLineInterface>() - 64usize];
    ["Size of ConUnit"][::core::mem::size_of::<ConUnit>() - 296usize];
    ["Size of Conductor"][::core::mem::size_of::<Conductor>() - 54usize];
    ["Size of ConfigDev"][::core::mem::size_of::<ConfigDev>() - 68usize];
    ["Size of ContextNode"][::core::mem::size_of::<ContextNode>() - 24usize];
    ["Size of CopIns"][::core::mem::size_of::<CopIns>() - 6usize];
    ["Size of CopList"][::core::mem::size_of::<CopList>() - 38usize];
    ["Size of CountryPrefs"][::core::mem::size_of::<CountryPrefs>() - 504usize];
    ["Size of CurrentBinding"][::core::mem::size_of::<CurrentBinding>() - 16usize];
    ["Size of Custom"][::core::mem::size_of::<Custom>() - 510usize];
    ["Size of DBufInfo"][::core::mem::size_of::<DBufInfo>() - 84usize];
    ["Size of DBufPacket"][::core::mem::size_of::<DBufPacket>() - 12usize];
    ["Size of DTHookContext"][::core::mem::size_of::<DTHookContext>() - 40usize];
    ["Size of DTMethod"][::core::mem::size_of::<DTMethod>() - 12usize];
    ["Size of DTSpecialInfo"][::core::mem::size_of::<DTSpecialInfo>() - 90usize];
    ["Size of DataType"][::core::mem::size_of::<DataType>() - 58usize];
    ["Size of DataTypeHeader"][::core::mem::size_of::<DataTypeHeader>() - 32usize];
    ["Size of DateStamp"][::core::mem::size_of::<DateStamp>() - 12usize];
    ["Size of DateTime"][::core::mem::size_of::<DateTime>() - 26usize];
    ["Size of DevInfo"][::core::mem::size_of::<DevInfo>() - 44usize];
    ["Size of DevProc"][::core::mem::size_of::<DevProc>() - 16usize];
    ["Size of Device"][::core::mem::size_of::<Device>() - 34usize];
    ["Size of DeviceData"][::core::mem::size_of::<DeviceData>() - 52usize];
    ["Size of DeviceList"][::core::mem::size_of::<DeviceList>() - 44usize];
    ["Size of DeviceNode"][::core::mem::size_of::<DeviceNode>() - 44usize];
    ["Size of DeviceTData"][::core::mem::size_of::<DeviceTData>() - 10usize];
    ["Size of DiagArea"][::core::mem::size_of::<DiagArea>() - 14usize];
    ["Size of DimensionInfo"][::core::mem::size_of::<DimensionInfo>() - 88usize];
    ["Size of DiscResource"][::core::mem::size_of::<DiscResource>() - 148usize];
    ["Size of DiscResourceUnit"][::core::mem::size_of::<DiscResourceUnit>() - 86usize];
    ["Size of DiskFontHeader"][::core::mem::size_of::<DiskFontHeader>() - 106usize];
    ["Size of DiskObject"][::core::mem::size_of::<DiskObject>() - 78usize];
    ["Size of DisplayInfo"][::core::mem::size_of::<DisplayInfo>() - 56usize];
    ["Size of DisplayMode"][::core::mem::size_of::<DisplayMode>() - 106usize];
    ["Size of DosEnvec"][::core::mem::size_of::<DosEnvec>() - 80usize];
    ["Size of DosInfo"][::core::mem::size_of::<DosInfo>() - 158usize];
    ["Size of DosLibrary"][::core::mem::size_of::<DosLibrary>() - 70usize];
    ["Size of DosList"][::core::mem::size_of::<DosList>() - 44usize];
    ["Size of DosPacket"][::core::mem::size_of::<DosPacket>() - 48usize];
    ["Size of DrawInfo"][::core::mem::size_of::<DrawInfo>() - 50usize];
    ["Size of DrawerData"][::core::mem::size_of::<DrawerData>() - 62usize];
    ["Size of DriveGeometry"][::core::mem::size_of::<DriveGeometry>() - 32usize];
    ["Size of EClockVal"][::core::mem::size_of::<EClockVal>() - 8usize];
    ["Size of EasyStruct"][::core::mem::size_of::<EasyStruct>() - 20usize];
    ["Size of ErrorString"][::core::mem::size_of::<ErrorString>() - 8usize];
    ["Size of ExAllControl"][::core::mem::size_of::<ExAllControl>() - 16usize];
    ["Size of ExAllData"][::core::mem::size_of::<ExAllData>() - 40usize];
    ["Size of ExecBase"][::core::mem::size_of::<ExecBase>() - 632usize];
    ["Size of ExpansionBase"][::core::mem::size_of::<ExpansionBase>() - 88usize];
    ["Size of ExpansionControl"][::core::mem::size_of::<ExpansionControl>() - 16usize];
    ["Size of ExpansionRom"][::core::mem::size_of::<ExpansionRom>() - 16usize];
    ["Size of ExtGadget"][::core::mem::size_of::<ExtGadget>() - 56usize];
    ["Size of ExtIntuiMessage"][::core::mem::size_of::<ExtIntuiMessage>() - 56usize];
    ["Size of ExtNewScreen"][::core::mem::size_of::<ExtNewScreen>() - 36usize];
    ["Size of ExtNewWindow"][::core::mem::size_of::<ExtNewWindow>() - 52usize];
    ["Size of ExtSprite"][::core::mem::size_of::<ExtSprite>() - 16usize];
    ["Size of ExtendedNode"][::core::mem::size_of::<ExtendedNode>() - 24usize];
    ["Size of FileHandle"][::core::mem::size_of::<FileHandle>() - 44usize];
    ["Size of FileInfoBlock"][::core::mem::size_of::<FileInfoBlock>() - 260usize];
    ["Size of FileLock"][::core::mem::size_of::<FileLock>() - 20usize];
    ["Size of FileRequester"][::core::mem::size_of::<FileRequester>() - 56usize];
    ["Size of FileSysEntry"][::core::mem::size_of::<FileSysEntry>() - 62usize];
    ["Size of FileSysHeaderBlock"][::core::mem::size_of::<FileSysHeaderBlock>() - 256usize];
    ["Size of FileSysResource"][::core::mem::size_of::<FileSysResource>() - 32usize];
    ["Size of FileSysStartupMsg"][::core::mem::size_of::<FileSysStartupMsg>() - 16usize];
    ["Size of FontContents"][::core::mem::size_of::<FontContents>() - 260usize];
    ["Size of FontContentsHeader"][::core::mem::size_of::<FontContentsHeader>() - 4usize];
    ["Size of FontPrefs"][::core::mem::size_of::<FontPrefs>() - 156usize];
    ["Size of FontRequester"][::core::mem::size_of::<FontRequester>() - 44usize];
    ["Size of FrameInfo"][::core::mem::size_of::<FrameInfo>() - 36usize];
    ["Size of FreeList"][::core::mem::size_of::<FreeList>() - 16usize];
    ["Size of Gadget"][::core::mem::size_of::<Gadget>() - 44usize];
    ["Size of GadgetInfo"][::core::mem::size_of::<GadgetInfo>() - 58usize];
    ["Size of GamePortTrigger"][::core::mem::size_of::<GamePortTrigger>() - 8usize];
    ["Size of GelsInfo"][::core::mem::size_of::<GelsInfo>() - 38usize];
    ["Size of GfxBase"][::core::mem::size_of::<GfxBase>() - 552usize]; // fixed for NDK3.2
    ["Size of GlyphEngine"][::core::mem::size_of::<GlyphEngine>() - 8usize];
    ["Size of GlyphMap"][::core::mem::size_of::<GlyphMap>() - 36usize];
    ["Size of GlyphWidthEntry"][::core::mem::size_of::<GlyphWidthEntry>() - 14usize];
    ["Size of Hook"][::core::mem::size_of::<Hook>() - 20usize];
    ["Size of IBox"][::core::mem::size_of::<IBox>() - 8usize];
    ["Size of IControlPrefs"][::core::mem::size_of::<IControlPrefs>() - 38usize]; // fixed for NDK3.2
    ["Size of IENewTablet"][::core::mem::size_of::<IENewTablet>() - 32usize];
    ["Size of IEPointerPixel"][::core::mem::size_of::<IEPointerPixel>() - 8usize];
    ["Size of IEPointerTablet"][::core::mem::size_of::<IEPointerTablet>() - 10usize];
    ["Size of IFFHandle"][::core::mem::size_of::<IFFHandle>() - 12usize];
    ["Size of IFFStreamCmd"][::core::mem::size_of::<IFFStreamCmd>() - 12usize];
    ["Size of IOAudio"][::core::mem::size_of::<IOAudio>() - 68usize];
    ["Size of IOClipReq"][::core::mem::size_of::<IOClipReq>() - 52usize];
    ["Size of IODRPReq"][::core::mem::size_of::<IODRPReq>() - 62usize];
    ["Size of IOExtPar"][::core::mem::size_of::<IOExtPar>() - 62usize];
    ["Size of IOExtSer"][::core::mem::size_of::<IOExtSer>() - 82usize];
    ["Size of IOExtTD"][::core::mem::size_of::<IOExtTD>() - 56usize];
    ["Size of IOPArray"][::core::mem::size_of::<IOPArray>() - 8usize];
    ["Size of IOPrtCmdReq"][::core::mem::size_of::<IOPrtCmdReq>() - 38usize];
    ["Size of IORequest"][::core::mem::size_of::<IORequest>() - 32usize];
    ["Size of IOStdReq"][::core::mem::size_of::<IOStdReq>() - 48usize];
    ["Size of IOTArray"][::core::mem::size_of::<IOTArray>() - 8usize];
    ["Size of Image"][::core::mem::size_of::<Image>() - 20usize];
    ["Size of InfoData"][::core::mem::size_of::<InfoData>() - 36usize];
    ["Size of InputEvent"][::core::mem::size_of::<InputEvent>() - 22usize];
    ["Size of InputPrefs"][::core::mem::size_of::<InputPrefs>() - 44usize];
    ["Size of InputXpression"][::core::mem::size_of::<InputXpression>() - 12usize];
    ["Size of IntVector"][::core::mem::size_of::<IntVector>() - 12usize];
    ["Size of Interrupt"][::core::mem::size_of::<Interrupt>() - 22usize];
    ["Size of IntuiMessage"][::core::mem::size_of::<IntuiMessage>() - 52usize];
    ["Size of IntuiText"][::core::mem::size_of::<IntuiText>() - 20usize];
    ["Size of IntuitionBase"][::core::mem::size_of::<IntuitionBase>() - 80usize];
    ["Size of IoBuff"][::core::mem::size_of::<IoBuff>() - 256usize];
    ["Size of Isrvstr"][::core::mem::size_of::<Isrvstr>() - 30usize];
    ["Size of KeyMap"][::core::mem::size_of::<KeyMap>() - 32usize];
    ["Size of KeyMapNode"][::core::mem::size_of::<KeyMapNode>() - 46usize];
    ["Size of KeyMapResource"][::core::mem::size_of::<KeyMapResource>() - 28usize];
    ["Size of KeyQuery"][::core::mem::size_of::<KeyQuery>() - 4usize];
    ["Size of LVDrawMsg"][::core::mem::size_of::<LVDrawMsg>() - 24usize];
    ["Size of Layer"][::core::mem::size_of::<Layer>() - 160usize];
    ["Size of Layer_Info"][::core::mem::size_of::<Layer_Info>() - 102usize];
    ["Size of Library"][::core::mem::size_of::<Library>() - 34usize];
    ["Size of Line"][::core::mem::size_of::<Line>() - 36usize];
    ["Size of List"][::core::mem::size_of::<List>() - 14usize];
    ["Size of LoadSegBlock"][::core::mem::size_of::<LoadSegBlock>() - 512usize];
    ["Size of LocalContextItem"][::core::mem::size_of::<LocalContextItem>() - 20usize];
    ["Size of LocalVar"][::core::mem::size_of::<LocalVar>() - 24usize];
    ["Size of Locale"][::core::mem::size_of::<Locale>() - 168usize];
    ["Size of LocaleBase"][::core::mem::size_of::<LocaleBase>() - 36usize];
    ["Size of LocalePrefs"][::core::mem::size_of::<LocalePrefs>() - 860usize];
    ["Size of MathIEEEBase"][::core::mem::size_of::<MathIEEEBase>() - 60usize];
    ["Size of MathIEEEResource"][::core::mem::size_of::<MathIEEEResource>() - 44usize];
    ["Size of MemChunk"][::core::mem::size_of::<MemChunk>() - 8usize];
    ["Size of MemEntry"][::core::mem::size_of::<MemEntry>() - 8usize];
    ["Size of MemHandlerData"][::core::mem::size_of::<MemHandlerData>() - 12usize];
    ["Size of MemHeader"][::core::mem::size_of::<MemHeader>() - 32usize];
    ["Size of MemList"][::core::mem::size_of::<MemList>() - 24usize];
    ["Size of Menu"][::core::mem::size_of::<Menu>() - 30usize];
    ["Size of MenuItem"][::core::mem::size_of::<MenuItem>() - 34usize];
    ["Size of Message"][::core::mem::size_of::<Message>() - 20usize];
    ["Size of MinList"][::core::mem::size_of::<MinList>() - 12usize];
    ["Size of MinNode"][::core::mem::size_of::<MinNode>() - 8usize];
    ["Size of MonitorInfo"][::core::mem::size_of::<MonitorInfo>() - 96usize];
    ["Size of MonitorSpec"][::core::mem::size_of::<MonitorSpec>() - 160usize];
    ["Size of MsgPort"][::core::mem::size_of::<MsgPort>() - 34usize];
    ["Size of NVEntry"][::core::mem::size_of::<NVEntry>() - 20usize];
    ["Size of NVInfo"][::core::mem::size_of::<NVInfo>() - 8usize];
    ["Size of NameInfo"][::core::mem::size_of::<NameInfo>() - 56usize];
    ["Size of NamedObject"][::core::mem::size_of::<NamedObject>() - 4usize];
    ["Size of NewAmigaGuide"][::core::mem::size_of::<NewAmigaGuide>() - 52usize];
    ["Size of NewBroker"][::core::mem::size_of::<NewBroker>() - 26usize];
    ["Size of NewGadget"][::core::mem::size_of::<NewGadget>() - 30usize];
    ["Size of NewMenu"][::core::mem::size_of::<NewMenu>() - 20usize];
    ["Size of NewScreen"][::core::mem::size_of::<NewScreen>() - 32usize];
    ["Size of NewWindow"][::core::mem::size_of::<NewWindow>() - 48usize];
    ["Size of NexxStr"][::core::mem::size_of::<NexxStr>() - 16usize];
    ["Size of Node"][::core::mem::size_of::<Node>() - 14usize];
    ["Size of NotifyMessage"][::core::mem::size_of::<NotifyMessage>() - 38usize];
    ["Size of NotifyRequest"][::core::mem::size_of::<NotifyRequest>() - 48usize];
    ["Size of OldDrawerData"][::core::mem::size_of::<OldDrawerData>() - 56usize];
    ["Size of OverscanPrefs"][::core::mem::size_of::<OverscanPrefs>() - 36usize];
    ["Size of PGX"][::core::mem::size_of::<PGX>() - 16usize];
    ["Size of PaletteExtra"][::core::mem::size_of::<PaletteExtra>() - 68usize];
    ["Size of PalettePrefs"][::core::mem::size_of::<PalettePrefs>() - 400usize];
    ["Size of PartitionBlock"][::core::mem::size_of::<PartitionBlock>() - 256usize];
    ["Size of Player"][::core::mem::size_of::<Player>() - 44usize];
    ["Size of PointerPrefs"][::core::mem::size_of::<PointerPrefs>() - 32usize];
    ["Size of PrefHeader"][::core::mem::size_of::<PrefHeader>() - 6usize];
    ["Size of Preferences"][::core::mem::size_of::<Preferences>() - 232usize];
    ["Size of PrinterData"][::core::mem::size_of::<PrinterData>() - 6842usize]; // fixed for NDK3.2
    ["Size of PrinterExtendedData"][::core::mem::size_of::<PrinterExtendedData>() - 78usize]; // fixed for NDK3.2
    ["Size of PrinterGfxPrefs"][::core::mem::size_of::<PrinterGfxPrefs>() - 38usize];
    ["Size of PrinterPSPrefs"][::core::mem::size_of::<PrinterPSPrefs>() - 124usize];
    ["Size of PrinterSegment"][::core::mem::size_of::<PrinterSegment>() - 90usize]; // fixed for NDK3.2
    ["Size of PrinterTxtPrefs"][::core::mem::size_of::<PrinterTxtPrefs>() - 64usize];
    ["Size of PrinterUnitPrefs"][::core::mem::size_of::<PrinterUnitPrefs>() - 56usize];
    ["Size of Process"][::core::mem::size_of::<Process>() - 228usize];
    ["Size of PropInfo"][::core::mem::size_of::<PropInfo>() - 22usize];
    ["Size of PrtInfo"][::core::mem::size_of::<PrtInfo>() - 128usize]; // fixed for NDK3.2
    ["Size of PubScreenNode"][::core::mem::size_of::<PubScreenNode>() - 30usize];
    ["Size of QCode"][::core::mem::size_of::<QCode>() - 12usize];
    ["Size of QueryHeader"][::core::mem::size_of::<QueryHeader>() - 16usize];
    ["Size of RDArgs"][::core::mem::size_of::<RDArgs>() - 32usize];
    ["Size of RGBTable"][::core::mem::size_of::<RGBTable>() - 4usize];
    ["Size of RMSF"][::core::mem::size_of::<RMSF>() - 4usize];
    ["Size of RasInfo"][::core::mem::size_of::<RasInfo>() - 12usize];
    ["Size of RastPort"][::core::mem::size_of::<RastPort>() - 100usize];
    ["Size of RealTimeBase"][::core::mem::size_of::<RealTimeBase>() - 48usize];
    ["Size of RecordLock"][::core::mem::size_of::<RecordLock>() - 16usize];
    ["Size of Rect32"][::core::mem::size_of::<Rect32>() - 16usize];
    ["Size of Rectangle"][::core::mem::size_of::<Rectangle>() - 8usize];
    ["Size of Region"][::core::mem::size_of::<Region>() - 12usize];
    ["Size of RegionRectangle"][::core::mem::size_of::<RegionRectangle>() - 16usize];
    ["Size of Remember"][::core::mem::size_of::<Remember>() - 12usize];
    ["Size of Requester"][::core::mem::size_of::<Requester>() - 112usize];
    ["Size of Resident"][::core::mem::size_of::<Resident>() - 26usize];
    ["Size of RexxArg"][::core::mem::size_of::<RexxArg>() - 16usize];
    ["Size of RexxMsg"][::core::mem::size_of::<RexxMsg>() - 128usize];
    ["Size of RexxMsgPort"][::core::mem::size_of::<RexxMsgPort>() - 80usize];
    ["Size of RexxRsrc"][::core::mem::size_of::<RexxRsrc>() - 32usize];
    ["Size of RexxTask"][::core::mem::size_of::<RexxTask>() - 330usize];
    ["Size of RigidDiskBlock"][::core::mem::size_of::<RigidDiskBlock>() - 256usize];
    ["Size of RootNode"][::core::mem::size_of::<RootNode>() - 56usize];
    ["Size of RxsLib"][::core::mem::size_of::<RxsLib>() - 256usize]; // fixed for NDK3.2
    ["Size of SCSICmd"][::core::mem::size_of::<SCSICmd>() - 30usize];
    ["Size of SGWork"][::core::mem::size_of::<SGWork>() - 44usize];
    ["Size of SatisfyMsg"][::core::mem::size_of::<SatisfyMsg>() - 26usize];
    ["Size of Screen"][::core::mem::size_of::<Screen>() - 346usize];
    ["Size of ScreenBuffer"][::core::mem::size_of::<ScreenBuffer>() - 8usize];
    ["Size of ScreenModePrefs"][::core::mem::size_of::<ScreenModePrefs>() - 28usize];
    ["Size of ScreenModeRequester"][::core::mem::size_of::<ScreenModeRequester>() - 48usize];
    ["Size of Segment"][::core::mem::size_of::<Segment>() - 16usize];
    ["Size of Semaphore"][::core::mem::size_of::<Semaphore>() - 36usize];
    ["Size of SemaphoreMessage"][::core::mem::size_of::<SemaphoreMessage>() - 24usize];
    ["Size of SemaphoreRequest"][::core::mem::size_of::<SemaphoreRequest>() - 12usize];
    ["Size of SerialPrefs"][::core::mem::size_of::<SerialPrefs>() - 34usize];
    ["Size of SignalSemaphore"][::core::mem::size_of::<SignalSemaphore>() - 46usize];
    ["Size of SimpleSprite"][::core::mem::size_of::<SimpleSprite>() - 12usize];
    ["Size of SoftIntList"][::core::mem::size_of::<SoftIntList>() - 16usize];
    ["Size of SoundPrefs"][::core::mem::size_of::<SoundPrefs>() - 284usize];
    ["Size of SpecialMonitor"][::core::mem::size_of::<SpecialMonitor>() - 58usize];
    //["Size of SpriteDef"][::core::mem::size_of::<SpriteDef>() - 8usize]; // not found in NDK3.2
    ["Size of SrcNode"][::core::mem::size_of::<SrcNode>() - 16usize];
    ["Size of StackSwapStruct"][::core::mem::size_of::<StackSwapStruct>() - 12usize];
    ["Size of StandardPacket"][::core::mem::size_of::<StandardPacket>() - 68usize];
    ["Size of StoredProperty"][::core::mem::size_of::<StoredProperty>() - 8usize];
    ["Size of StringExtend"][::core::mem::size_of::<StringExtend>() - 36usize];
    ["Size of StringInfo"][::core::mem::size_of::<StringInfo>() - 36usize];
    ["Size of TAvailFonts"][::core::mem::size_of::<TAvailFonts>() - 14usize];
    ["Size of TDU_PublicUnit"][::core::mem::size_of::<TDU_PublicUnit>() - 72usize]; // fixed for NDK3.2
    ["Size of TFontContents"][::core::mem::size_of::<TFontContents>() - 260usize];
    ["Size of TOCEntry"][::core::mem::size_of::<TOCEntry>() - 6usize];
    ["Size of TOCSummary"][::core::mem::size_of::<TOCSummary>() - 6usize];
    ["Size of TP_AmigaXIP"][::core::mem::size_of::<TP_AmigaXIP>() - 8usize];
    ["Size of TTextAttr"][::core::mem::size_of::<TTextAttr>() - 12usize];
    ["Size of TabletData"][::core::mem::size_of::<TabletData>() - 24usize];
    ["Size of TabletHookData"][::core::mem::size_of::<TabletHookData>() - 16usize];
    ["Size of TagItem"][::core::mem::size_of::<TagItem>() - 8usize];
    ["Size of Task"][::core::mem::size_of::<Task>() - 92usize];
    ["Size of TextAttr"][::core::mem::size_of::<TextAttr>() - 8usize];
    ["Size of TextExtent"][::core::mem::size_of::<TextExtent>() - 12usize];
    ["Size of TextFont"][::core::mem::size_of::<TextFont>() - 52usize];
    ["Size of TextFontExtension"][::core::mem::size_of::<TextFontExtension>() - 24usize];
    ["Size of TmpRas"][::core::mem::size_of::<TmpRas>() - 8usize];
    ["Size of Tool"][::core::mem::size_of::<Tool>() - 8usize];
    ["Size of ToolNode"][::core::mem::size_of::<ToolNode>() - 26usize];
    ["Size of UCopList"][::core::mem::size_of::<UCopList>() - 12usize];
    ["Size of Unit"][::core::mem::size_of::<Unit>() - 38usize];
    ["Size of UtilityBase"][::core::mem::size_of::<UtilityBase>() - 36usize];
    ["Size of VSprite"][::core::mem::size_of::<VSprite>() - 60usize];
    ["Size of VecInfo"][::core::mem::size_of::<VecInfo>() - 40usize];
    ["Size of View"][::core::mem::size_of::<View>() - 18usize];
    ["Size of ViewExtra"][::core::mem::size_of::<ViewExtra>() - 34usize];
    ["Size of ViewPort"][::core::mem::size_of::<ViewPort>() - 40usize];
    ["Size of ViewPortExtra"][::core::mem::size_of::<ViewPortExtra>() - 66usize];
    ["Size of VoiceHeader"][::core::mem::size_of::<VoiceHeader>() - 20usize];
    ["Size of WBArg"][::core::mem::size_of::<WBArg>() - 8usize];
    ["Size of WBPatternPrefs"][::core::mem::size_of::<WBPatternPrefs>() - 24usize];
    ["Size of WBStartup"][::core::mem::size_of::<WBStartup>() - 40usize];
    ["Size of Window"][::core::mem::size_of::<Window>() - 136usize];
    ["Size of XRef"][::core::mem::size_of::<XRef>() - 40usize]; // fixed for NDK3.2
    ["Size of _Object"][::core::mem::size_of::<_Object>() - 12usize];
    ["Size of adtFrame"][::core::mem::size_of::<adtFrame>() - 40usize];
    ["Size of adtStart"][::core::mem::size_of::<adtStart>() - 8usize];
    ["Size of bltnode"][::core::mem::size_of::<bltnode>() - 18usize];
    ["Size of collTable"][::core::mem::size_of::<collTable>() - 64usize];
    ["Size of copinit"][::core::mem::size_of::<copinit>() - 192usize];
    ["Size of cprlist"][::core::mem::size_of::<cprlist>() - 10usize];
    ["Size of dtDraw"][::core::mem::size_of::<dtDraw>() - 36usize];
    ["Size of dtFrameBox"][::core::mem::size_of::<dtFrameBox>() - 24usize];
    ["Size of dtGeneral"][::core::mem::size_of::<dtGeneral>() - 8usize];
    ["Size of dtGoto"][::core::mem::size_of::<dtGoto>() - 16usize];
    ["Size of dtPrint"][::core::mem::size_of::<dtPrint>() - 16usize];
    ["Size of dtSelect"][::core::mem::size_of::<dtSelect>() - 16usize];
    ["Size of dtTrigger"][::core::mem::size_of::<dtTrigger>() - 16usize];
    ["Size of dtWrite"][::core::mem::size_of::<dtWrite>() - 20usize];
    ["Size of gpGoInactive"][::core::mem::size_of::<gpGoInactive>() - 12usize];
    ["Size of gpHitTest"][::core::mem::size_of::<gpHitTest>() - 12usize];
    ["Size of gpInput"][::core::mem::size_of::<gpInput>() - 24usize];
    ["Size of gpLayout"][::core::mem::size_of::<gpLayout>() - 12usize];
    ["Size of gpRender"][::core::mem::size_of::<gpRender>() - 16usize];
    ["Size of impDraw"][::core::mem::size_of::<impDraw>() - 24usize];
    ["Size of impErase"][::core::mem::size_of::<impErase>() - 16usize];
    ["Size of impFrameBox"][::core::mem::size_of::<impFrameBox>() - 20usize];
    ["Size of impHitTest"][::core::mem::size_of::<impHitTest>() - 12usize];
    ["Size of mouth_rb"][::core::mem::size_of::<mouth_rb>() - 92usize];
    ["Size of narrator_rb"][::core::mem::size_of::<narrator_rb>() - 88usize];
    ["Size of opAddTail"][::core::mem::size_of::<opAddTail>() - 8usize];
    ["Size of opExpungeNode"][::core::mem::size_of::<opExpungeNode>() - 8usize];
    ["Size of opFindHost"][::core::mem::size_of::<opFindHost>() - 28usize];
    ["Size of opGet"][::core::mem::size_of::<opGet>() - 12usize];
    ["Size of opMember"][::core::mem::size_of::<opMember>() - 8usize];
    ["Size of opNodeIO"][::core::mem::size_of::<opNodeIO>() - 28usize];
    ["Size of opSet"][::core::mem::size_of::<opSet>() - 12usize];
    ["Size of opUpdate"][::core::mem::size_of::<opUpdate>() - 16usize];
    ["Size of pmState"][::core::mem::size_of::<pmState>() - 8usize];
    ["Size of pmTime"][::core::mem::size_of::<pmTime>() - 8usize];
    ["Size of timerequest"][::core::mem::size_of::<timerequest>() - 40usize];
    ["Size of timeval"][::core::mem::size_of::<timeval>() - 8usize];
    ["Size of CDTOC"][::core::mem::size_of::<CDTOC>() - 6usize];
    ["Size of LSNMSF"][::core::mem::size_of::<LSNMSF>() - 4usize];
    ["Size of colorEntry"][::core::mem::size_of::<colorEntry>() - 4usize];
    ["Size of printerIO"][::core::mem::size_of::<printerIO>() - 62usize];
};
