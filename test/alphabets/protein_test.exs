defmodule ExBio.Alphabets.ProteinTest do
  use ExUnit.Case
  doctest ExBio
  alias ExBio.Alphabets.{Protein, Alphabet}

  describe "alphabet" do
    test "creates new default alphabet reference" do
      {:ok, alphabet} = Protein.alphabet()
      assert is_reference(alphabet) == true
    end
  end

  describe "is_word" do
    test "created protein has word" do
      {:ok, alphabet} = Protein.alphabet()
      assert Alphabet.is_word(alphabet, "DEQsga") == true
    end

    test "created protein is missing word" do
      {:ok, alphabet} = Protein.alphabet()
      assert Alphabet.is_word(alphabet, "BzJ") == false
    end
  end
end
