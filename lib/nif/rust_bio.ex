defmodule ExBio.Nif.RustBio do
  use Rustler, otp_app: :exbio, crate: :exbio

  # PatternMatching
  def pm_bom(_text, _pattern), do: :erlang.nif_error(:nif_not_loaded)

  # Alignment
  def alignment_dist_hdist(_x, _y), do: :erlang.nif_error(:nif_not_loaded)
  def alignment_dist_ldist(_x, _y), do: :erlang.nif_error(:nif_not_loaded)

  # Predefined score functions
  def scores_blosum62(), do: :erlang.nif_error(:nif_not_loaded)
  def scores_pam120(), do: :erlang.nif_error(:nif_not_loaded)
  def scores_pam200(), do: :erlang.nif_error(:nif_not_loaded)
  def scores_pam250(), do: :erlang.nif_error(:nif_not_loaded)
  def scores_pam40(), do: :erlang.nif_error(:nif_not_loaded)
  def scores_apply(_scores_fn, _a, _b), do: :erlang.nif_error(:nif_not_loaded)
end
