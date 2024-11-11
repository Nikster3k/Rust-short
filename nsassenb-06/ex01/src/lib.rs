type GoldNugget = u16;
type Iron = u32;
type Mercure = u64;

struct PhilosopherStone;

#[allow(dead_code)]
impl PhilosopherStone {
    fn transmute_iron(self, iron: Iron) -> [GoldNugget; 2] {
        let mut nuggets: [GoldNugget; 2] = [0; 2];

        nuggets[0] = iron as GoldNugget;
        nuggets[1] = (iron >> 16) as GoldNugget;
        nuggets
    }

    fn transmute_mercure(self, mercure: Mercure) -> [GoldNugget; 4] {
        let mut nuggets: [GoldNugget; 4] = [0; 4];
        let mut cmercure = mercure;

        for item in &mut nuggets {
            *item = cmercure as GoldNugget;
            cmercure >>= 16;
        }

        nuggets
    }

    fn transmute_metal<M: Metal>(self, metal: &M) -> &Gold {
        let size = size_of::<M>();

         // SAFETY:
        // - `metal` must be a valid pointer to an instance of `M`.
        // - `metal` must be properly aligned.
        // - The size of `M` must be correct and not exceed the bounds of the allocated memory.
        // - The resulting slice must be valid for reads.
        // - The transmute operation must be valid, meaning the resulting type must be valid for the given memory layout.
        // - The resulting reference must not outlive the original data.
        unsafe {
            let slice = std::slice::from_raw_parts(metal, size / size_of::<u16>());
            std::mem::transmute::<&[M], &[u16]>(slice)
        }
    }
}

type Gold = [GoldNugget];

/// # Safety
///
/// - The type implementing this trait must have a well-defined memory layout.
/// - The type must not contain any invalid values.
/// - The type must be safe to transmute into a slice of `u16`.
unsafe trait Metal {
    // SAFETY:
    // - Implementors of this trait must ensure that the type is safe to be used in the context of `transmute_metal`.
    // - This typically means that the type must have a well-defined memory layout and must not contain any invalid values.
}

/// # Safety
///
/// - `GoldNugget` has a well-defined memory layout.
/// - `GoldNugget` does not contain any invalid values.
/// - `GoldNugget` is safe to transmute into a slice of `u16`.
unsafe impl Metal for GoldNugget {
    // SAFETY:
    // - `GoldNugget` is a type that adheres to the requirements of the `Metal` trait.
    // - It has a well-defined memory layout and does not contain any invalid values.
}

/// # Safety
///
/// - `Iron` has a well-defined memory layout.
/// - `Iron` does not contain any invalid values.
/// - `Iron` is safe to transmute into a slice of `u16`.
unsafe impl Metal for Iron {
    // SAFETY:
    // - `Iron` is a type that adheres to the requirements of the `Metal` trait.
    // - It has a well-defined memory layout and does not contain any invalid values.
}

/// # Safety
///
/// - `Mercure` has a well-defined memory layout.
/// - `Mercure` does not contain any invalid values.
/// - `Mercure` is safe to transmute into a slice of `u16`.
unsafe impl Metal for Mercure {
    // SAFETY:
    // - `Mercure` is a type that adheres to the requirements of the `Metal` trait.
    // - It has a well-defined memory layout and does not contain any invalid values.
}


#[test]
fn test1_subject() {
    // On a LITTLE-ENDIAN machine! On big-endian machines, the result will be different.
    let iron = 0x12345678;
    assert_eq!(PhilosopherStone.transmute_iron(iron), [0x5678, 0x1234]);
    let mercure = 0x0123456789ABCDEF;
    assert_eq!(
        PhilosopherStone.transmute_mercure(mercure),
        [0xCDEF, 0x89AB, 0x4567, 0x0123],
    );
}

#[test]
fn test2_subjcet() {
    let mercure: Mercure = 0x0123456789ABCDEF;
    assert_eq!(
        PhilosopherStone.transmute_metal(&mercure),
        &[0xCDEF, 0x89AB, 0x4567, 0x0123],
);
}