use crate::prelude::*;
use crate::time_t;

s! {
    pub struct stat64 {
        pub st_dev: crate::dev_t,
        __pad1: c_uint,
        __st_ino: crate::ino_t,
        pub st_mode: crate::mode_t,
        pub st_nlink: crate::nlink_t,
        pub st_uid: crate::uid_t,
        pub st_gid: crate::gid_t,
        pub st_rdev: crate::dev_t,
        __pad2: c_uint,
        pub st_size: crate::off64_t,
        pub st_blksize: crate::blksize_t,
        pub st_blocks: crate::blkcnt64_t,
        pub st_atime: time_t,
        pub st_atime_nsec: c_long,
        pub st_mtime: time_t,
        pub st_mtime_nsec: c_long,
        pub st_ctime: time_t,
        pub st_ctime_nsec: c_long,
        pub st_ino: crate::ino64_t,
    }
}
