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

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Debug)]
pub(crate) struct Type0 {
    pub(crate) Type: u8,
    pub(crate) Length: u8,
    pub(crate) Handle: u16,
    pub(crate) Vendor: u8,
    pub(crate) BIOSVersion: u8,
    pub(crate) BIOSStartingAddressSegment: u16,
    pub(crate) BIOSReleaseDate: u8,
    pub(crate) BIOSROMSize: u8, /* 64K * (n + 1) */
    pub(crate) BIOSCharacteristics: u64,
    pub(crate) BIOSCharacteristicsExtensionBytes: u16,
    pub(crate) SystemBIOSMajorRelease: u8,
    pub(crate) SystemBIOSMinorRelease: u8,
    pub(crate) EmbeddedControllerFirmwareMajorRelease: u8,
    pub(crate) EmbeddedControllerFirmwareMinorRelease: u8,
    pub(crate) ExtendedBIOSROMSize: u16,
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Debug)]
pub(crate) struct Type1 {
    pub(crate) Type: u8,
    pub(crate) Length: u8,
    pub(crate) Handle: u16,
    pub(crate) Manufacturer: u8,
    pub(crate) ProductName: u8,
    pub(crate) Version: u8,
    pub(crate) SerialNumber: u8,
    pub(crate) UUID: [u8; 16],
    pub(crate) WakeUpType: u8,
    pub(crate) SKUNumber: u8,
    pub(crate) Family: u8,
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Debug)]
pub(crate) struct Type4 {
    pub(crate) Type: u8,
    pub(crate) Length: u8,
    pub(crate) Handle: u16,
    pub(crate) SocketDesignation: u8,
    pub(crate) ProcessorType: u8,
    pub(crate) ProcessorFamily: u8,
    pub(crate) ProcessorManufacturer: u8,
    pub(crate) ProcessorId: u64,
    pub(crate) ProcessorVersion: u8,
    pub(crate) Voltage: u8,
    pub(crate) ExternalClock: u16,
    pub(crate) MaxSpeed: u16,
    pub(crate) CurrentSpeed: u16,
    pub(crate) Status: u8,
    pub(crate) ProcessorUpgrade: u8,
    pub(crate) L1CacheHandle: u16,
    pub(crate) L2CacheHandle: u16,
    pub(crate) L3CacheHandle: u16,
    pub(crate) SerialNumber: u8,
    pub(crate) AssetTag: u8,
    pub(crate) PartNumber: u8,
    pub(crate) CoreCount: u8,
    pub(crate) CoreEnabled: u8,
    pub(crate) ThreadCount: u8,
    pub(crate) ProcessorCharacteristics: u16,
    pub(crate) ProcessorFamily2: u16,
    pub(crate) CoreCount2: u16,
    pub(crate) CoreEnabled2: u16,
    pub(crate) ThreadCount2: u16,
    pub(crate) ThreadEnabled: u16,
}

#[derive(Debug)]
pub enum Table {
    Type0(Type0),
    Type1(Type1),
    Type4(Type4),
}

impl Table {
    pub fn new(data: &[u8]) -> Option<Table> {
        if let Some(header) = Header::from_le_bytes(data) {
            println!("header: {:?}", header);
            match header.Type {
                0 => {
                    return Some(Table::Type0(Type0 {
                        Type: header.Type,
                        Length: header.Length,
                        Handle: header.Handle,
                        Vendor: data[4],
                        BIOSVersion: data[5],
                        BIOSStartingAddressSegment: u16::from_le_bytes(
                            data[6..8]
                                .try_into()
                                .expect("BIOS starting address segment"),
                        ),
                        BIOSReleaseDate: data[8],
                        BIOSROMSize: data[9],
                        BIOSCharacteristics: u64::from_le_bytes(
                            data[10..18].try_into().expect("BIOS Characteristics"),
                        ),
                        BIOSCharacteristicsExtensionBytes: u16::from_le_bytes(
                            data[18..20].try_into().expect("BIOS characteristics bytes"),
                        ),
                        SystemBIOSMajorRelease: data[20],
                        SystemBIOSMinorRelease: data[21],
                        EmbeddedControllerFirmwareMajorRelease: data[22],
                        EmbeddedControllerFirmwareMinorRelease: data[23],
                        ExtendedBIOSROMSize: u16::from_le_bytes(
                            data[23..25].try_into().expect("extended BIOS ROM size"),
                        ),
                    }))
                }
                1 => {
                    return Some(Table::Type1(Type1 {
                        Type: header.Type,
                        Length: header.Length,
                        Handle: header.Handle,
                        Manufacturer: data[4],
                        ProductName: data[5],
                        Version: data[6],
                        SerialNumber: data[7],
                        UUID: data[8..][..16].try_into().expect("uuid"),
                        WakeUpType: data[24],
                        SKUNumber: data[25],
                        Family: data[26],
                    }));
                }
                2 => {}
                3 => {}
                4 => {}
                _ => return None,
            }
        }
        todo!()
    }

    pub fn build_string_table(&self, data: &[u8]) -> Option<Vec<String>> {
        let mut strings: Vec<_> = Vec::new();
        let mut curr_pos = 0 as usize;

        loop {
            if (data[curr_pos] == 0 && data[curr_pos + 1] == 0) || curr_pos >= data.len() {
                break;
            }

            if data[curr_pos] == 0 {
                curr_pos = curr_pos.wrapping_add(1);
            }

            let str = std::ffi::CStr::from_bytes_until_nul(&data[curr_pos..]);

            if str.is_err() {
                /* Abort on failure for now... handle this better in the future */
                return None;
            }

            let str = str.unwrap().to_str().expect("unable to parse string");
            strings.push(str.to_string());
            curr_pos = curr_pos.wrapping_add(str.len());
            println!("string: {:?}", str);
        }

        Some(strings)
    }

    pub fn len(&self) -> usize {
        match self {
            Table::Type0(e) => return e.Length as usize,
            Table::Type1(e) => return e.Length as usize,
            Table::Type4(e) => return e.Length as usize,
        }
    }

    pub fn total_len(&self, string_table: &Vec<String>) -> usize {
        /*
           There's a limit of 255 strings per table in SMBIOS, however, there is no limit on their length.
           Therefore, to calculate the total size of an entry we must perform the following sum:
               - Length of the table as given by the header;
               - Length of all strings, including their NULL terminators; and
               - Add two to account for the table terminator
        */
        let string_table_len =
            string_table.iter().fold(0, |curr, s| curr + s.len()) + (1 * string_table.len()) + 2;
        string_table_len + self.len() - 1
    }
}

struct SMBiosTableEntry {
    table: Table,
    strings: Vec<String>,
}
