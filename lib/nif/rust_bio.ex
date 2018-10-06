defmodule ExBio.Nif.RustBio do
  use Rustler, otp_app: :exbio, crate: :exbio

  defp err(), do: :erlang.nif_error(:nif_not_loaded)

  # PatternMatching
  def pm_bom(_text, _pattern), do: err()

  # Alignment
  def alignment_dist_hdist(_x, _y), do: err()
  def alignment_dist_ldist(_x, _y), do: err()

  # Alignment Pairwise MatchFunc
  def match_func_eq(), do: err()
  def match_func_ne(), do: err()
  def match_func_lt(), do: err()
  def match_func_lte(), do: err()
  def match_func_gt(), do: err()
  def match_func_gte(), do: err()
  def match_func_apply(_fun, _a, _b, _left_right), do: err()

  # Predefined score functions
  def scores_blosum62(), do: err()
  def scores_pam120(), do: err()
  def scores_pam200(), do: err()
  def scores_pam250(), do: err()
  def scores_pam40(), do: err()
  def scores_apply(_scores_fn, _a, _b), do: err()
end
