defmodule ExBio.Alignment.Pairwise.MatchFunc do
  alias ExBio.Nif.RustBio

  def eq(), do: RustBio.match_func_eq()

  def ne(), do: RustBio.match_func_ne()

  def lt(), do: RustBio.match_func_lt()

  def lte(), do: RustBio.match_func_lte()

  def gt(), do: RustBio.match_func_gt()

  def gte(), do: RustBio.match_func_gte()

  def apply(fun, a, b, left_right), do: RustBio.match_func_apply(fun, a, b, left_right)
end
