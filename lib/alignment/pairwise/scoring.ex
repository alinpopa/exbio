defmodule ExBio.Alignment.Pairwise.Scoring do
  alias ExBio.Nif.RustBio

  defstruct gap_open: 0,
            gap_extend: 0,
            match_scores: {:none, {0, 0}},
            xclip_prefix: 0,
            xclip_suffix: 0,
            yclip_prefix: 0,
            yclip_suffix: 0,
            match_func: nil

  @min_score -858_993_459

  defmacro min_score do
    quote do
      unquote(@min_score)
    end
  end

  def new(gap_open, gap_extend, match_func) when gap_open <= 0 and gap_extend <= 0,
    do: RustBio.alignment_pairwise_scoring_new(gap_open, gap_extend, match_func)

  def new(_gap_open, _gap_extend_, _match_func), do: {:error, :invalid_args}

  def from_scores(gap_open, gap_extend, match_score, mismatch_score)
      when gap_open <= 0 and gap_extend <= 0,
      do:
        RustBio.alignment_pairwise_scoring_from_scores(
          gap_open,
          gap_extend,
          match_score,
          mismatch_score
        )

  def from_scores(_gap_open, _gap_extend, _match_score, _mismatch_score),
    do: {:error, :invalid_args}

  def from_scoring(scoring),
    do: RustBio.alignment_pairwise_scoring_from_scoring(scoring)

  def to_scoring(scoring),
    do: RustBio.alignment_pairwise_scoring_to_scoring(scoring)

  def xclip(scoring, penalty),
    do: RustBio.alignment_pairwise_scoring_xclip(scoring, penalty)

  def yclip(scoring, penalty),
    do: RustBio.alignment_pairwise_scoring_yclip(scoring, penalty)
end
