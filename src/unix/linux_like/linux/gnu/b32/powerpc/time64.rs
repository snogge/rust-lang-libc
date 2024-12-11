use crate::{c_ulong, pid_t, stat, time_t};
pub type stat64 = stat;

s! {
    pub struct shmid_ds {
        pub shm_perm: crate::ipc_perm,
        pub shm_segsz: crate::size_t,
        pub shm_atime: time_t,
        pub shm_dtime: time_t,
        pub shm_ctime: time_t,
        pub shm_cpid: pid_t,
        pub shm_lpid: pid_t,
        pub shm_nattch: crate::shmatt_t,
        __glibc_reserved5: c_ulong,
        __glibc_reserved6: c_ulong,
    }
}
