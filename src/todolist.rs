use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub struct TodoList {
    items: HashMap<String, bool>,
}

impl TodoList {
    pub fn new() -> TodoList {
        let items: HashMap<String, bool> = HashMap::new();

        TodoList { items: items }
    }

    pub fn add(&mut self, key: String) {
        match self.items.entry(key) {
            Entry::Vacant(e) => {
                e.insert(true);
            }
            Entry::Occupied(_) => {
                println!("Task already exists");
            }
        }
    }

    pub fn mark(&mut self, key: String, value: bool) -> Result<String, String> {
        let x = self.items.get_mut(&key).ok_or(&key)?;
        *x = value;
        Ok(key)
    }

    pub fn list(&self) -> (impl Iterator<Item = &String>, impl Iterator<Item = &String>) {
        (
            self.items.iter().filter(|x| *x.1 == true).map(|x| x.0),
            self.items.iter().filter(|x| *x.1 == false).map(|x| x.0),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_todo() {
        let todo = TodoList::new();
    }

    #[test]
    fn add_item() {
        let mut todo = TodoList::new();
        todo.add(String::from("Dummy task"));
        assert_eq!(todo.items.get("Dummy task"), Some(&true))
    }

    #[test]
    fn add_item_already_exist() {
        let mut todo = TodoList::new();
        todo.add(String::from("task1"));
        todo.add(String::from("task1")); // duplicate
        assert_eq!(todo.items.get("task1"), Some(&true));
        assert_eq!(todo.items.len(), 1);
    }

    #[test]
    fn add_item_does_not_change_value() {
        let mut todo = TodoList::new();
        todo.add(String::from("task1"));

        if let Some(x) = todo.items.get_mut("task1") {
            *x = false;
        }

        todo.add(String::from("task1"));
        assert_eq!(todo.items.get("task1"), Some(&false));
        assert_eq!(todo.items.len(), 1);
    }

    #[test]
    fn mark_item() {
        let mut todo = TodoList::new();
        todo.add(String::from("task1"));
        todo.mark(String::from("task1"), false);
        assert_eq!(todo.items.get("task1"), Some(&false));
        todo.mark(String::from("task1"), true);
        assert_eq!(todo.items.get("task1"), Some(&true))
    }

    #[test]
    fn mark_item_does_not_exist() {
        let mut todo = TodoList::new();
        assert_eq!(
            todo.mark(String::from("mark done"), false),
            Err(String::from("task didn't exist"))
        );
    }

    #[test]
    fn list_items() {
        let mut todo = TodoList::new();
        todo.add(String::from("Something to do"));
        todo.add(String::from("Something else to do"));
        todo.add(String::from("Something done"));
        todo.mark(String::from("Something done"), false);

        let (todo_items, done_items) = todo.list();

        let todo_items: Vec<String> = todo_items.cloned().collect();
        let done_items: Vec<String> = done_items.cloned().collect();

        assert!(todo_items.iter().any(|e| e == "Something to do"));
        assert!(todo_items.iter().any(|e| e == "Something else to do"));
        assert_eq!(todo_items.len(), 2);
        assert!(done_items.iter().any(|e| e == "Something done"));
        assert_eq!(done_items.len(), 1);
    }
}
