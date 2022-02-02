%builtins output

from starkware.cairo.common.registers import get_fp_and_pc

struct Message:
    member x : felt
    member y : felt
end

# version 1
func main{output_ptr : felt*}():
    alloc_locals

    local messages_tuple : (Message, Message) = (
        Message(x=276768161078691357748506014484008718823, y=24127044263607486132772889641222586723),
        Message(x=276768161078691357748506014484008718823, y=24127044263607486132772889641222586723),
    )

    let (__fp__, _) = get_fp_and_pc()

    send_batch(message_tuple=cast(&messages_tuple, Message*), size=2)

    return ()
end

func send_batch(message_tuple : Message*, size):
    if size == 0:
        return ()
    end

    %{
        print(ids.message_tuple.x)
        print(ids.message_tuple.y)
    %}
    # Do the calling here whatever cool this could work

    send_batch(message_tuple=message_tuple + Message.SIZE, size=size - 1)
    return ()
end
