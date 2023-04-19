void stack_dump()
{
    for (Value *ptr = stack; ptr <= stack_ptr; ptr++)
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
            // TODO: Error
        }

        for (int i = 0; i < ptr->name_count; i++)
        {
            printf("@%s", get_name(ptr->names[i]));
        }

        printf(" ");
    }
    printf("\n");
}