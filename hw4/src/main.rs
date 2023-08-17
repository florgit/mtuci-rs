struct CustomVector<T> {
    data: Vec<T>,
}

impl<T: std::default::Default> CustomVector<T> {
    fn new() -> Self {
        CustomVector { data: Vec::new() }
    }

    fn with_capacity(capacity: usize) -> Self {
        CustomVector { data: Vec::with_capacity(capacity) }
    }

    fn push(&mut self, item: T) {
        self.data.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        if index < self.data.len() {
            Some(self.data.remove(index))
        } else {
            None
        }
    }

    fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    fn resize(&mut self, new_size: usize) {
        if new_size > self.data.len() {
            let additional = new_size - self.data.len();
            self.data.reserve_exact(additional);
            self.data.resize_with(new_size, Default::default);
        } else {
            self.data.truncate(new_size);
        }
    }
}

fn main() {
    let mut my_vector = CustomVector::new();

    my_vector.push(10);
    my_vector.push(20);
    my_vector.push(30);

    println!("Vector: {:?}", my_vector.data);

    my_vector.remove(1);
    println!("Vector after remove: {:?}", my_vector.data);

    if let Some(item) = my_vector.pop() {
        println!("Popped item: {}", item);
    }

    my_vector.resize(2);
    println!("Vector after resize: {:?}", my_vector.data);
}
