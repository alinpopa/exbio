defmodule ExBio.Alignment.DistanceTest do
  use ExUnit.Case
  doctest ExBio

  test "hamming" do
    assert ExBio.Alignment.Distance.hamming("ABCBD", "ABCBD") == {:ok, 0}
  end

  test "levenshtein" do
    assert ExBio.Alignment.Distance.levenshtein("ABCBD", "ABCBD") == {:ok, 0}
  end
end
