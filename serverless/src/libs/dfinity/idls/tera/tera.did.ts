export default ({ IDL }: { IDL: any }) => {
  const Result = IDL.Variant({ 'Ok': IDL.Bool, 'Err': IDL.Text });
  const OutgoingMessage = IDL.Record({
    msg_hash: IDL.Text,
    msg_key: IDL.Vec(IDL.Nat8),
  });
  const Result1 = IDL.Variant({ Ok: OutgoingMessage, Err: IDL.Text });
  const CallResult = IDL.Record({ return: IDL.Vec(IDL.Nat8) });
  const Result2 = IDL.Variant({ Ok: CallResult, Err: IDL.Text });

  return IDL.Service({
    authorize: IDL.Func([IDL.Principal], [], []),
    consume_message: IDL.Func(
      [IDL.Principal, IDL.Nat, IDL.Vec(IDL.Nat)],
      [Result],
      [],
    ),
    get_messages: IDL.Func([], [IDL.Vec(OutgoingMessage)], []),
    get_nonces: IDL.Func([], [IDL.Vec(IDL.Nat)], []),
    remove_messages: IDL.Func(
      [IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text))],
      [Result],
      [],
    ),
    send_message: IDL.Func(
      [IDL.Principal, IDL.Vec(IDL.Nat)],
      [Result1],
      [],
    ),
    store_message: IDL.Func(
      [IDL.Principal, IDL.Principal, IDL.Nat, IDL.Vec(IDL.Nat)],
      [Result2],
      [],
    ),
    trigger_call: IDL.Func(
      [IDL.Principal, IDL.Principal, IDL.Nat, IDL.Vec(IDL.Nat)],
      [Result2],
      [],
    ),
  });
};
export const init = ({ IDL }) => { return []; };
