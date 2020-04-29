use crate::cdsl::settings::{PredicateNode, SettingGroup, SettingGroupBuilder};

pub(crate) fn define(shared: &SettingGroup) -> SettingGroup {
    let mut settings = SettingGroupBuilder::new("x86");

    // CPUID.01H:ECX
    let has_sse3 = settings.add_bool("has_sse3", "SSE3: CPUID.01H:ECX.SSE3[bit 0]", false);
    let has_ssse3 = settings.add_bool("has_ssse3", "SSSE3: CPUID.01H:ECX.SSSE3[bit 9]", false);
    let has_sse41 = settings.add_bool("has_sse41", "SSE4.1: CPUID.01H:ECX.SSE4_1[bit 19]", false);
    let has_sse42 = settings.add_bool("has_sse42", "SSE4.2: CPUID.01H:ECX.SSE4_2[bit 20]", false);
    let has_avx = settings.add_bool("has_avx", "AVX: CPUID.01H:ECX.AVX[bit 28]", false);
    let has_avx2 = settings.add_bool("has_avx2", "AVX2: CPUID.07H:EBX.AVX2[bit 5]", false);
    let has_avx512dq = settings.add_bool(
        "has_avx512dq",
        "AVX512DQ: CPUID.07H:EBX.AVX512DQ[bit 17]",
        false,
    );
    let has_avx512vl = settings.add_bool(
        "has_avx512vl",
        "AVX512DQ: CPUID.07H:EBX.AVX512VL[bit 31]",
        false,
    );
    let has_popcnt = settings.add_bool("has_popcnt", "POPCNT: CPUID.01H:ECX.POPCNT[bit 23]", false);

    // CPUID.(EAX=07H, ECX=0H):EBX
    let has_bmi1 = settings.add_bool(
        "has_bmi1",
        "BMI1: CPUID.(EAX=07H, ECX=0H):EBX.BMI1[bit 3]",
        false,
    );
    let has_bmi2 = settings.add_bool(
        "has_bmi2",
        "BMI2: CPUID.(EAX=07H, ECX=0H):EBX.BMI2[bit 8]",
        false,
    );

    // CPUID.EAX=80000001H:ECX
    let has_lzcnt = settings.add_bool(
        "has_lzcnt",
        "LZCNT: CPUID.EAX=80000001H:ECX.LZCNT[bit 5]",
        false,
    );

    let shared_enable_simd = shared.get_bool("enable_simd");

    settings.add_predicate("use_ssse3", predicate!(has_ssse3));
    settings.add_predicate("use_sse41", predicate!(has_sse41));
    settings.add_predicate("use_sse42", predicate!(has_sse41 && has_sse42));

    settings.add_predicate(
        "use_ssse3_simd",
        predicate!(shared_enable_simd && has_ssse3),
    );
    settings.add_predicate(
        "use_sse41_simd",
        predicate!(shared_enable_simd && has_sse41),
    );
    settings.add_predicate(
        "use_sse42_simd",
        predicate!(shared_enable_simd && has_sse41 && has_sse42),
    );

    settings.add_predicate("use_avx_simd", predicate!(shared_enable_simd && has_avx));
    settings.add_predicate("use_avx2_simd", predicate!(shared_enable_simd && has_avx2));
    settings.add_predicate(
        "use_avx512dq_simd",
        predicate!(shared_enable_simd && has_avx512dq),
    );
    settings.add_predicate(
        "use_avx512vl_simd",
        predicate!(shared_enable_simd && has_avx512vl),
    );

    settings.add_predicate("use_popcnt", predicate!(has_popcnt && has_sse42));
    settings.add_predicate("use_bmi1", predicate!(has_bmi1));
    settings.add_predicate("use_lzcnt", predicate!(has_lzcnt));

    // Some shared boolean values are used in x86 instruction predicates, so we need to group them
    // in the same TargetIsa, for compabitibity with code generated by meta-python.
    // TODO Once all the meta generation code has been migrated from Python to Rust, we can put it
    // back in the shared SettingGroup, and use it in x86 instruction predicates.

    let is_pic = shared.get_bool("is_pic");
    let emit_all_ones_funcaddrs = shared.get_bool("emit_all_ones_funcaddrs");
    settings.add_predicate("is_pic", predicate!(is_pic));
    settings.add_predicate("not_is_pic", predicate!(!is_pic));
    settings.add_predicate(
        "all_ones_funcaddrs_and_not_is_pic",
        predicate!(emit_all_ones_funcaddrs && !is_pic),
    );
    settings.add_predicate(
        "not_all_ones_funcaddrs_and_not_is_pic",
        predicate!(!emit_all_ones_funcaddrs && !is_pic),
    );

    // Presets corresponding to x86 CPUs.

    settings.add_preset("baseline", preset!());
    let nehalem = settings.add_preset(
        "nehalem",
        preset!(has_sse3 && has_ssse3 && has_sse41 && has_sse42 && has_popcnt),
    );
    let haswell = settings.add_preset(
        "haswell",
        preset!(nehalem && has_bmi1 && has_bmi2 && has_lzcnt),
    );
    let broadwell = settings.add_preset("broadwell", preset!(haswell));
    let skylake = settings.add_preset("skylake", preset!(broadwell));
    let cannonlake = settings.add_preset("cannonlake", preset!(skylake));
    settings.add_preset("icelake", preset!(cannonlake));
    settings.add_preset(
        "znver1",
        preset!(
            has_sse3
                && has_ssse3
                && has_sse41
                && has_sse42
                && has_popcnt
                && has_bmi1
                && has_bmi2
                && has_lzcnt
        ),
    );

    settings.build()
}
