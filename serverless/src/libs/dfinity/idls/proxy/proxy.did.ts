export default ({ IDL }: { IDL: any }) => {
  const MessageStatus = IDL.Variant({
    'MessageHandlerFailed' : IDL.Null,
    'BurnFailed' : IDL.Null,
    'Succeeded' : IDL.Null,
    'ConsumeMessageFailed' : IDL.Null,
    'SendMessageFailed' : IDL.Null,
    'MintFailed' : IDL.Null,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : MessageStatus });
  return IDL.Service({
    'burn' : IDL.Func([IDL.Principal, IDL.Nat], [Result], []),
    'handle_message' : IDL.Func(
        [IDL.Principal, IDL.Vec(IDL.Nat)],
        [Result],
        [],
      ),
    'mint' : IDL.Func([IDL.Vec(IDL.Nat)], [Result], []),
  });
};
