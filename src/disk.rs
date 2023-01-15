mod hdd;

pub fn init() -> Result<(), &'static str> {
    Ok(())
}

pub enum DisKType {
    HDD,
    SSD,
}

pub fn create_new_disk(disk_type: DisKType) -> Result<(), &'static str> {
    match disk_type {
        DisKType::HDD => {
            println!("Start create a HDD disk");
            Ok(())
        }
        DisKType::SSD => {
            println!("Start create a SSD disk");
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hdd() {
        if let Err(e) = create_new_disk(super::DisKType::HDD) {
            assert!(false, "{}", e);
        }
    }

    #[test]
    fn test_create_ssd() {
        if let Err(e) = create_new_disk(super::DisKType::SSD) {
            assert!(false, "{}", e);
        }
    }
}
