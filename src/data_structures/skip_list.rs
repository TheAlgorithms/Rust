use rand::random_range;
use std::{cmp::Ordering, marker::PhantomData, ptr::null_mut};

struct Node<K: Ord, V> {
    key: Option<K>,
    value: Option<V>,
    forward: Vec<*mut Node<K, V>>,
}

impl<K: Ord, V> Node<K, V> {
    pub fn new() -> Self {
        let mut forward = Vec::with_capacity(4);
        forward.resize(4, null_mut());
        Node {
            key: None,
            value: None,
            forward,
        }
    }

    pub fn make_node(capacity: usize, key: K, value: V) -> Self {
        let mut new_node = Self::new();
        new_node.key = Some(key);
        new_node.value = Some(value);
        new_node.forward = Vec::<*mut Node<K, V>>::with_capacity(capacity);
        new_node.forward.resize(capacity, null_mut());
        new_node
    }
}

pub struct SkipList<K: Ord, V> {
    header: *mut Node<K, V>,
    level: usize,
    max_level: usize,
}

impl<K: Ord, V> SkipList<K, V> {
    pub fn new(max_level: usize) -> Self {
        let mut node = Box::new(Node::<K, V>::new());
        node.forward = Vec::with_capacity(max_level);
        node.forward.resize(max_level, null_mut());

        SkipList {
            header: Box::into_raw(node),
            level: 0,
            max_level,
        }
    }

    pub fn search(&self, searched_key: K) -> Option<&V> {
        let mut x = self.header;

        unsafe {
            'outer: for i in (0..self.level).rev() {
                loop {
                    let forward_i = (&*x).forward[i];
                    if forward_i.is_null() {
                        continue 'outer;
                    }

                    let forward_i_key = (*forward_i).key.as_ref();
                    match forward_i_key.cmp(&Some(&searched_key)) {
                        Ordering::Less => {
                            x = forward_i;
                        }
                        _ => {
                            break;
                        }
                    }
                }
            }

            x = (&*x).forward[0];
            if x.is_null() {
                return None;
            }

            match (*x).key.as_ref().cmp(&Some(&searched_key)) {
                Ordering::Equal => {
                    return (*x).value.as_ref();
                }
                _ => {
                    return None;
                }
            }
        }
    }

    pub fn insert(&mut self, searched_key: K, new_value: V) -> bool {
        let mut update = Vec::<*mut Node<K, V>>::with_capacity(self.max_level);
        update.resize(self.max_level, null_mut());

        let mut x = self.header;

        unsafe {
            for i in (0..self.level).rev() {
                loop {
                    let x_forward_i = (&*x).forward[i];
                    if x_forward_i.is_null() {
                        break;
                    }

                    let x_forward_i_key = (*x_forward_i).key.as_ref();
                    match x_forward_i_key.cmp(&Some(&searched_key)) {
                        Ordering::Less => {
                            x = x_forward_i;
                        }
                        _ => {
                            break;
                        }
                    }
                }
                let update_i = &mut update[i];
                *update_i = x;
            }

            let x_forward_i = (&*x).forward[0];
            x = x_forward_i;
            if x.is_null() || (*x).key.as_ref().cmp(&Some(&searched_key)) != Ordering::Equal {
                let v = random_value(self.max_level);
                if v > self.level {
                    for update_i in update.iter_mut().take(v).skip(self.level) {
                        *update_i = self.header;
                    }
                    self.level = v;
                }
                let new_node = Node::make_node(v, searched_key, new_value);
                x = Box::into_raw(Box::new(new_node));

                for (i, t) in update.iter_mut().enumerate().take(self.level) {
                    let x_forward_i = (&mut *x).forward.get_mut(i);
                    if x_forward_i.is_none() {
                        break;
                    }
                    let x_forward_i = x_forward_i.unwrap();
                    let update_i = t;
                    let update_i_forward_i = &mut (&mut **update_i).forward[i];
                    *x_forward_i = *update_i_forward_i;
                    *update_i_forward_i = x;
                }
                return true;
            }
            (*x).value.replace(new_value);
            return true;
        }
    }

    pub fn delete(&mut self, searched_key: K) -> bool {
        let mut update = Vec::<*mut Node<K, V>>::with_capacity(self.max_level);
        update.resize(self.max_level, null_mut());

        let mut x = self.header;

        unsafe {
            for i in (0..self.level).rev() {
                loop {
                    let x_forward_i = (&*x).forward[i];
                    if x_forward_i.is_null() {
                        break;
                    }

                    let x_forward_i_key = (*x_forward_i).key.as_ref();
                    match x_forward_i_key.cmp(&Some(&searched_key)) {
                        Ordering::Less => {
                            x = x_forward_i;
                        }
                        _ => {
                            break;
                        }
                    }
                }
                let update_i = &mut update[i];
                *update_i = x;
            }

            let x_forward_i = *((&*x).forward.first().unwrap());
            x = x_forward_i;

            if x.is_null() {
                return false;
            }

            match (*x).key.as_ref().cmp(&Some(&searched_key)) {
                Ordering::Equal => {
                    for (i, update_i) in update.iter_mut().enumerate().take(self.level) {
                        let update_i_forward_i = &mut (&mut **update_i).forward[i];
                        if update_i_forward_i.is_null() {
                            break;
                        }

                        let x_forward_i = (&mut *x).forward.get_mut(i);
                        if x_forward_i.is_none() {
                            break;
                        }
                        let x_forward_i = x_forward_i.unwrap();
                        *update_i_forward_i = *x_forward_i;
                    }

                    let _v = Box::from_raw(x);

                    loop {
                        if self.level == 0 {
                            break;
                        }

                        let header_forward_level = &(&*self.header).forward[self.level - 1];
                        if header_forward_level.is_null() {
                            self.level -= 1;
                        } else {
                            break;
                        }
                    }
                    return true;
                }
                _ => {
                    return false;
                }
            }
        }
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter::new(self)
    }
}

impl<K: Ord, V> Drop for SkipList<K, V> {
    fn drop(&mut self) {
        let mut node = unsafe { Box::from_raw(self.header) };
        loop {
            let node_forward_0 = node.forward.first().unwrap();
            if node_forward_0.is_null() {
                break;
            }
            node = unsafe { Box::from_raw(*node_forward_0) };
        }
    }
}

pub fn random_value(max: usize) -> usize {
    let mut v = 1usize;
    loop {
        if random_range(1usize..10usize) > 5 && v < max {
            v += 1;
        } else {
            break;
        }
    }
    v
}

pub struct Iter<'a, K: Ord, V> {
    current_node: *mut Node<K, V>,
    _marker: PhantomData<&'a SkipList<K, V>>,
}

impl<'a, K: Ord, V> Iter<'a, K, V> {
    pub fn new(skip_list: &'a SkipList<K, V>) -> Self {
        Iter {
            current_node: skip_list.header,
            _marker: PhantomData,
        }
    }
}

impl<'a, K: Ord, V> Iterator for Iter<'a, K, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let forward_0 = (&*self.current_node).forward.first();
            forward_0?;
            let forward_0 = *forward_0.unwrap();
            if forward_0.is_null() {
                return None;
            }
            self.current_node = forward_0;
            return (*forward_0).value.as_ref();
        }
    }
}

mod test {
    #[test]
    fn insert_and_delete() {
        let mut skip_list = super::SkipList::<&'static str, i32>::new(8);
        skip_list.insert("a", 10);
        skip_list.insert("b", 12);

        {
            let result = skip_list.search("b");
            assert!(result.is_some());
            assert_eq!(result, Some(&12));
        }

        {
            skip_list.delete("b");
            let result = skip_list.search("b");
            assert!(result.is_none());
        }
    }

    #[test]
    fn iterator() {
        let mut skip_list = super::SkipList::<&'static str, i32>::new(8);
        skip_list.insert("h", 22);
        skip_list.insert("a", 12);
        skip_list.insert("c", 11);

        let result: Vec<&i32> = skip_list.iter().collect();
        assert_eq!(result, vec![&12, &11, &22]);
    }
}
