{
    Value *i = stack_ptr--;
    Value *v = stack_ptr--;
    Value *s = stack_ptr--;

    assert_type("stack-set!", "stack", TAG_STACK, s, names);
    assert_type("stack-set!", "integer", TAG_NUMBER_INTEGER, i, names);

    ValueStack *stack = s->as_stack;
    vs_set(stack, i->as_integer, *v);
}