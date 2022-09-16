use ouroboros::self_referencing;
use std::cell::RefCell;

#[derive(Debug)]
struct A {
    pub name: String,
}

#[derive(Debug)]
struct B {
    val: String,
}

#[self_referencing]
struct C {
    captures: A,
    thunks: B,
    locals: RefCell<A>,
    #[borrows(captures, thunks, locals)]
    #[covariant]
    fun: Box<dyn Fn(A) -> B + 'this>,
}

fn main() {
    let foo = CBuilder {
        captures: A {
            name: "caps".to_string(),
        },
        thunks: B {
            val: "thunks".to_string(),
        },
        locals: RefCell::new(A {
            name: "locals".to_string(),
        }),
        fun_builder: |_caps: &A, _thunks: &B, loc: &RefCell<A>| {
            Box::new(|init: A| {
                // Should print 'Before call A { name: "locals" }'
                // In practice, using the locals before they are set is UB,
                // as the state of the locals is just unknown at this point.
                println!("Before call {:?}", loc.borrow());
                loc.borrow_mut().name = init.name;
                // Should print 'After call A { name: "changed" }'
                println!("After call {:?}", loc.borrow());
                B { val: "done".to_string() }
            })
        },
    }
    .build();

    let res = (foo.borrow_fun())(A {
        name: "changed".to_string(),
    });
    println!("{:?}", res);
}

