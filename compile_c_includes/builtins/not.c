{
    Value *a = stack_ptr--;

    assert_type("not", "boolean", TAG_BOOLEAN, a, names);

    Value v = {.type = TAG_BOOLEAN, .as_boolean = !a->as_boolean};
    *(++stack_ptr) = v;
}