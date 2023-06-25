mod ds;

use ds::linked_list::List;

fn main() {

    let mut list: List<i32> = List::new();

    println!("{:?}", list);

    list.add(1);

    println!("{:?}", list);

    list.add(2);
    list.add(3);
    list.add(4);

    println!("{:?}", list);

    list.remove_node_by_value(10);

    let x = list.remove_node_by_index(2).expect("Deu ruim irmaõ");

    println!("Removed: {}", x);

    println!("{:?}", list);

}
