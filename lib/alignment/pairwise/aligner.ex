defmodule ExBio.Alignment.Pairwise.Aligner do
  alias ExBio.Nif.RustBio

  def new(gap_open, gap_extend, fun_match) when gap_open <= 0 and gap_extend <= 0,
    do: RustBio.alignment_pairwise_aligner_new(gap_open, gap_extend, fun_match)

  def new(_gap_open, _gap_extend, _fun_match), do: {:error, :invalid_args}

  def with_capacity(x_len, y_len, gap_open, gap_extend, fun_match)
      when x_len >= 0 and y_len >= 0 and gap_open <= 0 and gap_extend <= 0,
      do:
        RustBio.alignment_pairwise_aligner_with_capacity(
          x_len,
          y_len,
          gap_open,
          gap_extend,
          fun_match
        )

  def with_capacity(_x_len, _y_len, _gap_open, _gap_extend, _fun_match),
    do: {:error, :invalid_args}

  def with_scoring(scoring),
    do: RustBio.alignment_pairwise_aligner_with_scoring(scoring)

  def custom(aligner, x, y),
    do: RustBio.alignment_pairwise_aligner_custom(aligner, x, y)

  def semiglobal(aligner, x, y),
    do: RustBio.alignment_pairwise_aligner_semiglobal(aligner, x, y)
end
