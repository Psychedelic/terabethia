%lang starknet
%builtins pedersen range_check

from starkware.cairo.common.alloc import alloc
from starkware.starknet.common.messages import send_message_to_l1
from starkware.cairo.common.cairo_builtins import HashBuiltin
from starkware.starknet.common.syscalls import get_caller_address

@contract_interface
namespace ITerabethiaContract:
    func set_l1_contract(contract_addr : felt):
    end

    func send_message(tx_nonce : felt, msg_hash : felt):
    end

    # func send_message_batch(tx_nonce : felt, msg_hashes_len : felt, msg_hashes : felt*):
    # end

    func get_nonce() -> (res : felt):
    end
end

@storage_var
func operator() -> (res : felt):
end

@storage_var
func impl_contract() -> (res : felt):
end

# Initialise operator address
@constructor
func constructor{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}(
        contract_addr : felt):
    let (caller_address) = get_caller_address()
    # set operator
    operator.write(value=caller_address)

    # set implementation
    set_impl_contract(contract_addr)
    return ()
end

@external
func set_operator{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}(
        operator_address : felt):
    require_operator()

    # Save new operator
    operator.write(value=operator_address)
    return ()
end

@external
func set_l1_contract{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}(
        contract_addr : felt):
    require_operator()

    # save new contract addr
    let (impl_contract_address) = impl_contract.read()

    ITerabethiaContract.delegate_set_l1_contract(
        contract_address=impl_contract_address, contract_addr=contract_addr)

    return ()
end

@external
func set_impl_contract{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}(
        impl_contract_address : felt):
    require_operator()

    # save new implementation contract
    impl_contract.write(value=impl_contract_address)
    return ()
end

@external
func send_message{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}(
        tx_nonce : felt, msg_hash : felt):
    require_operator()
    let (impl_contract_address) = impl_contract.read()

    ITerabethiaContract.delegate_send_message(
        contract_address=impl_contract_address, tx_nonce=tx_nonce, msg_hash=msg_hash)

    return ()
end

# this is causing the issues, skipping
# @external
# func send_message_batch{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}(
#         tx_nonce : felt, msg_hashes_len : felt, msg_hashes : felt*):
#     require_operator()

# let (impl_contract_address) = impl_contract.read()

# ITerabethiaContract.delegate_send_message_batch(
#         contract_address=impl_contract_address,
#         tx_nonce=tx_nonce,
#         msg_hashes_len=msg_hashes_len,
#         msg_hashes=msg_hashes)

# return ()
# end

@view
func get_nonce{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}() -> (res : felt):
    let (impl_contract_address) = impl_contract.read()
    let (res) = ITerabethiaContract.delegate_get_nonce(contract_address=impl_contract_address)

    return (res)
end

@view
func require_operator{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}():
    let (caller_address : felt) = get_caller_address()
    let (approved_caller) = operator.read()
    assert caller_address = approved_caller
    return ()
end
