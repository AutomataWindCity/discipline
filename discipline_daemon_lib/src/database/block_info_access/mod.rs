mod datum;
use datum::*;

mod datum_collection;
pub use datum_collection::*;

mod vault;
use vault::*;

mod vault_name;

mod datum_text;

mod vault_collection;
pub use vault_collection::*;

mod vault_protector;
use vault_protector::*;

use crate::x::{Database, TextualError, UuidV4, conditionals, database::*};
use crate::x::block_info_access::*;

mod singleton;
pub use singleton::SingletonSchema;