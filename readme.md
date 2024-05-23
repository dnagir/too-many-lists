# Learning Rust With Entirely Too Many Linked Lists

Playing with https://rust-unofficial.github.io/too-many-lists/index.html


# Learnings

- [Option.as_ref](https://doc.rust-lang.org/std/option/enum.Option.html#method.as_ref), [Option.as_mut](https://doc.rust-lang.org/std/option/enum.Option.html#method.as_mut) and [Option.rs_deref](https://doc.rust-lang.org/std/option/enum.Option.html#method.as_deref)
- [Option.take](https://doc.rust-lang.org/std/option/enum.Option.html#method.take)
- [Option.and_then](https://doc.rust-lang.org/std/option/enum.Option.html#method.and_then)
- [Rc.try_unwrap](https://doc.rust-lang.org/std/rc/struct.Rc.html#method.try_unwrap)
- [RefCell.borrow_mut](https://doc.rust-lang.org/std/cell/struct.RefCell.html#method.borrow_mut)
- Crazy: `Rc::try_unwrap(old_head).ok().unwrap().into_inner().data`. When would it panic with the `unwrap` here?
- [Big lesson](https://rust-unofficial.github.io/too-many-lists/fourth-peek.html) API has to break when using `RefCell` - from simple `Option<&T>` to `Option<Ref<T>>`.
