//! #Warning test-only
//!
//! This module provides the structure containing the cryptographic parameters only intended to be
//! used to test some operations.
//! These parameters are *NOT guaranteed to be safe*.
use crate::core_crypto::prelude::*;
use crate::shortint::ciphertext::MaxNoiseLevel;
use crate::shortint::parameters::{CarryModulus, ClassicPBSParameters, MessageModulus};

// TODO To remove once casting is available
pub const PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_PBS_KS_TUNIFORM_2M64: ClassicPBSParameters =
    ClassicPBSParameters {
        lwe_dimension: LweDimension(1024),
        glwe_dimension: GlweDimension(1),
        polynomial_size: PolynomialSize(2048),
        lwe_noise_distribution: DynamicDistribution::new_t_uniform(41),
        glwe_noise_distribution: DynamicDistribution::new_t_uniform(14),
        pbs_base_log: DecompositionBaseLog(23),
        pbs_level: DecompositionLevelCount(1),
        ks_base_log: DecompositionBaseLog(5),
        ks_level: DecompositionLevelCount(4),
        message_modulus: MessageModulus(4),
        carry_modulus: CarryModulus(4),
        max_noise_level: MaxNoiseLevel::new(5),
        log2_p_fail: -66.873,
        ciphertext_modulus: CiphertextModulus::new_native(),
        encryption_key_choice: EncryptionKeyChoice::Small,
    };
