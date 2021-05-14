use std::cmp::PartialOrd;

pub enum Type {
    Max,
    Min,
}

fn lchild(root: usize) -> usize {
    2 * root + 1
}

fn rchild(root: usize) -> usize {
    2 * root + 2
}

fn heapify<T: PartialOrd>(arr: &'_ mut [T], root: usize, t: &Type) {
    let cmp = |i: &T, j: &T| -> bool {
        match t {
            Type::Max => i < j,
            Type::Min => i > j,
        }
    };

    let mut r = root;
    let lc = lchild(root);
    let rc = rchild(root);
    if lc < arr.len() && cmp(&arr[root], &arr[lc]) {
        r = lc;
    }

    if rc < arr.len() && cmp(&arr[r], &arr[rc]) {
        r = rc;
    }

    if r != root {
        arr.swap(root, r);
        heapify(arr, r, t);
    }
}

pub fn heap_sort<T: PartialOrd>(arr: &'_ mut [T], t: Type) {
    for i in (0..arr.len() / 2).rev() {
        heapify(arr, i, &t);
    }

    for i in (1..arr.len()).rev() {
        arr.swap(0, i);
        heapify(&mut arr[0..i], 0, &t);
    }
}
