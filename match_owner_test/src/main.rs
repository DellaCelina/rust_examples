enum TwoString {
    AString(String),
    BString(String),
}

fn enum_in_value_test() {
    let estr = String::from("test");
    let two = TwoString::AString(estr);
    // a is moved
    // match two {
    // TwoString::AString(a) => (),
    // TwoString::BString(b) => (),
    // }

    // moved a cannot used
    // match two {
    // TwoString::AString(a) => println!("astr {}", a),
    // TwoString::BString(b) => println!("bstr {}", b),
    // }
}

fn enum_in_value_ref_test() {
    let estr = String::from("test");
    let two = TwoString::BString(estr);
    // To not move, a and b should be ref
    // value two itself isn't moved
    // match two {
    // TwoString::AString(ref a) => println!("astr {}", a),
    // TwoString::BString(ref b) => println!("bstr {}", b),
    // }

    // moved a cannot used
    // match two {
    // TwoString::AString(a) => println!("astr {}", a),
    // TwoString::BString(ref b) => println!("bstr {}", b),
    // }

    // a cannot used because a can be moved
    // match two {
    // TwoString::AString(ref a) => println!("astr {}", a),
    // TwoString::BString(b) => println!("bstr {}", b),
    // }
}

fn if_let_test() {
    let estr = String::from("test");
    let mut two = TwoString::AString(estr);

    // if let also move a
    // if let TwoString::AString(a) = two {
    // println!("astr {}", a);
    // } else {
    // println!("no astr");
    // }

    // if let TwoString::AString(ref a) = two {
    // println!("astr {}", a);
    // } else {
    // println!("no astr");
    // }

    // if let also can return result
    // let astr = if let TwoString::AString(a) = two {
    // println!("astr {}", a);
    // a
    // } else {
    // println!("no astr");
    // String::from("test2")
    // };

    // println!("astr test {}", astr);

    // to get &mut, ref mut should be used
    let astr: &mut String = match two {
        TwoString::AString(ref mut a) => a,
        TwoString::BString(ref mut b) => b,
    };

    println!("astr {}", astr);
    println!("dref astr {}", *astr);

    *astr = String::from("test2");
    println!("astr {}", astr);
    println!("dref astr {}", *astr);
}

fn main() {
    println!("Hello, world!");

    enum_in_value_test();
    enum_in_value_ref_test();
    if_let_test();
}
