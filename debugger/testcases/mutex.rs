use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex, RwLock},
};

fn mut_cell(cell: &RefCell<u64>) {
    *cell.borrow_mut() += 1;
}

fn mut_rc_cell(cell: Rc<RefCell<u64>>) {
    *cell.borrow_mut() += 1;
}

fn mutex(m: Arc<Mutex<u64>>) {
    let mut v = m.lock().unwrap();
    *v += 1;
}

fn rwlock(m: Arc<RwLock<u64>>) {
    let mut v = m.write().unwrap();
    *v += 1;
}

fn main() {
    let cell = RefCell::new(1u64);
    mut_cell(&cell);
    mut_cell(&cell);
    mut_cell(&cell);
    assert_eq!(*cell.borrow(), 4);
    let cell = Rc::new(cell);
    mut_rc_cell(cell.clone());
    mut_rc_cell(cell.clone());
    mut_rc_cell(cell.clone());
    assert_eq!(*cell.borrow(), 7);
    let m = Arc::new(Mutex::new(1u64));
    mutex(m.clone());
    mutex(m.clone());
    mutex(m.clone());
    assert_eq!(*m.lock().unwrap(), 4);
    let m = Arc::new(RwLock::new(1u64));
    rwlock(m.clone());
    rwlock(m.clone());
    rwlock(m.clone());
    assert_eq!(*m.read().unwrap(), 4);
}
