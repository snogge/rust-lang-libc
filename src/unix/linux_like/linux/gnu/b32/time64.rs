//! 32-bit specific definitions for linux-like values when gnu_time64_abi is set

use crate::prelude::*;

s! {
    pub struct timex {
        pub modes: c_uint,

        __unused_pad1: i32,
        pub offset: c_longlong,
        pub freq: c_longlong,
        pub maxerror: c_longlong,
        pub esterror: c_longlong,
        pub status: c_int,
        __unused_pad2: i32,
        pub constant: c_longlong,
        pub precision: c_longlong,
        pub tolerance: c_longlong,
        pub time: crate::timeval,
        pub tick: c_longlong,
        pub ppsfreq: c_longlong,
        pub jitter: c_longlong,
        pub shift: c_int,
        __unused_pad3: i32,
        pub stabil: c_longlong,
        pub jitcnt: c_longlong,
        pub calcnt: c_longlong,
        pub errcnt: c_longlong,
        pub stbcnt: c_longlong,
        pub tai: c_int,
        __unused1: i32,
        __unused2: i32,
        __unused3: i32,
        __unused4: i32,
        __unused5: i32,
        __unused6: i32,
        __unused7: i32,
        __unused8: i32,
        __unused9: i32,
        __unused10: i32,
        __unused11: i32,
    }
}
