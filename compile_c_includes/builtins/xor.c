{
    Value *b = stack_ptr--;
    Value *a = stack_ptr--;

    assert_type("xor", "boolean", TAG_BOOLEAN, a, names);
    assert_type("xor", "boolean", TAG_BOOLEAN, b, names);

    Value v = {.type = TAG_BOOLEAN, .as_boolean = a->as_boolean && !b->as_boolean || !a->as_boolean && b->as_boolean};
    *(++stack_ptr) = v;
}