{
    Value *v = stack_ptr;

    if (v->type == TAG_NUMBER_INTEGER)
    {
        v->as_float = (double)v->as_integer;
    }
    else if (v->type == TAG_NUMBER_FLOAT)
    {
        // Do nothing, already a float
    }
    else if (v->type == TAG_STRING)
    {
        v->as_float = strtod(v->as_string);
    }
    else if (v->type == TAG_BOOLEAN)
    {
        v->as_float = v->as_boolean ? 1.0 : 0.0;
    }
    else if (v->type == TAG_BLOCK)
    {
        printf("error: cannot cast block to float");
        exit(1);
    }
    else
    {
        printf("error: unknown type to cast to float");
        exit(1);
    }

    v->type = TAG_NUMBER_FLOAT;
}