{
    Value *c = stack_ptr--;

    assert_type("cond", "stack", TAG_STACK, c, names);

    ValueStack *cases = c->as_stack;

    if (cases->size % 2 != 1)
    {
        printf("cond: expected odd number of arguments, got %zu\n", cases->size);
        exit(1);
    }

    bool found = false;
    for (int i = 0; i < cases->size; i += 2)
    {
        Value *test = vs_get(cases, i);
        Value *body = vs_get(cases, i + 1);

        assert_type("cond (test block)", "block", TAG_BLOCK, test, names);

        void *f = test->as_block;
        ((void (*)(Name *))f)(names);

        Value *test_result = stack_ptr--;

        assert_type("cond (test result)", "boolean", TAG_BOOLEAN, test_result, names);

        if (test_result->as_boolean)
        {
            if (body->type == TAG_BLOCK)
            {
                void *f = body->as_block;
                ((void (*)(Name *))f)(names);
            }
            else
            {
                *(stack_ptr++) = *body;
            }

            found = true;
            break;
        }
    }

    if (!found)
    {
        Value *body = vs_get(cases, cases->size - 1);

        if (body->type == TAG_BLOCK)
        {
            void *f = body->as_block;
            ((void (*)(Name *))f)(names);
        }
        else
        {
            *(stack_ptr++) = *body;
        }
    }
}