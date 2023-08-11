{
    Value *v = stack_ptr--;
    Value *s = stack_ptr--;

    assert_type("stack-push!", "stack", TAG_STACK, s, names);

    ValueStack *stack = s->as_stack;
    vs_push(stack, *v);
}