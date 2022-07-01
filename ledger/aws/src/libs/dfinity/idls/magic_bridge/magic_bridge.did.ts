export default ({ IDL }: { IDL: any }) => {
  const GetCanisterResponse = IDL.Opt(
    IDL.Principal
  );
  return IDL.Service({
    get_canister: IDL.Func(
      [IDL.Principal],
      [GetCanisterResponse],
      ['query'],
    )
  })
}