use rand::{distributions::Alphanumeric, thread_rng, Rng};

const PRE_LEN: usize = 12;

pub fn next() -> String {
    let mut rng = thread_rng();
    rng.sample_iter(&Alphanumeric)
        .take(PRE_LEN)
        .map(char::from)
        .collect()
}