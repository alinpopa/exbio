defmodule ExBio.Nif.RustBio do
  use Rustler, otp_app: :exbio, crate: :exbio

  defp err(), do: :erlang.nif_error(:nif_not_loaded)

  # PatternMatching
  def pm_bom(_text, _pattern), do: err()

  # Alignment
  def alignment_dist_hdist(_x, _y), do: err()
  def alignment_dist_ldist(_x, _y), do: err()

  # Alignment Pairwise MatchFunc
  def match_func(_op, _left, _right), do: err()
  def match_func_apply(_fun, _a, _b), do: err()

  # Alignment Pairwise Aligner
  def alignment_pairwise_aligner_new(_gap_open, _gap_extend, _match_fun), do: err()

  def alignment_pairwise_aligner_with_capacity(_m, _n, _gap_open, _gap_extend, _match_fun),
    do: err()

  def alignment_pairwise_aligner_with_scoring(_scoring), do: err()
  def alignment_pairwise_aligner_with_capacity_and_scoring(_m, _n, _scoring), do: err()
  def alignment_pairwise_aligner_apply(_aligner, _op, _x, _y), do: err()

  # Alignment Pairwise Scoring
  def alignment_pairwise_scoring_new(_gap_open, _gap_extend, _match_func), do: err()

  def alignment_pairwise_scoring_from_scores(
        _gap_open,
        _gap_extend,
        _match_score,
        _mismatch_score
      ),
      do: err()

  def alignment_pairwise_scoring_from_scoring(_scoring), do: err()
  def alignment_pairwise_scoring_to_scoring(_scoring), do: err()

  def alignment_pairwise_scoring_xclip(_scoring, _penalty), do: err()
  def alignment_pairwise_scoring_yclip(_scoring, _penalty), do: err()

  # Alignment Pairwise TracebackCell
  def alignment_pairwise_tracebackcell_new(), do: err()
  def alignment_pairwise_tracebackcell_set_bits(_tracebackcell, _op, _value), do: err()
  def alignment_pairwise_tracebackcell_get_bits(_tracebackcell, _op), do: err()

  # Predefined score functions
  def scores_fun(_fun), do: err()
  def scores_apply(_scores_fn, _a, _b), do: err()

  # Bio Types Alignment
  def types_alignment_new(_alignment), do: err()
  def types_alignment_pretty(_alignment, _x, _y), do: err()
  def types_alignment_cigar(_alignment, _hard_clip), do: err()
  def types_alignment_path(_alignment), do: err()
  def types_alignment_filter_clip_operations(_alignment), do: err()
  def types_alignment_y_aln_len(_alignment), do: err()
  def types_alignment_x_aln_len(_alignment), do: err()
end
