use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt::Debug;

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub struct JunctionBox {
    x: usize,
    y: usize,
    z: usize,
}

impl JunctionBox {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BoxPair {
    box1: JunctionBox,
    box2: JunctionBox,
}

impl BoxPair {
    pub fn new(box1: JunctionBox, box2: JunctionBox) -> Self {
        Self { box1, box2 }
    }

    pub fn squared_distance(&self) -> usize {
        let x_diff = self.box1.x.abs_diff(self.box2.x);
        let y_diff = self.box1.y.abs_diff(self.box2.y);
        let z_diff = self.box1.z.abs_diff(self.box2.z);
        x_diff * x_diff + y_diff * y_diff + z_diff * z_diff
    }
}

#[derive(Clone)]
pub struct Circuit {
    boxes: HashSet<JunctionBox>,
}

impl Circuit {
    pub fn new() -> Self {
        Self { boxes: HashSet::new() }
    }

    pub fn size(&self) -> usize {
        self.boxes.len()
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        !self.boxes.is_disjoint(&other.boxes)
    }

    pub fn merge(&mut self, other: &Self) {
        self.boxes.extend(other.boxes.iter().cloned());
    }
}

impl Debug for Circuit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\nCircuit: {:?}", self.boxes)
    }
}

impl From<BoxPair> for Circuit {
    fn from(pair: BoxPair) -> Self {
        let mut circuit = Self::new();
        circuit.boxes.insert(pair.box1);
        circuit.boxes.insert(pair.box2);
        circuit
    }
}

impl From<JunctionBox> for Circuit {
    fn from(box_: JunctionBox) -> Self {
        let mut circuit = Self::new();
        circuit.boxes.insert(box_);
        circuit
    }
}

impl Eq for BoxPair {}

impl PartialEq<Self> for BoxPair {
    fn eq(&self, other: &Self) -> bool {
        self.squared_distance() == other.squared_distance()
    }
}

impl PartialOrd<Self> for BoxPair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BoxPair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.squared_distance().cmp(&other.squared_distance()) //.reverse()
    }
}

impl Eq for Circuit {}

impl PartialEq<Self> for Circuit {
    fn eq(&self, other: &Self) -> bool {
        self.size() == other.size()
    }
}

impl PartialOrd<Self> for Circuit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Circuit {
    fn cmp(&self, other: &Self) -> Ordering {
        self.size().cmp(&other.size()).reverse()
    }
}
