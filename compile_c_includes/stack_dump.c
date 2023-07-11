void stack_dump(Name *names)
{
    if (stack_ptr == stack)
    {
        printf(" STACK: <empty>");
    }
    else
    {
        printf(" STACK: ");
        for (Value *ptr = stack_ptr; ptr != stack; ptr--)
        {
            if (ptr->type == TAG_NUMBER_INTEGER)
            {
                printf("%lld", ptr->as_integer);
            }
            else if (ptr->type == TAG_NUMBER_FLOAT)
            {
                printf("%f", ptr->as_float);
            }
            else if (ptr->type == TAG_STRING)
            {
                printf("%s", ptr->as_string);
            }
            else if (ptr->type == TAG_BOOLEAN)
            {
                printf(ptr->as_boolean ? "true" : "false");
            }
            else if (ptr->type == TAG_BLOCK)
            {
                printf("{block}");
            }
            else
            {
                printf("{UNKNOWN}");
            }
            printf(" ");
        }
    }

    if (names != NULL)
    {
        printf("NAMES: ");
        while (names != NULL)
        {
            printf("%s=", get_name(names->name));
            if (names->value->type == TAG_NUMBER_INTEGER)
            {
                printf("%lld", names->value->as_integer);
            }
            else if (names->value->type == TAG_NUMBER_FLOAT)
            {
                printf("%f", names->value->as_float);
            }
            else if (names->value->type == TAG_STRING)
            {
                printf("%s", names->value->as_string);
            }
            else if (names->value->type == TAG_BOOLEAN)
            {
                printf(names->value->as_boolean ? "true" : "false");
            }
            else if (names->value->type == TAG_BLOCK)
            {
                printf("{block}");
            }
            else
            {
                printf("{UNKNOWN}");
            }

            if (names->boundary && names->prev != NULL)
            {
                printf(" | ");
            }
            else
            {
                printf(" ");
            }

            names = names->prev;
        }
    }

    printf("\n");
}