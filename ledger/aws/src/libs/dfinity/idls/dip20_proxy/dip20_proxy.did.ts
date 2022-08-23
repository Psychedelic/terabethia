export default ({ IDL }: { IDL: any }) => {
  const RemoveClaimableResponse = IDL.Variant({
    Ok: IDL.Bool,
    Err: IDL.Text
  });
  return IDL.Service({
    remove_claimable: IDL.Func(
      [IDL.Principal, IDL.Principal, IDL.Nat],
      [RemoveClaimableResponse],
      [],
    )
  })
}