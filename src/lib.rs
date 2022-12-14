//! # Callback Manager
//! 
//! `callback_manager` is for registering and triggering callback functions taking arbitrary number of argument lists.

use std::sync::{Arc, Weak, Mutex};


/// Enumeration of callback handlers.
pub enum CallbackHandler<'a, T: Copy + 'a> {
    Callback0(Box<dyn FnMut() -> () + Send + 'a>),
    Callback1(Box<dyn FnMut(T) -> () + Send + 'a>),
    Callback2(Box<dyn FnMut(T, T) -> () + Send + 'a>),
    Callback3(Box<dyn FnMut(T, T, T) -> () + Send + 'a>),
    Callback4(Box<dyn FnMut(T, T, T, T) -> () + Send + 'a>),
    Callback5(Box<dyn FnMut(T, T, T, T, T) -> () + Send + 'a>),
    Callback6(Box<dyn FnMut(T, T, T, T, T, T) -> () + Send + 'a>),
    Callback7(Box<dyn FnMut(T, T, T, T, T, T, T) -> () + Send + 'a>),
    Callback8(Box<dyn FnMut(T, T, T, T, T, T, T, T) -> () + Send + 'a>),
    Callback9(Box<dyn FnMut(T, T, T, T, T, T, T, T, T) -> () + Send + 'a>),
    Callback10(Box<dyn FnMut(T, T, T, T, T, T, T, T, T, T) -> () + Send + 'a>,),
    Callback11(Box<dyn FnMut(T, T, T, T, T, T, T, T, T, T, T) -> () + Send + 'a>,),
    Callback12(Box<dyn FnMut(T, T, T, T, T, T, T, T, T, T, T, T) -> () + Send + 'a>,),
}

/// Enumeration of parameter lists for each callback handler types.
pub enum CallbackParams<T: Copy> {
    CallParams0(),
    CallParams1(T),
    CallParams2(T, T),
    CallParams3(T, T, T),
    CallParams4(T, T, T, T),
    CallParams5(T, T, T, T, T),
    CallParams6(T, T, T, T, T, T),
    CallParams7(T, T, T, T, T, T, T),
    CallParams8(T, T, T, T, T, T, T, T),
    CallParams9(T, T, T, T, T, T, T, T, T),
    CallParams10(T, T, T, T, T, T, T, T, T, T),
    CallParams11(T, T, T, T, T, T, T, T, T, T, T),
    CallParams12(T, T, T, T, T, T, T, T, T, T, T, T),
}

/// A callback manager struct which holds and triggers collback handlers.
pub struct CallbackManager<'a, T: Copy + 'a> {
    pub handlers: Vec<Weak<Mutex<CallbackHandler<'a, T>>>>
}

impl<'a, T: Copy + 'a> CallbackManager<'a, T> {
    /// Creates a new `CallbackManager` instance.
    /// 
    /// # Examples
    /// ```
    /// let _cb_manager = callback_manager::CallbackManager::<i32>::new();
    /// 
    /// assert_eq!(_cb_manager.handlers.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            handlers: vec![]
        }
    }

    /// Adds a new callback handler.
    /// 
    /// # Examples
    /// 
    /// ```
    /// fn print(x: i32) {
    ///     println!("number is {x}");
    /// }
    /// 
    /// let mut cb_manager = callback_manager::CallbackManager::<i32>::new();
    /// 
    /// cb_manager.add(callback_manager::CallbackHandler::Callback1(Box::new(print)));
    /// 
    /// assert_eq!(cb_manager.handlers.len(), 1);
    /// ```
    pub fn add(&mut self, handler: CallbackHandler<'a, T>) -> Arc<Mutex<CallbackHandler<'a, T>>> {
        let strong_handler = Arc::new(Mutex::new(handler));
        self.handlers.push(Arc::downgrade(&strong_handler));
        Arc::clone(&strong_handler)
    }

    /// Returns active handler counts.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut cb_manager = callback_manager::CallbackManager::<i32>::new();
    /// 
    /// let _handler = cb_manager.add(callback_manager::CallbackHandler::Callback1(Box::new(|x: i32| {println!("number is {x}")})));
    /// 
    /// assert_eq!(cb_manager.active_count(), 1);
    /// ```
    pub fn active_count(&self) -> usize {
        let mut r = 0;
        for weak_handler in self.handlers.iter() {
            if let Some(_) = Weak::upgrade(weak_handler) {
                r += 1;
            }
        }
        r
    }

    fn drop_inactive(&mut self) {
        self.handlers = self.handlers.clone().into_iter().filter(
            |x| if let Some(_) = Weak::upgrade(x) {
                true
            } else {
                false
            }
        ).collect::<Vec<Weak<Mutex<CallbackHandler<'a, T>>>>>();
    }

    fn try_match_params(&self, params: &Vec<CallbackParams<T>>) -> Result<(), String> {
        if params.len() != self.active_count() {
            return Err(String::from("mismatched param counts to active handlers"));
        }

        let mismatching_results = self.handlers.iter().enumerate().into_iter().filter(
            |item| 
                if let Some(mutex_handler) = Weak::upgrade(item.1) {
                    if let Ok(guard_handler) = mutex_handler.lock() {
                        match *guard_handler {
                            CallbackHandler::Callback0(_) => {
                                if let Some(CallbackParams::CallParams0()) = params.get(item.0) {
                                    false
                                } else {
                                    true
                                }
                            },
                            CallbackHandler::Callback1(_) => {
                                if let Some(CallbackParams::CallParams1(_)) = params.get(item.0) {
                                    false
                                } else {
                                    true
                                }
                            },
                            CallbackHandler::Callback2(_) => {
                                if let Some(CallbackParams::CallParams2(..)) = params.get(item.0) {
                                    false
                                } else {
                                    true
                                }
                            },
                            CallbackHandler::Callback3(_) => {
                                if let Some(CallbackParams::CallParams3(..)) = params.get(item.0) {
                                    false
                                } else {
                                    true
                                }
                            },
                            CallbackHandler::Callback4(_) => {
                                if let Some(CallbackParams::CallParams4(..)) = params.get(item.0) {
                                    false
                                } else {
                                    true
                                }
                            },
                            CallbackHandler::Callback5(_) => {
                                if let Some(CallbackParams::CallParams5(..)) = params.get(item.0) {
                                    false
                                } else {
                                    true
                                }
                            },
                            CallbackHandler::Callback6(_) => {
                                if let Some(CallbackParams::CallParams6(..)) = params.get(item.0) {
                                    false
                                } else {
                                    true
                                }
                            },
                            CallbackHandler::Callback7(_) => {
                                if let Some(CallbackParams::CallParams7(..)) = params.get(item.0) {
                                    false
                                } else {
                                    true
                                }
                            },
                            CallbackHandler::Callback8(_) => {
                                if let Some(CallbackParams::CallParams8(..)) = params.get(item.0) {
                                    false
                                } else {
                                    true
                                }
                            },
                            CallbackHandler::Callback9(_) => {
                                if let Some(CallbackParams::CallParams9(..)) = params.get(item.0) {
                                    false
                                } else {
                                    true
                                }
                            },
                            CallbackHandler::Callback10(_) => {
                                if let Some(CallbackParams::CallParams10(..)) = params.get(item.0) {
                                    false
                                } else {
                                    true
                                }
                            },
                            CallbackHandler::Callback11(_) => {
                                if let Some(CallbackParams::CallParams11(..)) = params.get(item.0) {
                                    false
                                } else {
                                    true
                                }
                            },
                            CallbackHandler::Callback12(_) => {
                                if let Some(CallbackParams::CallParams12(..)) = params.get(item.0) {
                                    false
                                } else {
                                    true
                                }
                            },
                        }
                    } else {
                        true
                    }
                } else {
                    true
                }
        ).collect::<Vec<(usize, &Weak<Mutex<CallbackHandler<T>>>)>>();

        if mismatching_results.len() > 0 {
            return Err(format!("mismatching params for {} handlers", mismatching_results.len()));
        }

        Ok(())
    }

    /// Runs all active callback handlers with specific parameter lists.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut cb_manager = callback_manager::CallbackManager::<i32>::new();
    /// 
    /// let mut sum = 0;
    /// let sum_pointer = &sum as *const i32;
    /// 
    /// let _handler0 = cb_manager.add(callback_manager::CallbackHandler::Callback1(Box::new(|x: i32| {sum += x;})));
    /// 
    /// cb_manager.run_all(vec![
    ///     callback_manager::CallbackParams::CallParams1(100),
    /// ]);
    /// 
    /// unsafe {assert_eq!(*sum_pointer, 100);}
    /// ```
    pub fn run_all(&mut self, params: Vec<CallbackParams<T>>) -> Result<(), String> {
        self.drop_inactive();

        self.try_match_params(&params)?;

        for (index, weak_handler) in self.handlers.iter().enumerate().into_iter() {
            if let Some(mutex_handler) = Weak::upgrade(weak_handler) {
                if let Ok(mut guard_handler) = mutex_handler.lock() {
                    match &mut *guard_handler {
                        CallbackHandler::Callback0(handler) => {
                            if let Some(CallbackParams::CallParams0()) = params.get(index) {
                                (*handler)();
                            } else {
                                return Err(String::from("unexpected mismatching param"));
                            }
                        },
                        CallbackHandler::Callback1(handler) => {
                            if let Some(CallbackParams::CallParams1(p1)) = params.get(index) {
                                (*handler)(*p1);
                            } else {
                                return Err(String::from("unexpected mismatching param"));
                            }
                        },
                        CallbackHandler::Callback2(handler) => {
                            if let Some(CallbackParams::CallParams2(p1, p2)) = params.get(index) {
                                (*handler)(*p1, *p2);
                            } else {
                                return Err(String::from("unexpected mismatching param"));
                            }
                        },
                        CallbackHandler::Callback3(handler) => {
                            if let Some(CallbackParams::CallParams3(p1, p2, p3)) = params.get(index) {
                                (*handler)(*p1, *p2, *p3);
                            } else {
                                return Err(String::from("unexpected mismatching param"));
                            }
                        },
                        CallbackHandler::Callback4(handler) => {
                            if let Some(CallbackParams::CallParams4(p1, p2, p3, p4)) = params.get(index) {
                                (*handler)(*p1, *p2, *p3, *p4);
                            } else {
                                return Err(String::from("unexpected mismatching param"));
                            }
                        },
                        CallbackHandler::Callback5(handler) => {
                            if let Some(CallbackParams::CallParams5(p1, p2, p3, p4, p5)) = params.get(index) {
                                (*handler)(*p1, *p2, *p3, *p4, *p5);
                            } else {
                                return Err(String::from("unexpected mismatching param"));
                            }
                        },
                        CallbackHandler::Callback6(handler) => {
                            if let Some(CallbackParams::CallParams6(p1, p2, p3, p4, p5, p6)) = params.get(index) {
                                (*handler)(*p1, *p2, *p3, *p4, *p5, *p6);
                            } else {
                                return Err(String::from("unexpected mismatching param"));
                            }
                        },
                        CallbackHandler::Callback7(handler) => {
                            if let Some(CallbackParams::CallParams7(p1, p2, p3, p4, p5, p6, p7)) = params.get(index) {
                                (*handler)(*p1, *p2, *p3, *p4, *p5, *p6, *p7);
                            } else {
                                return Err(String::from("unexpected mismatching param"));
                            }
                        },
                        CallbackHandler::Callback8(handler) => {
                            if let Some(CallbackParams::CallParams8(p1, p2, p3, p4, p5, p6, p7, p8)) = params.get(index) {
                                (*handler)(*p1, *p2, *p3, *p4, *p5, *p6, *p7, *p8);
                            } else {
                                return Err(String::from("unexpected mismatching param"));
                            }
                        },
                        CallbackHandler::Callback9(handler) => {
                            if let Some(CallbackParams::CallParams9(p1, p2, p3, p4, p5, p6, p7, p8, p9)) = params.get(index) {
                                (*handler)(*p1, *p2, *p3, *p4, *p5, *p6, *p7, *p8, *p9);
                            } else {
                                return Err(String::from("unexpected mismatching param"));
                            }
                        },
                        CallbackHandler::Callback10(handler) => {
                            if let Some(CallbackParams::CallParams10(p1, p2, p3, p4, p5, p6, p7, p8, p9, p10)) = params.get(index) {
                                (*handler)(*p1, *p2, *p3, *p4, *p5, *p6, *p7, *p8, *p9, *p10);
                            } else {
                                return Err(String::from("unexpected mismatching param"));
                            }
                        },
                        CallbackHandler::Callback11(handler) => {
                            if let Some(CallbackParams::CallParams11(p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11)) = params.get(index) {
                                (*handler)(*p1, *p2, *p3, *p4, *p5, *p6, *p7, *p8, *p9, *p10, *p11);
                            } else {
                                return Err(String::from("unexpected mismatching param"));
                            }
                        },
                        CallbackHandler::Callback12(handler) => {
                            if let Some(CallbackParams::CallParams12(p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12)) = params.get(index) {
                                (*handler)(*p1, *p2, *p3, *p4, *p5, *p6, *p7, *p8, *p9, *p10, *p11, *p12);
                            } else {
                                return Err(String::from("unexpected mismatching param"));
                            }
                        },
                    }
                } else {
                    return Err(String::from("retreiving mutex guard of handler failure"));
                }
            } else {
                return Err(String::from("unexpected dropped handler"));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static mut OUTPUT: Vec<String> = vec![];

    fn func0() {
        unsafe { OUTPUT.push("calling func0".to_string()); }
    }

    fn func1(p1: i32) {
        unsafe { OUTPUT.push(format!("calling func1: {}", p1)); }
    }

    fn func2(p1: i32, p2: i32) {
        unsafe { OUTPUT.push(format!("calling func2: {}, {}", p1, p2)); }
    }

    fn func3(p1: i32, p2: i32, p3: i32) {
        unsafe { OUTPUT.push(format!("calling func3: {}, {}, {}", p1, p2, p3)); }
    }

    fn func4(p1: i32, p2: i32, p3: i32, p4: i32) {
        unsafe { OUTPUT.push(format!("calling func4: {}, {}, {}, {}", p1, p2, p3, p4)); }
    }

    #[test]
    #[allow(unused_variables)]
    fn test_callback_manager() {
        let mut callback_manager = CallbackManager::<i32>::new();
        assert_eq!(callback_manager.handlers.len(), 0);
        let h1 = callback_manager.add(CallbackHandler::Callback0(Box::new(func0)));
        let h2 = callback_manager.add(CallbackHandler::Callback1(Box::new(func1)));
        {
            let h3 = callback_manager.add(CallbackHandler::Callback2(Box::new(func2)));
        }
        let h4 = callback_manager.add(CallbackHandler::Callback3(Box::new(func3)));
        let h5 = callback_manager.add(CallbackHandler::Callback4(Box::new(func4)));

        if let Err(err) = callback_manager.run_all(vec![
            CallbackParams::CallParams0(),
            CallbackParams::CallParams1(1),
            CallbackParams::CallParams2(1, 2),
            CallbackParams::CallParams3(1, 2, 3),
            CallbackParams::CallParams4(1, 2, 3, 4),
        ]) {
            assert_eq!(err, "mismatched param counts to active handlers".to_string());
        } else {
            panic!("should return error but not");
        }

        if let Err(err) = callback_manager.run_all(
            vec![
                CallbackParams::CallParams0(),
                CallbackParams::CallParams2(1, 2),  // mismatching
                CallbackParams::CallParams2(1, 2),  // mismatching
                CallbackParams::CallParams4(1, 2, 3, 4),
            ]
        ) {
            assert_eq!(err, "mismatching params for 2 handlers".to_string());
        } else {
            panic!("should return error but not");
        }

        callback_manager.run_all(vec![
            CallbackParams::CallParams0(),
            CallbackParams::CallParams1(1),
            CallbackParams::CallParams3(1, 2, 3),
            CallbackParams::CallParams4(1, 2, 3, 4),
        ]).unwrap();

        unsafe {
            assert_eq!(
                OUTPUT,
                vec![
                    "calling func0".to_string(),
                    "calling func1: 1".to_string(),
                    "calling func3: 1, 2, 3".to_string(),
                    "calling func4: 1, 2, 3, 4".to_string(),
                ]
            );
        }
    }
}