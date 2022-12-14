#[derive(Clone, Hash)]
/// Flags group `x86`.
pub struct Flags {
    bytes: [u8; 5],
}
impl Flags {
    /// Create flags x86 settings group.
    #[allow(unused_variables)]
    pub fn new(shared: &settings::Flags, builder: Builder) -> Self {
        let bvec = builder.state_for("x86");
        let mut x86 = Self { bytes: [0; 5] };
        debug_assert_eq!(bvec.len(), 2);
        x86.bytes[0..2].copy_from_slice(&bvec);
        // Precompute #15.
        if shared.emit_all_ones_funcaddrs() && !(shared.is_pic()) {
            x86.bytes[1] |= 1 << 7;
        }
        // Precompute #16.
        if shared.is_pic() {
            x86.bytes[2] |= 1 << 0;
        }
        // Precompute #17.
        if !(shared.emit_all_ones_funcaddrs()) && !(shared.is_pic()) {
            x86.bytes[2] |= 1 << 1;
        }
        // Precompute #18.
        if !(shared.is_pic()) {
            x86.bytes[2] |= 1 << 2;
        }
        // Precompute #19.
        if shared.enable_simd() && x86.has_avx2() {
            x86.bytes[2] |= 1 << 3;
        }
        // Precompute #20.
        if shared.enable_simd() && x86.has_avx512bitalg() {
            x86.bytes[2] |= 1 << 4;
        }
        // Precompute #21.
        if shared.enable_simd() && x86.has_avx512dq() {
            x86.bytes[2] |= 1 << 5;
        }
        // Precompute #22.
        if shared.enable_simd() && x86.has_avx512f() {
            x86.bytes[2] |= 1 << 6;
        }
        // Precompute #23.
        if shared.enable_simd() && x86.has_avx512vbmi() {
            x86.bytes[2] |= 1 << 7;
        }
        // Precompute #24.
        if shared.enable_simd() && x86.has_avx512vl() {
            x86.bytes[3] |= 1 << 0;
        }
        // Precompute #25.
        if shared.enable_simd() && x86.has_avx() {
            x86.bytes[3] |= 1 << 1;
        }
        // Precompute #26.
        if x86.has_bmi1() {
            x86.bytes[3] |= 1 << 2;
        }
        // Precompute #27.
        if x86.has_lzcnt() {
            x86.bytes[3] |= 1 << 3;
        }
        // Precompute #28.
        if x86.has_popcnt() && x86.has_sse42() {
            x86.bytes[3] |= 1 << 4;
        }
        // Precompute #29.
        if x86.has_sse41() {
            x86.bytes[3] |= 1 << 5;
        }
        // Precompute #30.
        if shared.enable_simd() && x86.has_sse41() {
            x86.bytes[3] |= 1 << 6;
        }
        // Precompute #31.
        if x86.has_sse41() && x86.has_sse42() {
            x86.bytes[3] |= 1 << 7;
        }
        // Precompute #32.
        if shared.enable_simd() && x86.has_sse41() && x86.has_sse42() {
            x86.bytes[4] |= 1 << 0;
        }
        // Precompute #33.
        if x86.has_ssse3() {
            x86.bytes[4] |= 1 << 1;
        }
        // Precompute #34.
        if shared.enable_simd() && x86.has_ssse3() {
            x86.bytes[4] |= 1 << 2;
        }
        x86
    }
}
impl Flags {
    /// Iterates the setting values.
    pub fn iter(&self) -> impl Iterator<Item = Value> {
        let mut bytes = [0; 2];
        bytes.copy_from_slice(&self.bytes[0..2]);
        DESCRIPTORS.iter().filter_map(move |d| {
            let values = match &d.detail {
                detail::Detail::Preset => return None,
                detail::Detail::Enum { last, enumerators } => Some(TEMPLATE.enums(*last, *enumerators)),
                _ => None
            };
            Some(Value{ name: d.name, detail: d.detail, values, value: bytes[d.offset as usize] })
        })
    }
}
/// User-defined settings.
#[allow(dead_code)]
impl Flags {
    /// Get a view of the boolean predicates.
    pub fn predicate_view(&self) -> crate::settings::PredicateView {
        crate::settings::PredicateView::new(&self.bytes[0..])
    }
    /// Dynamic numbered predicate getter.
    fn numbered_predicate(&self, p: usize) -> bool {
        self.bytes[0 + p / 8] & (1 << (p % 8)) != 0
    }
    /// Has support for SSE3.
    /// SSE3: CPUID.01H:ECX.SSE3[bit 0]
    pub fn has_sse3(&self) -> bool {
        self.numbered_predicate(0)
    }
    /// Has support for SSSE3.
    /// SSSE3: CPUID.01H:ECX.SSSE3[bit 9]
    pub fn has_ssse3(&self) -> bool {
        self.numbered_predicate(1)
    }
    /// Has support for SSE4.1.
    /// SSE4.1: CPUID.01H:ECX.SSE4_1[bit 19]
    pub fn has_sse41(&self) -> bool {
        self.numbered_predicate(2)
    }
    /// Has support for SSE4.2.
    /// SSE4.2: CPUID.01H:ECX.SSE4_2[bit 20]
    pub fn has_sse42(&self) -> bool {
        self.numbered_predicate(3)
    }
    /// Has support for AVX.
    /// AVX: CPUID.01H:ECX.AVX[bit 28]
    pub fn has_avx(&self) -> bool {
        self.numbered_predicate(4)
    }
    /// Has support for AVX2.
    /// AVX2: CPUID.07H:EBX.AVX2[bit 5]
    pub fn has_avx2(&self) -> bool {
        self.numbered_predicate(5)
    }
    /// Has support for AVX512BITALG.
    /// AVX512BITALG: CPUID.07H:ECX.AVX512BITALG[bit 12]
    pub fn has_avx512bitalg(&self) -> bool {
        self.numbered_predicate(6)
    }
    /// Has support for AVX512DQ.
    /// AVX512DQ: CPUID.07H:EBX.AVX512DQ[bit 17]
    pub fn has_avx512dq(&self) -> bool {
        self.numbered_predicate(7)
    }
    /// Has support for AVX512VL.
    /// AVX512VL: CPUID.07H:EBX.AVX512VL[bit 31]
    pub fn has_avx512vl(&self) -> bool {
        self.numbered_predicate(8)
    }
    /// Has support for AVX512VMBI.
    /// AVX512VBMI: CPUID.07H:ECX.AVX512VBMI[bit 1]
    pub fn has_avx512vbmi(&self) -> bool {
        self.numbered_predicate(9)
    }
    /// Has support for AVX512F.
    /// AVX512F: CPUID.07H:EBX.AVX512F[bit 16]
    pub fn has_avx512f(&self) -> bool {
        self.numbered_predicate(10)
    }
    /// Has support for POPCNT.
    /// POPCNT: CPUID.01H:ECX.POPCNT[bit 23]
    pub fn has_popcnt(&self) -> bool {
        self.numbered_predicate(11)
    }
    /// Has support for BMI1.
    /// BMI1: CPUID.(EAX=07H, ECX=0H):EBX.BMI1[bit 3]
    pub fn has_bmi1(&self) -> bool {
        self.numbered_predicate(12)
    }
    /// Has support for BMI2.
    /// BMI2: CPUID.(EAX=07H, ECX=0H):EBX.BMI2[bit 8]
    pub fn has_bmi2(&self) -> bool {
        self.numbered_predicate(13)
    }
    /// Has support for LZCNT.
    /// LZCNT: CPUID.EAX=80000001H:ECX.LZCNT[bit 5]
    pub fn has_lzcnt(&self) -> bool {
        self.numbered_predicate(14)
    }
    /// Computed predicate `shared.emit_all_ones_funcaddrs() && !(shared.is_pic())`.
    pub fn all_ones_funcaddrs_and_not_is_pic(&self) -> bool {
        self.numbered_predicate(15)
    }
    /// Computed predicate `shared.is_pic()`.
    pub fn is_pic(&self) -> bool {
        self.numbered_predicate(16)
    }
    /// Computed predicate `!(shared.emit_all_ones_funcaddrs()) && !(shared.is_pic())`.
    pub fn not_all_ones_funcaddrs_and_not_is_pic(&self) -> bool {
        self.numbered_predicate(17)
    }
    /// Computed predicate `!(shared.is_pic())`.
    pub fn not_is_pic(&self) -> bool {
        self.numbered_predicate(18)
    }
    /// Computed predicate `shared.enable_simd() && x86.has_avx2()`.
    pub fn use_avx2_simd(&self) -> bool {
        self.numbered_predicate(19)
    }
    /// Computed predicate `shared.enable_simd() && x86.has_avx512bitalg()`.
    pub fn use_avx512bitalg_simd(&self) -> bool {
        self.numbered_predicate(20)
    }
    /// Computed predicate `shared.enable_simd() && x86.has_avx512dq()`.
    pub fn use_avx512dq_simd(&self) -> bool {
        self.numbered_predicate(21)
    }
    /// Computed predicate `shared.enable_simd() && x86.has_avx512f()`.
    pub fn use_avx512f_simd(&self) -> bool {
        self.numbered_predicate(22)
    }
    /// Computed predicate `shared.enable_simd() && x86.has_avx512vbmi()`.
    pub fn use_avx512vbmi_simd(&self) -> bool {
        self.numbered_predicate(23)
    }
    /// Computed predicate `shared.enable_simd() && x86.has_avx512vl()`.
    pub fn use_avx512vl_simd(&self) -> bool {
        self.numbered_predicate(24)
    }
    /// Computed predicate `shared.enable_simd() && x86.has_avx()`.
    pub fn use_avx_simd(&self) -> bool {
        self.numbered_predicate(25)
    }
    /// Computed predicate `x86.has_bmi1()`.
    pub fn use_bmi1(&self) -> bool {
        self.numbered_predicate(26)
    }
    /// Computed predicate `x86.has_lzcnt()`.
    pub fn use_lzcnt(&self) -> bool {
        self.numbered_predicate(27)
    }
    /// Computed predicate `x86.has_popcnt() && x86.has_sse42()`.
    pub fn use_popcnt(&self) -> bool {
        self.numbered_predicate(28)
    }
    /// Computed predicate `x86.has_sse41()`.
    pub fn use_sse41(&self) -> bool {
        self.numbered_predicate(29)
    }
    /// Computed predicate `shared.enable_simd() && x86.has_sse41()`.
    pub fn use_sse41_simd(&self) -> bool {
        self.numbered_predicate(30)
    }
    /// Computed predicate `x86.has_sse41() && x86.has_sse42()`.
    pub fn use_sse42(&self) -> bool {
        self.numbered_predicate(31)
    }
    /// Computed predicate `shared.enable_simd() && x86.has_sse41() && x86.has_sse42()`.
    pub fn use_sse42_simd(&self) -> bool {
        self.numbered_predicate(32)
    }
    /// Computed predicate `x86.has_ssse3()`.
    pub fn use_ssse3(&self) -> bool {
        self.numbered_predicate(33)
    }
    /// Computed predicate `shared.enable_simd() && x86.has_ssse3()`.
    pub fn use_ssse3_simd(&self) -> bool {
        self.numbered_predicate(34)
    }
}
static DESCRIPTORS: [detail::Descriptor; 23] = [
    detail::Descriptor {
        name: "has_sse3",
        description: "Has support for SSE3.",
        offset: 0,
        detail: detail::Detail::Bool { bit: 0 },
    },
    detail::Descriptor {
        name: "has_ssse3",
        description: "Has support for SSSE3.",
        offset: 0,
        detail: detail::Detail::Bool { bit: 1 },
    },
    detail::Descriptor {
        name: "has_sse41",
        description: "Has support for SSE4.1.",
        offset: 0,
        detail: detail::Detail::Bool { bit: 2 },
    },
    detail::Descriptor {
        name: "has_sse42",
        description: "Has support for SSE4.2.",
        offset: 0,
        detail: detail::Detail::Bool { bit: 3 },
    },
    detail::Descriptor {
        name: "has_avx",
        description: "Has support for AVX.",
        offset: 0,
        detail: detail::Detail::Bool { bit: 4 },
    },
    detail::Descriptor {
        name: "has_avx2",
        description: "Has support for AVX2.",
        offset: 0,
        detail: detail::Detail::Bool { bit: 5 },
    },
    detail::Descriptor {
        name: "has_avx512bitalg",
        description: "Has support for AVX512BITALG.",
        offset: 0,
        detail: detail::Detail::Bool { bit: 6 },
    },
    detail::Descriptor {
        name: "has_avx512dq",
        description: "Has support for AVX512DQ.",
        offset: 0,
        detail: detail::Detail::Bool { bit: 7 },
    },
    detail::Descriptor {
        name: "has_avx512vl",
        description: "Has support for AVX512VL.",
        offset: 1,
        detail: detail::Detail::Bool { bit: 0 },
    },
    detail::Descriptor {
        name: "has_avx512vbmi",
        description: "Has support for AVX512VMBI.",
        offset: 1,
        detail: detail::Detail::Bool { bit: 1 },
    },
    detail::Descriptor {
        name: "has_avx512f",
        description: "Has support for AVX512F.",
        offset: 1,
        detail: detail::Detail::Bool { bit: 2 },
    },
    detail::Descriptor {
        name: "has_popcnt",
        description: "Has support for POPCNT.",
        offset: 1,
        detail: detail::Detail::Bool { bit: 3 },
    },
    detail::Descriptor {
        name: "has_bmi1",
        description: "Has support for BMI1.",
        offset: 1,
        detail: detail::Detail::Bool { bit: 4 },
    },
    detail::Descriptor {
        name: "has_bmi2",
        description: "Has support for BMI2.",
        offset: 1,
        detail: detail::Detail::Bool { bit: 5 },
    },
    detail::Descriptor {
        name: "has_lzcnt",
        description: "Has support for LZCNT.",
        offset: 1,
        detail: detail::Detail::Bool { bit: 6 },
    },
    detail::Descriptor {
        name: "baseline",
        description: "A baseline preset with no extensions enabled.",
        offset: 0,
        detail: detail::Detail::Preset,
    },
    detail::Descriptor {
        name: "nehalem",
        description: "Nehalem microarchitecture.",
        offset: 2,
        detail: detail::Detail::Preset,
    },
    detail::Descriptor {
        name: "haswell",
        description: "Haswell microarchitecture.",
        offset: 4,
        detail: detail::Detail::Preset,
    },
    detail::Descriptor {
        name: "broadwell",
        description: "Broadwell microarchitecture.",
        offset: 6,
        detail: detail::Detail::Preset,
    },
    detail::Descriptor {
        name: "skylake",
        description: "Skylake microarchitecture.",
        offset: 8,
        detail: detail::Detail::Preset,
    },
    detail::Descriptor {
        name: "cannonlake",
        description: "Canon Lake microarchitecture.",
        offset: 10,
        detail: detail::Detail::Preset,
    },
    detail::Descriptor {
        name: "icelake",
        description: "Ice Lake microarchitecture.",
        offset: 12,
        detail: detail::Detail::Preset,
    },
    detail::Descriptor {
        name: "znver1",
        description: "Zen (first generation) microarchitecture.",
        offset: 14,
        detail: detail::Detail::Preset,
    },
];
static ENUMERATORS: [&str; 0] = [
];
static HASH_TABLE: [u16; 32] = [
    0xffff,
    0xffff,
    0xffff,
    0,
    22,
    10,
    18,
    0xffff,
    7,
    0xffff,
    16,
    8,
    0xffff,
    3,
    0xffff,
    13,
    2,
    12,
    1,
    17,
    6,
    0xffff,
    4,
    9,
    11,
    20,
    5,
    19,
    0xffff,
    21,
    14,
    15,
];
static PRESETS: [(u8, u8); 16] = [
    // baseline
    (0b00000000, 0b00000000),
    (0b00000000, 0b00000000),
    // nehalem
    (0b00001111, 0b00001111),
    (0b00001000, 0b00001000),
    // haswell
    (0b00001111, 0b00001111),
    (0b01111000, 0b01111000),
    // broadwell
    (0b00001111, 0b00001111),
    (0b01111000, 0b01111000),
    // skylake
    (0b00001111, 0b00001111),
    (0b01111000, 0b01111000),
    // cannonlake
    (0b00001111, 0b00001111),
    (0b01111000, 0b01111000),
    // icelake
    (0b00001111, 0b00001111),
    (0b01111000, 0b01111000),
    // znver1
    (0b00001111, 0b00001111),
    (0b01111000, 0b01111000),
];
static TEMPLATE: detail::Template = detail::Template {
    name: "x86",
    descriptors: &DESCRIPTORS,
    enumerators: &ENUMERATORS,
    hash_table: &HASH_TABLE,
    defaults: &[0x0f, 0x00],
    presets: &PRESETS,
};
/// Create a `settings::Builder` for the x86 settings group.
pub fn builder() -> Builder {
    Builder::new(&TEMPLATE)
}
impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "[x86]")?;
        for d in &DESCRIPTORS {
            if !d.detail.is_preset() {
                write!(f, "{} = ", d.name)?;
                TEMPLATE.format_toml_value(d.detail, self.bytes[d.offset as usize], f)?;
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
