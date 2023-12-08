const ANCHOR21_LENGTH: usize = 4;
const ANCHOR30_LENGTH: usize = 5;
const INTERMEDIATE_ANCHOR_LENGTH: usize = 5;

const FORMATTED_AREA_SIZE: usize = 5;

const ANCHOR21: [u8; ANCHOR21_LENGTH] = [0x5f, 0x53, 0x4d, 0x5f];
const ANCHOR30: [u8; ANCHOR30_LENGTH] = [0x5f, 0x53, 0x5d, 0x33, 0x5f];

const INTERMEDIATE_ANCHOR: [u8; INTERMEDIATE_ANCHOR_LENGTH] = [0x5f, 0x44, 0x4d, 0x49, 0x5f];

pub struct WrapTable<T>(T);

#[repr(C)]
#[allow(non_snake_case)]
struct SMBios21Entry {
    AnchorString: [u8; ANCHOR21_LENGTH], /* _SM_  */
    Checksum: u8,
    Length: u8,
    Major: u8,
    Minor: u8,
    MaximumSize: u16,
    Revision: u8,
    FormattedArea: [u8; FORMATTED_AREA_SIZE],
    IntermediateAnchor: [u8; INTERMEDIATE_ANCHOR_LENGTH], /* _DMI_ */
    IntermediateChecksum: u8,
    StructureTableLength: u16,
    StructureTableAddress: u32, /* Physical address  */
    NumOfStructures: u16,
    BcdRevision: u8,
}

impl SMBios21Entry {
    fn new() -> Self {
        Self {
            AnchorString: ANCHOR21,
            Checksum: 0,
            Length: 0,
            Major: 2,
            Minor: 1,
            MaximumSize: 0,
            Revision: 0,
            FormattedArea: [0; FORMATTED_AREA_SIZE],
            IntermediateAnchor: INTERMEDIATE_ANCHOR,
            IntermediateChecksum: 0,
            StructureTableLength: 0,
            StructureTableAddress: 0,
            NumOfStructures: 0,
            BcdRevision: 0x21,
        }
    }
}

impl<T> TryFrom<WrapTable<T>> for SMBios21Entry {
    type Error = std::io::Error;

    fn try_from(value: WrapTable<T>) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl From<&[u8]> for SMBios21Entry {
    fn from(value: &[u8]) -> Self {
        todo!()
    }
}

#[repr(C)]
#[allow(non_snake_case)]
struct SMBios30Entry {
    AnchorString: [u8; ANCHOR30_LENGTH],
    Checksum: u8,
    Length: u8,
    Major: u8,
    Minor: u8,
    DocRev: u8,
    Revision: u8,
    Reserved: u8,
    StructureTableLength: u32,
    StructureTableAddress: u64, /* Physical address  */
}

impl SMBios30Entry {
    fn new() -> Self {
        Self {
            AnchorString: ANCHOR30,
            Checksum: 0,
            Length: 0,
            Major: 3,
            Minor: 0,
            DocRev: 0,
            Revision: 0,
            Reserved: 0,
            StructureTableLength: 0,
            StructureTableAddress: 0,
        }
    }
}

impl From<&[u8]> for SMBios30Entry {
    fn from(value: &[u8]) -> Self {
        todo!()
    }
}

impl<T> TryFrom<WrapTable<T>> for SMBios30Entry {
    type Error = std::io::Error;

    fn try_from(value: WrapTable<T>) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Debug)]
pub(crate) struct Header {
    pub(crate) Type: u8,
    pub(crate) Length: u8,
    pub(crate) Handle: u16,
}

impl Header {
    pub fn from_le_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() <= std::mem::size_of::<Header>() {
            return None;
        }

        Some(Self {
            Type: bytes[0],
            Length: bytes[1],
            Handle: u16::from_le_bytes(
                bytes[2..4]
                    .try_into()
                    .expect("unable to parse handle from header"),
            ),
        })
    }
}

macro_rules! smbios_table {
    ($type:ident, $doc: expr, $(($field: ident, $ty: ty)), *) => {
        #[repr(C)]
        #[allow(non_snake_case)]
        #[derive(Debug)]
        #[doc = $doc ]
        pub(crate) struct $type {
            pub(crate) Type: u8,
            pub(crate) Length: u8,
            pub(crate) Handle: u16,
            $(
                pub(crate) $field: $ty
            ), *
        }
    };
}

smbios_table!(
    Type0,
    "BIOS Information",
    (BIOSVersion, u8),
    (BIOSStartingAddressSegment, u16),
    (BIOSReleaseDate, u8),
    (BIOSROMSize, u8),
    (BIOSCharacteristics, u64),
    (BIOSCharacteristicsExtensionBytes, u16),
    (SystemBIOSMajorRelease, u8),
    (SystemBIOSMinorRelease, u8),
    (EmbeddedControllerFirmwareMajorRelease, u8),
    (EmbeddedControllerFirmwareMinorRelease, u8),
    (ExtendedBIOSROMSize, u16)
);

smbios_table!(
    Type1,
    "System Information",
    (Manufacturer, u8),
    (ProductName, u8),
    (Version, u8),
    (SerialNumber, u8),
    (UUID, [u8; 16]),
    (WakeUpType, u8),
    (SKUNumber, u8),
    (Family, u8)
);

smbios_table!(
    Type2,
    "Baseboard information",
    (Manufacturer, u8),
    (Product, u8),
    (Version, u8),
    (SerialNumber, u8),
    (AssetTag, u8),
    (FeatureFlags, u8),
    (LocationInChassis, u8),
    (ChassisHandle, u16),
    (BoardType, u8),
    (NumOfContainedObjectHandles, u8),
    (ContainedObjectHandles, [u16; 0]) /* VLA */
);

smbios_table!(
    Type3,
    "System enclosure",
    (Manufacturer, u8),
    (ChassisType, u8),
    (Version, u8),
    (SerialNumber, u8),
    (AssetTagNubmer, u8),
    (BootupState, u8),
    (PowerSupplyState, u8),
    (ThermalState, u8),
    (SecurityStatus, u8),
    (OEMDefined, u32),
    (Height, u8),
    (NumOfPowerCords, u8),
    (ContainedElementCount, u8),
    (ContainedElementRecordLength, u8),
    (ContainedElements, [u8; 0]), /* VLA */
    (SKUNumber, u8)
);

smbios_table!(
    Type4,
    "Processor information",
    (SocketDesignation, u8),
    (ProcessorType, u8),
    (ProcessorFamily, u8),
    (ProcessorManufacturer, u8),
    (ProcessorId, u64),
    (ProcessorVersion, u8),
    (Voltage, u8),
    (ExternalClock, u16),
    (MaxSpeed, u16),
    (CurrentSpeed, u16),
    (Status, u8),
    (ProcessorUpgrade, u8),
    (L1CacheHandle, u16),
    (L2CacheHandle, u16),
    (L3CacheHandle, u16),
    (SerialNumber, u8),
    (AssetTag, u8),
    (PartNumber, u8),
    (CoreCount, u8),
    (CoreEnabled, u8),
    (ThreadCount, u8),
    (ProcessorCharacteristics, u16),
    (ProcessorFamily2, u16),
    (CoreCount2, u16),
    (CoreEnabled2, u16),
    (ThreadCount2, u16),
    (ThreadEnabled, u16)
);

/* Type 5 and 6 are deprecated. */

smbios_table!(
    Type7,
    "Cache information",
    (SocketDesignation, u8),
    (CacheConfiguration, u16),
    (MaximumCacheSize, u16),
    (InstalledSize, u16),
    (SupportedSRAMType, u16),
    (CurrentSRAMType, u16),
    (CacheSpeed, u8),
    (ErrorCorrectionType, u8),
    (SystemCacheType, u8),
    (Associativity, u8),
    (MaximumCacheSize2, u32),
    (InstalledCacheSize, u32)
);

smbios_table!(
    Type8,
    "Port connector",
    (InternalReferenceDesignator, u8),
    (InternalConnectorType, u8),
    (ExternalReferenceDesignator, u8),
    (ExternalConnectorType, u8),
    (PortType, u8)
);

smbios_table!(
    Type9,
    "System slots",
    (SlotDesignation, u8),
    (SlotType, u8),
    (SlotDataBusWidth, u8),
    (CurrentUsage, u8),
    (SlotLength, u8),
    (SlotId, u16),
    (SlotCharacteristics, u8),
    (SlotCharacteristics2, u8),
    (SegmentGroupNumber, u16),
    (BusNumber, u8),
    (DeviceNumber, u8),
    (DataBusWidth, u8),
    (PeerGroupingCount, u8),
    (PeerGroups, [u8; 0]), /* VLA */
    (SlotInformation, u8),
    (SlotPhysicalWidth, u8),
    (SlotPitch, u16),
    (SlotHeight, u8)
);

/* Type 10 is obsolete */

smbios_table!(Type11, "OEM Strings", (Count, u8));
smbios_table!(Type12, "System configuration options", (Count, u8));

smbios_table!(
    Type13,
    "BIOS Language information",
    (InstallableLanguages, u8),
    (Flags, u8),
    (Reserved, [u8; 15]),
    (CurrentLanguage, u8)
);

smbios_table!(
    Type14,
    "Group associations",
    (GroupName, u8),
    (ItemType, u8),
    (ItemHandle, u16)
);

smbios_table!(
    Type15,
    "System event log",
    (LogAreaLength, u16),
    (LogHeaderStartOffset, u16),
    (LogDataStartOffset, u16),
    (AccessMethod, u8),
    (LogStatus, u8),
    (LogChangeToken, u32),
    (AccessMethodAddress, u32),
    (LogHeaderFormat, u8),
    (NumberOfSupportedLogTypeDescriptors, u8),
    (LengthOfEachLogTypeDescriptor, u8),
    (ListOfSupportedEventLogTypeDescriptors, [u8; 0]) /* VLA */
);
