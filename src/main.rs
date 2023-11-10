
// Network implementation
struct Network {
    nodes: Vec<Node>
}

impl Network {
    fn new() -> Self {
        Network {
            nodes: Vec::new()
        }
    }

    pub fn add_node(&mut self) {
        let last_node = self.nodes.last();
        let mut node_id = 1;
        if !last_node.is_none() {
            node_id = last_node.unwrap().id + 1;
        }

        let node = Node::new(node_id);
        self.nodes.push(node)
    }
}

// Node implementation
struct Node{
    id: i8,
    blocks: Vec<Block>
}

impl Node {

    fn new(id: i8) -> Self {
        let genesis_block = Block::new();
        let blocks = vec![genesis_block];
        Node {
            id,
            blocks,
        }
    }
}

// Block implementation
struct Block {
    data: String
}

impl Block {
    fn new() -> Self {
        Self {
            data: "genesis_block".to_string(),
        }
    }
}

fn main() {
    // create a network
    let mut network = Network::new();

    //create 10 nodes
    for _ in 0..10 {
        network.add_node();
    }

   for node in network.nodes {
        println!("node = '{}' | genesis block data => '{}'", node.id, node.blocks.first().unwrap().data);
   }
}
