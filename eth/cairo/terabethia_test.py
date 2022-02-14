import os
import pytest

from starkware.starknet.testing.starknet import Starknet

# The path to the contract source code.
CONTRACT_FILE = os.path.join(
    os.path.dirname(__file__), "Terabethia.cairo")


# The testing library uses python's asyncio. So the following
# decorator and the ``async`` keyword are needed.
@pytest.mark.asyncio
async def test_send_message():
    # Create a new Starknet class that simulates the StarkNet
    # system.
    starknet = await Starknet.empty()

    # Deploy the contract.
    contract = await starknet.deploy(
        source=CONTRACT_FILE,
    )

    # Invoke send_message() twice.
    await contract.send_message(1,1).invoke()
    await contract.send_message(2,2).invoke()

    # Invoke send_message_batch()
    await contract.send_message_batch([
        276768161078691357748506014484008718823,24127044263607486132772889641222586723,
        276768161078691357748506014484008711,241270442636074861327728896412225855,
        276768161078691357748506014484008722,241270442636074861327728896412225866,
        276768161078691357748506014484008733,241270442636074861327728896412225877,
        276768161078691357748506014484008744,241270442636074861327728896412225888,
    ]).invoke()