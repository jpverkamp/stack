{
    Value *b = stack_ptr--;
    Value *a = stack_ptr--;

    assert_type("mod", "integer", TAG_NUMBER_INTEGER, a, names);
    assert_type("mod", "integer", TAG_NUMBER_INTEGER, b, names);

    Value result = {.type = TAG_NUMBER_INTEGER, .as_integer = a->as_integer % b->as_integer};
    *(++stack_ptr) = result;
}
