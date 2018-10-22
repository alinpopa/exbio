defmodule ExBio.Alphabets.Alphabet do
  alias ExBio.Nif.RustBio

  def new(text),
    do: RustBio.alph_alphabet_new(text)

  def insert(alphabet, char),
    do: RustBio.alph_alphabet_insert(alphabet, char)

  def is_word(alphabet, text),
    do: RustBio.alph_alphabet_is_word(alphabet, text)

  def len(alphabet),
    do: RustBio.alph_alphabet_len(alphabet)
end
