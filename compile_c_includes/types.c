
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
} Value;
