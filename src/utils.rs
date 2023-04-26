use rand::Rng;

pub fn random_vec<T>(len: usize, min: T, max: T) -> Vec<T> 
where T: rand::distributions::uniform::SampleUniform + Ord + Copy {
    let mut arr = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..len {
        arr.push(rng.gen_range(min..max));
    }
    arr
}
