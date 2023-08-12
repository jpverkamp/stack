{
    Value iter = *(stack_ptr--);
    Value block = *(stack_ptr--);

    ValueStack *s = vs_init();

    if (block.type != TAG_BLOCK)
    {
        printf("Error: loop block must be a block\n");
        exit(1);
    }

    if (iter.type == TAG_NUMBER_INTEGER)
    {
        for (int i = 0; i < iter.as_integer; i++)
        {
            Value v = {.type = TAG_NUMBER_INTEGER, .as_integer = i};
            *(++stack_ptr) = v;

            void *f = block.as_block;
            ((void (*)(Name *))f)(names);

            vs_push(s, *(stack_ptr--));
        }
    }
    else
    {
        fprintf(stderr, "Error: loop iterator must be an integer (others todo)\n");
        exit(1);
    }

    Value v = {.type = TAG_STACK, .as_stack = s};
    *(++stack_ptr) = v;
}