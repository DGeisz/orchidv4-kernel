use std::collections::HashMap;

#[derive(Clone)]
pub struct FeatureSubtreeRefRecord {
    /// If a feature in the subtree is referenced, it's listed
    /// here along with the number of times it is referenced
    /// outside the subtree
    ref_count_of_subtree_features: HashMap<u128, u32>,

    /// Contains the number of feature references within
    /// the subtree to features outside the subtree
    feature_refs_in_subtree: HashMap<u128, u32>,
}

impl FeatureSubtreeRefRecord {
    pub fn new() -> FeatureSubtreeRefRecord {
        FeatureSubtreeRefRecord {
            ref_count_of_subtree_features: HashMap::new(),
            feature_refs_in_subtree: HashMap::new(),
        }
    }

    fn fully_reduce(&self) -> FeatureSubtreeRefRecord {
        let mut ref_counts = self.ref_count_of_subtree_features.clone();
        let mut feature_refs = self.feature_refs_in_subtree.clone();

        for (ref_id, ref_count) in &self.ref_count_of_subtree_features {
            if let Some(num_feature_refs) = self.feature_refs_in_subtree.get(ref_id) {
                if num_feature_refs >= ref_count {
                    ref_counts.remove(ref_id);
                    feature_refs.remove(ref_id);
                } else {
                    feature_refs.remove(ref_id);
                    ref_counts.insert(ref_id.clone(), ref_count - num_feature_refs);
                }
            }
        }

        FeatureSubtreeRefRecord {
            ref_count_of_subtree_features: ref_counts,
            feature_refs_in_subtree: feature_refs,
        }
    }

    pub fn add_ref_count(&mut self, feature_id: u128, ref_count: u32) {
        if ref_count > 0 {
            self.ref_count_of_subtree_features
                .insert(feature_id, ref_count);
        }
    }

    pub fn add_feature_ref(&mut self, feature_id: u128) {
        let current_count = self.feature_refs_in_subtree.get(&feature_id);

        match current_count {
            Some(count) => self.feature_refs_in_subtree.insert(feature_id, count + 1),
            None => self.feature_refs_in_subtree.insert(feature_id, 1),
        };
    }

    pub fn reconcile(&self, other: &FeatureSubtreeRefRecord) -> FeatureSubtreeRefRecord {
        /* First combine the two records */
        let mut new_ref_record = self.clone();

        for (id, count) in &other.ref_count_of_subtree_features {
            match new_ref_record.ref_count_of_subtree_features.get(id) {
                Some(old_count) => {
                    new_ref_record
                        .ref_count_of_subtree_features
                        .insert(id.clone(), old_count + count);
                }
                None => {
                    new_ref_record
                        .ref_count_of_subtree_features
                        .insert(id.clone(), count.clone());
                }
            }
        }

        for (id, count) in &other.feature_refs_in_subtree {
            match new_ref_record.feature_refs_in_subtree.get(id) {
                Some(old_count) => {
                    new_ref_record
                        .feature_refs_in_subtree
                        .insert(id.clone(), old_count + count);
                }
                None => {
                    new_ref_record
                        .feature_refs_in_subtree
                        .insert(id.clone(), count.clone());
                }
            }
        }

        /* Then reduce the new record */
        new_ref_record.fully_reduce()
    }

    pub fn any_external_dependents(&self) -> bool {
        let full_reduction = self.fully_reduce();

        !full_reduction.ref_count_of_subtree_features.is_empty()
    }
}
