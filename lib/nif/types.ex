defmodule ExBio.Nif.Types do
  use Rustler, otp_app: :exbio, crate: :types

  defp err(), do: :erlang.nif_error(:nif_not_loaded)

  def alignment_new(_alignment), do: err()
  def alignment_pretty(_alignment, _x, _y), do: err()
  def alignment_cigar(_alignment, _hard_clip), do: err()
  def alignment_path(_alignment), do: err()
  def alignment_filter_clip_operations(_alignment), do: err()
  def alignment_y_aln_len(_alignment), do: err()
  def alignment_x_aln_len(_alignment), do: err()
end
