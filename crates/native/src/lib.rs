extern crate rustler;
extern crate threshold_crypto;

use rustler::{Env, Term};

mod atom;
mod bivar_commitment;
mod bivar_poly;
mod commitment;
mod fr;
mod g1;
mod g1_affine;
mod poly;

fn load(env: Env, _: Term) -> bool {
    poly::load(env);
    commitment::load(env);
    bivar_poly::load(env);
    bivar_commitment::load(env);
    fr::load(env);
    g1::load(env);
    g1_affine::load(env);

    true
}

rustler::init!(
    "erlang_tc",
    [
        // Polynomial API
        poly::poly_from_coeffs,
        poly::gen_monomial,
        poly::random_poly,
        poly::zero_poly,
        poly::add_poly,
        poly::sub_poly,
        poly::mul_poly,
        poly::constant_poly,
        poly::add_scalar_poly,
        poly::sub_scalar_poly,
        poly::mul_scalar_poly,
        poly::eval_uni_poly,
        poly::eval_uni_poly_from_fr,
        poly::cmp_poly,
        poly::interpolate_uni_poly,
        poly::interpolate_uni_poly_from_fr,
        poly::zeroize_poly,
        poly::is_zero_poly,
        poly::degree_poly,
        poly::reveal_poly,
        poly::commitment_poly,
        // Fr API
        fr::into_fr,
        fr::cmp_fr,
        fr::zero_fr,
        fr::add_assign_fr,
        // G1 API
        g1::g1_zero,
        g1::cmp_g1,
        // G1Affine API
        g1_affine::g1_affine_one,
        g1_affine::g1_affine_mul,
        // BivarPoly API
        bivar_poly::random_bivar_poly,
        bivar_poly::degree_bivar_poly,
        bivar_poly::reveal_bivar_poly,
        bivar_poly::eval_bivar_poly,
        bivar_poly::row_bivar_poly,
        bivar_poly::commitment_bivar_poly,
        bivar_poly::zeroize_bivar_poly,
        // Commitment API
        commitment::degree_commitment,
        commitment::eval_commitment,
        commitment::cmp_commitment,
        commitment::reveal_commitment,
        commitment::add_commitment,
        // BivarCommitment API
        bivar_commitment::degree_bivar_commitment,
        bivar_commitment::eval_bivar_commitment,
        bivar_commitment::row_bivar_commitment,
        bivar_commitment::cmp_bivar_commitment,
        bivar_commitment::reveal_bivar_commitment,
    ],
    load = load
);