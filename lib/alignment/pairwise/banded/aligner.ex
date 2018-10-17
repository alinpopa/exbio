defmodule ExBio.Alignment.Pairwise.Banded.Aligner do
  alias ExBio.Nif.RustBio

  def new(gap_open, gap_extend, fun_match, k, w) when gap_open <= 0 and gap_extend <= 0,
    do: RustBio.alignment_pairwise_banded_aligner_new(gap_open, gap_extend, fun_match, k, w)

  def new(_gap_open, _gap_extend, _fun_match, _k, _w), do: {:error, :invalid_args}

  def with_capacity(x_len, y_len, gap_open, gap_extend, fun_match, k, w)
      when x_len >= 0 and y_len >= 0 and gap_open <= 0 and gap_extend <= 0,
      do:
        RustBio.alignment_pairwise_banded_aligner_with_capacity(
          x_len,
          y_len,
          gap_open,
          gap_extend,
          fun_match,
          k,
          w
        )

  def with_capacity(_x_len, _y_len, _gap_open, _gap_extend, _fun_match, _k, _w),
    do: {:error, :invalid_args}

  def with_scoring(scoring, k, w),
    do: RustBio.alignment_pairwise_banded_aligner_with_scoring(scoring, k, w)

  def with_capacity_and_scoring(m, n, scoring, k, w),
    do: RustBio.alignment_pairwise_banded_aligner_with_capacity_and_scoring(m, n, scoring, k, w)

  def custom(aligner, x, y),
    do: RustBio.alignment_pairwise_banded_aligner_apply(aligner, :custom, x, y)

  def custom_with_prehash(aligner, x, y, {:y, k}),
    do:
      RustBio.alignment_pairwise_banded_aligner_apply_with_prehash(
        aligner,
        :custom,
        x,
        y,
        {:y, k}
      )

  def semiglobal(aligner, x, y),
    do: RustBio.alignment_pairwise_banded_aligner_apply(aligner, :semiglobal, x, y)

  def global(aligner, x, y),
    do: RustBio.alignment_pairwise_banded_aligner_apply(aligner, :global, x, y)

  def local(aligner, x, y),
    do: RustBio.alignment_pairwise_banded_aligner_apply(aligner, :local, x, y)
end
