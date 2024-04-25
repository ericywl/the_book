mod collection;

use collection::AveragedCollection;

fn main() {
    let mut collection = AveragedCollection::new();

    collection.add(23);
    println!("average: {}", collection.average());

    collection.add(184);
    println!("average: {}", collection.average());

    collection.remove();
    println!("average: {}", collection.average());
}
