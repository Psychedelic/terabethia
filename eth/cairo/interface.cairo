%lang starknet

@contract_interface
namespace ITerabethiaContract:
    func set_l1_contract(contract_addr : felt):
    end

    func send_message(tx_nonce : felt, msg_hash : felt):
    end

    func send_message_batch(tx_nonce : felt, msg_hashes_len : felt, msg_hashes : felt*):
    end

    func get_nonce() -> (res : felt):
    end
end
