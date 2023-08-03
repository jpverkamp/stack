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
        v->as_integer = atoi(v->as_string);
    }
    else if (v->type == TAG_BOOLEAN)
    {
        v->as_integer = v->as_boolean ? 1 : 0;
    }
    else if (v->type == TAG_BLOCK)
    {
        fprintf(stderr, "error: cannot cast block to int");
        exit(1);
    }
    else
    {
        fprintf(stderr, "error: unknown type to cast to int");
        exit(1);
    }

    v->type = TAG_NUMBER_INTEGER;
}