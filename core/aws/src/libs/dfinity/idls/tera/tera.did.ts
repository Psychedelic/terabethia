export default ({ IDL }: { IDL: any }) => {
  const ConsumeMessageResponse = IDL.Variant({
    Ok: IDL.Bool,
    Err: IDL.Text,
  });
  const OutgoingMessagePair = IDL.Record({
    msg_hash: IDL.Text,
    msg_key: IDL.Text,
  });
  const OutgoingMessage = IDL.Record({
    msg_hash: IDL.Text,
    msg_key: IDL.Vec(IDL.Nat8),
  });
  const SendMessageResponse = IDL.Variant({
    Ok: OutgoingMessage,
    Err: IDL.Text,
  });
  const CallResult = IDL.Record({ return: IDL.Vec(IDL.Nat8) });
  const StoreMessageResponse = IDL.Variant({
    Ok: CallResult,
    Err: IDL.Text,
  });
  return IDL.Service({
    authorize: IDL.Func([IDL.Principal], [], []),
    consume_message: IDL.Func(
      [IDL.Principal, IDL.Nat, IDL.Vec(IDL.Nat)],
      [ConsumeMessageResponse],
      [],
    ),
    get_messages: IDL.Func([], [IDL.Vec(OutgoingMessagePair)], []),
    get_nonces: IDL.Func([], [IDL.Vec(IDL.Nat)], ['query']),
    remove_messages: IDL.Func(
      [IDL.Vec(OutgoingMessagePair)],
      [ConsumeMessageResponse],
      [],
    ),
    send_message: IDL.Func(
      [IDL.Principal, IDL.Vec(IDL.Nat)],
      [SendMessageResponse],
      [],
    ),
    store_message: IDL.Func(
      [IDL.Principal, IDL.Principal, IDL.Nat, IDL.Vec(IDL.Nat)],
      [StoreMessageResponse],
      [],
    ),
    trigger_call: IDL.Func(
      [IDL.Principal, IDL.Principal, IDL.Nat, IDL.Vec(IDL.Nat)],
      [StoreMessageResponse],
      [],
    ),
  });
};
