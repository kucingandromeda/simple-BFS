#[derive(Debug)]
enum BfsFormat{
    Parent(i32),
    Children_type_1([i32;1]),
    Children_type_2([i32;2]),
    Children_type_3([i32;3]),
}

fn main() {

    let data_bfs = [
        [
            BfsFormat::Parent(0),
            BfsFormat::Children_type_3([1, 2, 3])
        ],
        [
            BfsFormat::Parent(1),
            BfsFormat::Children_type_2([4, 5])
        ],
        [
            BfsFormat::Parent(2),
            BfsFormat::Children_type_2([6, 7])
        ],
        [
            BfsFormat::Parent(3),
            BfsFormat::Children_type_3([8, 9, 10])
        ],
        [
            BfsFormat::Parent(8),
            BfsFormat::Children_type_2([11, 12])
        ],
        [
            BfsFormat::Parent(10),
            BfsFormat::Children_type_2([12, 13])
        ]
    ];

    calculate_BFS_algo(data_bfs);
}

fn calculate_BFS_algo(BFS:[[BfsFormat; 2]; 6]){
   
   let first = BFS.get(0)
    .expect("fail to get parent of graph");

    let mut queue = Vec::new();
    let mut processed:Vec<i32> = Vec::new();
    let target_parent = first.get(0)
        .unwrap();

    if let BfsFormat::Parent(value) = target_parent{
        queue.push(value.clone());
    }

    loop {

        if queue.len() <= 0 {
            break;
        }

        processing(&mut queue, &mut processed, &BFS);   
        println!("==============================================");
        println!("queue:\n {:?} \n", queue);
        println!("processed:\n {:?} \n", processed);
        println!("==============================================");
    }
}

fn processing(queue: &mut Vec<i32>, processed:&mut Vec<i32>, BFS:&[[BfsFormat; 2]; 6]){
    let processing_value = *queue.get(0)
        .unwrap();
    processed.push(processing_value);
    queue.remove(0);


        for i in BFS{
            for ii in i{
                match ii {
                    BfsFormat::Parent(value) => {
                        if value != &processing_value{
                            break;
                        }
                    },
                    BfsFormat::Children_type_1(value) => {
                        for item in value{
                            if queue.contains(item){
                                continue;
                            }

                            queue.push(*item);
                        }
                    },
                    BfsFormat::Children_type_2(value) => {
                        for item in value{

                            if queue.contains(item){
                                continue;
                            }

                            queue.push(*item);
                        }
                    },
                    BfsFormat::Children_type_3(value) => {
                        for item in value{

                            if queue.contains(item){
                                continue;
                            }

                            queue.push(*item);
                        }
                    }
                }
            }
        }
    

    // processing(queue, BFS);

}