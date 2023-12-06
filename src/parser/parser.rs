#[cfg(test)]
mod tests {
    const SMBIOS_TABLE_SIGNATURE: u32 = 0x52534D42;

    use crate::parser::tables::{Header, Table, Type0};
    use std::{alloc, io::Cursor, ops::Add};
    use windows_sys::Win32::System;

    #[test]
    fn test_type1() {
        let req_size = unsafe {
            System::SystemInformation::GetSystemFirmwareTable(
                SMBIOS_TABLE_SIGNATURE,
                0,
                std::ptr::null_mut(),
                0,
            )
        };

        let mut b: Vec<u8> = vec![0; req_size as usize];

        let write_count = unsafe {
            System::SystemInformation::GetSystemFirmwareTable(
                SMBIOS_TABLE_SIGNATURE,
                0,
                b.as_mut_ptr(),
                req_size,
            )
        };

        debug_assert!(write_count == req_size);

        const TABLE_DATA_START: usize = 8 as usize;
        let data = &b[TABLE_DATA_START..];

        let current_table = Table::new(data).expect("unable to parse table");
        let string_table = current_table
            .build_string_table(&data[current_table.len()..])
            .unwrap();

        let data = &data[current_table.total_len(&string_table)..];

        let t1 = Table::new(&data[..]).expect("unable to parse table");
        let string_table = t1.build_string_table(&data[t1.len()..]).unwrap();
        let total_len = t1.total_len(&string_table);

        println!("total: {:?} table: {:#?}", total_len, t1);
    }
}
