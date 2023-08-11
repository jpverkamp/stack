{
    Value *s = stack_ptr--;

    assert_type("stack-size", "stack", TAG_STACK, s, names);

    ValueStack *stack = s->as_stack;
    Value v = {.type = TAG_NUMBER_INTEGER, .as_integer = stack->size};
    *(++stack_ptr) = v;
}
