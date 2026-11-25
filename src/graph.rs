use rand::RngExt;

#[derive(Default, Debug, Clone, Copy)]
pub struct Edge {
    pub capacity: f32,
    pub flow: f32,
}
impl Edge {
    fn new(capacity: f32, flow: f32) -> Self {
        Edge { capacity, flow }
    }
    fn residual_capacity(&self) -> f32 {
        self.capacity - self.flow
    }
    pub fn is_saturated(&self) -> bool{
        self.residual_capacity() <= 0.0
    }
}

pub fn get_random()->f32{
    rand::rng().random_range(1..=30) as f32
}

#[derive(Default, Debug, Clone)]
pub struct Node {
    pub id: i32,
    pub neighbors: Vec<i32>,
}

#[derive(Default, Debug, Clone)]
pub struct Graph{
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}
impl Graph {
    pub fn add_node_wh_id(&mut self, id: i32, neighbors: Vec<i32>) {
        //проверка на уникальность
        let mut not_is_free = true;
        while not_is_free{
            for node in self.nodes.iter_mut(){
                if id == node.id{
                    return;
                }
            }
            not_is_free = false;
        }

        //добовление соседям
        for node in &mut self.nodes {
            if neighbors.contains(&node.id) {
                node.neighbors.push(id);
            }
        }

        self.nodes.push(Node{id, neighbors});//создание самой заметки
    }
    pub fn add_node(&mut self, neighbors: Vec<i32>) {
        let id = self.generate_id();
        for node in &mut self.nodes {
            if neighbors.contains(&node.id) {
                node.neighbors.push(id);
            }
        }

        self.nodes.push(Node{id, neighbors});

    }
    pub fn add_neighbors(&mut self, id: i32, neighbors: Vec<i32>){

        // находим node и добовляем ему соседей
        for node in &mut self.nodes {
            if node.id == id {
                if!node.neighbors.iter().any(|item| neighbors.contains(item)){
                    node.neighbors.extend_from_slice(&neighbors);
                    break;
                }
                else { println!("Вершина:{} уже имеет соседей из {neighbors:?}", node.id); return; }

            }
        }
        //находим всех соседей и им добовляем node
        for node in &mut self.nodes {
            if neighbors.contains(&node.id) {
                node.neighbors.push(id);
            }
        }
    }
    pub fn generate_id(&self) -> i32 {
        loop {
            let candidate = rand::rng().random_range(1..1_000_000);
            // Если кандидата нет среди существующих узлов — возвращаем его:
            if !self.nodes.iter().any(|n| n.id == candidate) {
                return candidate;
            }
        }
    }
}