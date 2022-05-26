export default ({ IDL }: { IDL: any }) => {
  const RemoveClaimableResponse = IDL.Variant({
    Ok: IDL.bool,
    Err: IDL.Text
  });
  return IDL.Service({
    remove_claimable: IDL.Func(
      [IDL.Principal, IDL.Nat],
      [RemoveClaimableResponse],
      [],
    )
  })
}