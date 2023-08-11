{
    Value *i = stack_ptr--;
    Value *s = stack_ptr--;

    assert_type("stack-ref", "stack", TAG_STACK, s, names);
    assert_type("stack-ref", "integer", TAG_NUMBER_INTEGER, i, names);

    ValueStack *stack = s->as_stack;
    Value *v = vs_get(stack, i->as_integer);
    ++stack_ptr = v;
}