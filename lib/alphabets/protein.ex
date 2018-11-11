defmodule ExBio.Alphabets.Protein do
  alias ExBio.Nif.RustBio

  def alphabet,
    do: RustBio.alph_alphabet_alphabet(:protein, nil)
end
