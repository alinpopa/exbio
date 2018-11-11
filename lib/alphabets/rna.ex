defmodule ExBio.Alphabets.Rna do
  alias ExBio.Nif.RustBio

  def alphabet(type \\ :default)

  def alphabet(type) when type in [:default, :iupac, :n],
    do: RustBio.alph_alphabet_alphabet(:rna, type)

  def complement(char),
    do: RustBio.alph_alphabet_complement(:rna, char)

  def revcomp(text),
    do: RustBio.alph_alphabet_revcomp(:rna, text)
end
