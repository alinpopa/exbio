defmodule ExBio.Scores do
  alias ExBio.Nif.RustBio

  def blosum62(), do: RustBio.scores_fun(:blosum62)

  def pam120(), do: RustBio.scores_fun(:pam120)

  def pam200(), do: RustBio.scores_fun(:pam200)

  def pam250(), do: RustBio.scores_fun(:pam250)

  def pam40(), do: RustBio.scores_fun(:pam40)

  def apply(fun, a, b), do: RustBio.scores_apply(fun, a, b)
end
