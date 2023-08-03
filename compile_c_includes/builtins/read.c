{
    // Read a line from stdin into newly allocated memory
    // TODO: (When) can we free this memory?
    char *line = malloc(sizeof(char) * 1024);
    if (fgets(line, 1024, stdin) == NULL)
    {
        fprintf(stderr, "Error reading from stdin\n");
        exit(1);
    }

    // Strip newline
    line[strlen(line) - 1] = '\0';

    Value v = {.type = TAG_STRING, .as_string = line};
    *(++stack_ptr) = v;
}