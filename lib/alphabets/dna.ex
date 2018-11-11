defmodule ExBio.Alphabets.Dna do
  alias ExBio.Nif.RustBio

  def alphabet(type \\ :default)

  def alphabet(type) when type in [:default, :iupac, :n],
    do: RustBio.alph_alphabet_alphabet(:dna, type)

  def complement(char),
    do: RustBio.alph_alphabet_complement(:dna, char)

  def revcomp(text),
    do: RustBio.alph_alphabet_revcomp(:dna, text)
end
