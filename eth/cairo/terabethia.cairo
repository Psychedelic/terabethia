%lang starknet
%builtins pedersen range_check

from starkware.cairo.common.alloc import alloc
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
        tx_nonce : felt, msg_hash : felt):
    let (res) = nonce.read()

    let next_nonce = res + 1

    # Verify nonce
    assert tx_nonce = next_nonce

    let (contract_addr) = l1_contract.read()

    let (arr : felt*) = alloc()
    assert arr[0] = msg_hash

    send_message_to_l1(to_address=contract_addr, payload_size=1, payload=arr)

    # Save nonce
    nonce.write(next_nonce)
    return ()
end

@external
func send_message_batch{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}(
        tx_nonce : felt, msg_hashes_len : felt, msg_hashes : felt*):
    # @todo: loop through msg_hashes
    # https://github.com/starkware-libs/cairo-lang/blob/fc97bdd8322a7df043c87c371634b26c15ed6cee/src/starkware/cairo/common/hash_state.cairo#L50

    return ()
end

@view
func get_nonce{syscall_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}() -> (res : felt):
    let (res) = nonce.read()
    return (res)
end
