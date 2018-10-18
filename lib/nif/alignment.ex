defmodule ExBio.Nif.Alignment do
  use Rustler, otp_app: :exbio, crate: :alignment

  defp err(), do: :erlang.nif_error(:nif_not_loaded)

  # Distance
  def dist_hdist(_x, _y), do: err()
  def dist_ldist(_x, _y), do: err()

  # Pairwise MatchFunc
  def match_func(_op, _left, _right), do: err()
  def match_func_apply(_fun, _a, _b), do: err()

  # Pairwise Aligner
  def pairwise_aligner_new(_gap_open, _gap_extend, _match_fun), do: err()

  def pairwise_aligner_with_capacity(_m, _n, _gap_open, _gap_extend, _match_fun),
    do: err()

  def pairwise_aligner_with_scoring(_scoring), do: err()
  def pairwise_aligner_with_capacity_and_scoring(_m, _n, _scoring), do: err()
  def pairwise_aligner_apply(_aligner, _op, _x, _y), do: err()

  # Pairwise Banded Aligner
  def pairwise_banded_aligner_new(_gap_open, _gap_extend, _match_fun, _k, _w), do: err()

  def pairwise_banded_aligner_with_capacity(
        _m,
        _n,
        _gap_open,
        _gap_extend,
        _match_fun,
        _k,
        _w
      ),
      do: err()

  def pairwise_banded_aligner_with_scoring(_scoring, _k, _w), do: err()

  def pairwise_banded_aligner_with_capacity_and_scoring(_m, _n, _scoring, _k, _w),
    do: err()

  def pairwise_banded_aligner_apply(_aligner, _op, _x, _y), do: err()

  def pairwise_banded_aligner_apply_with_prehash(_aligner, _op, _x, _y, _kmer_hash),
    do: err()

  # Pairwise Scoring
  def pairwise_scoring_new(_gap_open, _gap_extend, _match_func), do: err()

  def pairwise_scoring_from_scores(
        _gap_open,
        _gap_extend,
        _match_score,
        _mismatch_score
      ),
      do: err()

  def pairwise_scoring_from_scoring(_scoring), do: err()
  def pairwise_scoring_to_scoring(_scoring), do: err()

  def pairwise_scoring_xclip(_scoring, _penalty), do: err()
  def pairwise_scoring_yclip(_scoring, _penalty), do: err()

  # Pairwise TracebackCell
  def pairwise_tracebackcell_new(), do: err()
  def pairwise_tracebackcell_set_bits(_tracebackcell, _op, _value), do: err()
  def pairwise_tracebackcell_get_bits(_tracebackcell, _op), do: err()
end
