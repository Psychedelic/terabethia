%lang starknet
%builtins pedersen range_check

from starkware.cairo.common.alloc import alloc
from starkware.cairo.common.math import assert_nn
from starkware.starknet.common.messages import send_message_to_l1
from starkware.cairo.common.cairo_builtins import HashBuiltin
from starkware.cairo.common.registers import get_fp_and_pc

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

    assert_nn(msg_hashes_len)

    recurse_message_send(msg_hash_len=msg_hashes_len / 2, msg_hash=msg_hashes)

    return ()
end

@external
func recurse_message_send{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}(
        msg_hash_len, msg_hash : felt*):
    alloc_locals

    if msg_hash_len == 0:
        return ()
    end

    let (contract_addr) = l1_contract.read()
    let (message_payload : felt*) = alloc()

    assert message_payload[0] = msg_hash[0]
    assert message_payload[1] = msg_hash[1]

    send_message_to_l1(to_address=contract_addr, payload_size=2, payload=message_payload)

    recurse_message_send(msg_hash_len=msg_hash_len - 1, msg_hash=&msg_hash[2])

    return ()
end