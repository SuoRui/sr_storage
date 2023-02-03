mod hdd;

pub fn init() -> Result<(), &'static str> {
    hdd::init_hdd()?;
    Ok(())
}

pub enum DiskType {
    HDD,
    SSD,
}

pub fn create_new_disk(disk_type: DiskType, size: u64) -> Result<u32, &'static str> {
    match disk_type {
        DiskType::HDD => {
            println!("Start create a HDD disk");
            let disk_id = hdd::create_hdd(size)?;
            Ok(disk_id)
        }
        DiskType::SSD => {
            println!("Start create a SSD disk");
            Ok(0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hdd() {
        let id = create_new_disk(DiskType::HDD, 1024 * 1024 * 1024);
        assert!(id.is_ok());
        let id = id.unwrap();
        println!("id = {}", id);
    }

    #[test]
    fn test_create_ssd() {
        if let Err(e) = create_new_disk(DiskType::SSD, 1024 * 1024 * 1024) {
            assert!(false, "{}", e);
        }
    }
}
