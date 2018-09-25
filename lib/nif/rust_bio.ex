defmodule ExBio.Nif.RustBio do
  use Rustler, otp_app: :exbio, crate: :exbio

  # PatternMatching
  def pm_bom(_text, _pattern), do: :erlang.nif_error(:nif_not_loaded)

  # Alignment
  def alignment_dist_hdist(_x, _y), do: :erlang.nif_error(:nif_not_loaded)
  def alignment_dist_ldist(_x, _y), do: :erlang.nif_error(:nif_not_loaded)
end
