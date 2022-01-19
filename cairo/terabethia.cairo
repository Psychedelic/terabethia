%lang starknet
%builtins pedersen range_check

from starkware.cairo.common.alloc import alloc
from starkware.starknet.common.messages import send_message_to_l1
from starkware.cairo.common.cairo_builtins import HashBuiltin
from starkware.starknet.common.syscalls import get_caller_address

@storage_var
func nonce() -> (res : felt):
end

@storage_var
func operator() -> (res : felt):
end

# Terabethia Ethereum Address
@storage_var
func l1_contract() -> (res : felt):
end

# Initialise operator address
@constructor
func constructor{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}():
    let (caller_address) = get_caller_address()
    operator.write(value=caller_address)
    return ()
end

@external
func set_operator{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}(
        operator_address : felt):
    let (res) = nonce.read()

    let (caller_address) = get_caller_address()
    let (current_operator) = operator.read()

    assert caller_address = current_operator

    # Save new operator
    operator.write(value=operator_address)
    return ()
end

@external
func set_l1_contract{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}(
        contract_addr : felt):
    let (res) = nonce.read()

    let (caller_address) = get_caller_address()
    let (current_operator) = operator.read()

    assert caller_address = current_operator

    # Save new contract addr
    l1_contract.write(value=contract_addr)
    return ()
end

@external
func send_message{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}(
        msg_nonce : felt, msg : felt):
    let (res) = nonce.read()

    let next_nonce = res + 1

    # Verify nonce
    assert msg_nonce = next_nonce

    let (caller_address) = get_caller_address()
    let (current_operator) = operator.read()

    assert caller_address = current_operator

    let (message_payload : felt*) = alloc()
    assert message_payload[0] = msg

    let (contract_addr) = l1_contract.read()
    send_message_to_l1(to_address=contract_addr, payload_size=1, payload=message_payload)

    # Save nonce
    nonce.write(next_nonce)
    return ()
end
