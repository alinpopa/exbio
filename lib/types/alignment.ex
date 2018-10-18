defmodule ExBio.Types.Alignment do
  alias ExBio.Nif.Types

  defstruct score: 0,
            ystart: 0,
            xstart: 0,
            yend: 0,
            xend: 0,
            ylen: 0,
            xlen: 0,
            operations: [],
            mode: :custom

  def new(alignment) do
    case Types.alignment_new(alignment) do
      {:ok, ref} when is_reference(ref) ->
        {:ok, ref}

      _err ->
        {:error, :invalid_args}
    end
  end

  def pretty(alignment, x, y),
    do: Types.alignment_pretty(alignment, x, y)

  def cigar(alignment, hard_clip),
    do: Types.alignment_cigar(alignment, hard_clip)

  def path(alignment),
    do: Types.alignment_path(alignment)

  def filter_clip_operations(alignment),
    do: Types.alignment_filter_clip_operations(alignment)

  def y_aln_len(alignment),
    do: Types.alignment_y_aln_len(alignment)

  def x_aln_len(alignment),
    do: Types.alignment_x_aln_len(alignment)
end
