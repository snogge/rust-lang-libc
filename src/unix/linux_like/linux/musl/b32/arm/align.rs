s_no_extra_traits! {
    #[allow(missing_debug_implementations)]
    #[repr(align(8))]
    #[allow(dead_code)]
    pub struct max_align_t {
        priv_: (i64, i64)
    }
}
