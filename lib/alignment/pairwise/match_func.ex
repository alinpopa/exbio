defmodule ExBio.Alignment.Pairwise.MatchFunc do
  alias ExBio.Nif.RustBio

  def eq(left, right), do: RustBio.match_func(:eq, left, right)

  def ne(left, right), do: RustBio.match_func(:ne, left, right)

  def lt(left, right), do: RustBio.match_func(:lt, left, right)

  def lte(left, right), do: RustBio.match_func(:lte, left, right)

  def gt(left, right), do: RustBio.match_func(:gt, left, right)

  def gte(left, right), do: RustBio.match_func(:gte, left, right)

  def apply(fun, a, b), do: RustBio.match_func_apply(fun, a, b)
end
