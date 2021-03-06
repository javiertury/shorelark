use crate::*;

#[derive(Clone, Debug)]
pub struct RouletteWheelSelection;

pub struct RouletteWheelSelector {
    max_fitness: f32,
}

impl RouletteWheelSelection {
    pub fn new() -> Self {
        Self
    }
}

impl RouletteWheelSelector {
    pub fn new<I: Individual>(population: &[I]) -> Self {
        let max_fitness = population
            .iter()
            .map(|individual| individual.fitness())
            .max_by(|a, b| a.partial_cmp(&b).expect("got a fitness equal to NaN"))
            .expect("got an empty population");

        Self { max_fitness }
    }
}

impl SelectionPolicy for RouletteWheelSelection {
    type Selector = RouletteWheelSelector;

    fn init<I: Individual>(&self, population: &[I]) -> Self::Selector {
        RouletteWheelSelector::new(population)
    }
}

/// Roulette-wheel selection via stochastic acceptance
impl Selector for RouletteWheelSelector {
    fn select<'a, I: Individual>(&self, population: &'a [I], rng: &mut ChaCha8Rng) -> &'a I {
        loop {
            let individual = population
                .iter()
                .choose(rng)
                .expect("got an empty population");

            if rng.gen_bool((individual.fitness() / self.max_fitness) as _) {
                return individual;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::SeedableRng;
    use std::collections::BTreeMap;

    #[test]
    fn test() {
        let population = vec![
            TestIndividual::new(2.0),
            TestIndividual::new(1.0),
            TestIndividual::new(4.0),
            TestIndividual::new(3.0),
        ];

        let selector = RouletteWheelSelection::new().init(&population);

        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let individuals: Vec<_> = (0..1000)
            .map(|_| selector.select(&population, &mut rng))
            .collect();

        let actual: BTreeMap<_, _> = (1..=4)
            .map(|individual_fitness| {
                let individual_count = individuals
                    .iter()
                    .filter(|individual| individual.fitness() as usize == individual_fitness)
                    .count();

                (individual_fitness, individual_count)
            })
            .collect();

        let expected = maplit::btreemap! {
            // individual's fitness => how many times this individual has been chosen
            1 => 101,
            2 => 183,
            3 => 288,
            4 => 428,
        };

        assert_eq!(actual, expected);
    }
}
