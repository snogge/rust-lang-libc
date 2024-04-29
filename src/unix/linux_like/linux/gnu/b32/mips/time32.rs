s! {
    pub struct stat64 {
        pub st_dev: c_ulong,
        st_pad1: [c_long; 3],
        pub st_ino: crate::ino64_t,
        pub st_mode: crate::mode_t,
        pub st_nlink: crate::nlink_t,
        pub st_uid: crate::uid_t,
        pub st_gid: crate::gid_t,
        pub st_rdev: c_ulong,
        st_pad2: [c_long; 2],
        pub st_size: crate::off64_t,
        pub st_atime: crate::time_t,
        pub st_atime_nsec: c_long,
        pub st_mtime: crate::time_t,
        pub st_mtime_nsec: c_long,
        pub st_ctime: crate::time_t,
        pub st_ctime_nsec: c_long,
        pub st_blksize: crate::blksize_t,
        st_pad3: c_long,
        pub st_blocks: crate::blkcnt64_t,
        st_pad5: [c_long; 14],
    }
}
