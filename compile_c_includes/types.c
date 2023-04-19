
// Values on the stack
typedef struct
{
    uint8_t type;
    union
    {
        int64_t as_integer;
        double as_float;

        char *as_string;
        bool as_boolean;
        void *as_block;
    };

    uint8_t name_count;
    uint8_t names[4];
    // TODO: more than 4 names
} Value;