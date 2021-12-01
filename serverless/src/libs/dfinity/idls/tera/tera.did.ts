export default ({ IDL }: { IDL: any }) => {
  const Result = IDL.Variant({ Ok: IDL.Bool, Err: IDL.Text });
  const CallResult = IDL.Record({ return: IDL.Vec(IDL.Nat8) });
  const Result_1 = IDL.Variant({ Ok: CallResult, Err: IDL.Text });
  return IDL.Service({
    consume_message: IDL.Func([IDL.Nat, IDL.Vec(IDL.Nat)], [Result], []),
    send_message: IDL.Func([IDL.Nat, IDL.Vec(IDL.Nat)], [Result], []),
    store_message: IDL.Func(
      [IDL.Nat, IDL.Principal, IDL.Vec(IDL.Nat)],
      [Result_1],
      []
    ),
    trigger_call: IDL.Func(
      [IDL.Nat, IDL.Principal, IDL.Vec(IDL.Nat)],
      [Result_1],
      []
    ),
  });
};
