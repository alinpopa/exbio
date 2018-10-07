defmodule ExBio.Types.Alignment do
  alias ExBio.Nif.RustBio

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
    case RustBio.types_alignment_new(alignment) do
      {:ok, ref} when is_reference(ref) ->
        {:ok, ref}
      _err ->
        {:error, :invalid_args}
    end
  end
end