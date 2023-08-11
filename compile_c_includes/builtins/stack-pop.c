{
    Value *s = stack_ptr--;

    assert_type("stack-pop!", "stack", TAG_STACK, s, names);

    ValueStack *stack = s->as_stack;
    Value *v = vs_pop(stack);
    *(++stack_ptr) = *v;
}