{
    Value cond = *(stack_ptr--);
    Value block = *(stack_ptr--);

    assert_type("when", "boolean", TAG_BOOLEAN, &cond, names);
    assert_type("when", "block", TAG_BLOCK, &block, names);

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
