use curve25519_dalek::{
    ristretto::{CompressedRistretto, RistrettoPoint},
    scalar::Scalar,
};

pub fn ristretto_add(left: RistrettoPoint, right: RistrettoPoint) -> RistrettoPoint {
    left + right
}

pub fn ristretto_decompress(point: CompressedRistretto) -> RistrettoPoint {
    point.decompress().unwrap()
}

pub fn ristretto_mul(point: RistrettoPoint, scalar: Scalar) -> RistrettoPoint {
    point * scalar
}

