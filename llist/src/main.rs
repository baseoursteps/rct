use llist::linked_list::LList;

fn main() {
    let mut l: LList<i32> = LList::default();

    for i in 0..100 {
        l.push(i);
    }

    for i in 0..10 {
        let v = l.pop();
        if v.is_some() {
            println!("{} -> {} ", i, v.unwrap());
        } else {
            println!("got none");
        }
    }
}
