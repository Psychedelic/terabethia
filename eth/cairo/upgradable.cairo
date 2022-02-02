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

    func send_message(msg_1 : felt, msg_2 : felt):
    end

    func send_message_batch(msg_hashes_len : felt, msg_hashes : felt*):
    end
end

# Operator can send messages to L1
@storage_var
func operator() -> (res : felt):
end

# Admin can change implementation, operator and L1 contract
@storage_var
func admin() -> (res : felt):
end

# Current implementation
@storage_var
func implementation() -> (res : felt):
end

# Terabethia Ethereum Address
@storage_var
func l1_contract() -> (res : felt):
end

# Initialise settings
@constructor
func constructor{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}(
        approved_admin : felt, approved_operator : felt, implementation_addr : felt,
        l1_contract_address : felt):
    # set admin account
    admin.write(value=approved_admin)

    # set operator account
    operator.write(value=approved_operator)

    # set implementation
    implementation.write(value=implementation_addr)

    # set Terabethia L1 contract
    l1_contract.write(value=l1_contract_address)

    return ()
end

@external
func set_operator{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}(
        operator_address : felt):
    require_admin()

    # Save new operator
    operator.write(value=operator_address)
    return ()
end

@external
func set_implementation{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}(
        implementation_address : felt):
    require_admin()

    # save new implementation contract
    implementation.write(value=implementation_address)
    return ()
end

@external
func set_admin{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}(
        approved_admin : felt):
    require_admin()

    # save new implementation contract
    admin.write(value=approved_admin)
    return ()
end

#
# Guards operator / admin
#
@view
func require_operator{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}():
    let (caller_address : felt) = get_caller_address()
    let (approved_caller) = operator.read()
    assert caller_address = approved_caller
    return ()
end

@view
func require_admin{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}():
    let (caller_address : felt) = get_caller_address()
    let (approved_caller) = admin.read()
    assert caller_address = approved_caller
    return ()
end

#
# Getters for transparency
# anyone can verify what's currently set
#
@view
func get_operator{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}() -> (
        res : felt):
    let (res) = operator.read()
    return (res=res)
end

@view
func get_admin{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}() -> (res : felt):
    let (res) = admin.read()
    return (res=res)
end

@view
func get_implementation{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}() -> (
        res : felt):
    let (res) = implementation.read()
    return (res=res)
end

@view
func get_l1_contract{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}() -> (
        res : felt):
    let (res) = l1_contract.read()
    return (res=res)
end

#
#  Delegated calls
#
@external
func set_l1_contract{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}(
        contract_addr : felt):
    require_admin()

    # save new contract addr
    let (implementation_address) = implementation.read()

    ITerabethiaContract.delegate_set_l1_contract(
        contract_address=implementation_address, contract_addr=contract_addr)

    return ()
end

@external
func send_message{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}(
        msg_1 : felt, msg_2 : felt):
    require_operator()
    let (implementation_address) = implementation.read()

    ITerabethiaContract.delegate_send_message(
        contract_address=implementation_address, msg_1=msg_1, msg_2=msg_2)

    return ()
end

@external
func send_message_batch{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}(
        msg_hashes_len : felt, msg_hashes : felt*):
    alloc_locals
    require_operator()

    let (implementation_address) = implementation.read()

    ITerabethiaContract.delegate_send_message_batch(
        contract_address=implementation_address,
        msg_hashes_len=msg_hashes_len,
        msg_hashes=msg_hashes)

    return ()
end
