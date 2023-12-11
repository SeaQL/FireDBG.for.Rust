car = {
    "type": "Struct",
    "typename": "object::Car",
    "fields": {
        "brand": {
            "type": "String",
            "typename": "&str",
            "value": "Ford"
        },
        "engine": {
            "type": "Struct",
            "typename": "object::Engine",
            "fields": {
                "config": {
                    "type": "Union",
                    "typeinfo": {
                        "name": "object::EngineConfig",
                        "variants": [
                            "Inline",
                            "Vshape"
                        ]
                    },
                    "variant": "Inline",
                    "fields": {
                        "i": {
                            "type": "Prim",
                            "typename": "i32",
                            "value": 4
                        }
                    }
                },
                "pistons": {
                    "type": "Array",
                    "typename": "vec",
                    "data": [
                        {
                            "type": "Struct",
                            "typename": "object::Piston",
                            "fields": {
                                "0": {
                                    "type": "Prim",
                                    "typename": "u8",
                                    "value": 1
                                }
                            }
                        },
                        {
                            "type": "Struct",
                            "typename": "object::Piston",
                            "fields": {
                                "0": {
                                    "type": "Prim",
                                    "typename": "u8",
                                    "value": 2
                                }
                            }
                        },
                        {
                            "type": "Struct",
                            "typename": "object::Piston",
                            "fields": {
                                "0": {
                                    "type": "Prim",
                                    "typename": "u8",
                                    "value": 3
                                }
                            }
                        },
                        {
                            "type": "Struct",
                            "typename": "object::Piston",
                            "fields": {
                                "0": {
                                    "type": "Prim",
                                    "typename": "u8",
                                    "value": 4
                                }
                            }
                        }
                    ]
                }
            }
        },
        "gearbox": {
            "type": "Enum",
            "typename": "object::Gearbox",
            "variant": "Manual"
        }
    }
}
car = {
    "type": "Struct",
    "typename": "object::Car",
    "fields": {
        "brand": {
            "type": "String",
            "typename": "&str",
            "value": "Mazda"
        },
        "engine": {
            "type": "Struct",
            "typename": "object::Engine",
            "fields": {
                "config": {
                    "type": "Union",
                    "typeinfo": {
                        "name": "object::EngineConfig",
                        "variants": [
                            "Inline",
                            "Vshape"
                        ]
                    },
                    "variant": "Vshape",
                    "fields": {
                        "0": {
                            "type": "Prim",
                            "typename": "i16",
                            "value": 3
                        },
                        "1": {
                            "type": "Prim",
                            "typename": "i16",
                            "value": 3
                        }
                    }
                },
                "pistons": {
                    "type": "Array",
                    "typename": "vec",
                    "data": []
                }
            }
        },
        "gearbox": {
            "type": "Enum",
            "typename": "object::Gearbox",
            "variant": "Automatic"
        }
    }
}