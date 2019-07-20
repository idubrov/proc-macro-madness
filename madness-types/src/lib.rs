#[cfg(feature = "desugar")]
use madness_macro::madness_desugar as madness;

#[cfg(feature = "direct")]
use madness_macro::madness_direct as madness;

include!("gen.rs");
