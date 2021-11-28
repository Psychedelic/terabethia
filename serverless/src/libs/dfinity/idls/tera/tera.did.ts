export default ({ IDL }: { IDL: any }) => {
  const ContractAddress = IDL.Text;
  const Payload = IDL.Vec(IDL.Nat8);
  return IDL.Service({
    consume_message: IDL.Func([ContractAddress, Payload], [IDL.Bool], []),
    send_message: IDL.Func([ContractAddress, Payload], [IDL.Bool], []),
    store_message: IDL.Func([ContractAddress, IDL.Principal, Payload], [], []),
    trigger_call: IDL.Func([ContractAddress, IDL.Principal, Payload], [], []),
  });
};
