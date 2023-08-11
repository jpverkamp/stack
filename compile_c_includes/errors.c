void assert_type(char *name, char *type_name, uint8_t type_tag, Value *value, Name *names)
{
    if (value->type != type_tag)
    {
        fprintf(stderr, "Error in %s, expected a %s, got: ", name, type_name);
        value_write(stderr, value);
        fprintf(stderr, " with");
        stack_dump(names);
        exit(1);
    }
}
