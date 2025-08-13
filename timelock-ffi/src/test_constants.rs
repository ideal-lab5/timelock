// Test what constants are available and validate our hardcoded values
#[cfg(test)]
mod test_available_constants {
    use ark_bls12_381::{G1Affine, G2Affine};
    use ark_serialize::CanonicalSerialize;
    use crate::{BLS_G1_SIZE, BLS_G2_SIZE};
    
    #[test]
    fn test_size_constants_match_library() {
        // Validate that our hardcoded constants match the library
        let g1_dummy = G1Affine::identity();
        let g2_dummy = G2Affine::identity();
        
        let g1_compressed_size = g1_dummy.compressed_size();
        let g2_compressed_size = g2_dummy.compressed_size();
        
        // Ensure our constants match what the library actually produces
        assert_eq!(g1_compressed_size, BLS_G1_SIZE, 
            "BLS_G1_SIZE constant ({}) doesn't match library compressed size ({})", 
            BLS_G1_SIZE, g1_compressed_size);
        assert_eq!(g2_compressed_size, BLS_G2_SIZE,
            "BLS_G2_SIZE constant ({}) doesn't match library compressed size ({})", 
            BLS_G2_SIZE, g2_compressed_size);
            
        println!("✓ G1 compressed size: {} (matches constant)", g1_compressed_size);
        println!("✓ G2 compressed size: {} (matches constant)", g2_compressed_size);
    }
}
