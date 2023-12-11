obj = {
    "type": "Struct",
    "typename": "&dyn pointer::Shape",
    "fields": {
        "pointer": {
            "type": "Ref",
            "typename": "ptr",
            "addr": "0x00007fffffffe3f0",
            "value": {
                "type": "Opaque"
            }
        },
        "vtable": {
            "type": "Ref",
            "typename": "ref",
            "addr": "0x00005555555b5008",
            "value": {
                "type": "Arr",
                "data": [
                    {
                        "type": "Prim",
                        "typename": "usize",
                        "value": 93824992267824
                    },
                    {
                        "type": "Prim",
                        "typename": "usize",
                        "value": 24
                    },
                    {
                        "type": "Prim",
                        "typename": "usize",
                        "value": 8
                    }
                ]
            }
        }
    }
}
boxed = {
    "type": "Struct",
    "typename": "alloc::boxed::Box<dyn pointer::Shape>",
    "fields": {
        "pointer": {
            "type": "Ref",
            "typename": "ptr",
            "addr": "0x00005555555b9be0",
            "value": {
                "type": "Opaque"
            }
        },
        "vtable": {
            "type": "Ref",
            "typename": "ref",
            "addr": "0x00005555555b5028",
            "value": {
                "type": "Arr",
                "data": [
                    {
                        "type": "Prim",
                        "typename": "usize",
                        "value": 93824992267712
                    },
                    {
                        "type": "Prim",
                        "typename": "usize",
                        "value": 24
                    },
                    {
                        "type": "Prim",
                        "typename": "usize",
                        "value": 8
                    }
                ]
            }
        }
    }
}
arc = {
    "type": "Struct",
    "typename": "alloc::sync::Arc<dyn pointer::Shape>",
    "fields": {
        "ptr": {
            "type": "Struct",
            "typename": "core::ptr::non_null::NonNull<alloc::sync::ArcInner<dyn pointer::Shape>>",
            "fields": {
                "pointer": {
                    "type": "Struct",
                    "typename": "*const alloc::sync::ArcInner<dyn pointer::Shape>",
                    "fields": {
                        "pointer": {
                            "type": "Ref",
                            "typename": "ptr",
                            "addr": "0x00005555555b9ae0",
                            "value": {
                                "type": "Struct",
                                "typename": "alloc::sync::ArcInner<dyn pointer::Shape>",
                                "fields": {
                                    "strong": {
                                        "type": "Struct",
                                        "typename": "core::sync::atomic::AtomicUsize",
                                        "fields": {
                                            "value": {
                                                "type": "Prim",
                                                "typename": "usize",
                                                "value": 1
                                            }
                                        }
                                    },
                                    "weak": {
                                        "type": "Struct",
                                        "typename": "core::sync::atomic::AtomicUsize",
                                        "fields": {
                                            "value": {
                                                "type": "Prim",
                                                "typename": "usize",
                                                "value": 1
                                            }
                                        }
                                    },
                                    "data": {
                                        "type": "Struct",
                                        "typename": "dyn pointer::Shape",
                                        "fields": {}
                                    }
                                }
                            }
                        },
                        "vtable": {
                            "type": "Ref",
                            "typename": "ref",
                            "addr": "0x00005555555b5028",
                            "value": {
                                "type": "Arr",
                                "data": [
                                    {
                                        "type": "Prim",
                                        "typename": "usize",
                                        "value": 93824992267712
                                    },
                                    {
                                        "type": "Prim",
                                        "typename": "usize",
                                        "value": 24
                                    },
                                    {
                                        "type": "Prim",
                                        "typename": "usize",
                                        "value": 8
                                    }
                                ]
                            }
                        }
                    }
                }
            }
        },
        "phantom": {
            "type": "Struct",
            "typename": "core::marker::PhantomData<alloc::sync::ArcInner<dyn pointer::Shape>>",
            "fields": {}
        }
    }
}
rc = {
    "type": "Struct",
    "typename": "alloc::rc::Rc<dyn pointer::Shape>",
    "fields": {
        "ptr": {
            "type": "Struct",
            "typename": "core::ptr::non_null::NonNull<alloc::rc::RcBox<dyn pointer::Shape>>",
            "fields": {
                "pointer": {
                    "type": "Struct",
                    "typename": "*const alloc::rc::RcBox<dyn pointer::Shape>",
                    "fields": {
                        "pointer": {
                            "type": "Ref",
                            "typename": "ptr",
                            "addr": "0x00005555555b9a10",
                            "value": {
                                "type": "Struct",
                                "typename": "alloc::rc::RcBox<dyn pointer::Shape>",
                                "fields": {
                                    "strong": {
                                        "type": "Struct",
                                        "typename": "core::cell::Cell<usize>",
                                        "fields": {
                                            "value": {
                                                "type": "Prim",
                                                "typename": "usize",
                                                "value": 1
                                            }
                                        }
                                    },
                                    "weak": {
                                        "type": "Struct",
                                        "typename": "core::cell::Cell<usize>",
                                        "fields": {
                                            "value": {
                                                "type": "Prim",
                                                "typename": "usize",
                                                "value": 1
                                            }
                                        }
                                    },
                                    "value": {
                                        "type": "Struct",
                                        "typename": "dyn pointer::Shape",
                                        "fields": {}
                                    }
                                }
                            }
                        },
                        "vtable": {
                            "type": "Ref",
                            "typename": "ref",
                            "addr": "0x00005555555b5028",
                            "value": {
                                "type": "Arr",
                                "data": [
                                    {
                                        "type": "Prim",
                                        "typename": "usize",
                                        "value": 93824992267712
                                    },
                                    {
                                        "type": "Prim",
                                        "typename": "usize",
                                        "value": 24
                                    },
                                    {
                                        "type": "Prim",
                                        "typename": "usize",
                                        "value": 8
                                    }
                                ]
                            }
                        }
                    }
                }
            }
        },
        "phantom": {
            "type": "Struct",
            "typename": "core::marker::PhantomData<alloc::rc::RcBox<dyn pointer::Shape>>",
            "fields": {}
        }
    }
}
