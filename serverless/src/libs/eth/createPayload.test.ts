import { createPayload } from "./createPayload";

test("createPayload", () => {
  const messagesToL1 = [
    "0x33ccbcf525a78f667fbac0ffe776d6e1dd28e23bef3c33563b43a45d6b50c1d4",
  ];
  const messagesToL2: string[] = [];

  const payload = createPayload(messagesToL1, messagesToL2);

  expect(payload.length).toBe(3);
  expect(payload[0]).toBe(
    "0x0000000000000000000000000000000000000000000000000000000000000001"
  );
  expect(payload[1]).toBe(
    "0x33ccbcf525a78f667fbac0ffe776d6e1dd28e23bef3c33563b43a45d6b50c1d4"
  );
  expect(payload[2]).toBe(
    "0x0000000000000000000000000000000000000000000000000000000000000000"
  );
});
