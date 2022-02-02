// 0xd0379be15bb6f33737b756e512dad1e71226b31fa648da57811f930badf6c163
import { splitUint256 } from './utils';

describe('utils.ts', () => {
  it('splitUint256 into two Uint128', () => {
    expect.assertions(3);
    const hash = 'd0379be15bb6f33737b756e512dad1e71226b31fa648da57811f930badf6c163';

    const res = splitUint256(hash);

    expect(res).toHaveLength(2);
    expect(res[0]).toBe(
      BigInt('276768161078691357748506014484008718823'),
    );
    expect(res[1]).toBe(
      BigInt('24127044263607486132772889641222586723'),
    );
  });
});
