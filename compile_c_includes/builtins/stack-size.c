{
    Value *s = stack_ptr--;

    assert_type("stack-size", "stack", TAG_STACK, s, names);

    Value v = {.type = TAG_NUMBER_INTEGER, .as_integer = v->as_stack->size};
    *(++stack_ptr) = v;
}
