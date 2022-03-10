use super::Cluster;

impl Cluster {
    pub fn rm_dupl(self, threshold: f64) -> Self {
        match self.kind {
            super::polyhedron::Polyhedron::Icosahedron => self.remove_duplicates(threshold),
            _ => self,
        }
    }

    fn remove_duplicates(self, threshold: f64) -> Self {
        todo!()
    }
}
