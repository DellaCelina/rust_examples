fn main() {
    let i_vec = vec![1,2,3,4];

    let s_vec = vec![
        String::from("t"),
        String::from("e"),
        String::from("s"),
        String::from("t")
    ];

    // i_vec in 'for' would be moved, into_iter is called
    /*
    for v in i_vec {
        println!("elem {}", v);
    }
    */

    // into iter cannot be &v
    /*
    for &v in i_vec {
        println!("elem {}", v);
    }
    */

    // s_vec in 'for' won't be moved, iter is called
    for v in &s_vec {
        println!("elem {}", v);
    }

    // &v can't be used becase &v with iter means copy,
    // but string can't be copied
    /*
    for &v in &s_vec {
        println!("elem {}", v);
    }
    */

    // because &v means copy, i_vec can use &v
    for &v in &i_vec {
        let t: u32 = v;
        println!("elem {}", t);
    }

    // beacause v means reference, deref should be used to use this
    for v in &i_vec {
        let t: u32 = *v;
        println!("elem {}", t);
    }

    // because v is reference, it can't be copied
    for v in &s_vec {
        let t: String = *v;
        println!("elem {}", t);
    }
}
