#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#define TAG_NUMBER 0
#define TAG_NUMBER_INTEGER 1
#define TAG_NUMBER_RATIONAL 2
#define TAG_NUMBER_FLOAT 3
#define TAG_NUMBER_COMPLEX 4

#define TAG_STRING 16
#define TAG_BOOLEAN 17
#define TAG_BLOCK 18

#define STACKED_NAME_MAX 4

typedef struct
{
    uint8_t type;
    union
    {
        int64_t as_integer;
        double as_float;

        char *as_string;
        bool as_boolean;
        uint8_t as_block;
    };

    uint8_t name_count;
    uint8_t names[4];
    // TODO: more than 4 names
} Value;

Value *lookup(Value *stack, Value *stack_ptr, uint8_t name)
{
    Value *ptr = stack_ptr;
    while (ptr >= stack)
    {
        for (int i = 0; i < ptr->name_count; i++)
        {
            if (ptr->names[i] == name)
            {
                return ptr;
            }
        }
        ptr--;
    }

    printf("Name not found: %d", name);
    exit(1);
}

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
        b->as_float = (double)a->as_integer;
    }
}
