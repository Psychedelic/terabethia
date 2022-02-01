import os
import pytest

from starkware.starknet.testing.starknet import Starknet

# The path to the contract source code.
CONTRACT_FILE = os.path.join(
    os.path.dirname(__file__), "Account.cairo")


# The testing library uses python's asyncio. So the following
# decorator and the ``async`` keyword are needed.
@pytest.mark.asyncio
async def test_secp256k1_signature():
    # Create a new Starknet class that simulates the StarkNet
    # system.
    starknet = await Starknet.empty()
    calldata = [0x35dec240d9f76e20b48b41, 0x27fcb378b533f57a6b585, 0xbff381888b165f92dd33d, 0x1711d8fb6fbbf53986b57f, 0x2e56f964d38cb8dbdeb30b, 0xe4be2a8547d802dc42041 ]
    
    # Deploy the contract.
    contract = await starknet.deploy(
        source=CONTRACT_FILE,
        constructor_calldata=calldata
    )

    execution_info = await contract.is_valid_signature(
        # message hash
        (0x38a23ca66202c8c2a72277, 0x6730e765376ff17ea8385, 0xca1ad489ab60ea581e6c1),
        # sig r
        (0x2e6c77fee73f3ac9be1217, 0x3f0c0b121ac1dc3e5c03c6, 0xeee3e6f50c576c07d7e4a),
        # sig s
        (0x20a4b46d3c5e24cda81f22, 0x967bf895824330d4273d0, 0x541e10c21560da25ada4c)
    ).call()
    assert execution_info.result == ()