use super::{block_account_access, block_device_access, block_info_access, block_internet_access};

struct Schema {
  block_device_access: block_device_access::database::Schema,
  block_account_access: block_account_access::database::Schema,
  block_internet_access: block_internet_access::database::Schema,
}