{
    Value v = *(stack_ptr--);
    if (v.type == TAG_NUMBER_INTEGER)
    {
        printf("%lld", v.as_integer);
    }
    else if (v.type == TAG_NUMBER_FLOAT)
    {
        printf("%f", v.as_float);
    }
    else if (v.type == TAG_STRING)
    {
        printf("%s", v.as_string);
    }
    else if (v.type == TAG_BOOLEAN)
    {
        printf("%s", v.as_boolean ? "true" : "false");
    }
    else if (v.type == TAG_BLOCK)
    {
        printf("{block}");
    }
    else
    {
        // TODO: Error
    }
}