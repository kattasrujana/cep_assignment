use std::collections::HashMap;
use std::hash::Hash; 
trait SortByKey<K, V>
{
    fn sort_by_key(&mut self);
}

impl<K, V> SortByKey<K, V> for HashMap<K, V>
where
    K: Ord + Clone + Hash, 
    V: Clone,
{
    fn sort_by_key(&mut self)
     {
        let mut sorted_pairs: Vec<_> = self.drain().collect();
        sorted_pairs.sort_by_key(|&(ref key, _)| key.clone());
        for (key, value) in sorted_pairs 
        {
            self.insert(key, value);
        }
    }
}

fn main() 
{
    let mut my_map: HashMap<i32, &str> = HashMap::new();
    my_map.insert(3, "apple");
    my_map.insert(1, "banana");
    my_map.insert(2, "orange");

    println!("Original map: {:?}", my_map);

    my_map.sort_by_key();

    println!("Sorted map: {:?}", my_map);
}