export default ({ IDL }: { IDL: any }) => {
  const ContractAddress = IDL.Text;
  const Payload = IDL.Vec(IDL.Nat8);
  return IDL.Service({
    receiveMessageFromL1: IDL.Func(
      [ContractAddress, IDL.Principal, Payload],
      [],
      []
    ),
  });
};
