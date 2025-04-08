use nalgebra::Vector3;
use rand::prelude::*;

pub fn random_vector<T: Rng>(rng: &mut T) -> Vector3<f64> {
    Vector3::new(
        rng.random_range(-1.0f64..1.0f64),
        rng.random_range(-1.0f64..1.0f64),
        rng.random_range(-1.0f64..1.0f64),
    )
}

pub fn random_normal<T: Rng>(rng: &mut T) -> Vector3<f64> {
    loop {
        let position = random_vector(rng);
        let lensq = position.norm_squared();
        if f64::EPSILON < lensq && lensq <= 1.0 {
            return position / lensq.sqrt();
        }
    }
}

pub fn near_zero(v: &Vector3<f64>) -> bool {
    v.x.abs() < 1e-8 && v.y.abs() < 1e-8 && v.z.abs() < 1e-8
}

pub fn reflect(a: &Vector3<f64>, n: &Vector3<f64>) -> Vector3<f64> {
    a - n * (a.dot(n) * 2.0)
}

pub fn refract(uv: &Vector3<f64>, n: &Vector3<f64>, etai_over_etat: f64) -> Vector3<f64> {
    let cos_theta = f64::min((-uv).dot(n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -((1.0 - r_out_perp.norm_squared()).abs()).sqrt() * n;
    r_out_perp + r_out_parallel
}

pub fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r1 = r0 * r0;
    r1 + (1.0 - r1) * (1.0 - cosine).powf(5.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test_random_vector_within_bounds() {
        let mut rng = ChaCha8Rng::seed_from_u64(0xdeadbeef);
        let v = random_vector(&mut rng);
        assert_eq!(
            v,
            Vector3::new(0.9922237988865596, 0.15952643993251803, -0.9882828747974468)
        );
        for _ in 0..10000 {
            let v = random_vector(&mut rng);
            assert!((-1.0..=1.0).contains(&v.x));
            assert!((-1.0..=1.0).contains(&v.y));
            assert!((-1.0..=1.0).contains(&v.z));
        }
    }

    #[test]
    fn test_random_normal_unit_length() {
        let mut rng = ChaCha8Rng::seed_from_u64(0xdeadbeef);
        let v = random_normal(&mut rng);
        assert_eq!(
            v,
            Vector3::new(
                0.7951738577477956,
                -0.09622173403179604,
                -0.5986985166629384
            )
        );
        for _ in 0..10000 {
            let v = random_normal(&mut rng);
            let norm = v.norm();
            assert!((1.0 - norm).abs() < 1e-8);
        }
    }

    #[test]
    fn test_near_zero_true() {
        let v = Vector3::new(1e-9, 1e-9, 1e-9);
        assert!(near_zero(&v));
    }

    #[test]
    fn test_near_zero_false() {
        let v = Vector3::new(1e-7, 0.0, 0.0);
        assert!(!near_zero(&v));
    }

    #[test]
    fn test_reflect() {
        let a = Vector3::new(1.0, -1.0, 0.0);
        let n = Vector3::new(0.0, 1.0, 0.0);
        let reflected = reflect(&a, &n);
        assert_eq!(reflected, Vector3::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn test_refract() {
        let uv = Vector3::new(1.0, -1.0, 0.0).normalize();
        let n = Vector3::new(0.0, 1.0, 0.0);
        let etai_over_etat = 0.5;
        let refracted = refract(&uv, &n, etai_over_etat);
        assert_eq!(
            refracted,
            Vector3::new(0.35355339059327373, -0.9354143466934853, 0.0)
        );
    }

    #[test]
    fn test_reflectance() {
        let cosine = 0.5;
        let refraction_index = 1.5;
        let reflectance_value = reflectance(cosine, refraction_index);
        assert_eq!(reflectance_value, 0.07);
    }
}
