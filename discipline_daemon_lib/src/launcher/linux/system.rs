use libc::{gid_t, uid_t, group, passwd};
use std::ffi::{CStr, CString};
use std::{mem, ptr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId {
  inner: uid_t,
}

impl UserId {
  pub fn new(inner: uid_t) -> Self {
    Self { 
      inner
    }
  }

  pub fn inner(&self) -> uid_t {
    self.inner
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserName {
  inner: CString
}

impl UserName {
  pub fn new(inner: CString) -> Self {
    Self { 
      inner 
    }
  }

  pub fn inner(&self) -> &CString {
    &self.inner
  }

  pub fn as_ref(&self) -> UserNameRef<'_> {
    UserNameRef { inner: &self.inner }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserNameRef<'a> {
  inner: &'a CStr
}

impl<'a> UserNameRef<'a> {
  pub fn new(inner: &'a CStr) -> Self {
    Self { 
      inner 
    }
  }

  pub fn inner(&self) -> &'a CStr {
    &self.inner
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GroupId {
  inner: gid_t,
}

impl GroupId {
  pub fn new(inner: gid_t) -> Self {
    Self {
      inner
    }
  }

  pub fn inner(&self) -> gid_t {
    self.inner
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GroupName {
  inner: CString,
}

impl GroupName {
  pub fn new(inner: CString) -> Self {
    Self {
      inner
    }
  }

  pub fn inner(&self) -> &CStr {
    &self.inner
  }
}

#[derive(Debug, Clone)]
pub struct PasswordFileEntry {
  pub user_id: UserId,
  pub user_name: UserName,
  pub primary_group_id: GroupId,
}

#[derive(Debug, Clone)]
pub struct GroupFileEntry {
  group_id: GroupId,
  group_name: GroupName,
}

unsafe fn sanitize_password_file_entry(entry: passwd) -> PasswordFileEntry {
  let user_id = UserId::new(entry.pw_uid);
  let user_name = unsafe { CStr::from_ptr(entry.pw_name) };
  let user_name = UserName::new(user_name.to_owned());
  let primary_group_id = GroupId::new(entry.pw_gid);

  PasswordFileEntry { 
    user_id, 
    user_name, 
    primary_group_id,
  } 
}

unsafe fn sanitize_group_file_entry(entry: group) -> GroupFileEntry {
  let group_id = GroupId::new(entry.gr_gid);
  let group_name = unsafe { CStr::from_ptr(entry.gr_name) };
  let group_name = GroupName::new(group_name.to_owned());

  GroupFileEntry {
    group_id,
    group_name,
  }
}

// Invariant: maximum_memory_allocation_retries >= 1
// Invariant: memory_allocation_increment_factor >= 2048
// Invariant: maximum_memory_allocation_retries * memory_allocation_increment_factor < isize::MAX
pub struct AllocationConfig {
  pub maximum_memory_allocation_retries: usize,
  pub memory_allocation_increment_factor: usize,
}

impl AllocationConfig {
  pub fn create_allocation_control(&self) -> AllocationControl {
    AllocationControl { 
      maximum_memory_allocation_retries: self.maximum_memory_allocation_retries, 
      memory_allocation_increment_factor: self.memory_allocation_increment_factor, 
      retry: 1,
    }
  }
}

pub struct AllocationControl {
  maximum_memory_allocation_retries: usize,
  memory_allocation_increment_factor: usize,
  retry: usize,
}

impl AllocationControl {
  pub fn initial_allocation_size(&mut self) -> usize {
    self.memory_allocation_increment_factor
  }

  // Make sure not to return an size larger than isize::MAX
  // to avoid paniking when resizing the buffer.
  pub fn next_allocation_size(&mut self) -> Option<usize> {
    if self.retry >= self.maximum_memory_allocation_retries {
      return None;
    }

    match self
      .memory_allocation_increment_factor
      .checked_mul(self.retry)
    {
      None => {
        None
      }
      Some(size) => {
        self.retry += 1;
        Some(size)
      }
    }
  }
}

pub enum GetPasswordFileEntryError {
  NotEnoughMemory,
  SystemCallFailed,
  NoSuchUser,
}

pub fn get_password_file_entry_with_user_id(
  user_id: UserId,
  allocation_config: &AllocationConfig,
) -> Result<PasswordFileEntry, GetPasswordFileEntryError> {
  let mut allocation_control = allocation_config.create_allocation_control();
  let mut entry = unsafe { mem::zeroed::<passwd>() };
  let mut buffer = vec![0; allocation_control.initial_allocation_size()];
  let mut result = ptr::null_mut::<passwd>();

  loop {
    let status = unsafe { 
      libc::getpwuid_r(
        user_id.inner(), 
        &mut entry, 
        buffer.as_mut_ptr(), 
        buffer.len(), 
        &mut result,
      ) 
    };

    if status != libc::ERANGE {
      break;
    }

    let Some(increased_allocation_size) = allocation_control.next_allocation_size() else {
      return Err(GetPasswordFileEntryError::NotEnoughMemory);
    };

    buffer.resize(increased_allocation_size, 0);
  }

  if result.is_null() {
    // There is no such user, or an error has occurred.
    // errno gets set if there’s an error.
    return Err(GetPasswordFileEntryError::SystemCallFailed);
  }

  if result != &mut entry {
    // The result of getpwuid_r should be its input passwd.
    return Err(GetPasswordFileEntryError::SystemCallFailed);
  }

  let sanitized_entry = unsafe { 
    sanitize_password_file_entry(result.read()) 
  };

  Ok(sanitized_entry)
}

pub fn get_password_file_entry_with_user_name(
  user_name: &UserName,
  allocation_config: &AllocationConfig,
) -> Result<PasswordFileEntry, GetPasswordFileEntryError> {
  let mut allocation_control = allocation_config.create_allocation_control();
  let mut entry = unsafe { mem::zeroed::<passwd>() };
  let mut buffer = vec![0; allocation_control.initial_allocation_size()];
  let mut result = ptr::null_mut::<passwd>();

  loop {
    let status = unsafe {
      libc::getpwnam_r(
        user_name.inner().as_ptr(),
        &mut entry,
        buffer.as_mut_ptr(),
        buffer.len(),
        &mut result,
      )
    };

    if status != libc::ERANGE {
      break;
    }

    let Some(increased_allocation_size) = allocation_control.next_allocation_size() else {
      return Err(GetPasswordFileEntryError::NotEnoughMemory);
    };

    buffer.resize(increased_allocation_size, 0);
  }

  if result.is_null() {
    // There is no such user, or an error has occurred.
    // errno gets set if there’s an error.
    return Err(GetPasswordFileEntryError::SystemCallFailed);
  }

  if result != &mut entry {
    // The result of getpwnam_r should be its input passwd.
    return Err(GetPasswordFileEntryError::SystemCallFailed);
  }

  let sanitized_entry = unsafe { 
    sanitize_password_file_entry(result.read()) 
  };

  Ok(sanitized_entry)
}

pub enum GetGroupFileEntryError {
  SystemCallFailed,
  NotEnoughMemory,
  NoSuchGroup,
}

pub fn get_group_file_entry_with_group_id(
  group_id: GroupId,
  allocation_config: &AllocationConfig,
) -> Result<GroupFileEntry, GetGroupFileEntryError> {
  let mut allocation_control = allocation_config.create_allocation_control();
  let mut entry = unsafe { mem::zeroed::<group>() };
  let mut buffer = vec![0; allocation_control.initial_allocation_size()];
  let mut result = ptr::null_mut::<group>();

  loop {
    let status = unsafe { 
      libc::getgrgid_r(
        group_id.inner(), 
        &mut entry, 
        buffer.as_mut_ptr(), 
        buffer.len(), 
        &mut result,
      )
    };

    if status != libc::ERANGE {
      break;
    }

    let Some(increased_allocation_size) = allocation_control.next_allocation_size() else {
      return Err(GetGroupFileEntryError::NotEnoughMemory);
    };

    buffer.resize(increased_allocation_size, 0);
  }

  if result.is_null() {
    // There is no such group, or an error has occurred.
    // errno gets set if there’s an error.
    return Err(GetGroupFileEntryError::SystemCallFailed);
  }

  if result != &mut entry {
    // The result of getgrgid_r should be its input struct.
    return Err(GetGroupFileEntryError::SystemCallFailed);
  }

  let sanitized_entry = unsafe { 
    sanitize_group_file_entry(result.read()) 
  };

  Ok(sanitized_entry)
}

pub fn get_group_file_entry_with_group_name(
  group_name: &GroupName,
  allocation_config: &AllocationConfig,
) -> Result<GroupFileEntry, GetGroupFileEntryError> {
  let mut allocation_control = allocation_config.create_allocation_control();
  let mut entry = unsafe { mem::zeroed::<group>() };
  let mut buffer = vec![0; allocation_control.initial_allocation_size()];
  let mut result = ptr::null_mut::<group>();

  loop {
    let status = unsafe {
      libc::getgrnam_r(
        group_name.inner().as_ptr(),
        &mut entry,
        buffer.as_mut_ptr(),
        buffer.len(),
        &mut result,
      )
    };

    if status != libc::ERANGE {
      break;
    }

    let Some(increased_allocation_size) = allocation_control.next_allocation_size() else {
      return Err(GetGroupFileEntryError::NotEnoughMemory);
    };

    buffer.resize(increased_allocation_size, 0);
  }

  if result.is_null() {
    // There is no such group, or an error has occurred.
    // errno gets set if there’s an error.
    return Err(GetGroupFileEntryError::SystemCallFailed);
  }

  if result != &mut entry {
    // The result of getgrnam_r should be its input struct.
    return Err(GetGroupFileEntryError::SystemCallFailed);
  }

  let sanitized_entry = unsafe { 
    sanitize_group_file_entry(result.read()) 
  };

  Ok(sanitized_entry)
}

pub fn get_real_user_id() -> UserId {
  unsafe { 
    UserId::new(libc::getuid())
  }
}

pub fn get_effective_user_id() -> UserId {
  unsafe {
    UserId::new(libc::geteuid())
  }
}

pub fn get_real_primary_group_id() -> GroupId {
  unsafe {
    GroupId::new(libc::getgid())
  }
}

pub fn get_effective_primary_group_id() -> GroupId {
  unsafe {
    GroupId::new(libc::getegid())
  }
}

struct PasswordFileIterator;

/// # Safety
///
/// This constructor is marked as `unsafe`, which is odd for a crate
/// that’s meant to be a safe interface. It *has* to be unsafe because
/// we cannot guarantee that the underlying C functions,
/// `getpwent`/`setpwent`/`endpwent` that iterate over the system’s
/// `passwd` entries, are called in a thread-safe manner.
///
/// These functions [modify a global
/// state](http://man7.org/linux/man-pages/man3/getpwent.3.html#ATTRIBUTES),
/// and if any are used at the same time, the state could be reset,
/// resulting in a data race. We cannot even place it behind an internal
/// `Mutex`, as there is nothing stopping another `extern` function
/// definition from calling it!
///
/// So to iterate all users, construct the iterator inside an `unsafe`
/// block, then make sure to not make a new instance of it until
/// iteration is over.
///
/// # Examples
///
/// ```
/// use users::all_users;
///
/// let iter = unsafe { all_users() };
/// for user in iter {
///     println!("User #{} ({:?})", user.uid(), user.name());
/// }
/// ```
pub unsafe fn all_users() -> impl Iterator<Item = PasswordFileEntry> {
  #[cfg(not(target_os = "android"))]
  unsafe { libc::setpwent(); }

  PasswordFileIterator
}

impl Drop for PasswordFileIterator {
  #[cfg(target_os = "android")]
  fn drop(&mut self) {
    // nothing to do here
  }

  #[cfg(not(target_os = "android"))]
  fn drop(&mut self) {
    unsafe { libc::endpwent() };
  }
}

impl Iterator for PasswordFileIterator {
  type Item = PasswordFileEntry;

  #[cfg(target_os = "android")]
  fn next(&mut self) -> Option<User> {
    None
  }

  #[cfg(not(target_os = "android"))]
  fn next(&mut self) -> Option<Self::Item> {
    let result = unsafe { libc::getpwent() };

    if result.is_null() {
      None
    } else {
      let entry = unsafe { sanitize_password_file_entry(result.read()) };
      Some(entry)
    }
  }
}

// /// Returns the group access list for the current process.
// ///
// /// # libc functions used
// ///
// /// - [`getgroups`](https://docs.rs/libc/*/libc/fn.getgroups.html)
// ///
// /// # Errors
// ///
// /// This function will return `Err` when an I/O error occurs during the
// /// `getgroups` call.
// ///
// /// # Examples
// ///
// /// ```no_run
// /// use users::group_access_list;
// ///
// /// for group in group_access_list().expect("Error looking up groups") {
// ///     println!("Process can access group #{} ({:?})", group.gid(), group.name());
// /// }
// /// ```
// pub fn group_access_list() -> io::Result<Vec<GroupFileEntry>> {
//   let mut buff: Vec<gid_t> = vec![0; 1024];

//   #[cfg(feature = "logging")]
//   trace!("Running getgroups");

//   let res = unsafe { libc::getgroups(1024, buff.as_mut_ptr()) };

//   if res < 0 {
//     Err(io::Error::last_os_error())
//   } else {
//     let mut groups = buff
//       .into_iter()
//       .filter_map(get_group_file_entry_with_group_id)
//       .collect::<Vec<_>>();
//     groups.dedup_by_key(|i| i.gid());
//     Ok(groups)
//   }
// }

// /// Returns groups for a provided user name and primary group id.
// ///
// /// # libc functions used
// ///
// /// - [`getgrouplist`](https://docs.rs/libc/*/libc/fn.getgrouplist.html)
// ///
// /// # Examples
// ///
// /// ```no_run
// /// use users::get_user_groups;
// ///
// /// for group in get_user_groups("stevedore", 1001).expect("Error looking up groups") {
// ///     println!("User is a member of group #{} ({:?})", group.gid(), group.name());
// /// }
// /// ```
// pub fn get_user_groups<S: AsRef<OsStr> + ?Sized>(username: &S, gid: gid_t) -> Option<Vec<GroupFileEntry>> {
//   // MacOS uses i32 instead of gid_t in getgrouplist for unknown reasons
//   #[cfg(all(unix, target_os = "macos"))]
//   let mut buff: Vec<i32> = vec![0; 1024];
//   #[cfg(all(unix, not(target_os = "macos")))]
//   let mut buff: Vec<gid_t> = vec![0; 1024];

//   let name = CString::new(username.as_ref().as_bytes()).unwrap();
//   let mut count = buff.len() as c_int;

//   #[cfg(feature = "logging")]
//   trace!(
//     "Running getgrouplist for user {:?} and group #{}",
//     username.as_ref(),
//     gid
//   );

//   // MacOS uses i32 instead of gid_t in getgrouplist for unknown reasons
//   #[cfg(all(unix, target_os = "macos"))]
//   let res = unsafe { libc::getgrouplist(name.as_ptr(), gid as i32, buff.as_mut_ptr(), &mut count) };

//   #[cfg(all(unix, not(target_os = "macos")))]
//   let res = unsafe { libc::getgrouplist(name.as_ptr(), gid, buff.as_mut_ptr(), &mut count) };

//   if res < 0 {
//     None
//   } else {
//     buff.dedup();
//     buff
//       .into_iter()
//       .filter_map(|i| get_group_file_entry_with_group_id(i as gid_t))
//       .collect::<Vec<_>>()
//       .into()
//   }
// }


// /// OS-specific extensions to users and groups.
// ///
// /// Every OS has a different idea of what data a user or a group comes with.
// /// Although they all provide a *username*, some OS’ users have an *actual name*
// /// too, or a set of permissions or directories or timestamps associated with
// /// them.
// ///
// /// This module provides extension traits for users and groups that allow
// /// implementors of this library to access this data *as long as a trait is
// /// available*, which requires the OS they’re using to support this data.
// ///
// /// It’s the same method taken by `Metadata` in the standard Rust library,
// /// which has a few cross-platform fields and many more OS-specific fields:
// /// traits in `std::os` provides access to any data that is not guaranteed to
// /// be there in the actual struct.
// pub mod os {

//   /// Extensions to users and groups for Unix platforms.
//   ///
//   /// Although the `passwd` struct is common among Unix systems, its actual
//   /// format can vary. See the definitions in the `base` module to check which
//   /// fields are actually present.
//   #[cfg(any(
//     target_os = "linux",
//     target_os = "android",
//     target_os = "macos",
//     target_os = "freebsd",
//     target_os = "dragonfly",
//     target_os = "openbsd",
//     target_os = "netbsd",
//     target_os = "solaris"
//   ))]
//   pub mod unix {
//     use std::ffi::{OsStr, OsString};
//     use std::path::Path;
//     use super::super::{group, passwd};

//     pub trait IsPasswordFileEntryExtension {
//       fn home_dir(&self) -> &Path;
//       fn shell(&self) -> &Path;
//       fn password(&self) -> &OsStr;
//     }

//     pub trait IsGroupFileEntryExtension {
//       fn members(&self) -> &[OsString];
//     }

//     #[derive(Clone, Debug)]
//     pub struct PasswordFileEntryExtension {
//       /// The path to the user’s home directory.
//       pub home_dir: CString,

//       /// The path to the user’s shell.
//       pub shell: CString,

//       /// The user’s encrypted password.
//       pub password: CString,
//     }

//     impl Default for PasswordFileEntryExtension {
//       fn default() -> Self {
//         Self {
//           home_dir:  "/var/empty".into(),
//           shell: "/bin/false".into(),
//           password: "*".into(),
//         }
//       }
//     }

//     use std::ffi::CStr;
//     use std::ffi::CString;

//     impl PasswordFileEntryExtension {
//       /// Extract the OS-specific fields from the C `passwd` struct that
//       /// we just read.
//       pub(crate) unsafe fn sanitize(entry: passwd) -> Self {
//         #[cfg(target_os = "android")]
//         {
//           Default::default()
//         }
//         #[cfg(not(target_os = "android"))]
//         {
//           let home_dir = CString::from(unsafe { CStr::from_ptr(entry.pw_dir) });
//           let shell = CString::from(unsafe { CStr::from_ptr(entry.pw_shell) });
//           let password = CString::from(unsafe { CStr::from_ptr(entry.pw_passwd) });

//           Self {
//             home_dir,
//             shell,
//             password,
//           }
//         }
//       }
//     }

//     #[cfg(any(target_os = "linux", target_os = "android", target_os = "solaris"))]
//     use super::super::PasswordFileEntry;

//     // #[cfg(any(target_os = "linux", target_os = "android", target_os = "solaris"))]
//     // impl IsPasswordFileEntryExtension for PasswordFileEntry {
//     //   fn home_dir(&self) -> &Path {
//     //     Path::new(&self.extras.home_dir)
//     //   }

//     //   fn shell(&self) -> &Path {
//     //     Path::new(&self.extras.shell)
//     //   }

//     //   fn with_shell<S: AsRef<OsStr> + ?Sized>(mut self, shell: &S) -> Self {
//     //     self.extras.shell = shell.into();
//     //     self
//     //   }

//     //   fn password(&self) -> &OsStr {
//     //     &self.extras.password
//     //   }

//     //   fn with_password<S: AsRef<OsStr> + ?Sized>(mut self, password: &S) -> Self {
//     //     self.extras.password = password.into();
//     //     self
//     //   }
//     // }

//     // /// Unix-specific fields for `Group`s.
//     // #[derive(Clone, Default, Debug)]
//     // pub struct GroupFileEntryExtension {
//     //   /// Vector of usernames that are members of this group.
//     //   pub members: Vec<OsString>,
//     // }

//     // impl GroupFileEntryExtension {
//     //   /// Extract the OS-specific fields from the C `group` struct that
//     //   /// we just read.
//     //   pub(crate) unsafe fn sanitize(entry: group) -> Self {
//     //     Self {
//     //       members: unsafe { members(entry.gr_mem) },
//     //     }
//     //   }
//     // }

//     // impl IsGroupFileEntryExtension for GroupFileEntry {
//     //   fn members(&self) -> &[OsString] {
//     //     &*self.extras.members
//     //   }

//     //   fn add_member<S: AsRef<OsStr> + ?Sized>(mut self, member: &S) -> Self {
//     //     self.extras.members.push(member.into());
//     //     self
//     //   }
//     // }
//   }

//   /// Extensions to users and groups for BSD platforms.
//   ///
//   /// These platforms have `change` and `expire` fields in their `passwd`
//   /// C structs.
//   #[cfg(any(
//     target_os = "macos",
//     target_os = "freebsd",
//     target_os = "dragonfly",
//     target_os = "openbsd",
//     target_os = "netbsd"
//   ))]
//   pub mod bsd {
//     use super::super::{PasswordFileEntry, passwd};
//     use libc::time_t;
//     use std::ffi::OsStr;
//     use std::path::Path;

//     /// BSD-specific fields for `User`s.
//     #[derive(Clone, Debug)]
//     pub struct UserExtras {
//       /// Fields specific to Unix, rather than just BSD. (This struct is
//       /// a superset, so it has to have all the other fields in it, too).
//       pub extras: super::unix::UserExtras,

//       /// Password change time.
//       pub change: time_t,

//       /// Password expiry time.
//       pub expire: time_t,
//     }

//     impl UserExtras {
//       /// Extract the OS-specific fields from the C `passwd` struct that
//       /// we just read.
//       pub(crate) unsafe fn from_passwd(passwd: c_passwd) -> Self {
//         Self {
//           change: passwd.pw_change,
//           expire: passwd.pw_expire,
//           extras: super::unix::UserExtras::from_passwd(passwd),
//         }
//       }
//     }

//     impl super::unix::UserExt for User {
//       fn home_dir(&self) -> &Path {
//         Path::new(&self.extras.extras.home_dir)
//       }

//       fn with_home_dir<S: AsRef<OsStr> + ?Sized>(mut self, home_dir: &S) -> Self {
//         self.extras.extras.home_dir = home_dir.into();
//         self
//       }

//       fn shell(&self) -> &Path {
//         Path::new(&self.extras.extras.shell)
//       }

//       fn with_shell<S: AsRef<OsStr> + ?Sized>(mut self, shell: &S) -> Self {
//         self.extras.extras.shell = shell.into();
//         self
//       }

//       fn password(&self) -> &OsStr {
//         &self.extras.extras.password
//       }

//       fn with_password<S: AsRef<OsStr> + ?Sized>(mut self, password: &S) -> Self {
//         self.extras.extras.password = password.into();
//         self
//       }
//     }

//     /// BSD-specific accessors for `User`s.
//     pub trait UserExt {
//       /// Returns this user’s password change timestamp.
//       fn password_change_time(&self) -> time_t;

//       /// Returns this user’s password expiry timestamp.
//       fn password_expire_time(&self) -> time_t;
//     }

//     impl UserExt for User {
//       fn password_change_time(&self) -> time_t {
//         self.extras.change
//       }

//       fn password_expire_time(&self) -> time_t {
//         self.extras.expire
//       }
//     }

//     impl Default for UserExtras {
//       fn default() -> Self {
//         Self {
//           extras: super::unix::UserExtras::default(),
//           change: 0,
//           expire: 0,
//         }
//       }
//     }
//   }

//   /// Any extra fields on a `User` specific to the current platform.
//   #[cfg(any(
//     target_os = "macos",
//     target_os = "freebsd",
//     target_os = "dragonfly",
//     target_os = "openbsd",
//     target_os = "netbsd"
//   ))]
//   pub type UserExtras = bsd::UserExtras;

//   /// Any extra fields on a `User` specific to the current platform.
//   #[cfg(any(target_os = "linux", target_os = "android", target_os = "solaris"))]
//   pub type UserExtras = unix::PasswordFileEntryExtension;

//   /// Any extra fields on a `Group` specific to the current platform.
//   #[cfg(any(
//     target_os = "linux",
//     target_os = "android",
//     target_os = "macos",
//     target_os = "freebsd",
//     target_os = "dragonfly",
//     target_os = "openbsd",
//     target_os = "netbsd",
//     target_os = "solaris"
//   ))]
//   pub type GroupExtras = unix::GroupFileEntryExtension;
// }

// // /// Reads data from a `*char` field in `c_passwd` or `g_group`. The return
// // /// type will be an `Arc<OsStr>` if the text is meant to be shared in a cache,
// // /// or a plain `OsString` if it’s not.
// // ///
// // /// The underlying buffer is managed by the C library, not by us, so we *need*
// // /// to move data out of it before the next user gets read.
// // unsafe fn from_raw_buf<'a, T>(p: *const c_char) -> T
// // where
// //   T: From<&'a OsStr>,
// // {
// //   T::from(OsStr::from_bytes(CStr::from_ptr(p).to_bytes()))
// // }

// // /// Expand a list of group members to a vector of strings.
// // ///
// // /// The list of members is, in true C fashion, a pointer to a pointer of
// // /// characters, terminated by a null pointer. We check `members[0]`, then
// // /// `members[1]`, and so on, until that null pointer is reached. It doesn’t
// // /// specify whether we should expect a null pointer or a pointer to a null
// // /// pointer, so we check for both here!
// // unsafe fn members(groups: *mut *mut c_char) -> Vec<OsString> {
// //   let mut members = Vec::new();

// //   for i in 0.. {
// //     let username = groups.offset(i);

// //     if username.is_null() || (*username).is_null() {
// //       break;
// //     } else {
// //       members.push(from_raw_buf(*username));
// //     }
// //   }

// //   members
// // }

pub const ALLOCATION_CONFIG: AllocationConfig = AllocationConfig {
  maximum_memory_allocation_retries: 3,
  memory_allocation_increment_factor: 3,
};