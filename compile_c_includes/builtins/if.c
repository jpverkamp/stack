{
    Value cond = *(stack_ptr--);
    Value if_false = *(stack_ptr--);
    Value if_true = *(stack_ptr--);

    assert_type("if", "boolean", TAG_BOOLEAN, &cond, names);

    Value v = (cond.as_boolean ? if_true : if_false);

    if (v.type == TAG_BLOCK)
    {
        void *f = v.as_block;
        ((void (*)(Name *))f)(names);
    }
    else
    {
        *(++stack_ptr) = v;
    }
}