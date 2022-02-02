%lang starknet
%builtins pedersen range_check

from starkware.cairo.common.alloc import alloc
from starkware.cairo.common.math import assert_nn
from starkware.starknet.common.messages import send_message_to_l1
from starkware.cairo.common.cairo_builtins import HashBuiltin
from starkware.cairo.common.registers import get_fp_and_pc

struct Message:
    member x : felt
    member y : felt
end

# Terabethia Ethereum Address
@storage_var
func l1_contract() -> (res : felt):
end

@external
func set_l1_contract{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}(
        contract_addr : felt):
    # save new contract addr
    l1_contract.write(value=contract_addr)
    return ()
end

@external
func send_message{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}(
        msg_1 : felt, msg_2 : felt):
    alloc_locals

    let (contract_addr) = l1_contract.read()

    let (message_payload : felt*) = alloc()

    assert message_payload[0] = msg_1
    assert message_payload[1] = msg_2

    send_message_to_l1(to_address=contract_addr, payload_size=2, payload=message_payload)

    return ()
end

@external
func send_message_batch{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}(
        msg_hashes_len : felt, msg_hashes : felt*):
    alloc_locals

    # what is this do exactly
    assert_nn(msg_hashes_len)

    local msg_hashes_tuple : (Message, Message, Message, Message, Message, Message, Message, Message, Message, Message) = (
        Message(x=276768161078691357748506014484008718823, y=24127044263607486132772889641222586723),
        Message(x=276768161078691357748506014484008718823, y=24127044263607486132772889641222586723),
        Message(x=276768161078691357748506014484008718823, y=24127044263607486132772889641222586723),
        Message(x=276768161078691357748506014484008718823, y=24127044263607486132772889641222586723),
        Message(x=276768161078691357748506014484008718823, y=24127044263607486132772889641222586723),
        Message(x=276768161078691357748506014484008718823, y=24127044263607486132772889641222586723),
        Message(x=276768161078691357748506014484008718823, y=24127044263607486132772889641222586723),
        Message(x=276768161078691357748506014484008718823, y=24127044263607486132772889641222586723),
        Message(x=276768161078691357748506014484008718823, y=24127044263607486132772889641222586723),
        Message(x=276768161078691357748506014484008718823, y=24127044263607486132772889641222586723),
    )

    let (__fp__, _) = get_fp_and_pc()

    let msg_hash : Message* = cast(&msg_hashes_tuple, Message*)

    recurse_message_send(msg_hash_len=10, msg_hash=msg_hash)

    return ()
end

@external
func recurse_message_send{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}(
        msg_hash_len, msg_hash : Message*):
    alloc_locals

    if msg_hash_len == 0:
        return ()
    end

    let (contract_addr) = l1_contract.read()
    let (message_payload : felt*) = alloc()

    assert message_payload[0] = msg_hash.x
    assert message_payload[1] = msg_hash.y

    send_message_to_l1(to_address=contract_addr, payload_size=Message.SIZE, payload=message_payload)

    recurse_message_send(msg_hash_len=msg_hash_len - 1, msg_hash=msg_hash + Message.SIZE)

    return ()
end