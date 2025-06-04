//! Adapted from: https://github.com/w3f/bls
//! ## Adaptation of `ark_ec::PairingEngine` to BLS-like signatures.
//!
//! We provide an `EngineBLS` trait that adapts `pairing::Engine`
//! to BLS-like signatures by permitting the group roles to be
//! transposed, which involves removing the field of definition,
//! while retaining the correct associations.  
//!
//! We support same-message aggregation strategies using wrappers
//! that satisfy `EngineBLS` as well, primarily because these
//! strategies must ocntroll access to the public key type.
//!
//! In future, we should support [Pixel](https://github.com/w3f/bls/issues/4)
//! by adding wrapper that replace `SignatureGroup` with a product
//! of both groups.  I think this requires abstracting `CruveAffine`
//! and `CruveProjective` without their base fields and wNAF windows,
//! but still with their affine, projective, and compressed forms,
//! and batch normalization.

use core::{borrow::Borrow, ops::MulAssign};

use alloc::{vec, vec::Vec};
use ark_ec::{
	hashing::{
		map_to_curve_hasher::{MapToCurve, MapToCurveBasedHasher},
		HashToCurve,
	},
	pairing::{MillerLoopOutput, Pairing, PairingOutput},
	AffineRepr, CurveGroup,
};
use ark_ff::{field_hashers::HashToField, Field, PrimeField, UniformRand};
use ark_serialize::CanonicalSerialize;
use rand::Rng;
use ark_std::rand::RngCore;

use core::fmt::Debug;

// Expand SHA256 from 256 bits to 1024 bits.
// let output_bits = 1024;
// let output_bytes = 1024 / 8;
// let mut hasher = FullDomainHash::<Sha256>::new(output_bytes).unwrap();
// hasher.update(b"ATTACK AT DAWN");
// let result = hasher.finalize_boxed().into_vec();

/// A weakening of `pairing::Engine` to permit transposing the groups.
///
/// You cannot transpose the two groups in a `pairing::Engine` without
/// first providing panicing implementations of `pairing::PrimeField`
/// for `Engine::Fqe`, which is not a prime field, and second,
/// providing wrapper types for the projective and affine group
/// representations, which makes interacting with the original
/// `pairing::Engine` annoying.  This trait merely replicates
/// transposable functionality from `pairing::Engine` by removing
/// the fields of definition, but leaves the actual BLS signature
/// scheme to wrapper types.
///
/// We also extract two functions users may with to override:
/// random scalar generation and hashing to the singature curve.
pub trait EngineBLS {
	type Engine: Pairing; //<Fr = Self::Scalar>;
	type Scalar: PrimeField; // = <Self::Engine as ScalarEngine>::Fr;
	/// Group where BLS public keys live
	///
	/// You should take this to be the `Engine::G1` curve usually
	/// becuase all verifiers perform additions on this curve, or
	/// even scalar multiplicaitons with delinearization.
	type PublicKeyGroupBaseField: Field;
	type PublicKeyGroupAffine: AffineRepr<ScalarField = Self::Scalar, Group = Self::PublicKeyGroup>
		+ From<Self::PublicKeyGroup>
		+ Into<Self::PublicKeyGroup>
		+ Into<Self::PublicKeyPrepared>;
	//+ Into<<Self::PublicKeyGroup as CurveGroup>::Affine>;

	type PublicKeyGroup: CurveGroup<
			Affine = Self::PublicKeyGroupAffine,
			ScalarField = Self::Scalar,
			BaseField = Self::PublicKeyGroupBaseField,
		> + From<Self::PublicKeyGroupAffine>
		+ Into<Self::PublicKeyGroupAffine>
		+ MulAssign<Self::Scalar>;

	type PublicKeyPrepared: Default
		+ Clone
		+ Send
		+ Sync
		+ Debug
		+ From<Self::PublicKeyGroupAffine>;

	const PUBLICKEY_SERIALIZED_SIZE: usize;
	const SECRET_KEY_SIZE: usize;

	// See https://www.ietf.org/archive/id/draft-irtf-cfrg-bls-signature-05.html#name-ciphersuites
	const CURVE_NAME: &'static [u8];
	const SIG_GROUP_NAME: &'static [u8];
	const CIPHER_SUIT_DOMAIN_SEPARATION: &'static [u8];

	/// Group where BLS signatures live
	///
	/// You should take this to be the `Engine::G2` curve usually
	/// becuase only aggregators perform additions on this curve, or
	/// scalar multiplicaitons with delinearization.
	type SignatureGroupBaseField: Field;

	type SignatureGroupAffine: AffineRepr<ScalarField = Self::Scalar, Group = Self::SignatureGroup>
		+ From<Self::SignatureGroup>
		+ Into<Self::SignatureGroup>
		+ Into<Self::SignaturePrepared>;

	type SignatureGroup: CurveGroup<
			Affine = Self::SignatureGroupAffine,
			ScalarField = Self::Scalar,
			BaseField = Self::SignatureGroupBaseField,
		> + Into<Self::SignatureGroupAffine>
		+ From<Self::SignatureGroupAffine>
		+ MulAssign<Self::Scalar>;

	type SignaturePrepared: Default
		+ Clone
		+ Send
		+ Sync
		+ Debug
		+ From<Self::SignatureGroupAffine>;

	const SIGNATURE_SERIALIZED_SIZE: usize;

	type HashToSignatureField: HashToField<Self::SignatureGroupBaseField>;
	type MapToSignatureCurve: MapToCurve<Self::SignatureGroup>;

	/// Generate a random scalar for use as a secret key.
	fn generate<R: Rng + RngCore>(rng: &mut R) -> Self::Scalar {
		Self::Scalar::rand(rng)
	}

	/// getter function for the hash to curve map
	fn hash_to_curve_map() -> MapToCurveBasedHasher<
		Self::SignatureGroup,
		Self::HashToSignatureField,
		Self::MapToSignatureCurve,
	>;

	/// Hash one message to the signature curve.
	fn hash_to_signature_curve<M: Borrow<[u8]>>(
		message: M,
	) -> Self::SignatureGroup {
		Self::hash_to_curve_map().hash(message.borrow()).unwrap().into_group()
	}

	/// Run the Miller loop from `Engine` but orients its arguments
	/// to be a `SignatureGroup` and `PublicKeyGroup`.
	fn miller_loop<'a, I>(i: I) -> MillerLoopOutput<Self::Engine>
	where
		Self::PublicKeyPrepared: 'a,
		Self::SignaturePrepared: 'a,
		I: IntoIterator<
			Item = &'a (
				<Self as EngineBLS>::PublicKeyPrepared,
				Self::SignaturePrepared,
			),
		>;

	/// Perform final exponentiation on the result of a Miller loop.
	fn final_exponentiation(
		e: MillerLoopOutput<Self::Engine>,
	) -> Option<PairingOutput<Self::Engine>> {
		Self::Engine::final_exponentiation(e)
	}

	/// Performs a pairing operation `e(p, q)` by calling `Engine::pairing`
	/// but orients its arguments to be a `PublicKeyGroup` and `SignatureGroup`.
	fn pairing<G1, G2>(p: G1, q: G2) -> <Self::Engine as Pairing>::TargetField
	where
		G1: Into<<Self::PublicKeyGroup as CurveGroup>::Affine>,
		G2: Into<<Self::SignatureGroup as CurveGroup>::Affine>;
	/*
	{
		Self::final_exponentiation(&Self::miller_loop(
			[(&(p.into().prepare()), &(q.into().prepare()))].into_iter(),
		)).unwrap()
	}
	*/

	/// Prepared negative of the generator of the public key curve.
	fn minus_generator_of_public_key_group_prepared() -> Self::PublicKeyPrepared;

	/// return the generator of signature group
	fn generator_of_signature_group() -> Self::SignatureGroup {
		<Self::SignatureGroup as CurveGroup>::Affine::generator().into()
	}

	/// Process the public key to be use in pairing. This has to be
	/// implemented by the type of BLS system implementing the engine
	/// by calling either prepare_g1 or prepare_g2 based on which group
	/// is used by the signature system to host the public key
	fn prepare_public_key(
		g: impl Into<Self::PublicKeyGroupAffine>,
	) -> Self::PublicKeyPrepared {
		let g_affine: Self::PublicKeyGroupAffine = g.into();
		Self::PublicKeyPrepared::from(g_affine)
	}

	/// Process the signature to be use in pairing. This has to be
	/// implemented by the type of BLS system implementing the engine
	/// by calling either prepare_g1 or prepare_g2 based on which group
	/// is used by the signature system to host the public key
	fn prepare_signature(
		g: impl Into<Self::SignatureGroupAffine>,
	) -> Self::SignaturePrepared {
		let g_affine: Self::SignatureGroupAffine = g.into();
		Self::SignaturePrepared::from(g_affine)
	}

	/// Serialization helper for various sigma protocols
	fn signature_point_to_byte(point: &Self::SignatureGroup) -> Vec<u8> {
		let mut point_as_bytes = vec![0; Self::SIGNATURE_SERIALIZED_SIZE];
		let point_affine = point.into_affine();
		point_affine.serialize_compressed(&mut point_as_bytes[..]).unwrap();
		point_as_bytes
	}

	fn public_key_point_to_byte(point: &Self::PublicKeyGroup) -> Vec<u8> {
		let mut point_as_bytes = vec![0; Self::PUBLICKEY_SERIALIZED_SIZE];
		let point_affine = point.into_affine();
		point_affine.serialize_compressed(&mut point_as_bytes[..]).unwrap();
		point_as_bytes
	}
}
