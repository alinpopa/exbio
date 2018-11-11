defmodule ExBio.Nif.RustBio do
  use Rustler, otp_app: :exbio, crate: :exbio

  defp err(), do: :erlang.nif_error(:nif_not_loaded)

  # PatternMatching
  def pm_bom(_text, _pattern), do: err()

  # Alignment
  def align_dist_hdist(_x, _y), do: err()
  def align_dist_ldist(_x, _y), do: err()

  # Alignment Pairwise MatchFunc
  def match_func(_op, _left, _right), do: err()
  def match_func_apply(_fun, _a, _b), do: err()

  # Alignment Pairwise Aligner
  def align_pair_aligner_new(_gap_open, _gap_extend, _match_fun), do: err()

  def align_pair_aligner_with_capacity(_m, _n, _gap_open, _gap_extend, _match_fun),
    do: err()

  def align_pair_aligner_with_scoring(_scoring), do: err()
  def align_pair_aligner_with_capacity_and_scoring(_m, _n, _scoring), do: err()
  def align_pair_aligner_apply(_aligner, _op, _x, _y), do: err()

  # Alignment Pairwise Banded Aligner
  def align_pair_banded_aligner_new(_gap_open, _gap_extend, _match_fun, _k, _w), do: err()

  def align_pair_banded_aligner_with_capacity(
        _m,
        _n,
        _gap_open,
        _gap_extend,
        _match_fun,
        _k,
        _w
      ),
      do: err()

  def align_pair_banded_aligner_with_scoring(_scoring, _k, _w), do: err()

  def align_pair_banded_aligner_with_capacity_and_scoring(_m, _n, _scoring, _k, _w),
    do: err()

  def align_pair_banded_aligner_apply(_aligner, _op, _x, _y), do: err()

  def align_pair_banded_aligner_apply_with_prehash(_aligner, _op, _x, _y, _kmer_hash),
    do: err()

  def align_pair_banded_aligner_custom_with_matches(_aligner, _x, _y, _matches),
    do: err()

  def align_pair_banded_aligner_custom_with_expanded_matches(
        _aligner,
        _x,
        _y,
        _matches,
        _allowed_mismatches,
        _use_lcskpp_union
      ),
      do: err()

  # Alignment Pairwise Scoring
  def align_pair_scoring_new(_gap_open, _gap_extend, _match_func), do: err()

  def align_pair_scoring_from_scores(
        _gap_open,
        _gap_extend,
        _match_score,
        _mismatch_score
      ),
      do: err()

  def align_pair_scoring_from_scoring(_scoring), do: err()
  def align_pair_scoring_to_scoring(_scoring), do: err()

  def align_pair_scoring_xclip(_scoring, _penalty), do: err()
  def align_pair_scoring_yclip(_scoring, _penalty), do: err()

  # Alignment Pairwise TracebackCell
  def align_pair_tracebackcell_new(), do: err()
  def align_pair_tracebackcell_set_bits(_tracebackcell, _op, _value), do: err()
  def align_pair_tracebackcell_get_bits(_tracebackcell, _op), do: err()

  # Predefined score functions
  def scores_fun(_fun), do: err()
  def scores_apply(_scores_fn, _a, _b), do: err()

  # Bio Types Alignment
  def types_align_new(_alignment), do: err()
  def types_align_pretty(_alignment, _x, _y), do: err()
  def types_align_cigar(_alignment, _hard_clip), do: err()
  def types_align_path(_alignment), do: err()
  def types_align_filter_clip_operations(_alignment), do: err()
  def types_align_y_aln_len(_alignment), do: err()
  def types_align_x_aln_len(_alignment), do: err()

  # Alphabets Alphabet
  def alph_alphabet_new(_text), do: err()
  def alph_alphabet_insert(_alphabet, _a), do: err()
  def alph_alphabet_is_word(_alphabet, _text), do: err()
  def alph_alphabet_max_symbol(_alphabet), do: err()
  def alph_alphabet_len(_alphabet), do: err()
  def alph_alphabet_is_empty(_alphabet), do: err()

  # Alphabets DNA
  def alph_dna_alphabet(_type), do: err()
  def alph_dna_complement(_char), do: err()
end
