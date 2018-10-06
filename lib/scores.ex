defmodule ExBio.Scores do
  alias ExBio.Nif.RustBio

  def blosum62(), do: RustBio.scores_blosum62()

  def pam120(), do: RustBio.scores_pam120()

  def pam200(), do: RustBio.scores_pam200()

  def pam250(), do: RustBio.scores_pam250()

  def pam40(), do: RustBio.scores_pam40()

  def apply(fun, a, b), do: RustBio.scores_apply(fun, a, b)
end
