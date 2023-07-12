{
    Value *v = stack_ptr;

    if (v->type == TAG_NUMBER_INTEGER)
    {
        // Do nothing, already an int
    }
    else if (v->type == TAG_NUMBER_FLOAT)
    {
        v->as_integer = (int64_t)v->as_float;
    }
    else if (v->type == TAG_STRING)
    {
        printf("todo: cast string to int");
        exit(1);
    }
    else if (v->type == TAG_BOOLEAN)
    {
        v->as_integer = v->as_boolean ? 1 : 0;
    }
    else if (v->type == TAG_BLOCK)
    {
        printf("error: cannot cast block to int");
        exit(1);
    }
    else
    {
        printf("error: unknown type to cast to int");
        exit(1);
    }

    v->type = TAG_NUMBER_INTEGER;
}