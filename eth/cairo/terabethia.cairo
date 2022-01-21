%lang starknet
%builtins pedersen range_check

from starkware.cairo.common.alloc import alloc
from starkware.cairo.common.math import assert_nn
from starkware.starknet.common.messages import send_message_to_l1
from starkware.cairo.common.cairo_builtins import HashBuiltin

@storage_var
func nonce() -> (res : felt):
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
        tx_nonce : felt, msg_1 : felt, msg_2 : felt):
    let (res) = nonce.read()

    let next_nonce = res + 1

    # Verify nonce
    assert tx_nonce = next_nonce

    let (contract_addr) = l1_contract.read()

    let (message_payload : felt*) = alloc()

    assert message_payload[0] = msg_1
    assert message_payload[1] = msg_2

    send_message_to_l1(to_address=contract_addr, payload_size=2, payload=message_payload)

    # Save nonce
    nonce.write(next_nonce)
    return ()
end

@external
func send_message_batch{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}(
        tx_nonce : felt, msg_hashes_len : felt, msg_hashes : felt*):
    alloc_locals

    assert_nn(msg_hashes_len)

    tempvar iterator = msg_hashes_len / 2
    tempvar nonce = tx_nonce

    send_loop:
    tempvar it = iterator * 2 - 1
    send_message(nonce, msg_hashes[it - 1], msg_hashes[it])
    tempvar iterator = iterator - 1
    tempvar nonce = nonce + 1
    jmp send_loop if iterator != 0

    return ()
end

@view
func get_nonce{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}() -> (res : felt):
    let (res) = nonce.read()
    return (res)
end
