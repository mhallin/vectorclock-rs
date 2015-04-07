use std::hash::Hash;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum TemporalRelation {
    Equal,
    Caused,
    EffectOf,
    Concurrent,
}

#[derive(PartialEq, Eq, Debug)]
pub struct VectorClock<HostType: Hash + Eq> {
    entries: HashMap<HostType, u64>,
}

impl<HostType: Clone + Hash + Eq> VectorClock<HostType> {
    pub fn new() -> VectorClock<HostType> {
        VectorClock {
            entries: HashMap::new(),
        }
    }

    pub fn incremented(&self, host: HostType) -> Self {
        let mut entries = self.entries.clone();

        match entries.entry(host) {
            Entry::Vacant(e) => { e.insert(1); },
            Entry::Occupied(mut e) => {
                let v = *e.get();
                e.insert(v + 1);
            },
        };

        VectorClock {
            entries: entries,
        }
    }

    pub fn temporal_relation(&self, other: &Self) -> TemporalRelation {
        if self == other {
            TemporalRelation::Equal
        }
        else if self.superseded_by(other) {
            TemporalRelation::Caused
        }
        else if other.superseded_by(self) {
            TemporalRelation::EffectOf
        }
        else {
            TemporalRelation::Concurrent
        }
    }

    fn superseded_by(&self, other: &Self) -> bool {
        let mut has_smaller = false;

        for (host, &self_n) in self.entries.iter() {
            let other_n = *other.entries.get(host).unwrap_or(&0);

            if self_n > other_n {
                return false;
            }

            has_smaller = has_smaller || (self_n < other_n);
        }

        for (host, &other_n) in other.entries.iter() {
            let self_n = *self.entries.get(host).unwrap_or(&0);

            if self_n > other_n {
                return false;
            }

            has_smaller = has_smaller || (self_n < other_n);
        }

        has_smaller
    }

    pub fn merge_with(&self, other: &Self) -> Self {
        let mut entries = self.entries.clone();

        for (host, &other_n) in other.entries.iter() {
            match entries.entry(host.clone()) {
                Entry::Vacant(e) => { e.insert(other_n); },
                Entry::Occupied(mut e) => {
                    let self_n = *e.get();

                    if other_n > self_n {
                        e.insert(other_n);
                    }
                }
            }
        }

        VectorClock {
            entries: entries,
        }
    }

    pub fn to_vec(&self) -> Vec<(HostType, u64)> {
        self.entries.iter().map(| (host, &n) | (host.clone(), n) ).collect()
    }

    pub fn from_vec(v: Vec<(HostType, u64)>) -> VectorClock<HostType> {
        VectorClock {
            entries: v.iter().cloned().collect(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{VectorClock, TemporalRelation};

    type StrVectorClock = VectorClock<&'static str>;

    #[test]
    fn test_empty_ordering() {
        let c1 = StrVectorClock::new();
        let c2 = StrVectorClock::new();

        assert_eq!(c1, c2);

        assert!(c1.temporal_relation(&c2) == TemporalRelation::Equal);
        assert!(c2.temporal_relation(&c1) == TemporalRelation::Equal);
    }

    #[test]
    fn test_incremented_ordering() {
        let c1 = StrVectorClock::new();
        let c2 = c1.incremented("A");

        assert!(!(c1 == c2));

        assert!(c1.temporal_relation(&c2) == TemporalRelation::Caused);
        assert!(c2.temporal_relation(&c1) == TemporalRelation::EffectOf);
    }

    #[test]
    fn test_diverged() {
        let base = StrVectorClock::new();
        let c1 = base.incremented("A");
        let c2 = base.incremented("B");

        assert!(!(c1 == c2));

        assert!(c1.temporal_relation(&c2) == TemporalRelation::Concurrent);
        assert!(c2.temporal_relation(&c1) == TemporalRelation::Concurrent);
    }

    #[test]
    fn test_merged() {
        let base = StrVectorClock::new();
        let c1 = base.incremented("A");
        let c2 = base.incremented("B");

        let m = c1.merge_with(&c2);

        assert!(m.temporal_relation(&c1) == TemporalRelation::EffectOf);
        assert!(c1.temporal_relation(&m) == TemporalRelation::Caused);

        assert!(m.temporal_relation(&c2) == TemporalRelation::EffectOf);
        assert!(c2.temporal_relation(&m) == TemporalRelation::Caused);
    }

    #[test]
    fn test_to_vec() {
        let c = StrVectorClock::new();
        let e : Vec<(&'static str, u64)> = vec![];

        assert_eq!(e, c.to_vec());

        let c = c.incremented("A");

        assert_eq!(vec![("A", 1)], c.to_vec());

        let c = c.incremented("B");
        let mut v = c.to_vec();
        v.sort();

        assert_eq!(vec![("A", 1), ("B", 1)], v);
    }
}
