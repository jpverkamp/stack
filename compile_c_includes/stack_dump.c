void stack_dump(Name *names)
{
    if (stack_ptr == stack)
    {
        fprintf(stderr, " STACK: <empty> ");
    }
    else
    {
        fprintf(stderr, " STACK: ");
        for (Value *ptr = stack_ptr; ptr != stack; ptr--)
        {
            value_write(stderr, ptr);
            fprintf(stderr, " ");
        }
    }

    if (names != NULL)
    {
        fprintf(stderr, "NAMES: ");
        while (names != NULL)
        {
            fprintf(stderr, "%s=", get_name(names->name));
            value_write(stderr, names->value);

            if (names->boundary && names->prev != NULL)
            {
                fprintf(stderr, " | ");
            }
            else
            {
                fprintf(stderr, " ");
            }

            names = names->prev;
        }
    }

    fprintf(stderr, "\n");
}