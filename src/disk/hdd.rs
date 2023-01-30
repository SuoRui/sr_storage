use std::collections::HashMap;

#[derive(Debug)]
struct HDD {
    id: u32,         // 磁盘id
    capacity: u64,   // 容量，单位字节
    max_iops: u32,   // 支持最大iops
    block_size: u32, // 块大小
}

impl HDD {
    fn new(id: u32, capacity: u64) -> HDD {
        HDD {
            id: id,
            capacity: capacity,
            max_iops: 100,
            block_size: 512,
        }
    }

    fn set_max_iops(&mut self, iops: u32) {
        self.max_iops = iops;
    }

    fn set_block_size(&mut self, blockSize: u32) {
        self.block_size = blockSize;
    }
}

static mut HDD_DISK_MAP: Option<HashMap<u32, HDD>> = None;

fn init_hdd_disk_map() {
    unsafe {
        HDD_DISK_MAP = Some(HashMap::new());
    }
}

pub fn init_hdd() -> Result<(), &'static str> {
    init_hdd_disk_map();
    Ok(())
}

fn insert_hdd_to_hdd_hash(disk: HDD) -> Result<(), &'static str> {
    unsafe {
        if let Some(hdd_hash) = &mut HDD_DISK_MAP {
            hdd_hash.insert(disk.id, disk);
        } else {
            return Err("Insert hdd check HDD hash map is none");
        }
    }

    Ok(())
}

// 创建一个hdd磁盘，capacity单位为字节
pub fn create_hdd(capacity: u64) -> Result<u32, &'static str> {
    // 寻找一个没有使用过的id
    let mut id = 0;
    unsafe {
        if let Some(hdd_hash) = &HDD_DISK_MAP {
            while hdd_hash.contains_key(&id) {
                id += 1;
            }
        } else {
            return Err("HDD hash map is none");
        }
    }

    let new_disk = HDD::new(id, capacity);
    insert_hdd_to_hdd_hash(new_disk)?;

    Ok(id)
}
