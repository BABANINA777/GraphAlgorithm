use rand::{Rng, RngExt};


fn main() {
    let mut saturation_counter = 0;
    let mut edges: [Edge; 5] = [Edge::default(); 5];
    //edges = std::array::from_fn(|i:i32| rand::rng().random_range(1..=100) )
    //edges = std::array::from_fn(get_random);
    for i in edges.iter_mut() {
        i.flow = get_random();
        i.capacity = 10.0;
    }
    for (index, edge) in edges.iter_mut().enumerate(){
        if(edge.is_saturated())
        {
            saturation_counter += 1;
            println!("РЕБРО {index} НАСЫЩЕННО")
        }
        else {
            println!("РЕБРО {index} НЕ НАСЫЩЕННО")
        }
        println!("Всего насыщенных ребер: {saturation_counter}");
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct Edge {
    capacity: f32,
    flow: f32,
}
impl Edge {
    fn new(capacity: f32, flow: f32) -> Self {
        Edge { capacity, flow }
    }
    fn residual_capacity(&self) -> f32 {
        self.capacity - self.flow
    }
    fn is_saturated(&self) -> bool{
        self.residual_capacity() <= 0.0
    }
}

fn get_random()->f32{
    rand::rng().random_range(1..=30) as f32
}
struct Node<'a> {
    id: usize,
    neighbors: Vec<&'a Node<'a>>, // Храним ссылки на соседей
} 
