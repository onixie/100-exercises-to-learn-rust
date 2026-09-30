// #![feature(layout_for_ptr)]

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

struct StringInternal {
    capacity: usize,
    // addr: usize, // UB because of loss of provenance
    addr: *mut u8,
    len: usize,
}

impl Ticket {
    pub fn title(&self) -> String {
        self.title.clone()
    }
}
// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct **stack size** for the respective type.
#[cfg(test)]
mod tests {
    use crate::StringInternal;

    use super::Ticket;
    use std::mem::size_of;

    #[test]
    fn string_size() {
        assert_eq!(size_of::<String>(), size_of::<usize>() * 3);
    }

    fn field_size<T, /*F, */ R>(_: fn(T) -> R) -> usize
// where
    //     F: Fn(T) -> R, // Fn <: FnMut, FnMut <: FnOnce
    //                    // fn pointer, non capturing closure, normal function only
    //                    // Fn call multiple times, immutable state, immutable
    //                    // FnMut call multipe times, mutable state, mutate the receiver
    //                    // FnOnce call only once, consume the receiver
    {
        std::mem::size_of::<R>()
    }

    macro_rules! size_of_field {
        ($Type:ty:$field:ident) => {{
            // UB
            // unsafe {
            //     let null_ptr = ::std::ptr::null::<$Type>();
            //     ::std::mem::size_of_val(&(*null_ptr).$field)
            // }
            // Create a pointer to uninitialized memory instead of null
            //
            // Better?
            let uninit = ::std::mem::MaybeUninit::<$Type>::uninit();
            let ptr = uninit.as_ptr();

            unsafe {
                // Use addr_of! to get a pointer to the field WITHOUT dereferencing ptr
                // let field_ptr = ::std::ptr::addr_of!((*ptr).$field);
                ::std::mem::size_of_val(&(*ptr).$field)
            }
        }};
    }

    #[test]
    fn ticket_size() {
        // let uninit = ::std::mem::MaybeUninit::<Ticket>::uninit();
        // let ptr = uninit.as_ptr();

        // This is a tricky question!
        // The "intuitive" answer happens to be the correct answer this time,
        // but, in general, the memory layout of structs is a more complex topic.
        // If you're curious, check out the "Type layout" section of The Rust Reference
        // https://doc.rust-lang.org/reference/type-layout.html for more information.
        assert_eq!(size_of::<Ticket>(), size_of::<usize>() * 3 * 3);
        assert_eq!(size_of::<Ticket>(), size_of::<String>() * 3);
        assert_eq!(
            size_of::<Ticket>(),
            field_size(Ticket::title)
                + field_size(|t: Ticket| t.description)
                + size_of_field!(Ticket:status)
        );

        let mut a = String::with_capacity(5);
        a += "a";
        let a_addr = a.as_ptr() as usize;
        let b: StringInternal = unsafe { core::mem::transmute(a) };
        assert_eq!(b.addr as usize, a_addr);
        assert_eq!(b.len, 1);
        assert_eq!(b.capacity, 5);

        // avoid memory leak
        let a: String = unsafe { core::mem::transmute(b) };
        // or
        // let a: String = unsafe { String::from_raw_parts(b.addr, b.len, b.capacity) };
        drop(a);
        println!("drop called");
    }
}
