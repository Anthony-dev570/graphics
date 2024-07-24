pub mod vertex;
pub mod vertex_array;
pub mod atomic;
pub mod program;
pub mod shader;
pub mod graphics_error;
pub mod graphics_pointer;
pub mod uniform;
pub mod font;
pub mod textures;
pub mod a_or_b;
pub mod model;
pub mod bundled_model;
pub mod bundled_mesh;
pub mod frame_buffer;
pub mod render_buffer;
pub mod ui;
pub mod ffi;

pub mod render_texture;

pub mod objects;

const L: &'static str = {
    r#"
    #version 330 core

    struct Light {
        int lightType;

        vec3 direction;
        vec3 position;

        float constant, linear, quadratic;
        float cut_off;
    }
"#
};

#[cfg(test)]
mod tests {
    use std::ptr::{null, null_mut};
    use bumpalo::Bump;

    #[test]
    fn sll() {
        // Definition for singly-linked list.
        #[derive(PartialEq, Eq, Clone, Debug)]
        pub struct ListNode {
            pub val: i32,
            pub next: Option<Box<ListNode>>,
        }

        impl ListNode {
            #[inline]
            fn new(val: i32) -> Self {
                ListNode {
                    next: None,
                    val,
                }
            }
        }

        let _l1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: None,
                })),
            })),
        }));

        let _l2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: None,
                })),
            })),
        }));

        pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut l1 = l1;
            let mut l2 = l2;

            let mut l3: Option<Box<ListNode>> = None;
            let mut l3_tail = null_mut();

            let mut carry = 0;

            while l1.is_some() {
                let a = {
                    let mut value = 0;
                    if let Some(_l1) = l1 {
                        value = _l1.val;
                        l1 = _l1.next;
                    }
                    value
                };
                let b = {
                    let mut value = 0;
                    if let Some(_l2) = l2 {
                        value = _l2.val;
                        l2 = _l2.next;
                    }
                    value
                };

                let mut c = a + b + carry;
                carry = c / 10;
                c = c - 10 * carry;

                match l3.is_none() {
                    true => {
                        l3 = Some(Box::new(ListNode {
                            val: c,
                            next: None,
                        }));
                        l3_tail = l3.as_mut().unwrap() as *mut Box<ListNode>;
                    }
                    false => {
                        unsafe {
                            let tail = &mut *l3_tail;
                            tail.next = Some(Box::new(ListNode {
                                val: c,
                                next: None,
                            }));
                            l3_tail = tail.next.as_mut().unwrap() as *mut Box<ListNode>;
                        }
                    }
                }
            }
            unsafe {
                if carry != 0 {
                    let tail = &mut *l3_tail;
                    tail.next = Some(Box::new(ListNode {
                        val: carry,
                        next: None,
                    }));
                }
            }
            l3
        }

        println!("{:?}", add_two_numbers(_l1, _l2));
    }

    #[test]
    fn it_works() {
        let expression = "5  * 4";

        let ctx = aftermath::Context::new();
        let mut bump = Bump::new();
        let expr = aftermath::Expr::parse(&mut bump, expression, &[]).unwrap();
        let e = ctx.eval(expr).unwrap();
        println!("{:?}", e.norm());
    }
}