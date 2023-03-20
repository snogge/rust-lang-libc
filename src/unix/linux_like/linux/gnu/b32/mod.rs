//! 32-bit specific definitions for linux-like values

use ntptimeval;
use pthread_mutex_t;

pub type c_long = i32;
pub type c_ulong = u32;
pub type clock_t = i32;

pub type shmatt_t = ::c_ulong;
pub type msgqnum_t = ::c_ulong;
pub type msglen_t = ::c_ulong;
pub type nlink_t = u32;
pub type __u64 = ::c_ulonglong;
pub type __s64 = ::c_longlong;
pub type __fsword_t = i32;
pub type fsblkcnt64_t = u64;
pub type fsfilcnt64_t = u64;
pub type __syscall_ulong_t = ::c_ulong;
pub type __syscall_slong_t = ::c_long;

cfg_if! {
    if #[cfg(target_arch = "riscv32")] {
        pub type time_t = i64;
        pub type suseconds_t = i64;
        pub type ino_t = u64;
        pub type off_t = i64;
        pub type blkcnt_t = i64;
        pub type fsblkcnt_t = u64;
        pub type fsfilcnt_t = u64;
        pub type rlim_t = u64;
        pub type blksize_t = i64;
    } else if #[cfg(gnu_time64_abi)] {
        pub type time_t = i64;
        pub type suseconds_t = i32;
        pub type ino_t = u64;
        pub type off_t = i64;
        pub type blkcnt_t = i64;
        pub type fsblkcnt_t = u64;
        pub type fsfilcnt_t = u64;
        pub type rlim_t = u64;
        pub type blksize_t = i32;
    } else {
        pub type time_t = i32;
        pub type suseconds_t = i32;
        pub type ino_t = u32;
        pub type off_t = i32;
        pub type blkcnt_t = i32;
        pub type fsblkcnt_t = ::c_ulong;
        pub type fsfilcnt_t = ::c_ulong;
        pub type rlim_t = c_ulong;
        pub type blksize_t = i32;
    }
}
cfg_if! {
    if #[cfg(any(target_arch = "arm", target_arch="riscv32", target_arch="sparc"))] {
        pub type stat64 = stat;
        pub type statfs64 = statfs;
        pub type statvfs64 = statvfs;
    }
}

s! {
    pub struct statvfs {
        pub f_bsize: ::c_ulong,
        pub f_frsize: ::c_ulong,
        pub f_blocks: ::fsblkcnt_t,
        pub f_bfree: ::fsblkcnt_t,
        pub f_bavail: ::fsblkcnt_t,
        pub f_files: ::fsfilcnt_t,
        pub f_ffree: ::fsfilcnt_t,
        pub f_favail: ::fsfilcnt_t,
        pub f_fsid: ::c_ulong,
        __f_unused: ::c_int,
        pub f_flag: ::c_ulong,
        pub f_namemax: ::c_ulong,
        __f_spare: [::c_int; 6],
    }

    pub struct pthread_attr_t {
        __size: [u32; 9]
    }

    pub struct sigset_t {
        __val: [::c_ulong; 32],
    }

    pub struct sysinfo {
        pub uptime: ::c_long,
        pub loads: [::c_ulong; 3],
        pub totalram: ::c_ulong,
        pub freeram: ::c_ulong,
        pub sharedram: ::c_ulong,
        pub bufferram: ::c_ulong,
        pub totalswap: ::c_ulong,
        pub freeswap: ::c_ulong,
        pub procs: ::c_ushort,
        #[deprecated(
            since = "0.2.58",
            note = "This padding field might become private in the future"
        )]
        pub pad: ::c_ushort,
        pub totalhigh: ::c_ulong,
        pub freehigh: ::c_ulong,
        pub mem_unit: ::c_uint,
        pub _f: [::c_char; 8],
    }

    pub struct semid_ds {
        pub sem_perm: ipc_perm,
        pub sem_otime: ::time_t,
        #[cfg(not(any(
            target_arch = "mips",
            target_arch = "mips32r6",
            target_arch = "powerpc",
            target_arch = "arm",
            target_arch = "x86"
        )))]
        __reserved: ::__syscall_ulong_t,
        pub sem_ctime: ::time_t,
        pub sem_nsems: ::__syscall_ulong_t,
        #[cfg(not(any(
            target_arch = "mips",
            target_arch = "mips32r6",
            target_arch = "powerpc",
            target_arch = "arm",
            target_arch = "x86"
        )))]
        __reserved2: ::__syscall_ulong_t,
        __glibc_reserved3: ::__syscall_ulong_t,
        __glibc_reserved4: ::__syscall_ulong_t,
    }

    pub struct timex {
        pub modes: ::c_uint,

        #[cfg(not(gnu_time64_abi))]
        pub offset: ::__syscall_slong_t,
        #[cfg(gnu_time64_abi)]
        pub __unused_pad1: i32,
        #[cfg(gnu_time64_abi)]
        pub offset: ::c_longlong,
        #[cfg(not(gnu_time64_abi))]
        pub freq: ::__syscall_slong_t,
        #[cfg(gnu_time64_abi)]
        pub freq: ::c_longlong,
        #[cfg(not(gnu_time64_abi))]
        pub maxerror: ::__syscall_slong_t,
        #[cfg(gnu_time64_abi)]
        pub maxerror: ::c_longlong,
        #[cfg(not(gnu_time64_abi))]
        pub esterror: ::__syscall_slong_t,
        #[cfg(gnu_time64_abi)]
        pub esterror: ::c_longlong,
        pub status: ::c_int,
        #[cfg(not(gnu_time64_abi))]
        pub constant: ::__syscall_slong_t,
        #[cfg(gnu_time64_abi)]
        pub __unused_pad2: i32,
        #[cfg(gnu_time64_abi)]
        pub constant: ::c_longlong,
        #[cfg(not(gnu_time64_abi))]
        pub precision: ::__syscall_slong_t,
        #[cfg(gnu_time64_abi)]
        pub precision: ::c_longlong,
        #[cfg(not(gnu_time64_abi))]
        pub tolerance: ::__syscall_slong_t,
        #[cfg(gnu_time64_abi)]
        pub tolerance: ::c_longlong,
        pub time: ::timeval,
        #[cfg(not(gnu_time64_abi))]
        pub tick: ::__syscall_slong_t,
        #[cfg(gnu_time64_abi)]
        pub tick: ::c_longlong,
        #[cfg(not(gnu_time64_abi))]
        pub ppsfreq: ::__syscall_slong_t,
        #[cfg(gnu_time64_abi)]
        pub ppsfreq: ::c_longlong,
        #[cfg(not(gnu_time64_abi))]
        pub jitter: ::__syscall_slong_t,
        #[cfg(gnu_time64_abi)]
        pub jitter: ::c_longlong,
        pub shift: ::c_int,
        #[cfg(not(gnu_time64_abi))]
        pub stabil: ::__syscall_slong_t,
        #[cfg(gnu_time64_abi)]
        pub __unused_pad3: i32,
        #[cfg(gnu_time64_abi)]
        pub stabil: ::c_longlong,
        #[cfg(not(gnu_time64_abi))]
        pub jitcnt: ::__syscall_slong_t,
        #[cfg(gnu_time64_abi)]
        pub jitcnt: ::c_longlong,
        #[cfg(not(gnu_time64_abi))]
        pub calcnt: ::__syscall_slong_t,
        #[cfg(gnu_time64_abi)]
        pub calcnt: ::c_longlong,
        #[cfg(not(gnu_time64_abi))]
        pub errcnt: ::__syscall_slong_t,
        #[cfg(gnu_time64_abi)]
        pub errcnt: ::c_longlong,
        #[cfg(not(gnu_time64_abi))]
        pub stbcnt: ::__syscall_slong_t,
        #[cfg(gnu_time64_abi)]
        pub stbcnt: ::c_longlong,
        pub tai: ::c_int,
        pub __unused1: i32,
        pub __unused2: i32,
        pub __unused3: i32,
        pub __unused4: i32,
        pub __unused5: i32,
        pub __unused6: i32,
        pub __unused7: i32,
        pub __unused8: i32,
        pub __unused9: i32,
        pub __unused10: i32,
        pub __unused11: i32,
    }
}

pub const POSIX_FADV_DONTNEED: ::c_int = 4;
pub const POSIX_FADV_NOREUSE: ::c_int = 5;

pub const F_OFD_GETLK: ::c_int = 36;
pub const F_OFD_SETLK: ::c_int = 37;
pub const F_OFD_SETLKW: ::c_int = 38;

pub const __SIZEOF_PTHREAD_CONDATTR_T: usize = 4;
pub const __SIZEOF_PTHREAD_MUTEX_T: usize = 24;
pub const __SIZEOF_PTHREAD_RWLOCK_T: usize = 32;
pub const __SIZEOF_PTHREAD_BARRIER_T: usize = 20;
pub const __SIZEOF_PTHREAD_MUTEXATTR_T: usize = 4;
pub const __SIZEOF_PTHREAD_RWLOCKATTR_T: usize = 8;
pub const __SIZEOF_PTHREAD_BARRIERATTR_T: usize = 4;

cfg_if! {
    if #[cfg(target_arch = "sparc")] {
        pub const O_NOATIME: ::c_int = 0x200000;
        pub const O_PATH: ::c_int = 0x1000000;
        pub const O_TMPFILE: ::c_int = 0x2000000 | O_DIRECTORY;

        pub const SA_ONSTACK: ::c_int = 1;

        pub const PTRACE_DETACH: ::c_uint = 11;

        pub const F_RDLCK: ::c_int = 1;
        pub const F_WRLCK: ::c_int = 2;
        pub const F_UNLCK: ::c_int = 3;

        pub const SFD_CLOEXEC: ::c_int = 0x400000;

        pub const NCCS: usize = 17;

        pub const O_TRUNC: ::c_int = 0x400;
        pub const O_CLOEXEC: ::c_int = 0x400000;

        pub const EBFONT: ::c_int = 109;
        pub const ENOSTR: ::c_int = 72;
        pub const ENODATA: ::c_int = 111;
        pub const ETIME: ::c_int = 73;
        pub const ENOSR: ::c_int = 74;
        pub const ENONET: ::c_int = 80;
        pub const ENOPKG: ::c_int = 113;
        pub const EREMOTE: ::c_int = 71;
        pub const ENOLINK: ::c_int = 82;
        pub const EADV: ::c_int = 83;
        pub const ESRMNT: ::c_int = 84;
        pub const ECOMM: ::c_int = 85;
        pub const EPROTO: ::c_int = 86;
        pub const EDOTDOT: ::c_int = 88;

        pub const SA_NODEFER: ::c_int = 0x20;
        pub const SA_RESETHAND: ::c_int = 0x4;
        pub const SA_RESTART: ::c_int = 0x2;
        pub const SA_NOCLDSTOP: ::c_int = 0x00000008;

        pub const EPOLL_CLOEXEC: ::c_int = 0x400000;

        pub const EFD_CLOEXEC: ::c_int = 0x400000;
    } else {
        pub const O_NOATIME: ::c_int = 0o1000000;
        pub const O_PATH: ::c_int = 0o10000000;
        pub const O_TMPFILE: ::c_int = 0o20000000 | O_DIRECTORY;

        pub const SA_ONSTACK: ::c_int = 0x08000000;

        pub const PTRACE_DETACH: ::c_uint = 17;

        pub const F_RDLCK: ::c_int = 0;
        pub const F_WRLCK: ::c_int = 1;
        pub const F_UNLCK: ::c_int = 2;

        pub const SFD_CLOEXEC: ::c_int = 0x080000;

        pub const NCCS: usize = 32;

        pub const O_TRUNC: ::c_int = 512;
        pub const O_CLOEXEC: ::c_int = 0x80000;
        pub const EBFONT: ::c_int = 59;
        pub const ENOSTR: ::c_int = 60;
        pub const ENODATA: ::c_int = 61;
        pub const ETIME: ::c_int = 62;
        pub const ENOSR: ::c_int = 63;
        pub const ENONET: ::c_int = 64;
        pub const ENOPKG: ::c_int = 65;
        pub const EREMOTE: ::c_int = 66;
        pub const ENOLINK: ::c_int = 67;
        pub const EADV: ::c_int = 68;
        pub const ESRMNT: ::c_int = 69;
        pub const ECOMM: ::c_int = 70;
        pub const EPROTO: ::c_int = 71;
        pub const EDOTDOT: ::c_int = 73;

        pub const SA_NODEFER: ::c_int = 0x40000000;
        pub const SA_RESETHAND: ::c_int = 0x80000000;
        pub const SA_RESTART: ::c_int = 0x10000000;
        pub const SA_NOCLDSTOP: ::c_int = 0x00000001;

        pub const EPOLL_CLOEXEC: ::c_int = 0x80000;

        pub const EFD_CLOEXEC: ::c_int = 0x80000;
    }
}
cfg_if! {
    if #[cfg(target_arch = "sparc")] {
        pub const F_SETLK: ::c_int = 8;
        pub const F_SETLKW: ::c_int = 9;
    } else if #[cfg(all(gnu_time64_abi, target_arch = "mips"))] {
        pub const F_SETLK: ::c_int = 34;
        pub const F_SETLKW: ::c_int = 35;
    } else if #[cfg(gnu_time64_abi)] {
        pub const F_SETLK: ::c_int = 13;
        pub const F_SETLKW: ::c_int = 14;
    } else {
        pub const F_SETLK: ::c_int = 6;
        pub const F_SETLKW: ::c_int = 7;
    }
}

#[cfg(target_endian = "little")]
pub const PTHREAD_RECURSIVE_MUTEX_INITIALIZER_NP: ::pthread_mutex_t = pthread_mutex_t {
    size: [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
};
#[cfg(target_endian = "little")]
pub const PTHREAD_ERRORCHECK_MUTEX_INITIALIZER_NP: ::pthread_mutex_t = pthread_mutex_t {
    size: [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
};
#[cfg(target_endian = "little")]
pub const PTHREAD_ADAPTIVE_MUTEX_INITIALIZER_NP: ::pthread_mutex_t = pthread_mutex_t {
    size: [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
};
#[cfg(target_endian = "big")]
pub const PTHREAD_RECURSIVE_MUTEX_INITIALIZER_NP: ::pthread_mutex_t = pthread_mutex_t {
    size: [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
};
#[cfg(target_endian = "big")]
pub const PTHREAD_ERRORCHECK_MUTEX_INITIALIZER_NP: ::pthread_mutex_t = pthread_mutex_t {
    size: [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
};
#[cfg(target_endian = "big")]
pub const PTHREAD_ADAPTIVE_MUTEX_INITIALIZER_NP: ::pthread_mutex_t = pthread_mutex_t {
    size: [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
};

pub const PTRACE_GETFPREGS: ::c_uint = 14;
pub const PTRACE_SETFPREGS: ::c_uint = 15;
pub const PTRACE_GETREGS: ::c_uint = 12;
pub const PTRACE_SETREGS: ::c_uint = 13;

extern "C" {
    pub fn sysctl(
        name: *mut ::c_int,
        namelen: ::c_int,
        oldp: *mut ::c_void,
        oldlenp: *mut ::size_t,
        newp: *mut ::c_void,
        newlen: ::size_t,
    ) -> ::c_int;
    #[cfg_attr(gnu_time64_abi, link_name = "__ntp_gettime64")]
    pub fn ntp_gettime(buf: *mut ntptimeval) -> ::c_int;
}

cfg_if! {
    if #[cfg(target_arch = "x86")] {
        mod x86;
        pub use self::x86::*;
    } else if #[cfg(target_arch = "arm")] {
        mod arm;
        pub use self::arm::*;
    } else if #[cfg(any(target_arch = "mips", target_arch = "mips32r6"))] {
        mod mips;
        pub use self::mips::*;
    } else if #[cfg(target_arch = "m68k")] {
        mod m68k;
        pub use self::m68k::*;
    } else if #[cfg(target_arch = "powerpc")] {
        mod powerpc;
        pub use self::powerpc::*;
    } else if #[cfg(target_arch = "sparc")] {
        mod sparc;
        pub use self::sparc::*;
    } else if #[cfg(target_arch = "riscv32")] {
        mod riscv32;
        pub use self::riscv32::*;
    } else if #[cfg(target_arch = "csky")] {
        mod csky;
        pub use self::csky::*;
    } else {
        // Unknown target_arch
    }
}

cfg_if! {
    if #[cfg(not(any(
        target_arch = "mips",
        target_arch = "mips32r6",
        target_arch = "powerpc",
        target_arch = "riscv32",
        target_arch = "sparc",
    )))] {
        mod generic;
        pub use self::generic::*;
    }
}
