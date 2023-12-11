boxed = {
    "type": "Ref",
    "typename": "Box",
    "addr": "<redacted>",
    "value": {
        "type": "Struct",
        "typename": "pointer::Object",
        "fields": {
            "name": {
                "type": "String",
                "typename": "String",
                "value": "Boxed"
            }
        }
    }
}
arc = {
    "type": "Struct",
    "typename": "alloc::sync::Arc<pointer::Object>",
    "fields": {
        "ptr": {
            "type": "Struct",
            "typename": "core::ptr::non_null::NonNull<alloc::sync::ArcInner<pointer::Object>>",
            "fields": {
                "pointer": {
                    "type": "Ref",
                    "typename": "ptr",
                    "addr": "<redacted>",
                    "value": {
                        "type": "Struct",
                        "typename": "alloc::sync::ArcInner<pointer::Object>",
                        "fields": {
                            "strong": {
                                "type": "Struct",
                                "typename": "core::sync::atomic::AtomicUsize",
                                "fields": {
                                    "value": {
                                        "type": "Prim",
                                        "typename": "usize",
                                        "value": 2
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
                                        "value": 3
                                    }
                                }
                            },
                            "data": {
                                "type": "Struct",
                                "typename": "pointer::Object",
                                "fields": {
                                    "name": {
                                        "type": "String",
                                        "typename": "String",
                                        "value": "Arced"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        },
        "phantom": {
            "type": "Struct",
            "typename": "core::marker::PhantomData<alloc::sync::ArcInner<pointer::Object>>",
            "fields": {}
        }
    }
}
rc = {
    "type": "Struct",
    "typename": "alloc::rc::Rc<pointer::Object>",
    "fields": {
        "ptr": {
            "type": "Struct",
            "typename": "core::ptr::non_null::NonNull<alloc::rc::RcBox<pointer::Object>>",
            "fields": {
                "pointer": {
                    "type": "Ref",
                    "typename": "ptr",
                    "addr": "<redacted>",
                    "value": {
                        "type": "Struct",
                        "typename": "alloc::rc::RcBox<pointer::Object>",
                        "fields": {
                            "strong": {
                                "type": "Struct",
                                "typename": "core::cell::Cell<usize>",
                                "fields": {
                                    "value": {
                                        "type": "Prim",
                                        "typename": "usize",
                                        "value": 3
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
                                        "value": 2
                                    }
                                }
                            },
                            "value": {
                                "type": "Struct",
                                "typename": "pointer::Object",
                                "fields": {
                                    "name": {
                                        "type": "String",
                                        "typename": "String",
                                        "value": "Rced"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        },
        "phantom": {
            "type": "Struct",
            "typename": "core::marker::PhantomData<alloc::rc::RcBox<pointer::Object>>",
            "fields": {}
        }
    }
}
