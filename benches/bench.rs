#![feature(concat_idents, test)]

extern crate test;

const ITERATION_COUNT: usize = 10_000;

macro_rules! create_benches {
    ($($name: ident: $size: literal),*) => {
        $(
            mod $name {
                struct Data {
                    a: u64,
                    bytes: [u8; $size],
                }

                #[bench]
                fn mutate_self(b: &mut test::Bencher) {
                    enum Value {
                        A(Option<Data>),
                        B(Option<Data>),
                    }

                    fn f(value: &mut Value, i: u64) {
                        match value {
                            Value::A(ref mut data) => {
                                if i == 1 {
                                    let mut data = data.take().unwrap();
                                    data.a += 1;
                                    *value = Value::B(Some(data));
                                }
                            }
                            Value::B(ref mut data) => {
                                if i == 1 {
                                    let mut data = data.take().unwrap();
                                    data.a += 1;
                                    *value = Value::A(Some(data));
                                }
                            }
                        }
                    }

                    b.iter(|| {
                        for _ in 0..$crate::ITERATION_COUNT {
                            let mut value = test::black_box(Value::A(Some(Data {
                                a: 0,
                                bytes: [0; $size],
                            })));
                            f(&mut value, test::black_box(1));
                            test::black_box(value);
                        }
                    });
                }

                #[bench]
                fn mutate_self_unsafe(b: &mut test::Bencher) {
                    enum Value {
                        A(Option<Data>),
                        B(Option<Data>),
                    }

                    fn f(value: &mut Value, i: u64) {
                        match value {
                            Value::A(ref mut data) => {
                                if i == 1 {
                                    let mut data = unsafe { data.take().unwrap_unchecked() };
                                    data.a += 1;
                                    *value = Value::B(Some(data));
                                }
                            }
                            Value::B(ref mut data) => {
                                if i == 1 {
                                    let mut data = unsafe { data.take().unwrap_unchecked() };
                                    data.a += 1;
                                    *value = Value::A(Some(data));
                                }
                            }
                        }
                    }

                    b.iter(|| {
                        for _ in 0..$crate::ITERATION_COUNT {
                            let mut value = test::black_box(Value::A(Some(Data {
                                a: 0,
                                bytes: [0; $size],
                            })));
                            f(&mut value, test::black_box(1));
                            test::black_box(value);
                        }
                    });
                }

                #[bench]
                fn return_self(b: &mut test::Bencher) {
                    enum Value {
                        A(Data),
                        B(Data),
                    }

                    fn f(value: Value, i: u64) -> Value {
                        match value {
                            Value::A(mut data) => {
                                if i == 1 {
                                    data.a += 1;
                                    Value::B(data)
                                } else {
                                    Value::A(data)
                                }
                            }
                            Value::B(mut data) => {
                                if i == 1 {
                                    data.a += 1;
                                    Value::A(data)
                                } else {
                                    Value::B(data)
                                }
                            }
                        }
                    }

                    b.iter(|| {
                        for _ in 0..$crate::ITERATION_COUNT {
                            let value = test::black_box(Value::A(Data {
                                a: 0,
                                bytes: [0; $size],
                            }));
                            let value = f(value, test::black_box(1));
                            test::black_box(value);
                        }
                    });
                }
            }
        )*
    };
}

create_benches!(
    test_16: 16,
    test_32: 32,
    test_64: 64,
    test_128: 128,
    test_256: 256,
    test_512: 512,
    test_1024: 1024
);
