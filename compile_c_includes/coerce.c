
// Convert two values to have the same type by upgrading if necessary
void coerce(Value *a, Value *b)
{
    if (a->type == b->type)
    {
        return;
    }

    if (a->type == TAG_NUMBER_INTEGER && b->type == TAG_NUMBER_FLOAT)
    {
        a->type = TAG_NUMBER_FLOAT;
        a->as_float = (double)a->as_integer;
    }

    if (a->type == TAG_NUMBER_FLOAT && b->type == TAG_NUMBER_INTEGER)
    {
        b->type = TAG_NUMBER_FLOAT;
        b->as_float = (double)b->as_integer;
    }
}