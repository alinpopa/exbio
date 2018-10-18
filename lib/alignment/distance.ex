defmodule ExBio.Alignment.Distance do
  alias ExBio.Nif.Alignment

  def hamming(x, y), do: Alignment.dist_hdist(x, y)

  def levenshtein(x, y), do: Alignment.dist_ldist(x, y)
end
