void stack_dump(Name *names)
{
    if (stack_ptr == stack)
    {
        fprintf(stderr, " STACK: <empty>");
    }
    else
    {
        fprintf(stderr, " STACK: ");
        for (Value *ptr = stack_ptr; ptr != stack; ptr--)
        {
            if (ptr->type == TAG_NUMBER_INTEGER)
            {
                fprintf(stderr, "%lld", ptr->as_integer);
            }
            else if (ptr->type == TAG_NUMBER_FLOAT)
            {
                fprintf(stderr, "%f", ptr->as_float);
            }
            else if (ptr->type == TAG_STRING)
            {
                fprintf(stderr, "%s", ptr->as_string);
            }
            else if (ptr->type == TAG_BOOLEAN)
            {
                fprintf(stderr, ptr->as_boolean ? "true" : "false");
            }
            else if (ptr->type == TAG_BLOCK)
            {
                fprintf(stderr, "{block}");
            }
            else
            {
                fprintf(stderr, "{UNKNOWN}");
            }
            fprintf(stderr, " ");
        }
    }

    if (names != NULL)
    {
        fprintf(stderr, "NAMES: ");
        while (names != NULL)
        {
            fprintf(stderr, "%s=", get_name(names->name));
            if (names->value->type == TAG_NUMBER_INTEGER)
            {
                fprintf(stderr, "%lld", names->value->as_integer);
            }
            else if (names->value->type == TAG_NUMBER_FLOAT)
            {
                fprintf(stderr, "%f", names->value->as_float);
            }
            else if (names->value->type == TAG_STRING)
            {
                fprintf(stderr, "%s", names->value->as_string);
            }
            else if (names->value->type == TAG_BOOLEAN)
            {
                fprintf(stderr, names->value->as_boolean ? "true" : "false");
            }
            else if (names->value->type == TAG_BLOCK)
            {
                fprintf(stderr, "{block}");
            }
            else
            {
                fprintf(stderr, "{UNKNOWN}");
            }

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