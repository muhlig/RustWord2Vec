extern crate rand;
use rand::Rng;

pub struct Sampler {    
    counts: Vec<f64>,
    max: f64
}     

impl Sampler {

    pub fn new(counts: Vec<f64>) -> Sampler {
        let mut max = 0.0;
        for i in 0 .. counts.len() {
            if counts[i] > max {
                max = counts[i];
            }
        }
        Sampler {counts: counts, max: max} 
    }

    pub fn unigram(words: Vec<usize>, rows: usize) -> Sampler {
        let mut counts = vec![0.0; rows];
        for word in words.iter() {
            counts[*word] += 1.0;                                             
        }
        for i in 0 .. rows {
            counts[i] = f64::powf(counts[i], 0.75);
        }
        let mut so_far = 0.0;        
        for i in 0 .. rows {
            so_far += counts[i];
            counts[i] = so_far;
        }
        Sampler::new(counts)
    }

    fn bisect(&self, search: f64, lo: usize, hi: usize) -> usize {
        let center = ((hi - lo) / 2) + lo;
        if center == 0 || center == hi || search <= self.counts[center] && search > self.counts[center-1] {
            usize::min(center, hi - 1)        
        } else if search > self.counts[center] {
            self.bisect(search, center, hi)
        } else {
            self.bisect(search, lo, center)
        }
    }

    pub fn multinomial(&self) -> usize {
        let n = self.counts.len();
        let mut rng = rand::thread_rng();
        let uniform = rng.gen_range(0.0, self.max);
        self.bisect(uniform, 0, n)        
    }
    
}
