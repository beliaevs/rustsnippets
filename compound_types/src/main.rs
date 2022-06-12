fn main() {
    let a = (1, "hello", 7.9);
    let s = "avva";
    println!("tuple {} {} {}", a.0, a.1, a.2);
    println!("string: {}", s);

    let xs = [3, 6, 2];
    let months = ["January", "February", "March", "April", "May", "June"];
    for n in xs {
        print!("{} ", n);
    }
    println!("");

    for s in months {
        print!("{} ", s);
    }
    println!("");

    let mut buf = String::from("Hello");
    buf.push_str(", world!");
    println!("{}", buf);
    buf.push_str(" Bye world...");
    let mut buf2 = buf.clone();
    println!("buf = {}, buf2 = {}", buf, buf2);
    buf.push_str(" bbb nn");
    print_len_and_capacity(&buf);
    println!("length = {}, capacity = {}", buf.len(), buf.capacity());
    add_suffix(&mut buf2);
    println!("buf2 = {}", buf2); 
    {
        let mut s = String::from("hello");
        // two mutable refs
        // cannot borrow `s` as mutable more than once at a time
        // cannot borrow `s` as immutable because it is also borrowed as mutable
        let r1 = &s;
        let r2 = &s;
        println!("{}, {}", r1, r2);
    }
}

fn print_len_and_capacity(s: &String)
{
    println!("{}, {}", s.len(), s.capacity());
}

fn add_suffix(s: &mut String)
{
    s.push_str("ity");
}
