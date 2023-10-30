use lambdaworks_crypto::commitments::kzg::KateZaveruchaGoldberg;
use lambdaworks_math::unsigned_integer::element::U256;
use lambdaworks_math::{
    elliptic_curve::short_weierstrass::curves::bls12_381::{
        curve::BLS12381Curve,
        default_types::{FrElement as FE, FrField},
        pairing::BLS12381AtePairing,
        twist::BLS12381TwistCurve,
    },
    elliptic_curve::traits::IsEllipticCurve,
};
use rand::Rng;

pub type Curve = BLS12381Curve;
pub type TwistedCurve = BLS12381TwistCurve;

pub type FrElement = FE;

pub type Pairing = BLS12381AtePairing;

pub type KZG = KateZaveruchaGoldberg<FrField, Pairing>;

pub type G1Point = <BLS12381Curve as IsEllipticCurve>::PointRepresentation;
pub type G2Point = <BLS12381TwistCurve as IsEllipticCurve>::PointRepresentation;

pub fn sample_field_elem() -> FrElement {
    let mut rng = rand::thread_rng();
    FrElement::new(U256 {
        limbs: [
            rng.gen::<u64>(),
            rng.gen::<u64>(),
            rng.gen::<u64>(),
            rng.gen::<u64>(),
        ],
    })
}
