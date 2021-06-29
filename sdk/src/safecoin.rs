#![cfg(feature = "full")]

use sha2::{Sha256,Digest};

use solana_sdk::{
    hash::Hash,
    pubkey::Pubkey
};

pub const SC_FALLBACK_VALIDATOR_STR: &str = "83E5RMejo6d98FV1EAXTx5t4bvoDMoxE4DboDee3VJsu";
pub const SC_FALLBACK_VALIDATOR: &Pubkey = &Pubkey::new_from_array([104, 147, 193, 98, 234, 13, 57, 77,
                                                                    158, 79, 114, 179, 99, 46, 189, 80,
                                                                    207, 135, 95,179,175,254,58,186,99,
                                                                    134,161,27,27,136,224,204]);

// Slot where a version of a function replace the previous one.
// TODO: Replace with proper slot height before going in production.
const SC_IS_VOTING_DENIED_V3: &u64 = &2u64;
const SC_IS_VOTING_DENIED_V2: &u64 = &1u64;
const SC_IS_VOTING_DENIED_V1: &u64 = &0u64;

pub fn is_voting_denied(slot:&u64, voter_pubkey: &Pubkey,vote_hash: &Hash) -> bool {
    if slot >= SC_IS_VOTING_DENIED_V3 {
      // Uniform distribution of vote denial among validators.
      let mut hasher  = Sha256::new();
      hasher.update(voter_pubkey.to_bytes());
      hasher.update(vote_hash.to_bytes());
      let hash256 = hasher.finalize();

      // Convert the 256 bits hash to u32 by arbitraty selecting 4 bytes.
      // This is portable (not affected by endianess).
      let hash32 = ((hash256[0] as u32) <<   0) |
                   ((hash256[1] as u32) <<   8) |
                   ((hash256[2] as u32) <<  16) |
                   ((hash256[3] as u32) <<  24);

      // Deny voting 90% of the time, except for fallback validator.
      return (hash32%10) != 0 && voter_pubkey != SC_FALLBACK_VALIDATOR;

    } else if slot >= SC_IS_VOTING_DENIED_V2 {
      // Do not change for backward compatibility.
      return  (  ( slot % 10 ) as usize !=
                 ( ( ( slot % 9 + 1 ) as usize * ( voter_pubkey.to_string().chars().last().unwrap() as usize + vote_hash.to_string().chars().last().unwrap() as usize ) / 10 ) as usize +
                    voter_pubkey.to_string().chars().last().unwrap() as usize + vote_hash.to_string().chars().last().unwrap() as usize
                 ) % 10 as usize
              ) && voter_pubkey.to_string() != SC_FALLBACK_VALIDATOR_STR;
    } else {
      // Do not change for backward compatibility.
      return (vote_hash.to_string().to_lowercase().find("x").unwrap_or(3) % 10 as usize) !=
             (voter_pubkey.to_string().to_lowercase().find("x").unwrap_or(2) % 10 as usize) &&
             voter_pubkey.to_string() != SC_FALLBACK_VALIDATOR_STR
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::hash::HASH_BYTES;
    use std::str::FromStr;

    #[test]
    fn safecoin_fallback() {
        // Verify server fallback is never denied voting.

        // Build fallback pubkey from string, verify public constant is same.
        let fallback_pubkey = Pubkey::from_str(SC_FALLBACK_VALIDATOR_STR).unwrap();
        assert!( SC_FALLBACK_VALIDATOR == &fallback_pubkey );

        // Build a voter and slot combination known to be denied with all versions (up to now).
        let vote_hash = Hash::new_from_array([11u8; HASH_BYTES] );
        let denied_pubkey = Pubkey::new_from_array([8u8; HASH_BYTES]);

        // Test behavior by changing the denied pubkey with the fallback one.
        // Repeat test with older versions.
        {
            let slot: &u64 = SC_IS_VOTING_DENIED_V3;
            assert_eq!( is_voting_denied(&slot,&denied_pubkey,&vote_hash), true );
            assert_eq!( is_voting_denied(&slot,&fallback_pubkey,&vote_hash), false );
        }
        {
            let slot: &u64 = SC_IS_VOTING_DENIED_V2;
            assert_eq!( is_voting_denied(&slot,&denied_pubkey,&vote_hash), true );
            assert_eq!( is_voting_denied(&slot,&fallback_pubkey,&vote_hash), false );
        }
        {
            let slot: &u64 = SC_IS_VOTING_DENIED_V1;
            assert_eq!( is_voting_denied(&slot,&denied_pubkey,&vote_hash), true );
            assert_eq!( is_voting_denied(&slot,&fallback_pubkey,&vote_hash), false );
        }
    }

    //#[test]
    //fn safecoin_profiling() {
    // Not really a test, just code to help profiling some functions.
    //    let vote_hash = Hash::new_from_array([11u8; HASH_BYTES] );
    //    let mut n_denied: u32 = 0;
    //    let iter:u32 = 10000;
    //    for _ in 0..iter {
    //      let voter_pubkey = Pubkey::new_unique();
    //      if is_voting_denied(SC_IS_VOTING_DENIED_V1, &voter_pubkey, &vote_hash)  { n_denied += 1; }
    //    }
    //    println!("n_denied={}/{}%", n_denied, iter );
    //    assert!(false);
    //}
}
