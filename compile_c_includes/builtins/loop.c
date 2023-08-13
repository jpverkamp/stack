{
    Value iter = *(stack_ptr--);
    Value block = *(stack_ptr--);

    assert_type("loop", "block", TAG_BLOCK, &block, names);

    if (iter.type == TAG_NUMBER_INTEGER)
    {
        for (int i = 0; i < iter.as_integer; i++)
        {
            Value v = {.type = TAG_NUMBER_INTEGER, .as_integer = i};
            *(++stack_ptr) = v;

            void *f = block.as_block;
            ((void (*)(Name *))f)(names);
        }
    }
    else
    {
        fprintf(stderr, "Error: loop iterator must be an integer (others todo)\n");
        exit(1);
    }
}