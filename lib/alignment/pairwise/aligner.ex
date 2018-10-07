defmodule ExBio.Alignment.Pairwise.Aligner do
  alias ExBio.Nif.RustBio

  def new(gap_open, gap_extend, fun_match) when gap_open <= 0 and gap_extend <= 0,
    do: RustBio.alignment_pairwise_aligner_new(gap_open, gap_extend, fun_match)

  def new(_gap_open, _gap_extend, _fun_match), do: {:error, :invalid_args}
end
