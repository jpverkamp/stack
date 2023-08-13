{
    Value cond = *(stack_ptr--);
    Value block = *(stack_ptr--);

    if (cond.type != TAG_BOOLEAN)
    {
        fprintf(stderr, "Error: if condition must be a boolean\n");
        exit(1);
    }

    if (cond.as_boolean)
    {
        if (block.type == TAG_BLOCK)
        {
            void *f = block.as_block;
            ((void (*)(Name *))f)(names);
        }
        else
        {
            *(++stack_ptr) = block;
        }
    }
}
