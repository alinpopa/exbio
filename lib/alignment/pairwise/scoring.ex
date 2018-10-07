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

  def new(gap_open, gap_extend, match_func),
    do: RustBio.alignment_pairwise_scoring_new(gap_open, gap_extend, match_func)

  def from_scores(gap_open, gap_extend, match_score, mismatch_score),
    do:
      RustBio.alignment_pairwise_scoring_from_scores(
        gap_open,
        gap_extend,
        match_score,
        mismatch_score
      )

  def from_scoring(scoring),
    do: RustBio.alignment_pairwise_scoring_from_scoring(scoring)

  def xclip(scoring, penalty),
    do: RustBio.alignment_pairwise_scoring_xclip(scoring, penalty)

  def yclip(scoring, penalty),
    do: RustBio.alignment_pairwise_scoring_yclip(scoring, penalty)
end
