{
    Value cond = *(stack_ptr--);
    Value if_false = *(stack_ptr--);
    Value if_true = *(stack_ptr--);

    if (cond.type != TAG_BOOLEAN)
    {
        printf("Error: if condition must be a boolean\n");
        exit(1);
    }

    Value v = (cond.as_boolean ? if_true : if_false);

    if (v.type == TAG_BLOCK)
    {
        void *f = v.as_block;
        ((void (*)())f)();
    }
    else
    {
        *(++stack_ptr) = v;
    }
}