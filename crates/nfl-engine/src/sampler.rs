//! Token sampling strategies (greedy, temperature, top-k, top-p)

use rand::distributions::{WeightedIndex, Distribution};
use rand::thread_rng;

/// Sampling strategy for token generation
#[derive(Debug, Clone)]
pub enum SamplingStrategy {
    Greedy,
    Temperature(f32),
    TopK { k: usize, temperature: f32 },
    TopP { p: f32, temperature: f32 },
    BeamSearch { beam_width: usize },
}

/// Token sampler for generation
pub struct Sampler {
    strategy: SamplingStrategy,
}

impl Sampler {
    pub fn new(strategy: SamplingStrategy) -> Self {
        Self { strategy }
    }

    /// Sample next token from logits
    pub fn sample(&self, logits: &[f32]) -> usize {
        match self.strategy {
            SamplingStrategy::Greedy => self.greedy(logits),
            SamplingStrategy::Temperature(temp) => self.temperature_sample(logits, temp),
            SamplingStrategy::TopK { k, temperature } => {
                self.top_k_sample(logits, k, temperature)
            }
            SamplingStrategy::TopP { p, temperature } => {
                self.top_p_sample(logits, p, temperature)
            }
            SamplingStrategy::BeamSearch { beam_width: _ } => self.greedy(logits),
        }
    }

    fn greedy(&self, logits: &[f32]) -> usize {
        logits
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(idx, _)| idx)
            .unwrap_or(0)
    }

    fn temperature_sample(&self, logits: &[f32], temperature: f32) -> usize {
        let scaled = logits
            .iter()
            .map(|&x| (x / temperature).exp())
            .collect::<Vec<_>>();

        let sum: f32 = scaled.iter().sum();
        let probs: Vec<f32> = scaled.iter().map(|&x| x / sum).collect();

        let dist = WeightedIndex::new(&probs).unwrap();
        let mut rng = thread_rng();
        dist.sample(&mut rng)
    }

    fn top_k_sample(&self, logits: &[f32], k: usize, temperature: f32) -> usize {
        let mut indexed: Vec<(usize, f32)> = logits
            .iter()
            .enumerate()
            .map(|(i, &v)| (i, v))
            .collect();

        indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let top_k_indices: Vec<usize> = indexed.iter().take(k).map(|&(i, _)| i).collect();
        let top_k_logits: Vec<f32> = indexed.iter().take(k).map(|&(_, v)| v).collect();

        let scaled: Vec<f32> = top_k_logits
            .iter()
            .map(|&x| (x / temperature).exp())
            .collect();

        let sum: f32 = scaled.iter().sum();
        let probs: Vec<f32> = scaled.iter().map(|&x| x / sum).collect();

        let dist = WeightedIndex::new(&probs).unwrap();
        let mut rng = thread_rng();
        let selected = dist.sample(&mut rng);

        top_k_indices[selected]
    }

    fn top_p_sample(&self, logits: &[f32], p: f32, temperature: f32) -> usize {
        let mut indexed: Vec<(usize, f32)> = logits
            .iter()
            .enumerate()
            .map(|(i, &v)| (i, v))
            .collect();

        indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let scaled: Vec<f32> = indexed
            .iter()
            .map(|&(_, v)| (v / temperature).exp())
            .collect();

        let sum: f32 = scaled.iter().sum();
        let mut probs: Vec<f32> = scaled.iter().map(|&x| x / sum).collect();

        let mut cumsum = 0.0;
        let mut cutoff = probs.len();

        for (i, &prob) in probs.iter().enumerate() {
            cumsum += prob;
            if cumsum >= p {
                cutoff = i + 1;
                break;
            }
        }

        probs.truncate(cutoff);
        let sum: f32 = probs.iter().sum();
        probs.iter_mut().for_each(|p| *p /= sum);

        let dist = WeightedIndex::new(&probs).unwrap();
        let mut rng = thread_rng();
        let selected = dist.sample(&mut rng);

        indexed[selected].0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greedy_sampling() {
        let logits = vec![1.0, 5.0, 2.0, 3.0];
        let sampler = Sampler::new(SamplingStrategy::Greedy);
        assert_eq!(sampler.sample(&logits), 1);
    }
}
