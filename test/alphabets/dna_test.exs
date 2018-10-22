defmodule ExBio.Alphabets.DnaTest do
  use ExUnit.Case
  doctest ExBio
  alias ExBio.Alphabets.Dna

  describe "alphabet" do
    test "creates new alphabet reference" do
      {:ok, alphabet} = Dna.alphabet()
      assert is_reference(alphabet) == true
    end
  end
end
