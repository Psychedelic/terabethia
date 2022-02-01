%lang starknet

%builtins pedersen range_check

from starkware.cairo.common.registers import get_fp_and_pc
from starkware.starknet.common.syscalls import get_contract_address
from starkware.cairo.common.cairo_builtins import HashBuiltin, SignatureBuiltin
from starkware.starknet.common.syscalls import call_contract, get_caller_address, get_tx_signature
from starkware.cairo.common.hash_state import (
    hash_init, hash_finalize, hash_update, hash_update_single)

from cairo.secp.secp_ec import EcPoint
from cairo.secp.secp import verify_ecdsa
from cairo.secp.bigint import BigInt3, from_felt
#
# Structs
#

struct Message:
    member sender : felt
    member to : felt
    member selector : felt
    member calldata : felt*
    member calldata_size : felt
    member nonce : felt
end

#
# Storage
#

@storage_var
func current_nonce() -> (res : felt):
end

@storage_var
func public_key() -> (res : EcPoint):
end

#
# Guards
#

@view
func assert_only_self{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}():
    let (self) = get_contract_address()
    let (caller) = get_caller_address()
    assert self = caller
    return ()
end

#
# Getters
#

@view
func get_public_key{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}() -> (
        res : EcPoint):
    let (res) = public_key.read()
    return (res=res)
end

@view
func get_nonce{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}() -> (res : felt):
    let (res) = current_nonce.read()
    return (res=res)
end

@view
func is_account{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}() -> (
        res : felt):
    return (1)
end

#
# Setters
#

@external
func set_public_key{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}(
        new_public_key : EcPoint):
    assert_only_self()
    public_key.write(new_public_key)
    return ()
end

#
# Constructor
#

@constructor
func constructor{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}(
        _public_key : EcPoint):
    public_key.write(_public_key)
    return ()
end

#
# Business logic
#

@view
func is_valid_signature{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}(
        hash : BigInt3, sig_r : BigInt3, sig_s : BigInt3) -> ():
    alloc_locals

    let (_public_key) = public_key.read()

    # This interface expects a signature pointer and length to make
    # no assumption about signature validation schemes.
    # But this implementation does, and it expects a (sig_r, sig_s) pair.
    verify_ecdsa(public_key_pt=_public_key, msg_hash=hash, r=sig_r, s=sig_s)

    return ()
end

@external
func execute{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}(
        to : felt, selector : felt, calldata_len : felt, calldata : felt*, nonce : felt,
        sig_r : BigInt3, sig_s : BigInt3) -> (response_len : felt, response : felt*):
    alloc_locals

    let (__fp__, _) = get_fp_and_pc()
    let (_address) = get_contract_address()
    let (_current_nonce) = current_nonce.read()

    # validate nonce
    assert _current_nonce = nonce

    local message : Message = Message(
        _address,
        to,
        selector,
        calldata,
        calldata_size=calldata_len,
        _current_nonce
        )

    # validate transaction
    let (hash) = hash_message(&message)
    let (msg_hash : BigInt3) = from_felt(hash)

    is_valid_signature(msg_hash, sig_r, sig_s)

    # bump nonce
    current_nonce.write(_current_nonce + 1)

    # execute call
    let response = call_contract(
        contract_address=message.to,
        function_selector=message.selector,
        calldata_size=message.calldata_size,
        calldata=message.calldata)

    return (response_len=response.retdata_size, response=response.retdata)
end

func hash_message{pedersen_ptr : HashBuiltin*}(message : Message*) -> (res : felt):
    alloc_locals
    # we need to make `res_calldata` local
    # to prevent the reference from being revoked
    let (local res_calldata) = hash_calldata(message.calldata, message.calldata_size)
    let hash_ptr = pedersen_ptr
    with hash_ptr:
        let (hash_state_ptr) = hash_init()
        # first three iterations are 'sender', 'to', and 'selector'
        let (hash_state_ptr) = hash_update(hash_state_ptr, message, 3)
        let (hash_state_ptr) = hash_update_single(hash_state_ptr, res_calldata)
        let (hash_state_ptr) = hash_update_single(hash_state_ptr, message.nonce)
        let (res) = hash_finalize(hash_state_ptr)
        let pedersen_ptr = hash_ptr
        return (res=res)
    end
end

func hash_calldata{pedersen_ptr : HashBuiltin*}(calldata : felt*, calldata_size : felt) -> (
        res : felt):
    let hash_ptr = pedersen_ptr
    with hash_ptr:
        let (hash_state_ptr) = hash_init()
        let (hash_state_ptr) = hash_update(hash_state_ptr, calldata, calldata_size)
        let (res) = hash_finalize(hash_state_ptr)
        let pedersen_ptr = hash_ptr
        return (res=res)
    end
end
