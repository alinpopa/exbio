defmodule ExBio.Alignment.DistanceTest do
  use ExUnit.Case
  doctest ExBio

  describe "hamming" do
    test "valid args" do
      assert ExBio.Alignment.Distance.hamming("ABCBD", "ABCBD") == {:ok, 0}
    end

    test "different length" do
      assert ExBio.Alignment.Distance.hamming("ABCBD", "ABCBDE") == {:error, :invalid_args}
    end
  end

  describe "levenshtein" do
    test "valid args" do
      assert ExBio.Alignment.Distance.levenshtein("ABCBD", "ABCBD") == {:ok, 0}
    end
  end
end
