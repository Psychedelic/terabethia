export default ({ IDL }: { IDL: any }) => {
  const Result = IDL.Variant({ Ok: IDL.Bool, Err: IDL.Text });
  const OutgoingMessage = IDL.Record({
    id: IDL.Nat,
    hash: IDL.Text,
    produced: IDL.Bool,
  });
  const CallResult = IDL.Record({ return: IDL.Vec(IDL.Nat8) });
  const Result_1 = IDL.Variant({ Ok: CallResult, Err: IDL.Text });
  return IDL.Service({
    authorize: IDL.Func([IDL.Principal], [], []),
    consume_message: IDL.Func([IDL.Principal, IDL.Vec(IDL.Nat)], [Result], []),
    get_messages: IDL.Func([], [IDL.Vec(OutgoingMessage)], []),
    remove_messages: IDL.Func([IDL.Vec(IDL.Nat)], [Result], []),
    send_message: IDL.Func([IDL.Principal, IDL.Vec(IDL.Nat)], [Result], []),
    store_message: IDL.Func(
      [IDL.Principal, IDL.Principal, IDL.Vec(IDL.Nat)],
      [Result_1],
      []
    ),
    trigger_call: IDL.Func(
      [IDL.Principal, IDL.Principal, IDL.Vec(IDL.Nat)],
      [Result_1],
      []
    ),
  });
};
