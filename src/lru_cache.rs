use std::{collections::{HashMap, VecDeque}, option};

/*
move_to_front(&mut self, key: K):

Este método privado mueve un elemento existente al frente de la lista de orden de uso, indicando que ha sido el más recientemente accedido.
evict(&mut self):

Este método privado se encarga de eliminar el elemento menos utilizado recientemente cuando la cache alcanza su capacidad máxima. Se ejecuta automáticamente al agregar un nuevo elemento cuando la cache está llena.
*/

pub struct LRUCache<K, V> {
    capacity: usize,
    map: HashMap<K, V>,
    order: VecDeque<K>, 
}

impl<K: Clone + std::cmp::Eq + std::hash::Hash + for<'a> std::cmp::PartialEq<&'a K>, V> LRUCache<K, V> {
    pub fn new(capacity: usize) -> LRUCache<K, V> {
        LRUCache {
            capacity,
            map: HashMap::with_capacity(capacity),
            order: VecDeque::with_capacity(capacity),
        }
    }

    pub fn put(&mut self, key: &K, value: V) {
        if self.map.contains_key(&key) {
            self.order.retain(|k| k != &key);
        } else {
            self.evict();
        }
        
        self.map.insert(key.clone(), value);
        self.order.push_front(key.clone());
    }

    pub fn get(&mut self, key: K) -> Option<&V> {
        if let Some(value) = self.map.get(&key) {
            self.order.retain(|k|k!=&key);
            self.order.push_front(key);
            Some(value)  // Retornar el valor
        } else {
            None  // La clave no existe
        }
    }


    pub fn remove(&mut self, key: &K) {
        if self.map.contains_key(&key) {  
            self.order.retain(|k| k != key);
            self.map.remove(&key);
        } 
    }

    pub fn move_to_front(&mut self, key: K) {
        if self.map.contains_key(&key) {
            self.order.retain(|k|k!=&key);
            self.order.push_front(key);
        }
    }
    pub fn move_to_back(&mut self, key: K) {
        if self.map.contains_key(&key) {
            self.order.retain(|k|k!=&key);
            self.order.push_back(key);
        }
    }

    fn evict(&mut self) {
        if self.order.len() >= self.capacity {
            if let Some(t) = self.order.pop_back() {
                self.map.remove(&t);
            }
        }
    }
}